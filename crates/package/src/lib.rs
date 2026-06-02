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
// On `wasm32` the native-only loader + registry items below are unreachable;
// suppress the dead-code lints to keep the wasm build clean.
#![cfg_attr(target_arch = "wasm32", allow(dead_code, unused_imports))]

// Wasm-only shim: alias `leo_ast` as `snarkvm` so `use snarkvm::...` lines
// resolve against the slim re-exports already exposed by `leo-ast`. The
// `snarkvm_wasm` module lives in `leo-ast`; this crate just borrows it.
#[cfg(target_arch = "wasm32")]
extern crate leo_ast as snarkvm;

mod errors;

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

/// Maximum allowed program size in bytes for the given network.
///
/// Forwards to snarkVM's `LATEST_MAX_PROGRAM_SIZE` per-network helper. The
/// three networks happen to share 512_000 today, but they can diverge — this
/// function is the single point of truth so callers never need to guess.
pub fn max_program_size(network: NetworkName) -> usize {
    use snarkvm::prelude::{CanaryV0, MainnetV0, Network, TestnetV0};
    match network {
        NetworkName::MainnetV0 => <MainnetV0 as Network>::LATEST_MAX_PROGRAM_SIZE(),
        NetworkName::TestnetV0 => <TestnetV0 as Network>::LATEST_MAX_PROGRAM_SIZE(),
        NetworkName::CanaryV0 => <CanaryV0 as Network>::LATEST_MAX_PROGRAM_SIZE(),
    }
}

/// The edition of a deployed program on the Aleo network.
/// Edition 0 is the initial deployment, and increments with each upgrade.
pub type Edition = u16;

/// Default edition for local programs during local operations (run, execute, synthesize, build validation).
///
/// Local programs don't have an on-chain edition yet. Edition 1 avoids snarkVM's V8+ check
/// that rejects edition 0 programs without constructors — that check is only relevant for
/// deployed programs, not local development.
pub const LOCAL_PROGRAM_DEFAULT_EDITION: Edition = 1;

/// Threshold (% of the size limit) at which `leo build` warns about a program nearing the limit.
const PROGRAM_SIZE_WARNING_THRESHOLD: usize = 90;

/// Format a program size for human-readable build output.
///
/// Returns `(size_kb, max_kb, warning)` where `warning` is `Some` once `size` exceeds
/// [`PROGRAM_SIZE_WARNING_THRESHOLD`]% of `max_size`.
pub fn format_program_size(size: usize, max_size: usize) -> (f64, f64, Option<String>) {
    let size_kb = size as f64 / 1024.0;
    let max_kb = max_size as f64 / 1024.0;
    let percentage = (size as f64 / max_size as f64) * 100.0;
    let warning = (size > max_size * PROGRAM_SIZE_WARNING_THRESHOLD / 100)
        .then(|| format!("approaching the size limit ({percentage:.1}% of {max_kb:.2} KB)"));
    (size_kb, max_kb, warning)
}

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

// Name validation (uses snarkVM keyword tables) and the registry HTTP
// fetchers (use ureq) are native-only. The module is gated at its
// declaration so the file itself can stay `#[cfg]`-free.
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

pub fn filename_no_leo_extension(path: &Path) -> Option<&str> {
    filename_no_extension(path, ".leo")
}

pub fn filename_no_aleo_extension(path: &Path) -> Option<&str> {
    filename_no_extension(path, ".aleo")
}

fn filename_no_extension<'a>(path: &'a Path, extension: &'static str) -> Option<&'a str> {
    path.file_name().and_then(|os_str| os_str.to_str()).and_then(|s| s.strip_suffix(extension))
}
