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

use crate::{AleoConstructor, AleoExpr, AleoReg, CompilerState};

use leo_ast::{Function, Program, ProgramId, Variant};
use leo_span::Symbol;

use snarkvm::prelude::Network;

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::str::FromStr;

pub struct CodeGeneratingVisitor<'a> {
    pub state: &'a CompilerState,
    /// A counter to track the next available register.
    pub next_register: u64,
    /// Reference to the current function.
    pub current_function: Option<&'a Function>,
    /// Mapping of local variables to registers.
    /// Because these are local, we can identify them using only a `Symbol` (i.e. a path is not necessary here).
    pub variable_mapping: IndexMap<Symbol, AleoExpr>,
    /// Mapping of composite names to a tuple containing metadata associated with the name.
    /// The first element of the tuple indicate whether the composite is a record or not.
    pub composite_mapping: IndexMap<Vec<Symbol>, bool>,
    /// Mapping of global identifiers to their associated names.
    /// Because we only allow mappings in the top level program scope at this stage, we can identify them using only a
    /// `Symbol` (i.e. a path is not necessary here currently).
    pub global_mapping: IndexMap<Symbol, AleoExpr>,
    /// The variant of the function we are currently traversing.
    pub variant: Option<Variant>,
    /// A reference to program. This is needed to look up external programs.
    pub program: &'a Program,
    /// The program ID of the current program.
    pub program_id: Option<ProgramId>,
    /// A reference to the finalize caller.
    /// Because `async transition`s  can only appear in the top level program scope at this stage,
    /// it's safe to keep this a `Symbol` (i.e. a path is not necessary).
    pub finalize_caller: Option<Symbol>,
    /// A counter to track the next available label.
    pub next_label: u64,
    /// The depth of the current conditional block.
    pub conditional_depth: u64,
    /// Internal record input registers of the current function.
    /// This is necessary as if we output them, we need to clone them.
    pub internal_record_inputs: IndexSet<AleoExpr>,
}

/// This function checks whether the constructor is well-formed.
/// If an upgrade configuration is provided, it checks that the constructor matches the configuration.
pub(crate) fn check_snarkvm_constructor<N: Network>(actual: &AleoConstructor) -> snarkvm::prelude::Result<()> {
    use snarkvm::synthesizer::program::Constructor as SVMConstructor;
    // Parse the constructor as a snarkVM constructor.
    SVMConstructor::<N>::from_str(actual.to_string().trim())?;

    Ok(())
}

impl CodeGeneratingVisitor<'_> {
    pub(crate) fn next_register(&mut self) -> AleoReg {
        self.next_register += 1;
        AleoReg::R(self.next_register - 1)
    }

    /// Converts a path into a legal Aleo identifier, if possible.
    ///
    /// # Behavior
    /// - If the path is a single valid Leo identifier (`[a-zA-Z][a-zA-Z0-9_]*`), it's returned as-is.
    /// - If the last segment matches `Name::[args]` (e.g. `Vec3::[3, 4]`), it's converted to a legal identifier using hashing.
    /// - If the path has multiple segments, and all segments are valid identifiers except the last one (which may be `Name::[args]`),
    ///   it's hashed using the last segment as base.
    /// - Returns `None` if:
    ///   - The path is empty
    ///   - Any segment other than the last is not a valid identifier
    ///   - The last segment is invalid and not legalizable
    ///
    /// # Parameters
    /// - `path`: A slice of `Symbol`s representing a path to an item.
    ///
    /// # Returns
    /// - `Some(String)`: A valid Leo identifier.
    /// - `None`: If the path is invalid or cannot be legalized.
    pub(crate) fn legalize_path(path: &[Symbol]) -> Option<String> {
        /// Checks if a string is a legal Leo identifier: [a-zA-Z][a-zA-Z0-9_]*
        fn is_legal_identifier(s: &str) -> bool {
            let mut chars = s.chars();
            matches!(chars.next(), Some(c) if c.is_ascii_alphabetic())
                && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
                && s.len() <= 31
        }

        /// Generates a hashed Leo identifier from the full path, using the given base segment.
        fn generate_hashed_name(path: &[Symbol], base: &str) -> String {
            use base62::encode;
            use sha2::{Digest, Sha256};
            use std::fmt::Write;

            let full_path = path.iter().format("::").to_string();

            let mut hasher = Sha256::new();
            hasher.update(full_path.as_bytes());
            let hash = hasher.finalize();

            let hash_number = u64::from_be_bytes(hash[..8].try_into().unwrap());
            let hash_base62 = encode(hash_number);

            let fixed_suffix_len = 2 + hash_base62.len(); // "__" + hash
            let max_ident_len = 31 - fixed_suffix_len;

            let mut result = String::new();
            write!(&mut result, "{base:.max_ident_len$}__{hash_base62}").unwrap();
            result
        }

        let last = path.last()?.to_string();

        // Validate all segments except the last
        if path.len() > 1 && !path[..path.len() - 1].iter().all(|sym| is_legal_identifier(&sym.to_string())) {
            return None;
        }

        // === Case 1: Single, legal identifier ===
        if path.len() == 1 && is_legal_identifier(&last) {
            return Some(last);
        }

        // === Case 2: Storage-generated identifiers ===
        //
        // These occur after lowering storage variables into mappings.
        // They can take the forms:
        //   - `some_var__` (for singleton storage)
        //   - `some_var__len__` (for vector length mapping)
        //
        // Both can exceed 31 chars once the "__" or "__len__" suffix is added,
        // so we truncate the base portion to preserve the suffix and make space
        // for the hash. The truncation lengths below were chosen so that:
        // ```
        // total_length = prefix_len + suffix_len + "__" + hash_len <= 31
        // ```
        // where hash_len = 11.
        if let Some(prefix) = last.strip_suffix("__len__") {
            // "some_very_long_storage_variable__len__"
            //  - Keep at most 13 chars from the prefix
            //  - Produces something like: "some_very_lon__len__CpUbpLTf1Ow"
            let truncated_prefix = &prefix[..13.min(prefix.len())];
            return Some(generate_hashed_name(path, &(truncated_prefix.to_owned() + "__len")));
        }

        if let Some(prefix) = last.strip_suffix("__") {
            // "some_very_long_storage_variable__"
            //  - Keep at most 18 chars from the prefix
            //  - Produces something like: "some_very_long_sto__Hn1pThQeV3"
            let truncated_prefix = &prefix[..18.min(prefix.len())];
            return Some(generate_hashed_name(path, &(truncated_prefix.to_owned() + "__")));
        }

        // === Case 3: Matches special form like `path::to::Name::[3, 4]` ===
        let re = regex::Regex::new(r#"^([a-zA-Z_][\w]*)(?:::\[.*?\])?$"#).unwrap();

        if let Some(captures) = re.captures(&last) {
            let ident = captures.get(1)?.as_str();

            // The produced name here will be of the form: `<last>__AYMqiUeJeQN`.
            return Some(generate_hashed_name(path, ident));
        }

        // === Case 4: Matches special form like `path::to::Name?` (last always ends with `?`) ===
        if last.ends_with("?\"") {
            // Because the last segment of `path` always ends with `?` in case 3, we can guarantee
            // that there will be no conflicts with case 2 (which doesn't allow `?` anywhere).
            //
            // The produced name here will be of the form: `Optional__JZCpIGdQvEZ`.
            // The suffix after the `__` cannot conflict with the suffix in case 2 because of the `?`
            return Some(generate_hashed_name(path, "Optional"));
        }

        // Last segment is neither legal nor matches special pattern
        None
    }
}
