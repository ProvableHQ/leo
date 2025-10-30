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
    fs,
    path::{Path, PathBuf},
};

fn main() {
    // Relative path to your execution tests
    let exec_dir = PathBuf::from("../tests/tests/execution");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("generated_leo_tests.rs");

    let mut contents = String::new();

    for entry in fs::read_dir(&exec_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "leo").unwrap_or(false) {
            // Sanitize filename for Rust identifiers
            let mut file_name = path.file_stem().unwrap().to_string_lossy().to_string();
            file_name = file_name.replace("-", "_").replace(" ", "_");

            // Absolute path for cross-platform safety
            let abs_path = path.canonicalize().unwrap();
            let path_str = abs_path.to_string_lossy().replace('\\', "\\\\");

            contents.push_str(&format!(
                r#"
#[test]
fn test_{file_name}() {{
    crate::run_leo_test(std::path::Path::new("{}"));
}}
"#,
                path_str
            ));
        }
    }

    fs::write(dest, contents).unwrap();
}
