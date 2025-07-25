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
//! It relies on a snarkOS with the `test_network` feature. If snarkos is not installed,
//! it will be installed. If snarkos is installed, it will use the existing version
//! (which may or may not be the correct one).

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

    Command::new("pwd").status().unwrap();
    Command::new("ls").arg("-a").status().unwrap();

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
        Command::new("diff")
            .arg(test.expectation_directory.display().to_string())
            .arg(test.mismatch_directory.display().to_string())
            .status()
            .unwrap();

        false
    }
}

/// Replace strings in the stdout of a Leo execution that we don't need to match exactly.
fn filter_stdout(data: &str) -> String {
    use regex::Regex;
    let regexes = [
        (Regex::new("  - transaction ID: '[a-zA-Z0-9]*'").unwrap(), " - transaction ID: 'XXXXXX'"),
        (Regex::new("  - fee ID: '[a-zA-Z0-9]*'").unwrap(), " - fee ID: 'XXXXXX'"),
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

struct ChildRaii(std::process::Child);

impl Drop for ChildRaii {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

#[test]
fn integration_tests() {
    if !cfg!(target_os = "macos") {
        println!("Skipping CLI integration tests (they only run on macos).");
        return;
    }

    if !install_snarkos() {
        panic!("Failed to install snarkOS!");
    }

    let rewrite_expectations = !env::var("REWRITE_EXPECTATIONS").unwrap_or_default().trim().is_empty();

    const VALIDATOR_COUNT: usize = 4usize;

    let directory = tempfile::TempDir::new().expect("Failed to create temporary directory.");
    remove_all_in_dir(directory.path()).expect("Should be able to remove.");

    let mut children = Vec::new();
    {
        let _raii = CwdRaii::cwd(directory.path());

        for i in 0..VALIDATOR_COUNT {
            run_snarkos_clean(i).expect("OK");
        }

        for i in 0..VALIDATOR_COUNT {
            let child = run_snarkos_validator(i, VALIDATOR_COUNT).expect("OK");
            children.push(ChildRaii(child));
        }
    }

    // Sleep for a bit to let snarkos get started.
    std::thread::sleep(std::time::Duration::from_secs(60 * 1));

    // Wait until block height 16.
    loop {
        let height = current_height().expect("net");
        if height >= 16 {
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

    std::mem::drop(children);

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
        println!("SOURCE PATH {}", src_path.display());
        println!("DEST PATH {}", dst_path.display());

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

fn snarkos_installed() -> bool {
    Command::new("snarkos")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

const SNARKOS_VERSION: &str = "4.0.1";

fn install_snarkos() -> bool {
    snarkos_installed() || {
        println!("Installing snarkOS!");
        Command::new("cargo")
            .arg("install")
            .arg("snarkos")
            .arg("--version")
            .arg(SNARKOS_VERSION)
            .arg("--features")
            .arg("test_network") // Enable the testing consensus heights.
            .status()
            .is_ok_and(|status| status.success())
    }
}

fn run_snarkos_validator(i: usize, num_validators: usize) -> io::Result<Child> {
    Command::new("snarkos")
        .arg("start")
        .arg("--nodisplay")
        .arg("--network")
        .arg("1")
        .arg("--dev")
        .arg(i.to_string())
        .arg("--allow-external-peers")
        .arg("--dev-num-validators")
        .arg(num_validators.to_string())
        .arg("--validator")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

fn run_snarkos_clean(i: usize) -> io::Result<()> {
    Command::new("snarkos")
        .arg("clean")
        .arg("--network")
        .arg("1")
        .arg("--dev")
        .arg(i.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;
    Ok(())
}

fn current_height() -> Result<usize, anyhow::Error> {
    let height_url = "http://localhost:3030/testnet/block/height/latest";
    let height_str = leo_package::fetch_from_network_plain(height_url)?;
    height_str.parse().map_err(|e| anyhow!("error parsing height: {e}"))
}
