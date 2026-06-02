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

//! Backward-compat re-exports. The real implementations live in
//! `leo_cli_core::commands::{util, mod}` so both the CLI and the
//! `leo-wasm` bindings can reach them.

pub use leo_cli_core::commands::{
    LOCAL_PROGRAM_DEFAULT_EDITION,
    format_program_size,
    util::{check_edition_constructor_requirements, parse_input, print_program_source},
};

/// Backward-compat shim: the real `load_extra_programs_into_vm` lives in
/// `leo_cli_core::commands::util` and takes `&Path` for the registry home
/// directly. CLI callers still hand it the typed `Context`; we adapt here.
pub fn load_extra_programs_into_vm<N: snarkvm::prelude::Network>(
    entries: &[String],
    vm: &snarkvm::prelude::VM<N, snarkvm::prelude::store::helpers::memory::ConsensusMemory<N>>,
    context: &crate::cli::context::Context,
    network: leo_ast::NetworkName,
    endpoint: Option<&str>,
    network_retries: u32,
) -> leo_errors::Result<()> {
    leo_cli_core::commands::util::load_extra_programs_into_vm(
        entries,
        vm,
        &context.home()?,
        network,
        endpoint,
        network_retries,
    )
}
