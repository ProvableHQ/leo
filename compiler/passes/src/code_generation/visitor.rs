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

use leo_ast::{Function, Program, ProgramId, Variant};
use leo_span::Symbol;

use snarkvm::prelude::Network;

use indexmap::{IndexMap, IndexSet};
use std::str::FromStr;

pub struct CodeGeneratingVisitor<'a> {
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
    /// Internal record input registers of the current function.
    /// This is necessary as if we output them, we need to clone them.
    pub internal_record_inputs: IndexSet<String>,
}

/// This function checks whether the constructor is well-formed.
/// If an upgrade configuration is provided, it checks that the constructor matches the configuration.
pub(crate) fn check_snarkvm_constructor<N: Network>(actual: &str) -> snarkvm::prelude::Result<()> {
    use snarkvm::synthesizer::program::Constructor as SVMConstructor;
    // Parse the constructor as a snarkVM constructor.
    SVMConstructor::<N>::from_str(actual.trim())?;

    Ok(())
}

impl CodeGeneratingVisitor<'_> {
    pub(crate) fn next_register(&mut self) -> String {
        self.next_register += 1;
        format!("r{}", self.next_register - 1)
    }

    /// Legalize a struct name. If it's already legal, then just keep it as is. For now, this
    /// expects two possible struct name formats:
    /// - Names that are generated for const generic structs during monomorphization such as: `Foo::[1u32, 2u32]`. These
    ///   names are modified according to `transform_generic_struct_name`.
    /// - All other names which are assumed to be legal (this is not really checked but probably should be).
    pub(crate) fn legalize_struct_name(input: String) -> String {
        Self::transform_generic_struct_name(&input).unwrap_or(input)
    }

    /// Given a struct name as a `&str`, transform it into a legal name if the it happens to be a const generic struct.
    /// For example, if the name of the struct is `Foo::[1u32, 2u32]`, then transform it to `Foo__90ViPfqSIPb`. The
    /// suffix is computed using the hash of the string `Foo::[1u32, 2u32]`
    fn transform_generic_struct_name(input: &str) -> Option<String> {
        use base62::encode;
        use regex::Regex;
        use sha2::{Digest, Sha256};

        // Match format like: foo::[1, 2]
        let re = Regex::new(r#"^([a-zA-Z_][\w]*)::\[(.*?)\]$"#).unwrap();
        let captures = re.captures(input)?;

        let ident = captures.get(1)?.as_str();

        // Compute SHA-256 hash of the entire input
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();

        // Take first 8 bytes and encode as base62
        let hash_prefix = &hash[..8]; // 4 bytes = 32 bits
        let hash_number = u64::from_be_bytes([
            hash_prefix[0],
            hash_prefix[1],
            hash_prefix[2],
            hash_prefix[3],
            hash_prefix[4],
            hash_prefix[5],
            hash_prefix[6],
            hash_prefix[7],
        ]);
        let hash_base62 = encode(hash_number);

        // Format: <truncated_ident>__<hash>
        let fixed_suffix_len = 2 + hash_base62.len(); // __ + hash
        let max_ident_len = 31 - fixed_suffix_len;

        Some(format!("{ident:.max_ident_len$}__{hash_base62}"))
    }
}
