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

//! Native-only network fetch for `CompilationUnit`s.
//!
//! Lifted out of `crates/leo-package` (where it used to live as
//! `CompilationUnit::fetch`) so that crate stays purely wasm-buildable.

#![cfg(not(target_arch = "wasm32"))]

use crate::network::{fetch_from_network, fetch_latest_edition};

use leo_ast::NetworkName;
use leo_errors::Result;
use leo_package::{
    CompilationUnit,
    PackageKind,
    ProgramData,
    failed_to_retrieve_from_endpoint,
    find_cached_edition,
    parse_dependencies_from_aleo,
    util_file_io_error,
};
use leo_span::Symbol;

use indexmap::IndexMap;
use std::path::Path;

/// Fetch a `CompilationUnit` for a network dependency.
///
/// Resolves the edition (cached or via `fetch_latest_edition`), then either
/// reads cached bytecode from `~/.aleo/registry/<network>/<name>/<edition>/`
/// or downloads it via `fetch_from_network`. Writes the result back to the
/// cache and parses the dep set out of the bytecode.
///
/// Signature matches [`leo_package::CompilationUnitFetcher`] (concrete
/// `&Path` rather than generic `AsRef<Path>`) so it can be stored as the
/// fn-pointer field on `NetworkConfig`.
pub fn fetch_compilation_unit(
    name: Symbol,
    edition: Option<u16>,
    home_path: &Path,
    network: NetworkName,
    endpoint: &str,
    no_cache: bool,
    network_retries: u32,
) -> Result<CompilationUnit> {
    // Callers may pass the name with or without the ".aleo" suffix; normalise to bare name
    // here so cache paths and network URLs are constructed consistently.
    let name = Symbol::intern(name.to_string().strip_suffix(".aleo").unwrap_or(&name.to_string()));

    let cache_directory = home_path.join(format!("registry/{network}"));

    let edition = match edition {
        _ if name == Symbol::intern("credits") => 0,
        Some(edition) => edition,
        None if !no_cache => match find_cached_edition(&cache_directory, &name.to_string()) {
            Some(cached_edition) => cached_edition,
            None => fetch_latest_edition(&name.to_string(), endpoint, network, network_retries)?,
        },
        None => fetch_latest_edition(&name.to_string(), endpoint, network, network_retries)?,
    };

    let cache_directory = cache_directory.join(format!("{name}/{edition}"));
    let full_cache_path = cache_directory.join(format!("{name}.aleo"));
    if !cache_directory.exists() {
        std::fs::create_dir_all(&cache_directory)
            .map_err(|err| util_file_io_error(format!("Could not write path {}", cache_directory.display()), err))?;
    }

    let existing_bytecode = match full_cache_path.exists() {
        false => None,
        true => {
            let existing_contents = std::fs::read_to_string(&full_cache_path).map_err(|e| {
                util_file_io_error(format_args!("Trying to read cached file at {}", full_cache_path.display()), e)
            })?;
            Some(existing_contents)
        }
    };

    let bytecode = match (existing_bytecode, no_cache) {
        (Some(bytecode), false) => bytecode,
        (existing, _) => {
            let primary_url = if name == Symbol::intern("credits") {
                format!("{endpoint}/{network}/program/credits.aleo")
            } else {
                format!("{endpoint}/{network}/program/{name}.aleo/{edition}")
            };
            let secondary_url = format!("{endpoint}/{network}/program/{name}.aleo");
            let contents = fetch_from_network(&primary_url, network_retries)
                .or_else(|_| fetch_from_network(&secondary_url, network_retries))
                .map_err(|err| {
                    failed_to_retrieve_from_endpoint(
                        primary_url,
                        format_args!("Failed to fetch program `{name}` from network `{network}`: {err}"),
                    )
                })?;

            if let Some(existing_contents) = existing
                && existing_contents != contents
            {
                println!(
                    "Warning: The cached file at `{}` is different from the one fetched from the network. The cached file will be overwritten.",
                    full_cache_path.display()
                );
            }

            std::fs::write(&full_cache_path, &contents).map_err(|err| {
                util_file_io_error(format_args!("Could not open file `{}`", full_cache_path.display()), err)
            })?;

            contents
        }
    };

    let dependencies = parse_dependencies_from_aleo(name, &bytecode, &IndexMap::new())?;

    Ok(CompilationUnit {
        // Network programs store the name with the ".aleo" suffix (unlike local packages).
        // TODO: unify the invariant so the suffix is always absent.
        name: Symbol::intern(&(name.to_string() + ".aleo")),
        data: ProgramData::Bytecode(bytecode),
        edition: Some(edition),
        dependencies,
        is_local: false,
        kind: PackageKind::Program,
    })
}
