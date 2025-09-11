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

use leo_package::{Package, ProgramData};
use leo_span::Symbol;

use indexmap::IndexSet;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Collects paths to Leo source files for each program in the package.
///
/// For each non-test program, it searches the `src` directory for `.leo` files.
/// It separates the `main.leo` file from the rest and returns a tuple:
/// (`main.leo` path, list of other `.leo` file paths).
/// Test programs are included with an empty list of additional files.
/// Programs with bytecode data are ignored.
///
/// # Arguments
/// * `package` - Reference to the package containing programs.
///
/// # Returns
/// A vector of tuples with the main file and other source files.
pub fn collect_leo_paths(package: &Package) -> Vec<(PathBuf, Vec<PathBuf>)> {
    let mut partitioned_leo_paths = Vec::new();
    for program in &package.programs {
        match &program.data {
            ProgramData::SourcePath { directory, source } => {
                if program.is_test {
                    partitioned_leo_paths.push((source.clone(), vec![]));
                } else {
                    let src_dir = directory.join("src");
                    if !src_dir.exists() {
                        continue;
                    }

                    let mut all_files: Vec<PathBuf> = WalkDir::new(&src_dir)
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("leo"))
                        .map(|entry| entry.into_path())
                        .collect();

                    if let Some(index) =
                        all_files.iter().position(|p| p.file_name().and_then(|s| s.to_str()) == Some("main.leo"))
                    {
                        let main = all_files.remove(index);
                        partitioned_leo_paths.push((main, all_files));
                    }
                }
            }
            ProgramData::Bytecode(..) => {}
        }
    }
    partitioned_leo_paths
}

/// Collects paths to `.aleo` files that are external (non-local) dependencies.
///
/// Scans the package's `imports` directory and filters out files that match
/// the names of local source-based dependencies.
/// Only retains `.aleo` files corresponding to true external dependencies.
///
/// # Arguments
/// * `package` - Reference to the package whose imports are being examined.
///
/// # Returns
/// A vector of paths to `.aleo` files not associated with local source dependencies.
pub fn collect_aleo_paths(package: &Package) -> Vec<PathBuf> {
    let local_dependency_symbols: IndexSet<Symbol> = package
        .programs
        .iter()
        .flat_map(|program| match &program.data {
            ProgramData::SourcePath { .. } => {
                // It's a local Leo dependency.
                Some(program.name)
            }
            ProgramData::Bytecode(..) => {
                // It's a network dependency or local .aleo dependency.
                None
            }
        })
        .collect();

    package
        .imports_directory()
        .read_dir()
        .ok()
        .into_iter()
        .flatten()
        .flat_map(|maybe_filename| maybe_filename.ok())
        .filter(|entry| entry.file_type().ok().map(|filetype| filetype.is_file()).unwrap_or(false))
        .flat_map(|entry| {
            let path = entry.path();
            if let Some(filename) = leo_package::filename_no_aleo_extension(&path) {
                let symbol = Symbol::intern(filename);
                if local_dependency_symbols.contains(&symbol) { None } else { Some(path) }
            } else {
                None
            }
        })
        .collect()
}
