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

//! This crate deals with Leo packages on the file system and network.
//!
//! The main type is `Package`, which deals with Leo packages on the local filesystem.
//! A Leo package directory is intended to have a structure like this:
//! .
//! ├── program.json
//! ├── build
//! │   ├── imports
//! │   │   └── credits.aleo
//! │   └── main.aleo
//! ├── outputs
//! │   ├── program.TypeChecking.ast
//! │   └── program.TypeChecking.json
//! ├── src
//! │   └── main.leo
//! └── tests
//!     └── test_something.leo
//!
//! The file `program.json` is a manifest containing the program name, version, description,
//! and license, together with information about its dependencies.
//!
//! Such a directory structure, together with a `.gitignore` file, may be created
//! on the file system using `Package::initialize`.
//! ```no_run
//! # use leo_ast::NetworkName;
//! # use leo_package::{Package};
//! let path = Package::initialize("my_package", "path/to/parent", false).unwrap();
//! ```
//!
//! `tests` is where unit test files may be placed.
//!
//! Given an existing directory with such a structure, a `Package` may be created from it with
//! `Package::from_directory`:
//! ```no_run
//! # use leo_ast::NetworkName;
//! use leo_package::Package;
//! let package = Package::from_directory("path/to/package", "/home/me/.aleo", false, false, Some(NetworkName::TestnetV0), Some("http://localhost:3030")).unwrap();
//! ```
//! This will read the manifest and keep their data in `package.manifest`.
//! It will also process dependencies and store them in topological order in `package.compilation_units`. This processing
//! will involve fetching bytecode from the network for network dependencies.
//! If the `no_cache` option (3rd parameter) is set to `true`, the package will not use the dependency cache.
//! The endpoint and network are optional and are only needed if the package has network dependencies.
//!
//! If you want to simply read the manifest file without processing dependencies, use
//! `Package::from_directory_no_graph`.
//!
//! `CompilationUnit` generally doesn't need to be created directly, as `Package` will create `CompilationUnit`s
//! for the main program and all dependencies. However, if you'd like to fetch bytecode for
//! a program, you can use `CompilationUnit::fetch`.

#![forbid(unsafe_code)]

use leo_ast::NetworkName;
use leo_errors::{PackageError, Result, UtilError};
use leo_span::Symbol;

use std::path::Path;

mod dependency;
pub use dependency::*;

mod location;
pub use location::*;

mod manifest;
pub use manifest::*;

mod package;
pub use package::*;

mod compilation_unit;
pub use compilation_unit::*;

pub const SOURCE_DIRECTORY: &str = "src";

pub const MAIN_FILENAME: &str = "main.leo";

pub const LIB_FILENAME: &str = "lib.leo";

pub const IMPORTS_DIRECTORY: &str = "build/imports";

pub const OUTPUTS_DIRECTORY: &str = "outputs";

pub const BUILD_DIRECTORY: &str = "build";

pub const ABI_FILENAME: &str = "abi.json";

pub const TESTS_DIRECTORY: &str = "tests";

/// Maximum allowed program size in bytes.
pub const MAX_PROGRAM_SIZE: usize =
    <snarkvm::prelude::TestnetV0 as snarkvm::prelude::Network>::MAX_PROGRAM_SIZE.last().unwrap().1;

/// The edition of a deployed program on the Aleo network.
/// Edition 0 is the initial deployment, and increments with each upgrade.
pub type Edition = u16;

/// Converts a valid program or library name into a `Symbol`.
///
/// Names must either end with `.aleo` or contain no periods; otherwise an error is returned.
fn symbol(name: &str) -> Result<Symbol> {
    if name.ends_with(".aleo") || !name.contains('.') {
        Ok(Symbol::intern(name))
    } else {
        Err(PackageError::invalid_network_name(name).into())
    }
}

/// Checks whether a string is a valid Aleo program name.
///
/// A valid program name must end with `.aleo` and the base name (without the
/// suffix) must satisfy Aleo package naming rules.
pub fn is_valid_program_name(name: &str) -> bool {
    let Some(rest) = name.strip_suffix(".aleo") else {
        tracing::error!("Program names must end with `.aleo`.");
        return false;
    };

    is_valid_package_name(rest)
}

/// Checks whether a string is a valid Aleo library name.
///
/// Library names must satisfy Aleo package naming rules but do not require
/// a `.aleo` suffix.
pub fn is_valid_library_name(name: &str) -> bool {
    is_valid_package_name(name)
}

/// Checks whether a string satisfies general Aleo package naming rules.
///
/// Names must be nonempty, start with a letter, contain only ASCII alphanumeric
/// characters or underscores, avoid reserved keywords, and not contain "aleo".
fn is_valid_package_name(name: &str) -> bool {
    // Check that the name is nonempty.
    if name.is_empty() {
        tracing::error!("Aleo names must be nonempty");
        return false;
    }

    let first = name.chars().next().unwrap();

    // Check that the first character is not an underscore.
    if first == '_' {
        tracing::error!("Aleo names cannot begin with an underscore");
        return false;
    }

    // Check that the first character is not a number.
    if first.is_numeric() {
        tracing::error!("Aleo names cannot begin with a number");
        return false;
    }

    // Check valid characters.
    if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        tracing::error!("Aleo names can only contain ASCII alphanumeric characters and underscores.");
        return false;
    }

    // Check reserved keywords.
    if reserved_keywords().any(|kw| kw == name) {
        tracing::error!(
            "Aleo names cannot be a SnarkVM reserved keyword. Reserved keywords are: {}.",
            reserved_keywords().collect::<Vec<_>>().join(", ")
        );
        return false;
    }

    // Disallow "aleo"
    if name.contains("aleo") {
        tracing::error!("Aleo names cannot contain the keyword `aleo`.");
        return false;
    }

    true
}

/// Get the list of all reserved and restricted keywords from snarkVM.
/// These keywords cannot be used as program names.
/// See: https://github.com/ProvableHQ/snarkVM/blob/046a2964f75576b2c4afbab9aa9eabc43ceb6dc3/synthesizer/program/src/lib.rs#L192
pub fn reserved_keywords() -> impl Iterator<Item = &'static str> {
    use snarkvm::prelude::{Program, TestnetV0};

    // Flatten RESTRICTED_KEYWORDS by ignoring ConsensusVersion
    let restricted = Program::<TestnetV0>::RESTRICTED_KEYWORDS.iter().flat_map(|(_, kws)| kws.iter().copied());

    Program::<TestnetV0>::KEYWORDS.iter().copied().chain(restricted)
}

/// Creates a configured ureq agent for Leo network requests.
///
/// Disables `http_status_as_error` so 4xx/5xx responses return `Ok(Response)`
/// instead of `Err(StatusCode)`. This preserves response bodies which often
/// contain useful error details from the server.
pub fn create_http_agent() -> ureq::Agent {
    ureq::Agent::config_builder().max_redirects(0).http_status_as_error(false).build().new_agent()
}

/// Retries a fallible network operation with exponential backoff.
///
/// Attempts the operation `retries + 1` times. Delays between attempts are
/// 1 s, 2 s, 4 s, …, capped at 64 s. Returns the result of the last attempt.
///
/// Only use this for idempotent, read-only network calls (GET requests);
/// never use it for state-mutating calls such as transaction broadcasts.
pub fn retry_network_call<T, E: std::fmt::Display>(
    network_retries: u32,
    mut f: impl FnMut() -> std::result::Result<T, E>,
) -> std::result::Result<T, E> {
    let mut result = f();
    for attempt in 1..=network_retries {
        if result.is_ok() {
            break;
        }
        let delay_secs = 2u64.pow(attempt - 1).min(64);
        eprintln!("⚠️  Network request failed, retrying in {delay_secs}s (attempt {attempt}/{network_retries})...");
        std::thread::sleep(std::time::Duration::from_secs(delay_secs));
        result = f();
    }
    result
}

// Fetch the given endpoint url and return the sanitized response.
pub fn fetch_from_network(url: &str, network_retries: u32) -> Result<String, UtilError> {
    fetch_from_network_plain(url, network_retries).map(|s| s.replace("\\n", "\n").replace('\"', ""))
}

pub fn fetch_from_network_plain(url: &str, network_retries: u32) -> Result<String, UtilError> {
    // Retry only on transport-level failures (connection errors, timeouts, etc.).
    // HTTP 3xx/4xx/5xx responses are not retried since they reflect persistent conditions.
    let agent = create_http_agent();
    let mut response = retry_network_call(network_retries, || {
        agent
            .get(url)
            .header("X-Leo-Version", env!("CARGO_PKG_VERSION"))
            .call()
            .map_err(|e| UtilError::failed_to_retrieve_from_endpoint(url, e))
    })?;
    match response.status().as_u16() {
        200..=299 => Ok(response.body_mut().read_to_string().unwrap()),
        301 => Err(UtilError::endpoint_moved_error(url)),
        _ => Err(UtilError::network_error(url, response.status())),
    }
}

/// Fetch the given program from the network and return the program as a string.
// TODO (@d0cd) Unify with `leo_package::CompilationUnit::fetch`.
pub fn fetch_program_from_network(
    name: &str,
    endpoint: &str,
    network: NetworkName,
    network_retries: u32,
) -> Result<String, UtilError> {
    let url = format!("{endpoint}/{network}/program/{name}");
    let program = fetch_from_network(&url, network_retries)?;
    Ok(program)
}

/// Fetch the latest edition of a program from the network.
///
/// Returns the actual latest edition number for the given program.
/// This should be used instead of defaulting to arbitrary edition numbers.
pub fn fetch_latest_edition(
    name: &str,
    endpoint: &str,
    network: NetworkName,
    network_retries: u32,
) -> Result<Edition, UtilError> {
    // Strip the .aleo suffix if present for the URL.
    let name_without_suffix = name.strip_suffix(".aleo").unwrap_or(name);

    let url = format!("{endpoint}/{network}/program/{name_without_suffix}.aleo/latest_edition");
    let contents = fetch_from_network(&url, network_retries)?;
    contents
        .parse::<u16>()
        .map_err(|e| UtilError::failed_to_retrieve_from_endpoint(url, format!("Failed to parse edition as u16: {e}")))
}

// Verify that a fetched program is valid aleo instructions.
pub fn verify_valid_program(name: &str, program: &str) -> Result<(), UtilError> {
    use snarkvm::prelude::{Program, TestnetV0};
    use std::str::FromStr as _;

    // Check if the program size exceeds the maximum allowed limit.
    let program_size = program.len();

    if program_size > MAX_PROGRAM_SIZE {
        return Err(UtilError::program_size_limit_exceeded(name, program_size, MAX_PROGRAM_SIZE));
    }

    // Parse the program to verify it's valid Aleo instructions.
    match Program::<TestnetV0>::from_str(program) {
        Ok(_) => Ok(()),
        Err(_) => Err(UtilError::snarkvm_parsing_error(name)),
    }
}

pub fn filename_no_leo_extension(path: &Path) -> Option<&str> {
    filename_no_extension(path, ".leo")
}

pub fn filename_no_aleo_extension(path: &Path) -> Option<&str> {
    filename_no_extension(path, ".aleo")
}

fn filename_no_extension<'a>(path: &'a Path, extension: &'static str) -> Option<&'a str> {
    path.file_name().and_then(|os_str| os_str.to_str()).and_then(|s| s.strip_suffix(extension))
}
