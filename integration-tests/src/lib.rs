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

use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

/// Finds the `leo` binary under any `target/<something>` directories relative to current exe.
pub fn leo_binary_path() -> PathBuf {
    let exe_name = if cfg!(windows) { "leo.exe" } else { "leo" };

    let current_exe = env::current_exe().expect("Failed to get current exe path");

    // Walk up until we find a `target` directory
    let mut dir = current_exe.parent().expect("Current exe has no parent").to_path_buf();
    while dir.parent().is_some() {
        let target_dir = dir.join("target");
        if target_dir.is_dir() {
            // Iterate over all entries in `target/`
            if let Ok(entries) = fs::read_dir(&target_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Check `target/<any>/leo`
                        let candidate = path.join(exe_name);
                        if candidate.is_file() {
                            return candidate;
                        }
                    }
                }
            }
        }

        dir = dir.parent().unwrap().to_path_buf();
    }

    panic!("Could not find `leo` binary under any target/<something> directories");
}

pub fn run_leo_test(test_file_path: &std::path::Path) {
    let Some((_tempdir, project_root)) = setup_leo_test_project(test_file_path) else {
        return;
    };

    let leo_path = leo_binary_path();

    let output = std::process::Command::new(&leo_path)
        .arg("test")
        .current_dir(&project_root)
        .output()
        .expect("Failed to run leo test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("{}", stdout);
    println!("{}", stderr);

    // Only treat stderr as a failure if it contains something **other than benign warnings**
    let stderr_has_error = stderr.lines().any(|line| !line.contains("⚠️") && !line.contains("does not exist"));

    if stdout.contains("FAILED") || stdout.contains("❌") || stderr_has_error {
        panic!("Leo test failed: {}", test_file_path.display());
    }
}

// Include the generated Rust tests
include!(concat!(env!("OUT_DIR"), "/generated_leo_tests.rs"));

/// Sets up a temporary Leo test project based on the given test file.
///
/// Example layout:
/// ```text
/// .
/// ├── program.json
/// ├── src/main.leo
/// └── tests/test_simple.leo
/// ```
///
/// Returns the [`TempDir`] handle (so the directory isn’t deleted until dropped)
/// and the path to the project root.
pub fn setup_leo_test_project(test_file_path: &Path) -> Option<(tempfile::TempDir, PathBuf)> {
    // 1. Create a temporary directory for the test context
    let test_context_directory = tempfile::TempDir::new().expect("Failed to create temporary directory.");

    // 2. Prepare directory paths
    let project_root = test_context_directory.path().to_path_buf();
    let src_dir = project_root.join("src");
    let tests_dir = project_root.join("tests");

    // 3. Create necessary directories
    fs::create_dir_all(&src_dir).expect("Failed to create src directory.");
    fs::create_dir_all(&tests_dir).expect("Failed to create tests directory.");

    // 4. Read the test file contents
    let test_file_contents = fs::read_to_string(test_file_path).expect("Failed to read test file.");

    // 5. Split the file into the main program and the test program
    let parts: Vec<&str> = test_file_contents.split("// --- Test --- //").collect();
    if parts.len() != 2 {
        return None;
    }

    let main_program = parts[0].trim();
    let test_program = parts[1].trim();

    // 6. Extract the main program name, e.g., `simple.aleo`
    let program_name_line = main_program
        .lines()
        .find(|line| line.trim_start().starts_with("program "))
        .expect("Could not find program declaration in main program.");
    let program_name = program_name_line
        .trim_start()
        .strip_prefix("program ")
        .and_then(|s| s.split_whitespace().next())
        .expect("Invalid program declaration.");

    // Strip the `.aleo` suffix if present for filenames
    let program_name_base = program_name.strip_suffix(".aleo").unwrap_or(program_name);

    // 7. Write program.json
    let program_json = format!(
        r#"{{
  "program": "{program_name}",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "leo": "3.3.1",
  "dependencies": null,
  "dev_dependencies": null
}}"#
    );
    fs::write(project_root.join("program.json"), program_json).expect("Failed to write program.json.");

    // 8. Write src/main.leo
    fs::write(src_dir.join("main.leo"), format!("{main_program}\n")).expect("Failed to write src/main.leo.");

    // 9. Write tests/test_<program_name>.leo
    let test_file_name = format!("test_{program_name_base}.leo");
    fs::write(tests_dir.join(&test_file_name), format!("{test_program}\n")).expect("Failed to write test file.");

    // Return the temp dir (keeps it alive until dropped) and project root path
    Some((test_context_directory, project_root.to_path_buf()))
}
