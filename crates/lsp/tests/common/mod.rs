// Copyright (C) 2019-2026 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Helper for driving the `leo-lsp` binary in integration tests.
///
/// Stderr is drained continuously on a background thread so tests can wait on
/// observable server behavior instead of relying on timing sleeps.
pub(crate) struct TestServer {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    stderr: Arc<Mutex<String>>,
    stderr_reader: Option<JoinHandle<()>>,
}

impl TestServer {
    /// Spawn a `leo-lsp` subprocess with the provided environment overrides.
    pub(crate) fn spawn(extra_env: &[(&str, &str)]) -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_leo-lsp"));
        command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

        for (key, value) in extra_env {
            command.env(key, value);
        }

        let mut child = command.spawn().expect("spawn leo-lsp");
        let stdin = child.stdin.take().expect("child stdin");
        let stdout = BufReader::new(child.stdout.take().expect("child stdout"));
        let stderr = Arc::new(Mutex::new(String::new()));
        let mut stderr_pipe = BufReader::new(child.stderr.take().expect("child stderr"));
        let stderr_buffer = Arc::clone(&stderr);
        let stderr_reader = thread::spawn(move || {
            let mut line = String::new();

            loop {
                line.clear();

                match stderr_pipe.read_line(&mut line) {
                    Ok(0) => break,
                    // Preserve stderr in arrival order so assertions can wait
                    // for concrete server events without blocking stdout reads.
                    Ok(_) => stderr_buffer.lock().expect("lock stderr buffer").push_str(&line),
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(error) => panic!("read stderr: {error}"),
                }
            }
        });

        Self { child, stdin, stdout, stderr, stderr_reader: Some(stderr_reader) }
    }

    /// Send a JSON-RPC request and wait for the next response frame.
    pub(crate) fn request(&mut self, id: i64, method: &str, params: Value) -> Value {
        self.send_message(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));

        self.read_message()
    }

    /// Send a JSON-RPC notification without waiting for a response.
    pub(crate) fn notify(&mut self, method: &str, params: Value) {
        self.send_message(&json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        }));
    }

    /// Wait for the child process to exit and collect stderr output.
    ///
    /// This also joins the background stderr reader so no log output is lost at
    /// shutdown.
    pub(crate) fn finish(mut self) -> (ExitStatus, String) {
        drop(self.stdin);
        let status = self.child.wait().expect("wait for leo-lsp");
        self.stderr_reader.take().expect("stderr reader").join().expect("join stderr reader");
        let stderr = self.stderr.lock().expect("lock stderr buffer").clone();

        (status, stderr)
    }

    /// Wait for stderr to contain a substring before timing out.
    ///
    /// Integration tests use this to synchronize on worker progress and panic
    /// reporting without baking scheduler timing assumptions into the harness.
    pub(crate) fn wait_for_stderr_contains(&self, needle: &str, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;

        while Instant::now() < deadline {
            if self.stderr.lock().expect("lock stderr buffer").contains(needle) {
                return true;
            }

            thread::sleep(Duration::from_millis(10));
        }

        self.stderr.lock().expect("lock stderr buffer").contains(needle)
    }

    /// Return the stderr collected so far without shutting the server down.
    #[allow(dead_code)]
    pub(crate) fn stderr_contents(&self) -> String {
        self.stderr.lock().expect("lock stderr buffer").clone()
    }

    fn send_message(&mut self, value: &Value) {
        // Keep the harness speaking the same framed stdio protocol the real
        // editor transport uses.
        let body = serde_json::to_vec(value).expect("serialize json");
        write!(self.stdin, "Content-Length: {}\r\n\r\n", body.len()).expect("write content length");
        self.stdin.write_all(&body).expect("write message body");
        self.stdin.flush().expect("flush stdin");
    }

    fn read_message(&mut self) -> Value {
        let header = read_header_block(&mut self.stdout);
        let content_length = parse_content_length(&header);

        // Read exactly one response frame so tests stay synchronized with the
        // server's stdout stream.
        let mut body = vec![0_u8; content_length];
        self.stdout.read_exact(&mut body).expect("read message body");
        serde_json::from_slice(&body).expect("deserialize response")
    }
}

fn read_header_block(reader: &mut impl Read) -> String {
    let mut bytes = Vec::new();

    loop {
        let mut byte = [0_u8; 1];
        reader.read_exact(&mut byte).expect("read header byte");
        bytes.push(byte[0]);

        // LSP frames end the HTTP-style header section with a blank CRLF line.
        if bytes.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(bytes).expect("utf8 header block")
}

fn parse_content_length(header_block: &str) -> usize {
    // The harness only needs the mandatory LSP framing header here.
    header_block
        .lines()
        .find_map(|line| line.strip_prefix("Content-Length: "))
        .and_then(|value| value.trim().parse().ok())
        .expect("Content-Length header")
}
