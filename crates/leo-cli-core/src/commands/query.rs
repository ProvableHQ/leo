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

//! Native-only program-fetching query helpers shared across `handle_run`
//! / `handle_execute` / `handle_synthesize` / `handle_upgrade`. Moved out
//! of `crates/leo/src/cli/commands/common/query.rs`.

#![cfg(not(target_arch = "wasm32"))]

use crate::errors;

use leo_ast::NetworkName;
use leo_errors::Result;
use leo_package::ProgramData;
use leo_span::Symbol;

use indexmap::IndexSet;
use snarkvm::prelude::{Network, Program, ProgramID};
use std::{collections::HashMap, path::Path, str::FromStr as _};

/// Loads the latest edition of a program and all its imports from the
/// network, using an iterative DFS. `home_path` is the on-disk registry
/// root (typically `~/.aleo`) the fetcher caches into.
pub fn load_latest_programs_from_network<N: Network>(
    home_path: &Path,
    program_id: ProgramID<N>,
    network: NetworkName,
    endpoint: &str,
    network_retries: u32,
) -> Result<Vec<(Program<N>, Option<u16>)>> {
    use std::collections::HashSet;

    let mut programs = HashMap::new();
    let mut ordered_programs = IndexSet::new();
    let mut stack = vec![(program_id, false)];

    while let Some((current_id, seen)) = stack.pop() {
        if seen {
            ordered_programs.insert(current_id);
        } else {
            if programs.contains_key(&current_id) {
                continue;
            }
            let program = crate::package_fetch::fetch_compilation_unit(
                Symbol::intern(&current_id.name().to_string()),
                None,
                home_path,
                network,
                endpoint,
                true,
                network_retries,
            )
            .map_err(|_| errors::custom(format!("Failed to fetch program source for ID: {current_id}")))?;
            let ProgramData::Bytecode(program_src) = program.data else {
                panic!("Expected bytecode when fetching a remote program");
            };

            let bytecode = Program::<N>::from_str(&program_src)
                .map_err(|_| errors::custom(format!("Failed to parse program source for ID: {current_id}")))?;

            let imports = bytecode.imports().keys().cloned().collect::<HashSet<_>>();

            programs.insert(current_id, (bytecode, program.edition));
            stack.push((current_id, true));
            for import_id in imports {
                stack.push((import_id, false));
            }
        }
    }

    Ok(ordered_programs
        .iter()
        .map(|program_id| programs.remove(program_id).expect("Program not found in cache"))
        .collect())
}
