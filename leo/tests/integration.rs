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

/// Runs a single CLI integration test in isolation.
///
/// Sets up a temporary test environment, executes the test COMMANDS,
/// compares outputs against expectations, and reports mismatches.
/// Intended to be invoked by generated per-test `#[test]` functions.
fn run_single_cli_test(test_directory: &Path) {
    if !cfg!(target_family = "unix") {
        return;
    }

    let cli_expectation_directory: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "tests", "expectations", "cli"].iter().collect();

    let mismatch_directory: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "mismatches", "cli"].iter().collect();

    let test = Test {
        test_directory: test_directory.to_path_buf(),
        expectation_directory: cli_expectation_directory.join(test_directory.file_name().unwrap()),
        mismatch_directory: mismatch_directory.join(test_directory.file_name().unwrap()),
    };

    let rewrite_expectations = !std::env::var("REWRITE_EXPECTATIONS").unwrap_or_default().trim().is_empty();

    let mut devnode_process = run_leo_devnode().expect("devnode");
    wait_for_devnode();

    let test_result = run_test(&test, rewrite_expectations);

    #[cfg(unix)]
    unsafe {
        // Kill the entire process group: devnode_process + all its children
        let _ = libc::killpg(devnode_process.id() as i32, libc::SIGTERM);
    }

    let _ = devnode_process.wait();

    if let Some(err) = test_result {
        panic!("FAILED: {}\n{}", test_directory.display(), err);
    }
}

/// Blocks until the local Leo devnode is ready to accept requests.
///
/// Polls the devnode HTTP endpoint until it becomes reachable, then waits
/// for the network to reach the required consensus height. This ensures
/// CLI tests start only after the devnode is fully initialized.
fn wait_for_devnode() {
    let height_url = "http://localhost:3030/testnet/block/height/latest";
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(300);

    loop {
        match leo_package::fetch_from_network_plain(height_url) {
            Ok(_) => break,
            Err(_) if start.elapsed() < timeout => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => panic!("{e}"),
        }
    }

    loop {
        let height = current_height().expect("this should work now that the devnode is ready.");
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

fn run_test(test: &Test, force_rewrite: bool) -> Option<String> {
    let test_context_directory = tempfile::TempDir::new().expect("Failed to create temporary directory.");

    copy_recursively(&test.test_directory, test_context_directory.path()).expect("Failed to copy test directory.");

    let contents_path = test_context_directory.path().join("contents");

    let _raii = CwdRaii::cwd(&contents_path);

    let commands_path = test_context_directory.path().join("COMMANDS");

    let output = Command::new(&commands_path).arg(BINARY_PATH).output().expect("Failed to execute COMMANDS");

    let stdout_path = test_context_directory.path().join("STDOUT");
    let stdout_utf8 = std::str::from_utf8(&output.stdout).expect("stdout should be utf8");
    fs::write(&stdout_path, filter_stdout(stdout_utf8).as_bytes()).expect("Failed to write STDOUT");
    let stderr_path = test_context_directory.path().join("STDERR");
    let stderr_utf8 = std::str::from_utf8(&output.stderr).expect("stderr should be utf8");
    fs::write(&stderr_path, filter_stderr(stderr_utf8).as_bytes()).expect("Failed to write STDERR");

    if force_rewrite {
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
        // These are filtered out since the cache can frequently differ between local and CI runs.
        (Regex::new("Warning: The cached file.*\n").unwrap(), ""),
        (
            Regex::new(r"  • The program '[A-Za-z0-9_]+\.aleo' on the network does not match the local copy.*\n")
                .unwrap(),
            "",
        ),
        (Regex::new(r"  • The program '[A-Za-z0-9_]+\.aleo' does not exist on the network.*\n").unwrap(), ""),
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
fn filter_stderr(data: &str) -> String {
    use regex::Regex;
    use std::borrow::Cow;

    // Match `-->` followed by any path, capture only the filename with line/col
    let path_regex = Regex::new(r"-->\s+.*?/([^/]+\.leo:\d+:\d+)").unwrap();

    let mut cow = Cow::Borrowed(data);
    if let Cow::Owned(s) = path_regex.replace_all(&cow, "--> SOURCE_DIRECTORY/$1") {
        cow = Cow::Owned(s);
    }

    cow.into_owned()
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
/// This function launches a local devnode using the Leo CLI.
fn run_leo_devnode() -> io::Result<Child> {
    let mut leo_devnode_cmd = Command::new(BINARY_PATH);

    leo_devnode_cmd
        .arg("devnode")
        .arg("start")
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

fn current_height() -> Result<usize, anyhow::Error> {
    let height_url = "http://localhost:3030/testnet/block/height/latest".to_string();
    let height_str = leo_package::fetch_from_network_plain(&height_url)?;
    height_str.parse().map_err(|e| anyhow!("error parsing height: {e}"))
}

#[cfg(test)]
mod cli_tests {
    include!(concat!(env!("OUT_DIR"), "/cli_tests.rs"));
}
