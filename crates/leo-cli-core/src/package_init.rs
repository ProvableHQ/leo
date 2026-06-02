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

//! Native-only Leo package initialisation — creates the on-disk skeleton
//! for `leo new` / `leo new --library`.
//!
//! Migrated out of `crates/leo-package` (where it used to live as
//! `Package::initialize`) so that crate stays purely wasm-buildable. The
//! body is `#[cfg(not(target_arch = "wasm32"))]`-gated.

#![cfg(not(target_arch = "wasm32"))]

use crate::validation::{is_valid_library_name, is_valid_program_name};
use leo_errors::Result;
use leo_package::{
    LIB_FILENAME,
    MAIN_FILENAME,
    MANIFEST_FILENAME,
    Manifest,
    SOURCE_DIRECTORY,
    TESTS_DIRECTORY,
    cli_invalid_package_name,
    failed_path,
    failed_to_create_source_directory,
    failed_to_initialize_package,
    io_error_gitignore_file,
    util_file_io_error,
};

use std::path::{Path, PathBuf};

/// Create a Leo package by the name `package_name` in a subdirectory of `path`.
pub fn initialize_package<P: AsRef<Path>>(package_name: &str, path: P, is_library: bool) -> Result<PathBuf> {
    initialize_impl(package_name, path.as_ref(), is_library)
}

fn initialize_impl(package_name: &str, path: &Path, is_library: bool) -> Result<PathBuf> {
    let package_name = if is_library {
        if !is_valid_library_name(package_name) {
            return Err(cli_invalid_package_name("library", package_name).into());
        }

        package_name.to_string()
    } else {
        let program_name =
            if package_name.ends_with(".aleo") { package_name.to_string() } else { format!("{package_name}.aleo") };

        if !is_valid_program_name(&program_name) {
            return Err(cli_invalid_package_name("program", &program_name).into());
        }

        program_name
    };

    let path = path.canonicalize().map_err(|e| failed_path(path.display(), e))?;
    let full_path = path.join(package_name.strip_suffix(".aleo").unwrap_or(&package_name));

    if full_path.exists() {
        return Err(failed_to_initialize_package(package_name, &path, "Directory already exists").into());
    }

    std::fs::create_dir(&full_path).map_err(|e| failed_to_initialize_package(&package_name, &full_path, e))?;

    std::env::set_current_dir(&full_path).map_err(|e| failed_to_initialize_package(&package_name, &full_path, e))?;

    const GITIGNORE_TEMPLATE: &str = ".env\n*.avm\n*.prover\n*.verifier\nbuild/\n";
    const GITIGNORE_FILENAME: &str = ".gitignore";
    let gitignore_path = full_path.join(GITIGNORE_FILENAME);
    std::fs::write(gitignore_path, GITIGNORE_TEMPLATE).map_err(io_error_gitignore_file)?;

    let manifest = Manifest {
        program: package_name.clone(),
        version: "0.1.0".to_string(),
        description: String::new(),
        license: "MIT".to_string(),
        leo: env!("CARGO_PKG_VERSION").to_string(),
        dependencies: None,
        dev_dependencies: None,
    };
    let manifest_path = full_path.join(MANIFEST_FILENAME);
    manifest.write_to_file(manifest_path)?;

    let source_path = full_path.join(SOURCE_DIRECTORY);
    std::fs::create_dir(&source_path).map_err(|e| failed_to_create_source_directory(source_path.display(), e))?;

    let name_no_aleo = package_name.strip_suffix(".aleo").unwrap_or(&package_name);

    if is_library {
        let lib_path = source_path.join(LIB_FILENAME);
        std::fs::write(&lib_path, lib_template(name_no_aleo))
            .map_err(|e| util_file_io_error(format_args!("Failed to write `{}`", lib_path.display()), e))?;

        let tests_path = full_path.join(TESTS_DIRECTORY);
        std::fs::create_dir(&tests_path).map_err(|e| failed_to_create_source_directory(tests_path.display(), e))?;

        let test_file_path = tests_path.join(format!("test_{name_no_aleo}.leo"));
        std::fs::write(&test_file_path, lib_test_template(name_no_aleo))
            .map_err(|e| util_file_io_error(format_args!("Failed to write `{}`", test_file_path.display()), e))?;
    } else {
        let main_path = source_path.join(MAIN_FILENAME);
        std::fs::write(&main_path, main_template(name_no_aleo))
            .map_err(|e| util_file_io_error(format_args!("Failed to write `{}`", main_path.display()), e))?;

        let tests_path = full_path.join(TESTS_DIRECTORY);
        std::fs::create_dir(&tests_path).map_err(|e| failed_to_create_source_directory(tests_path.display(), e))?;

        let test_file_path = tests_path.join(format!("test_{name_no_aleo}.leo"));
        std::fs::write(&test_file_path, test_template(name_no_aleo))
            .map_err(|e| util_file_io_error(format_args!("Failed to write `{}`", test_file_path.display()), e))?;
    }

    Ok(full_path)
}

fn main_template(name: &str) -> String {
    format!(
        r#"// The '{name}' program.
program {name}.aleo {{
    // This is the constructor for the program.
    // The constructor allows you to manage program upgrades.
    // It is called when the program is deployed or upgraded.
    // It is currently configured to **prevent** upgrades.
    // Other configurations include:
    //  - @admin(address="aleo1...")
    //  - @checksum(mapping="credits.aleo/fixme", key="0field")
    //  - @custom
    // For more information, please refer to the documentation: `https://docs.leo-lang.org/guides/upgradability`
    @noupgrade
    constructor() {{}}

    fn main(public a: u32, b: u32) -> u32 {{
        let c: u32 = a + b;
        return c;
    }}
}}
"#
    )
}

fn test_template(name: &str) -> String {
    format!(
        r#"// The 'test_{name}' test program.
import {name}.aleo;
program test_{name}.aleo {{
    @test
    @should_fail
    fn test_main_fails() {{
        let result: u32 = {name}.aleo::main(2u32, 3u32);
        assert_eq(result, 3u32);
    }}

    @noupgrade
    constructor() {{}}
}}
"#
    )
}

fn lib_template(name: &str) -> String {
    format!(
        r#"// The '{name}' library.

// Returns the identity of x.
fn example(x: u32) -> u32 {{
    return x;
}}
"#
    )
}

fn lib_test_template(name: &str) -> String {
    format!(
        r#"// The 'test_{name}' test program.
program test_{name}.aleo {{
    @test
    fn test_example() {{
        assert_eq({name}::example(42u32), 42u32);
    }}

    @noupgrade
    constructor() {{}}
}}
"#
    )
}
