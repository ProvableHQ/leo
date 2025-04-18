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

//! This crate deals with Leo packages on the file system and network.
//!
//! The main type is `Package`, which deals with Leo packages on the local filesystem.
//! A Leo package directory is intended to have a structure like this:
//! .
//! ├── .env
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
//! The file `.env` contains definitions for the environment variables NETWORK, PRIVATE_KEY,
//! and ENDPOINT that may be used when deploying or executing the program.
//!
//! Such a directory structure, together with a `.gitignore` file, may be created
//! on the file system using `Package::initialize`.
//! ```no_run
//! # use leo_package::{NetworkName, Package};
//! let path = Package::initialize("my_package", "path/to/parent", NetworkName::TestnetV0, "http://localhost:3030").unwrap();
//! ```
//!
//! `tests` is where unit test files may be placed.
//!
//! Given an existing directory with such a structure, a `Package` may be created from it with
//! `Package::from_directory`:
//! ```no_run
//! # use leo_package::Package;
//! let package = Package::from_directory("path/to/package", "/home/me/.aleo").unwrap();
//! ```
//! This will read the manifest and env file and keep their data in `package.manifest` and `package.env`.
//! It will also process dependencies and store them in topological order in `package.programs`. This processing
//! will involve fetching bytecode from the network for network dependencies.
//!
//! If you want to simply read the manifest and env file without processing dependencies, use
//! `Package::from_directory_no_graph`.
//!
//! `Program` generally doesn't need to be created directly, as `Package` will create `Program`s
//! for the main program and all dependencies. However, if you'd like to fetch bytecode for
//! a program, you can use `Program::fetch`.

#![forbid(unsafe_code)]

use leo_errors::{PackageError, Result, UtilError};
use leo_span::Symbol;

use std::path::Path;

mod dependency;
pub use dependency::*;

mod env;
pub use env::*;

mod location;
pub use location::*;

mod manifest;
pub use manifest::*;

mod network_name;
pub use network_name::*;

mod package;
pub use package::*;

mod program;
pub use program::*;

pub const SOURCE_DIRECTORY: &str = "src";

pub const MAIN_FILENAME: &str = "main.leo";

pub const IMPORTS_DIRECTORY: &str = "build/imports";

pub const OUTPUTS_DIRECTORY: &str = "outputs";

pub const BUILD_DIRECTORY: &str = "build";

pub const TESTS_DIRECTORY: &str = "tests";

pub const TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

fn symbol(name: &str) -> Result<Symbol> {
    name.strip_suffix(".aleo").map(Symbol::intern).ok_or_else(|| PackageError::invalid_network_name(name).into())
}

/// Is this a valid name for an Aleo program?
///
/// Namely, it must be of the format "xxx.aleo" where `xxx` is nonempty,
/// consist solely of ASCII alphanumeric characters and underscore, and
/// begin with a letter.
pub fn is_valid_aleo_name(name: &str) -> bool {
    let Some(rest) = name.strip_suffix(".aleo") else {
        return false;
    };

    // Check that the name is nonempty.
    if rest.is_empty() {
        tracing::error!("Aleo names must be nonempty");
        return false;
    }

    let first = rest.chars().next().unwrap();

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

    // Iterate and check that the name is valid.
    if rest.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        tracing::error!("Aleo names must can only contain ASCII alphanumeric characters and underscores.");
        return false;
    }

    true
}

// Fetch the given endpoint url and return the sanitized response.
pub fn fetch_from_network(url: &str) -> Result<String, UtilError> {
    let response = ureq::AgentBuilder::new()
        .redirects(0)
        .build()
        .get(url)
        .set("X-Leo-Version", env!("CARGO_PKG_VERSION"))
        .call()
        .map_err(|err| UtilError::failed_to_retrieve_from_endpoint(err, Default::default()))?;
    match response.status() {
        200 => Ok(response.into_string().unwrap().replace("\\n", "\n").replace('\"', "")),
        301 => Err(UtilError::endpoint_moved_error(url)),
        _ => Err(UtilError::network_error(url, response.status(), Default::default())),
    }
}

// Verify that a fetched program is valid aleo instructions.
pub fn verify_valid_program(name: &str, program: &str) -> Result<(), UtilError> {
    use snarkvm::prelude::{Program, TestnetV0};
    use std::str::FromStr as _;
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
