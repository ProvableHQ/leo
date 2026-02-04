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

use std::{env, fs, path::PathBuf};

/// Build script that generates one Rust `#[test]` per `.leo` execution test.
///
/// Scans the `tests/tests/execution` directory for `.leo` files and emits
/// corresponding Rust test functions that call `run_single_test`.
/// The generated tests are written to `$OUT_DIR/execution_tests.rs` and
/// included at compile time via `include!`.
///
/// This allows each Leo test to appear as an individual Rust test, enabling
/// fine-grained filtering, parallel execution, and clearer failure reporting.
fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests_dir = manifest_dir.join("../../tests/tests/execution");

    let mut out = String::new();

    for entry in walkdir::WalkDir::new(&tests_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("leo"))
    {
        let path = entry.path();
        let rel = path.strip_prefix(&tests_dir).unwrap();
        let fn_name = format!("execution_{}", rel.to_string_lossy().replace(['/', '.'], "_"));

        out.push_str(&format!(
            r#"
#[test]
fn {fn_name}() {{
    leo_test_framework::run_single_test(
        "execution",
        std::path::Path::new(r"{path}"),
        crate::test_execution::execution_runner,
    );
}}
"#,
            fn_name = fn_name,
            path = path.display(),
        ));
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("execution_tests.rs"), out).unwrap();
}
