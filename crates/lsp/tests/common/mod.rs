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

use crossbeam_channel::{Receiver, RecvTimeoutError, unbounded};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStdin, Command, ExitStatus, Stdio},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Helper for driving the `leo-lsp` binary in integration tests.
///
/// Stdout and stderr are drained continuously on dedicated background threads
/// so tests can wait on observable server behavior with proper timeouts. PR 6
/// added the stdout reader thread because the server now sends notifications
/// (notably `textDocument/publishDiagnostics`) independently of any request;
/// the reader pushes parsed JSON frames into a channel so tests can wait for
/// notifications without blocking on a frame that may never come.
pub(crate) struct TestServer {
    child: Child,
    stdin: ChildStdin,
    stdout_rx: Receiver<Value>,
    stdout_reader: Option<JoinHandle<()>>,
    pending_notifications: VecDeque<Value>,
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
        let mut stdout = BufReader::new(child.stdout.take().expect("child stdout"));
        let (stdout_tx, stdout_rx) = unbounded::<Value>();
        let stdout_reader = thread::spawn(move || {
            // Best-effort frame read; on EOF or malformed input we exit the
            // loop and drop the sender, which makes subsequent `recv_timeout`
            // calls return `Disconnected` so tests can assert clean shutdown
            // without hanging.
            while let Some(header) = try_read_header_block(&mut stdout) {
                let Some(content_length) = parse_content_length_opt(&header) else {
                    break;
                };
                let mut body = vec![0_u8; content_length];
                if stdout.read_exact(&mut body).is_err() {
                    break;
                }
                let Ok(value) = serde_json::from_slice::<Value>(&body) else {
                    break;
                };
                if stdout_tx.send(value).is_err() {
                    break;
                }
            }
        });
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

        Self {
            child,
            stdin,
            stdout_rx,
            stdout_reader: Some(stdout_reader),
            pending_notifications: VecDeque::new(),
            stderr,
            stderr_reader: Some(stderr_reader),
        }
    }

    /// Send a JSON-RPC request and wait for the matching response.
    ///
    /// Any notifications observed while waiting for the response are stashed
    /// in `pending_notifications` so the test can drain them afterwards. Tests
    /// that need to assert the relative ordering of notifications and
    /// responses must use [`Self::recv_message_kind`] directly.
    pub(crate) fn request(&mut self, id: i64, method: &str, params: Value) -> Value {
        self.send_message(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));
        self.read_response()
    }

    /// Block until the server emits a notification with the requested method.
    ///
    /// Returns `Some` on success and `None` if no matching notification
    /// arrives before `timeout`. Notifications with other methods that arrive
    /// before the target are stashed back into the queue so subsequent calls
    /// can still observe them in arrival order.
    #[allow(dead_code)]
    pub(crate) fn recv_notification(&mut self, method: &str, timeout: Duration) -> Option<Value> {
        let deadline = Instant::now() + timeout;
        // Look at the already-queued notifications first to keep the original
        // ordering intact.
        let mut skipped: VecDeque<Value> = VecDeque::new();
        while let Some(notification) = self.pending_notifications.pop_front() {
            if notification.get("method").and_then(Value::as_str) == Some(method) {
                while let Some(returned) = skipped.pop_back() {
                    self.pending_notifications.push_front(returned);
                }
                return Some(notification);
            }
            skipped.push_back(notification);
        }
        for entry in skipped {
            self.pending_notifications.push_back(entry);
        }

        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return None;
            }
            match self.stdout_rx.recv_timeout(remaining) {
                Ok(message) => {
                    if message.get("id").is_some() {
                        // Unexpected response; stash it for later assertions.
                        self.pending_notifications.push_back(message);
                        continue;
                    }
                    if message.get("method").and_then(Value::as_str) == Some(method) {
                        return Some(message);
                    }
                    self.pending_notifications.push_back(message);
                }
                Err(RecvTimeoutError::Timeout) => return None,
                Err(RecvTimeoutError::Disconnected) => return None,
            }
        }
    }

    /// Drain every notification observed since the last call.
    ///
    /// Useful for assertions that need to inspect the complete notification
    /// stream emitted while a request was in flight.
    #[allow(dead_code)]
    pub(crate) fn take_notifications(&mut self) -> Vec<Value> {
        self.pending_notifications.drain(..).collect()
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
    /// Joins both reader threads so no output is lost at shutdown and the
    /// channels backing `recv_notification` close cleanly.
    pub(crate) fn finish(mut self) -> (ExitStatus, String) {
        drop(self.stdin);
        let status = self.child.wait().expect("wait for leo-lsp");
        self.stdout_reader.take().expect("stdout reader").join().expect("join stdout reader");
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

    /// Read messages until the next response frame, stashing any notifications.
    fn read_response(&mut self) -> Value {
        // Allow up to thirty seconds for a response — generous enough for the
        // slowest CI runs but tight enough that hung subprocesses surface as
        // test failures instead of stalling the suite.
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            let message = match self.stdout_rx.recv_timeout(remaining) {
                Ok(message) => message,
                Err(RecvTimeoutError::Timeout) => panic!("timed out waiting for response from server"),
                Err(RecvTimeoutError::Disconnected) => panic!("server stdout closed without response"),
            };
            if message.get("id").is_some() {
                return message;
            }
            self.pending_notifications.push_back(message);
        }
    }
}

/// Read one LSP-framed header block, returning `None` on EOF or read error.
///
/// Used by the background stdout reader thread, which is also responsible
/// for never panicking on shutdown — a frame ending in EOF is a normal
/// termination signal rather than a test failure.
fn try_read_header_block(reader: &mut impl Read) -> Option<String> {
    let mut bytes = Vec::new();

    loop {
        let mut byte = [0_u8; 1];
        if reader.read_exact(&mut byte).is_err() {
            return None;
        }
        bytes.push(byte[0]);

        if bytes.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(bytes).ok()
}

/// Parse `Content-Length:` out of an LSP header block, returning `None` on
/// malformed input. Used by the background reader thread.
fn parse_content_length_opt(header_block: &str) -> Option<usize> {
    header_block
        .lines()
        .find_map(|line| line.strip_prefix("Content-Length: "))
        .and_then(|value| value.trim().parse().ok())
}
