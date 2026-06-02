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
//! │   ├── my_program
//! │   │   ├── my_program.aleo
//! │   │   └── abi.json
//! │   └── credits
//! │       └── credits.aleo
//! ├── src
//! │   └── main.leo
//! └── tests
//!     └── test_something.leo
//!
//! Inside `build`, every compilation unit - the package's own program or
//! library, its local dependencies, and fetched network imports - gets its own
//! `build/<name>/` directory with the same shape. When compiler-debug AST
//! snapshots are requested they appear under `build/<name>/snapshots/`.
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
//! let package = Package::from_directory("path/to/package", "/home/me/.aleo", false, false, Some(NetworkName::TestnetV0), Some("http://localhost:3030"), 3).unwrap();
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

pub mod errors;
pub use errors::*;

use leo_ast::NetworkName;
use leo_errors::Result;
use leo_span::Symbol;

use std::path::Path;

mod dependency;
pub use dependency::*;

mod location;
pub use location::*;

mod manifest;
pub use manifest::*;

// `Package` is available on every target. Disk-bound entry points use
// `std::fs` (which compiles on wasm32 even though it fails at runtime); the
// callers that need to fetch network deps wire that in via the
// `NetworkConfig::fetcher` fn-pointer.
mod package;
pub use package::*;

mod compilation_unit;
pub use compilation_unit::*;

mod workspace;
pub use workspace::*;

pub const SOURCE_DIRECTORY: &str = "src";

pub const MAIN_FILENAME: &str = "main.leo";

pub const LIB_FILENAME: &str = "lib.leo";

pub const BUILD_DIRECTORY: &str = "build";

pub const ABI_FILENAME: &str = "abi.json";

/// Name of the per-unit subdirectory holding interface ABI JSON files.
pub const INTERFACES_DIRNAME: &str = "interfaces";

/// Name of the per-unit subdirectory holding compiler-debug AST snapshots.
/// Created lazily on first write; absent on builds that don't request snapshots.
pub const SNAPSHOTS_DIRNAME: &str = "snapshots";

pub const TESTS_DIRECTORY: &str = "tests";

/// Maximum allowed program size in bytes.
///
/// Both targets use the same literal — wasm builds can't pull in the full
/// snarkVM umbrella, but the value must agree with the last entry of
/// snarkVM's `TestnetV0::MAX_PROGRAM_SIZE`.
/// `leo_cli_core` carries a native-only compile-time assertion against
/// the snarkVM constant to catch drift.
pub const MAX_PROGRAM_SIZE: usize = 512_000;

/// The edition of a deployed program on the Aleo network.
/// Edition 0 is the initial deployment, and increments with each upgrade.
pub type Edition = u16;

/// Strips a trailing `.aleo` (the Aleo program-ID suffix) from a compilation
/// unit name, yielding the bare name.
///
/// `CompilationUnit` names are bare for local packages but `.aleo`-suffixed for
/// network programs; build paths key on the bare name so the two are unified.
pub fn bare_unit_name(name: &str) -> &str {
    name.strip_suffix(".aleo").unwrap_or(name)
}

/// Converts a valid program or library name into a `Symbol`.
///
/// Names must either end with `.aleo` or contain no periods; otherwise an error is returned.
fn symbol(name: &str) -> Result<Symbol> {
    if name.ends_with(".aleo") || !name.contains('.') {
        Ok(Symbol::intern(name))
    } else {
        Err(crate::errors::invalid_network_name(name).into())
    }
}

// Name validation (`is_valid_program_name`, `reserved_keywords`, …) and
// network helpers (`fetch_from_network`, `fetch_latest_edition`, …) live in
// `leo_cli_core::{validation,network}`. They depend on snarkVM's keyword
// tables and the `ureq` HTTP client respectively — neither belongs in a
// wasm-buildable `crates/leo-package`.

pub fn filename_no_leo_extension(path: &Path) -> Option<&str> {
    filename_no_extension(path, ".leo")
}

pub fn filename_no_aleo_extension(path: &Path) -> Option<&str> {
    filename_no_extension(path, ".aleo")
}

fn filename_no_extension<'a>(path: &'a Path, extension: &'static str) -> Option<&'a str> {
    path.file_name().and_then(|os_str| os_str.to_str()).and_then(|s| s.strip_suffix(extension))
}
