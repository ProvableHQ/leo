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

use crate::CompilerState;

use leo_ast::{
    Function,
    Program,
    ProgramId,
    Variant,
    snarkvm_admin_constructor,
    snarkvm_checksum_constructor,
    snarkvm_noupgrade_constructor,
};
use leo_package::UpgradeConfig;
use leo_span::Symbol;

use snarkvm::prelude::{Network, ensure};

use indexmap::IndexMap;
use std::str::FromStr;

pub struct CodeGeneratingVisitor<'a, N: Network> {
    pub state: &'a CompilerState,
    /// A counter to track the next available register.
    pub next_register: u64,
    /// Reference to the current function.
    pub current_function: Option<&'a Function>,
    /// Mapping of variables to registers.
    pub variable_mapping: IndexMap<Symbol, String>,
    /// Mapping of composite names to a tuple containing metadata associated with the name.
    /// The first element of the tuple indicate whether the composite is a record or not.
    /// The second element of the tuple is a string modifier used for code generation.
    pub composite_mapping: IndexMap<Symbol, (bool, String)>,
    /// Mapping of global identifiers to their associated names.
    pub global_mapping: IndexMap<Symbol, String>,
    /// The variant of the function we are currently traversing.
    pub variant: Option<Variant>,
    /// A reference to program. This is needed to look up external programs.
    pub program: &'a Program,
    /// The program ID of the current program.
    pub program_id: Option<ProgramId>,
    /// A reference to the finalize caller.
    pub finalize_caller: Option<Symbol>,
    /// A counter to track the next available label.
    pub next_label: u64,
    /// The depth of the current conditional block.
    pub conditional_depth: u64,
    pub _phantom: std::marker::PhantomData<N>,
}

/// This function checks whether or not the constructor is well-formed.
/// If an upgrade configuration is provided, it checks that the constructor matches the configuration.
pub(crate) fn check_snarkvm_constructor<N: Network>(
    actual: &str,
    upgrade_config: Option<&UpgradeConfig>,
) -> snarkvm::prelude::Result<()> {
    use snarkvm::synthesizer::program::Constructor as SVMConstructor;
    // Parse the constructor as a snarkVM constructor.
    let actual = SVMConstructor::<N>::from_str(actual)?;
    // Parse the expected constructor.
    if let Some(config) = upgrade_config {
        let expected_constructor_string = match &config {
            UpgradeConfig::Admin { address } => snarkvm_admin_constructor(address),
            UpgradeConfig::Checksum { mapping, key } => snarkvm_checksum_constructor(mapping, key),
            UpgradeConfig::NoUpgrade => snarkvm_noupgrade_constructor(),
            UpgradeConfig::Custom => {
                // Return, since there is no expected constructor.
                return Ok(());
            }
        };
        let expected = SVMConstructor::<N>::from_str(&expected_constructor_string)?;
        // Check that the expected constructor matches the given constructor.
        ensure!(actual == expected, "Constructor mismatch: expected {}, got {}", expected, actual)
    }
    Ok(())
}
