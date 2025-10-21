// Copyright (C) 2019-2025 Provable Inc.
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
//! It relies on a snarkOS with the `test_network` feature and uses `leo devnet`
//! to start a local devnet using snarkOS.

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
use snarkvm::prelude::ConsensusVersion;

const VALIDATOR_COUNT: usize = 4usize;

struct Test {
    test_directory: PathBuf,
    expectation_directory: PathBuf,
    mismatch_directory: PathBuf,
}

fn find_tests() -> Vec<Test> {
    let cli_test_directory: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "tests", "cli"].iter().collect::<PathBuf>();
    let cli_expectation_directory: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "tests", "expectations", "cli"].iter().collect::<PathBuf>();
    let mismatch_directory: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "tests", "mismatches", "cli"].iter().collect::<PathBuf>();

    let filter_string = env::var("TEST_FILTER").unwrap_or_default();

    let mut tests = Vec::new();

    for entry in cli_test_directory
        .read_dir()
        .unwrap_or_else(|e| panic!("Failed to read directory {}: {e}", cli_test_directory.display()))
    {
        let entry = entry.unwrap_or_else(|e| panic!("Failed to read directory {}: {e}", cli_test_directory.display()));
        let path = entry.path().canonicalize().expect("Failed to canonicalize");

        let path_str = path.to_str().unwrap_or_else(|| panic!("Path not unicode: {}", path.display()));

        if !path_str.contains(&filter_string) {
            continue;
        }

        let expectation_directory = cli_expectation_directory.join(path.file_name().unwrap());
        let mismatch_directory = mismatch_directory.join(path.file_name().unwrap());

        tests.push(Test { test_directory: path, expectation_directory, mismatch_directory })
    }

    tests
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

fn run_test(test: &Test, force_rewrite: bool) -> bool {
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
    fs::write(&stderr_path, &output.stderr).expect("Failed to write STDERR");

    if force_rewrite {
        copy_recursively(test_context_directory.path(), &test.expectation_directory)
            .expect("Failed to copy directory.");
        true
    } else if dirs_equal(test_context_directory.path(), &test.expectation_directory)
        .expect("Failed to compare directories.")
    {
        true
    } else {
        copy_recursively(test_context_directory.path(), &test.mismatch_directory).expect("Failed to copy directory.");
        false
    }
}

/// Replace strings in the stdout of a Leo execution that we don't need to match exactly.
fn filter_stdout(data: &str) -> String {
    use regex::Regex;
    let regexes = [
        (Regex::new(" - transaction ID: '[a-zA-Z0-9]*'").unwrap(), " - transaction ID: 'XXXXXX'"),
        (Regex::new(" - fee ID: '[a-zA-Z0-9]*'").unwrap(), " - fee ID: 'XXXXXX'"),
        (Regex::new(" - fee transaction ID: '[a-zA-Z0-9]*'").unwrap(), " - fee transaction ID: 'XXXXXX'"),
        (
            Regex::new("💰Your current public balance is [0-9.]* credits.").unwrap(),
            "💰Your current public balance is XXXXXX credits.",
        ),
        (Regex::new("Explored [0-9]* blocks.").unwrap(), "Explored XXXXXX blocks."),
        (Regex::new("Max Variables:        [0-9,]*").unwrap(), "Max Variables:        XXXXXX"),
        (Regex::new("Max Constraints:      [0-9,]*").unwrap(), "Max Constraints:      XXXXXX"),
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

#[test]
fn integration_tests() {
    if !cfg!(target_os = "macos") {
        println!("Skipping CLI integration tests (they only run on macos).");
        return;
    }

    let rewrite_expectations = !env::var("REWRITE_EXPECTATIONS").unwrap_or_default().trim().is_empty();

    let directory = tempfile::TempDir::new().expect("Failed to create temporary directory.");
    remove_all_in_dir(directory.path()).expect("Should be able to remove.");

    let _raii = CwdRaii::cwd(directory.path());

    let mut devnet_process = run_leo_devnet(VALIDATOR_COUNT).expect("OK");

    // Wait for snarkos to start listening on port 3030.
    let height_url = "http://localhost:3030/testnet/block/height/latest";
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(90);

    loop {
        match leo_package::fetch_from_network_plain(height_url) {
            Ok(_) => {
                break;
            }
            Err(_) if start.elapsed() < timeout => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => panic!("snarkos did not start within {timeout:?}: {e}"),
        }
    }

    // Wait until the appropriate block height.
    loop {
        let height = current_height().expect("net");
        if height > ConsensusVersion::latest() as usize {
            break;
        }
        // Avoid rate limits.
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    let tests = find_tests();
    let mut passed = Vec::new();
    let mut failed = Vec::new();
    for test in tests.into_iter() {
        if run_test(&test, rewrite_expectations) {
            passed.push(test);
        } else {
            failed.push(test);
        }
    }

    #[cfg(unix)]
    unsafe {
        // Kill the entire process group: devnet_process + all its children
        libc::killpg(devnet_process.id() as i32, libc::SIGTERM);
    }

    // Wait to reap the main devnet_process (avoid zombie)
    let _ = devnet_process.wait();

    if failed.is_empty() {
        println!("CLI Integration tests: All {} tests passed.", passed.len());
    } else {
        println!("CLI Integration tests: {}/{} tests failed.", failed.len(), failed.len() + passed.len());
        for test in &failed {
            println!(
                "FAILED: {}; produced files written to {}",
                test.test_directory.file_name().unwrap().display(),
                test.mismatch_directory.display()
            );
        }
        panic!()
    }
}

fn copy_recursively(src: &Path, dst: &Path) -> io::Result<()> {
    // Ensure destination directory exists
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
            fs::copy(&src_path, &dst_path)?;
        } else {
            panic!("Unexpected file type at {}", src_path.display())
        }
    }

    Ok(())
}

/// Recursively compares the contents of two directories
fn dirs_equal(dir1: &Path, dir2: &Path) -> io::Result<bool> {
    let entries1 = collect_files(dir1)?;
    let entries2 = collect_files(dir2)?;

    // Check both directories have the same files
    if entries1 != entries2 {
        return Ok(false);
    }

    // Compare contents of each file
    for relative_path in &entries1 {
        let path1 = dir1.join(relative_path);
        let path2 = dir2.join(relative_path);

        let bytes1 = fs::read(&path1)?;
        let bytes2 = fs::read(&path2)?;

        if bytes1 != bytes2 {
            return Ok(false);
        }
    }

    Ok(true)
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

fn remove_all_in_dir(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

/// Starts a Leo devnet for integration testing purposes.
///
/// This function launches a local devnet using the Leo CLI with snarkOS as the backend.
/// The devnet is configured specifically for testing with predefined consensus heights
/// and validators.
fn run_leo_devnet(num_validators: usize) -> io::Result<Child> {
    // Locate the path to the snarkOS binary using the `which` crate
    let snarkos_path: PathBuf = which::which("snarkos").unwrap_or_else(|_| panic!("Cannot find snarkos path.")); // fallback for CI
    assert!(snarkos_path.exists(), "snarkos binary not found at {snarkos_path:?}");

    // Create a new command using the Leo binary (defined by BINARY_PATH constant)
    let mut leo_devnet_cmd = Command::new(BINARY_PATH);

    let consensus_heights: String =
        (0..ConsensusVersion::latest() as usize).map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    // Configure the Leo devnet command with all necessary arguments
    leo_devnet_cmd.arg("devnet")        
        .arg("-y")
        .arg("--snarkos")
        .arg(&snarkos_path)
        .arg("--snarkos-features")
        .arg("test_network") // Use test network configuration
        .arg("--num-validators")
        .arg(num_validators.to_string())
        .arg("--consensus-heights")             // Define consensus heights for testing
        .arg(&consensus_heights)
        .arg("--clear-storage")                 
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // On Unix systems, configure the child process to be its own process group leader
    // This allows us to later kill the entire process group (parent + all children)
    // which is important for properly cleaning up the devnet and all its validator processes
    #[cfg(unix)]
    unsafe {
        leo_devnet_cmd.pre_exec(|| {
            libc::setpgid(0, 0); // make child its own process group leader
            Ok(())
        });
    }

    leo_devnet_cmd.spawn()
}

fn current_height() -> Result<usize, anyhow::Error> {
    let height_url = "http://localhost:3030/testnet/block/height/latest".to_string();
    let height_str = leo_package::fetch_from_network_plain(&height_url)?;
    height_str.parse().map_err(|e| anyhow!("error parsing height: {e}"))
}
