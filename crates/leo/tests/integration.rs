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

//! This code will examine the tests in `tests/tests/cli` and, for each,
//! execute its COMMAND file, comparing the output and resulting directory
//! structure to the corresponding directory in `tests/expectations/cli`.
//!
//! It uses an instance of `leo devnode`.

#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::{
    borrow::Cow,
    collections::HashSet,
    env,
    fs,
    io,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

use anyhow::anyhow;
use snarkvm::prelude::{ConsensusVersion, Network};

struct Test {
    test_directory: PathBuf,
    expectation_directory: PathBuf,
    mismatch_directory: PathBuf,
}

/// Finds an available TCP port by binding to port 0.
fn find_free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port 0").local_addr().unwrap().port()
}

/// Finds a base port where `count` consecutive ports are all available.
fn find_free_port_range(count: u16) -> u16 {
    for _ in 0..100 {
        let base = find_free_port();
        let all_free = (1..count).all(|offset| std::net::TcpListener::bind(("127.0.0.1", base + offset)).is_ok());
        if all_free {
            return base;
        }
    }
    panic!("Could not find {count} consecutive free ports");
}

/// Runs a single CLI integration test in isolation.
///
/// Sets up a temporary test environment, executes the test COMMANDS,
/// compares outputs against expectations, and reports mismatches.
/// Intended to be invoked by generated per-test `#[test]` functions.
fn run_single_cli_test(test_directory: &Path) {
    if !cfg!(target_family = "unix") {
        return;
    }

    // Tests that manage their own infrastructure can place a SKIP_DEVNODE marker file
    // in their directory to prevent the framework from starting an unused devnode.
    let skip_devnode = test_directory.join("SKIP_DEVNODE").exists();

    let cli_expectation_directory: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "..", "..", "tests", "expectations", "cli"].iter().collect();

    let mismatch_directory: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "..", "..", "tests", "mismatches", "cli"].iter().collect();

    let test = Test {
        test_directory: test_directory.to_path_buf(),
        expectation_directory: cli_expectation_directory.join(test_directory.file_name().unwrap()),
        mismatch_directory: mismatch_directory.join(test_directory.file_name().unwrap()),
    };

    let rewrite_expectations = !std::env::var("UPDATE_EXPECT").unwrap_or_default().trim().is_empty();

    // Only start devnode for tests that don't manage their own infrastructure.
    let port = find_free_port();
    let mut devnode_process = if !skip_devnode {
        let process = run_leo_devnode(port).expect("devnode");
        wait_for_devnode(port);
        Some(process)
    } else {
        None
    };

    let test_result = run_test(&test, rewrite_expectations, port);

    #[cfg(unix)]
    if let Some(ref devnode_process) = devnode_process {
        unsafe {
            // Kill the entire process group: devnode_process + all its children
            let _ = libc::killpg(devnode_process.id() as i32, libc::SIGTERM);
        }
    }

    if let Some(ref mut process) = devnode_process {
        let _ = process.wait();
    }

    if let Some(err) = test_result {
        panic!("FAILED: {}\n{}", test_directory.display(), err);
    }
}

/// Blocks until the local Leo devnode is ready to accept requests.
///
/// Polls the devnode HTTP endpoint until it becomes reachable, then waits
/// for the network to reach the required consensus height. This ensures
/// CLI tests start only after the devnode is fully initialized.
fn wait_for_devnode(port: u16) {
    let height_url = format!("http://127.0.0.1:{port}/testnet/block/height/latest");
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(300);

    loop {
        match leo_package::fetch_from_network_plain(&height_url, 2) {
            Ok(_) => break,
            Err(_) if start.elapsed() < timeout => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => panic!("{e}"),
        }
    }

    loop {
        let height = current_height(port).expect("this should work now that the devnode is ready.");
        if snarkvm::prelude::TestnetV0::CONSENSUS_VERSION(height as u32).unwrap() == ConsensusVersion::latest() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

struct CwdRaii {
    previous: PathBuf,
}

impl CwdRaii {
    fn cwd(new: &Path) -> Self {
        let previous = env::current_dir().expect("Can't find current directory.");
        env::set_current_dir(new).expect("Can't change directory.");
        Self { previous }
    }
}

impl Drop for CwdRaii {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.previous);
    }
}

fn run_test(test: &Test, force_rewrite: bool, port: u16) -> Option<String> {
    let test_context_directory = tempfile::TempDir::new().expect("Failed to create temporary directory.");

    copy_recursively(&test.test_directory, test_context_directory.path()).expect("Failed to copy test directory.");

    let contents_path = test_context_directory.path().join("contents");

    let _raii = CwdRaii::cwd(&contents_path);

    let commands_path = test_context_directory.path().join("COMMANDS");

    // Ensure plugin binaries (e.g. `leo-fmt`) in the same target directory as
    // `leo` are discoverable via PATH during integration tests.
    let leo_dir = Path::new(BINARY_PATH).parent().expect("leo binary must reside in a directory");
    let path_with_leo_dir = {
        let orig = env::var_os("PATH").unwrap_or_default();
        let mut dirs = vec![leo_dir.to_path_buf()];
        dirs.extend(env::split_paths(&orig));
        env::join_paths(dirs).expect("failed to join PATH entries")
    };

    // Allocate 12 consecutive ports in one call to avoid range overlap between
    // the three port types (REST, node, BFT) needed by the 4-validator devnet.
    let devnet_base = find_free_port_range(12);
    let output = Command::new(&commands_path)
        .arg(BINARY_PATH)
        .env("PATH", &path_with_leo_dir)
        .env("LEO_DEVNODE_PORT", port.to_string())
        .env("LEO_DEVNET_REST_PORT", devnet_base.to_string())
        .env("LEO_DEVNET_NODE_PORT", (devnet_base + 4).to_string())
        .env("LEO_DEVNET_BFT_PORT", (devnet_base + 8).to_string())
        .output()
        .expect("Failed to execute COMMANDS");

    let stdout_path = test_context_directory.path().join("STDOUT");
    let stdout_utf8 = std::str::from_utf8(&output.stdout).expect("stdout should be utf8");
    fs::write(&stdout_path, filter_stdout(stdout_utf8).as_bytes()).expect("Failed to write STDOUT");
    let stderr_path = test_context_directory.path().join("STDERR");
    let stderr_utf8 = std::str::from_utf8(&output.stderr).expect("stderr should be utf8");
    fs::write(&stderr_path, filter_stderr(stderr_utf8, test_context_directory.path()).as_bytes())
        .expect("Failed to write STDERR");

    let exitcode_path = test_context_directory.path().join("EXITCODE");
    if let Some(code) = output.status.code() {
        fs::write(&exitcode_path, code.to_string().as_bytes()).expect("Failed to write EXITCODE");
    }

    if force_rewrite {
        // Remove stale expectation directory so files deleted between runs don't linger.
        if test.expectation_directory.exists() {
            fs::remove_dir_all(&test.expectation_directory).expect("Failed to remove old expectation directory.");
        }
        copy_recursively(test_context_directory.path(), &test.expectation_directory)
            .expect("Failed to copy directory.");
        None
    } else if let Some(error) =
        dirs_equal(test_context_directory.path(), &test.expectation_directory).expect("Failed to compare directories.")
    {
        Some(error)
    } else {
        copy_recursively(test_context_directory.path(), &test.mismatch_directory).expect("Failed to copy directory.");
        None
    }
}

/// Replace strings in the stdout of a Leo execution that we don't need to match exactly.
fn filter_stdout(data: &str) -> String {
    use regex::Regex;
    let regexes = [
        (Regex::new(" - transaction ID: '[a-zA-Z0-9]*'").unwrap(), " - transaction ID: 'XXXXXX'"),
        (Regex::new(" - fee ID: '[a-zA-Z0-9]*'").unwrap(), " - fee ID: 'XXXXXX'"),
        (Regex::new(" - fee transaction ID: '[a-zA-Z0-9]*'").unwrap(), " - fee transaction ID: 'XXXXXX'"),
        (Regex::new(r#""transaction_id":\s*"[a-zA-Z0-9]*""#).unwrap(), r#""transaction_id": "XXXXXX""#),
        (Regex::new(r#""fee_id":\s*"[a-zA-Z0-9]*""#).unwrap(), r#""fee_id": "XXXXXX""#),
        (Regex::new(r#""fee_transaction_id":\s*"[a-zA-Z0-9]*""#).unwrap(), r#""fee_transaction_id": "XXXXXX""#),
        (Regex::new(r#""address":\s*"aleo1[a-zA-Z0-9]*""#).unwrap(), r#""address": "XXXXXX""#),
        (
            Regex::new("💰Your current public balance is [0-9.]* credits.").unwrap(),
            "💰Your current public balance is XXXXXX credits.",
        ),
        (Regex::new("Explored [0-9]* blocks.").unwrap(), "Explored XXXXXX blocks."),
        // Transaction confirmation can vary between environments (timing-dependent)
        (Regex::new("Transaction rejected\\.").unwrap(), "Could not find the transaction."),
        (Regex::new("Max Variables:        [0-9,]*").unwrap(), "Max Variables:        XXXXXX"),
        (Regex::new("Max Constraints:      [0-9,]*").unwrap(), "Max Constraints:      XXXXXX"),
        // Synthesize command produces checksums and sizes that may vary.
        (Regex::new(r#""prover_checksum":"[a-fA-F0-9]+""#).unwrap(), r#""prover_checksum":"XXXXXX""#),
        (Regex::new(r#""verifier_checksum":"[a-fA-F0-9]+""#).unwrap(), r#""verifier_checksum":"XXXXXX""#),
        (Regex::new(r#""prover_size":[0-9]+"#).unwrap(), r#""prover_size":0"#),
        (Regex::new(r#""verifier_size":[0-9]+"#).unwrap(), r#""verifier_size":0"#),
        (Regex::new(r"- Circuit ID: [a-zA-Z0-9]+").unwrap(), "- Circuit ID: XXXXXX"),
        // `leo clean` output contains absolute temp directory paths; strip the path portion.
        (Regex::new(r"🧹 Cleaned the (build|outputs) directory.*\n").unwrap(), ""),
        // These are filtered out since the cache can frequently differ between local and CI runs.
        (Regex::new("Warning: The cached file.*\n").unwrap(), ""),
        // snarkVM prints parameter download progress to STDOUT on fresh runners,
        // sometimes followed by whitespace-only lines from progress bar artifacts.
        (Regex::new(r"(Installation - .*\n)([ \t\r]+\n)*").unwrap(), ""),
        // snarkVM progress bar artifacts leave whitespace-only lines during deployment transaction creation.
        (Regex::new(r"(📦 Creating deployment transaction for '[^']*'\.\.\.\n\n)([ \t\r]+\n)+").unwrap(), "$1\n"),
        (
            Regex::new(r"  • The program '[A-Za-z0-9_]+\.aleo' on the network does not match the local copy.*\n")
                .unwrap(),
            "",
        ),
        (Regex::new(r"  • The program '[A-Za-z0-9_]+\.aleo' does not exist on the network.*\n").unwrap(), ""),
        // Strip ANSI color codes from `leo fmt --check` diff output.
        (Regex::new(r"\x1b\[[0-9;]*m").unwrap(), ""),
        // `leo fmt --check` outputs absolute paths in diff headers which include temp directories.
        (Regex::new(r"Diff in .*?([^/]+\.leo)").unwrap(), "Diff in SOURCE_DIRECTORY/$1"),
        // Normalize dynamic devnode ports back to 3030 for stable expectations.
        (Regex::new(r"http://localhost:\d+").unwrap(), "http://localhost:3030"),
        // Normalize `leo --version` output: replace commit hash, branch, and features with placeholders.
        (
            Regex::new(r"(leo \d+\.\d+\.\d+) \([a-f0-9]+ [^\)]+\) features=\[[^\]]*\]").unwrap(),
            "$1 (HASH BRANCH) features=[FEATURES]",
        ),
    ];

    let mut cow = Cow::Borrowed(data);
    for (regex, replacement) in regexes {
        if let Cow::Owned(s) = regex.replace_all(&cow, replacement) {
            cow = Cow::Owned(s);
        }
    }

    cow.into_owned()
}

/// Replace strings in the stderr of a Leo execution that we don't need to match exactly.
fn filter_stderr(data: &str, temp_dir: &Path) -> String {
    use regex::Regex;
    use std::borrow::Cow;

    // Rewrite the test's temp directory to a stable `TMPDIR` placeholder. The
    // harness created this directory, so substitute its exact path rather than
    // guessing the shape with a regex - `$TMPDIR` varies by environment.
    let data = redact_temp_dir(data, temp_dir);

    let regexes = [
        // Strip ANSI color codes so downstream regexes match cleanly.
        (Regex::new(r"\x1b\[[0-9;]*m").unwrap(), ""),
        // Match `-->` followed by any path, capture only the filename with line/col
        (Regex::new(r"-->\s+.*?/([^/]+\.leo:\d+:\d+)").unwrap(), "--> SOURCE_DIRECTORY/$1"),
        // Match ariadne's `╭─[ path:line:col ]` header, normalize the path portion.
        (Regex::new(r"╭─\[\s*.*?/([^/]+\.leo:\d+:\d+)\s*\]").unwrap(), "╭─[ SOURCE_DIRECTORY/$1 ]"),
        // Normalize dynamic devnode ports back to 3030 for stable expectations.
        (Regex::new(r"http://localhost:\d+").unwrap(), "http://localhost:3030"),
        // snarkVM prints parameter download warnings to stderr on fresh runners.
        (
            Regex::new(r#"[\r\n]*⚠️  ".*" does not exist\. Downloading and storing it \(in ".*"\)\.[\r\n \t]+"#)
                .unwrap(),
            "",
        ),
        // Strip transient network retry warnings so occasional failures don't break expectations.
        (Regex::new(r"⚠️  Network request failed, retrying in \d+s \(attempt \d+/\d+\)\.\.\.\n").unwrap(), ""),
    ];

    let mut cow = Cow::Borrowed(data.as_str());
    for (regex, replacement) in regexes {
        if let Cow::Owned(s) = regex.replace_all(&cow, replacement) {
            cow = Cow::Owned(s);
        }
    }

    cow.into_owned()
}

/// Rewrite every occurrence of the test's temporary directory in `data` to a
/// stable `TMPDIR` placeholder, so expectations don't depend on where `$TMPDIR`
/// points (it varies by environment - e.g. nix-shell uses `/tmp/nix-shell.XXX`).
///
/// Both the directory as created and its canonicalized form are rewritten: Leo
/// canonicalizes paths before printing them, and the canonical form differs from
/// the created path under a symlinked root (e.g. macOS, where `/var/folders/...`
/// resolves to `/private/var/folders/...`). The canonical form is rewritten
/// first since it can contain the created path as a substring.
fn redact_temp_dir(data: &str, temp_dir: &Path) -> String {
    let mut out = data.to_owned();
    if let Ok(canonical) = temp_dir.canonicalize() {
        out = out.replace(&*canonical.to_string_lossy(), "TMPDIR");
    }
    out.replace(&*temp_dir.to_string_lossy(), "TMPDIR")
}

/// Filter dynamic values in JSON output files to allow comparison across runs.
fn filter_json_file(data: &str) -> String {
    use regex::Regex;

    let regexes = [
        (Regex::new(r#""transaction_id":\s*"[a-zA-Z0-9]*""#).unwrap(), r#""transaction_id": "XXXXXX""#),
        (Regex::new(r#""fee_id":\s*"[a-zA-Z0-9]*""#).unwrap(), r#""fee_id": "XXXXXX""#),
        (Regex::new(r#""fee_transaction_id":\s*"[a-zA-Z0-9]*""#).unwrap(), r#""fee_transaction_id": "XXXXXX""#),
        (Regex::new(r#""address":\s*"aleo1[a-zA-Z0-9]*""#).unwrap(), r#""address": "XXXXXX""#),
        (Regex::new(r#""prover_checksum":\s*"[a-fA-F0-9]+""#).unwrap(), r#""prover_checksum": "XXXXXX""#),
        (Regex::new(r#""verifier_checksum":\s*"[a-fA-F0-9]+""#).unwrap(), r#""verifier_checksum": "XXXXXX""#),
        (Regex::new(r#""circuit_id":\s*"[a-fA-F0-9]+""#).unwrap(), r#""circuit_id": "XXXXXX""#),
        (Regex::new(r#""prover_size":\s*[0-9]+"#).unwrap(), r#""prover_size": 0"#),
        (Regex::new(r#""verifier_size":\s*[0-9]+"#).unwrap(), r#""verifier_size": 0"#),
        // Normalize dynamic devnode ports back to 3030 for stable expectations.
        (Regex::new(r"http://localhost:\d+").unwrap(), "http://localhost:3030"),
    ];

    let mut cow = Cow::Borrowed(data);
    for (regex, replacement) in regexes {
        if let Cow::Owned(s) = regex.replace_all(&cow, replacement) {
            cow = Cow::Owned(s);
        }
    }

    cow.into_owned()
}

const BINARY_PATH: &str = env!("CARGO_BIN_EXE_leo");

fn copy_recursively(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_recursively(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            let in_json_outputs = src_path.components().any(|c| c.as_os_str() == "json-outputs");
            if in_json_outputs && src_path.extension().is_some_and(|ext| ext == "json") {
                let content = fs::read_to_string(&src_path)?;
                let filtered = filter_json_file(&content);
                fs::write(&dst_path, filtered)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        } else {
            panic!("Unexpected file type at {}", src_path.display())
        }
    }

    Ok(())
}

/// Recursively compares the contents of two directories
fn dirs_equal(actual: &Path, expected: &Path) -> io::Result<Option<String>> {
    let entries1 = collect_files(actual)?;
    let entries2 = collect_files(expected)?;

    // Check both directories have the same files
    if entries1 != entries2 {
        return Ok(Some(format!(
            "Directory entries differ:\n  - Actual: {}\n  - Expected: {:?}",
            entries1.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>().join(","),
            entries2.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>().join(",")
        )));
    }

    // Compare contents of each file
    for relative_path in &entries1 {
        let path1 = actual.join(relative_path);
        let path2 = expected.join(relative_path);

        let bytes1 = fs::read(&path1)?;
        let bytes2 = fs::read(&path2)?;

        // Apply filtering to JSON files in json-outputs directory
        let is_json_output = relative_path.to_string_lossy().contains("json-outputs/")
            && relative_path.extension().is_some_and(|ext| ext == "json");

        let (content1, content2) = if is_json_output {
            let s1 = String::from_utf8_lossy(&bytes1);
            let s2 = String::from_utf8_lossy(&bytes2);
            (filter_json_file(&s1).into_bytes(), filter_json_file(&s2).into_bytes())
        } else {
            (bytes1, bytes2)
        };

        if content1 != content2 {
            let actual = String::from_utf8_lossy(&content1);
            let expected = String::from_utf8_lossy(&content2);
            return Ok(Some(format!(
                "File contents differ: {}\n  - Actual: {actual}\n  - Expected: {expected}",
                relative_path.display()
            )));
        }
    }

    Ok(None)
}

/// Collects all file paths relative to the base directory
fn collect_files(base: &Path) -> io::Result<HashSet<PathBuf>> {
    let mut files = HashSet::new();
    for entry in walkdir::WalkDir::new(base).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(base).unwrap().to_path_buf();
            files.insert(rel_path);
        }
    }
    Ok(files)
}

/// Starts a Leo devnode for integration testing purposes.
///
/// This function launches a local devnode using the Leo CLI on the given port.
fn run_leo_devnode(port: u16) -> io::Result<Child> {
    let mut leo_devnode_cmd = Command::new(BINARY_PATH);

    leo_devnode_cmd
        .arg("devnode")
        .arg("start")
        .arg("--socket-addr")
        .arg(format!("127.0.0.1:{port}"))
        .arg("--private-key")
        .arg("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // On Unix systems, configure the child process to be its own process group leader
    #[cfg(unix)]
    unsafe {
        leo_devnode_cmd.pre_exec(|| {
            libc::setpgid(0, 0); // make child its own process group leader
            Ok(())
        });
    }

    leo_devnode_cmd.spawn()
}

fn current_height(port: u16) -> Result<usize, anyhow::Error> {
    let height_url = format!("http://127.0.0.1:{port}/testnet/block/height/latest");
    let height_str = leo_package::fetch_from_network_plain(&height_url, 2)?;
    height_str.parse().map_err(|e| anyhow!("error parsing height: {e}"))
}

#[cfg(unix)]
#[test]
fn execution_approval_refusal_and_private_records() {
    use snarkvm::prelude::{Address, PrivateKey, Program, ProgramID, TestnetV0};
    use std::{
        io::{Read, Write},
        net::TcpListener,
        os::fd::{AsRawFd, FromRawFd},
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        time::{Duration, Instant},
    };

    let directory = tempfile::TempDir::new().expect("The fixture directory must exist");
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/tests/cli/test_dynamic_call_with_flag/contents");
    copy_recursively(&fixture, directory.path()).expect("The existing fixture must copy");
    fs::create_dir(directory.path().join("home")).expect("The isolated cache directory must exist");
    let private_key = "APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh";
    let key: PrivateKey<TestnetV0> = private_key.parse().expect("The fixture key must parse");
    let signer = Address::try_from(&key).expect("The signer address must derive").to_string();
    let program_address = "extra_prog.aleo"
        .parse::<ProgramID<TestnetV0>>()
        .expect("The program ID must parse")
        .to_address()
        .expect("The program address must derive")
        .to_string();
    let recipient = "aleo1qr2ha4pfs5l28aze88yn6fhleeythklkczrule2v838uwj65n5gqxt9djx";
    assert_ne!(signer, program_address);

    let listener = TcpListener::bind("127.0.0.1:0").expect("The endpoint must bind");
    listener.set_nonblocking(true).expect("The listener must support a timeout");
    let endpoint = format!("http://{}", listener.local_addr().expect("The endpoint address must exist"));
    let stop = Arc::new(AtomicBool::new(false));
    let server_stop = Arc::clone(&stop);
    let server = std::thread::spawn(move || {
        let credits =
            serde_json::to_string(&Program::<TestnetV0>::credits().expect("Bundled credits must load").to_string())
                .expect("Credits must serialize");
        let mut requests = Vec::with_capacity(32);
        let malformed = "program extra_prog.aleo;\nfunction main:\n    assert.eq true true;\n\x1b[2J\rwarning sentinel";
        while !server_stop.load(Ordering::Relaxed) {
            let (mut stream, _) = match listener.accept() {
                Ok(connection) => connection,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Err(error) => panic!("The fixture request must arrive: {error}"),
            };
            stream.set_nonblocking(false).expect("The accepted stream must block");
            stream.set_read_timeout(Some(Duration::from_secs(5))).expect("The request timeout must be set");
            let mut request = Vec::new();
            let mut buffer = [0; 1024];
            while !request.windows(4).any(|part| part == b"\r\n\r\n") {
                let count = stream.read(&mut buffer).expect("The request must be readable");
                assert!(count > 0 && request.len() < 8192, "Request headers must be bounded");
                request.extend_from_slice(&buffer[..count]);
            }
            let request = String::from_utf8(request).expect("HTTP headers must be UTF-8");
            let line = request.lines().next().expect("The request line must exist");
            assert!(requests.len() < 64, "The command must not make unbounded requests");
            requests.push(line.to_string());
            let mut words = line.split_whitespace();
            let (status, body) = match (words.next(), words.next()) {
                (Some("GET"), Some("/testnet/program/credits.aleo/latest_edition")) => ("200 OK", "0"),
                (Some("GET"), Some("/testnet/program/credits.aleo"))
                | (Some("GET"), Some("/testnet/program/credits.aleo/0")) => ("200 OK", credits.as_str()),
                (Some("GET"), Some("/testnet/program/extra_prog.aleo")) => ("200 OK", malformed),
                (Some("GET"), Some("/testnet/block/height/latest")) => ("200 OK", "20"),
                (Some("GET"), Some("/testnet/consensus_version")) => ("200 OK", "14"),
                _ => ("400 Bad Request", "Unexpected request after approval was denied"),
            };
            write!(stream, "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
                .expect("The fixture response must be written");
        }
        requests
    });

    // Always stop the endpoint, including when a test assertion fails.
    let result = std::panic::catch_unwind(|| {
        for skip_proof in [false, true] {
            let transaction_directory = directory.path().join("unapproved");
            let json_output = directory.path().join("declined.json");
            let command = |function: &str| {
                let mut command = Command::new(BINARY_PATH);
                command
                    .current_dir(directory.path())
                    .env("NO_COLOR", "1")
                    .env("TERM", "dumb")
                    .env("RAYON_NUM_THREADS", "1")
                    .arg("--home")
                    .arg(directory.path().join("home"))
                    .args([
                        "--disable-update-check",
                        "--path",
                        "./extra_prog.aleo",
                        "execute",
                        function,
                        "--network",
                        "testnet",
                        "--endpoint",
                        &endpoint,
                        "--network-retries",
                        "0",
                        "--private-key",
                        private_key,
                        "--consensus-version",
                        "14",
                        "--print",
                        "--broadcast",
                        "--save",
                    ])
                    .arg(&transaction_directory)
                    .arg(format!("--json-output={}", json_output.display()));
                if skip_proof {
                    command.arg("--skip-execute-proof");
                }
                command
            };
            let mut master = -1;
            let mut slave = -1;
            // SAFETY: Both output pointers are valid; null optional arguments request the default terminal settings.
            assert_eq!(
                unsafe {
                    libc::openpty(
                        &mut master,
                        &mut slave,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    )
                },
                0,
                "The test terminal must open"
            );
            // SAFETY: openpty returned two new descriptors, transferred to these files exactly once.
            let (mut master, slave) = unsafe { (fs::File::from_raw_fd(master), fs::File::from_raw_fd(slave)) };
            // SAFETY: The descriptor belongs to the live master file; O_NONBLOCK only changes read behavior.
            assert_ne!(unsafe { libc::fcntl(master.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) }, -1);
            let mut invocation = command("hidden_transfer");
            invocation
                .stdin(slave.try_clone().expect("The terminal must clone"))
                .stdout(slave.try_clone().expect("The terminal must clone"))
                .stderr(slave);
            // SAFETY: The child only creates its session and selects its inherited terminal before exec.
            unsafe {
                invocation.pre_exec(|| {
                    if libc::setsid() == -1 || libc::ioctl(0, libc::TIOCSCTTY as _, 0) == -1 {
                        return Err(io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
            let mut child = invocation.spawn().expect("The CLI must start with a terminal");
            let mut output = Vec::new();
            let mut refused = false;
            // Keep child cleanup outside assertions that can unwind.
            let terminal_result = (|| -> io::Result<_> {
                let deadline = Instant::now() + Duration::from_secs(60);
                let mut buffer = [0; 8192];
                loop {
                    match master.read(&mut buffer) {
                        Ok(count) => output.extend_from_slice(&buffer[..count]),
                        Err(error)
                            if error.kind() == io::ErrorKind::WouldBlock || error.raw_os_error() == Some(libc::EIO) => {
                        }
                        Err(error) => return Err(error),
                    }
                    if !refused && String::from_utf8_lossy(&output).contains("Approve all calls and fees shown above?")
                    {
                        master.write_all(b"n\n")?;
                        refused = true;
                    }
                    if let Some(status) = child.try_wait()? {
                        while let Ok(count) = master.read(&mut buffer) {
                            if count == 0 {
                                break;
                            }
                            output.extend_from_slice(&buffer[..count]);
                        }
                        return Ok(status);
                    }
                    if Instant::now() >= deadline {
                        return Err(io::Error::new(io::ErrorKind::TimedOut, "The approval prompt did not finish"));
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
            })();
            if terminal_result.is_err() {
                let _ = child.kill();
                let _ = child.wait();
            }
            let status = terminal_result.expect("The terminal execution must finish");
            let output = String::from_utf8(output).expect("The CLI output must be UTF-8").replace("\r\n", "\n");
            assert!(refused && status.success(), "{output}");
            assert!(output.contains("Execution aborted.") && !output.contains("Failed to prompt user"), "{output}");
            let review = output.split("Approve all calls and fees shown above?").next().expect("The review must exist");
            assert!(review.contains("Execution Cost Summary"), "{review}");
            for (function, debit, amount, other_amount) in
                [("transfer_public_as_signer", &signer, 123, 456), ("transfer_public", &program_address, 456, 123)]
            {
                let marker = format!(": credits.aleo/{function}\n");
                let block = review
                    .split_once(&marker)
                    .expect("The expected call must appear")
                    .1
                    .split("\n  Call ")
                    .next()
                    .expect("The call block must exist")
                    .split("\n  Final balances")
                    .next()
                    .expect("The call body must exist");
                for expected in [
                    format!("Signer: {signer}\n"),
                    format!("Debit: {debit}\n"),
                    format!("Recipient: {recipient}\n"),
                    format!("Amount: {amount}u64 microcredits\n"),
                    format!("function_name: {function},"),
                ] {
                    assert!(block.contains(&expected), "Missing {expected:?}: {block}");
                }
                assert!(!block.contains(&format!("{other_amount}u64")), "{block}");
            }
            assert!(review.contains(r"\u{1b}[2J\rwarning sentinel") && !review.contains('\x1b'), "{review}");
            assert!(!transaction_directory.exists(), "Refusal must not write a transaction");
            assert!(
                !output.contains("Printing execution for transaction") && !output.contains("Broadcasting execution"),
                "{output}"
            );
            let json: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&json_output).expect("The command result must exist"))
                    .expect("The command result must parse");
            assert_eq!(
                json,
                serde_json::json!({"program":"", "function":"", "outputs":[], "transaction_id":""}),
                "Refusal may write an empty command result, but must not release a transaction"
            );
            fs::remove_file(&json_output).expect("The empty command result must be removed");

            for (function, input, sentinel) in [
                (
                    "approval_record",
                    format!("{{ owner: {signer}.private, amount: 998714u64.private, _nonce: 0group.public }}"),
                    "998714",
                ),
                (
                    "approval_dynamic_record",
                    format!("{{ owner: {signer}, _root: 998715field, _nonce: 0group, _version: 0u8 }}"),
                    "998715",
                ),
            ] {
                let output_path = directory.path().join("record-output.txt");
                let file = fs::File::create(&output_path).expect("The record output file must open");
                let mut child = command(function)
                    .arg(input)
                    .stdin(Stdio::null())
                    .stdout(file.try_clone().expect("The record output file must clone"))
                    .stderr(file)
                    .spawn()
                    .expect("The record CLI must run");
                let deadline = Instant::now() + Duration::from_secs(60);
                let status = loop {
                    match child.try_wait() {
                        Ok(Some(status)) => break status,
                        Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(5)),
                        result => {
                            let _ = child.kill();
                            let _ = child.wait();
                            panic!("The record CLI did not finish: {result:?}");
                        }
                    }
                };
                let output = fs::read_to_string(output_path).expect("The record output must be readable");
                assert!(!status.success() && output.contains("Failed to prompt user"), "{output}");
                assert!(
                    output.contains(&format!("Call 1: extra_prog.aleo/{function}"))
                        && output.contains("Input 1: <private record>"),
                    "{output}"
                );
                assert!(
                    !output.contains(sentinel) && !output.contains("_nonce:") && !output.contains("_root:"),
                    "{output}"
                );
                assert!(
                    !transaction_directory.exists() && !json_output.exists(),
                    "No transaction or result may be released on prompt failure"
                );
                assert!(!output.contains("Printing execution for transaction"), "{output}");
            }
        }
    });
    stop.store(true, Ordering::Relaxed);
    let requests = server.join().expect("The endpoint must stop");
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
    assert!(requests.iter().any(|request| request == "GET /testnet/program/extra_prog.aleo HTTP/1.1"));
    assert!(
        requests.iter().all(|request| [
            "GET /testnet/program/credits.aleo/latest_edition HTTP/1.1",
            "GET /testnet/program/credits.aleo/0 HTTP/1.1",
            "GET /testnet/program/credits.aleo HTTP/1.1",
            "GET /testnet/program/extra_prog.aleo HTTP/1.1",
            "GET /testnet/block/height/latest HTTP/1.1",
            "GET /testnet/consensus_version HTTP/1.1",
        ]
        .contains(&request.as_str())),
        "Approval denial must not broadcast or query transaction state: {requests:?}"
    );
}

#[cfg(test)]
mod cli_tests {
    include!(concat!(env!("OUT_DIR"), "/cli_tests.rs"));
}
