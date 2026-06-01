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

//! Native-only program-name validation helpers (snarkVM keyword tables +
//! Aleo identifier rules) and bytecode-parse validation.
//!
//! Lives in `leo-cli-core` (rather than `crates/leo-package`) so the
//! package crate stays purely wasm-buildable.
//!
//! On `wasm32-unknown-unknown` callers validate identifiers earlier in the
//! pipeline via `leo-passes::name_validation` and never reach this layer.

#![cfg(not(target_arch = "wasm32"))]

use leo_errors::Backtraced;
use leo_package::{MAX_PROGRAM_SIZE, program_size_limit_exceeded, snarkvm_parsing_error};

/// Compile-time guard: `leo-package`'s literal must match snarkVM's
/// `TestnetV0::MAX_PROGRAM_SIZE`. Native-only because it depends on the
/// snarkVM umbrella.
const _: () = assert!(
    MAX_PROGRAM_SIZE == <snarkvm::prelude::TestnetV0 as snarkvm::prelude::Network>::MAX_PROGRAM_SIZE.last().unwrap().1,
    "MAX_PROGRAM_SIZE drift: update the literal in crates/leo-package/src/lib.rs to match snarkVM `TestnetV0::MAX_PROGRAM_SIZE`",
);

/// Checks whether a string is a valid Aleo program name.
///
/// A valid program name must end with `.aleo` and the base name (without the
/// suffix) must satisfy Aleo package naming rules.
pub fn is_valid_program_name(name: &str) -> bool {
    let Some(rest) = name.strip_suffix(".aleo") else {
        tracing::error!("Program names must end with `.aleo`.");
        return false;
    };
    is_valid_package_name(rest)
}

/// Checks whether a string is a valid Aleo library name.
///
/// Library names must satisfy Aleo package naming rules but do not require
/// a `.aleo` suffix.
pub fn is_valid_library_name(name: &str) -> bool {
    is_valid_package_name(name)
}

/// Checks whether a string satisfies general Aleo package naming rules.
fn is_valid_package_name(name: &str) -> bool {
    if name.is_empty() {
        tracing::error!("Aleo names must be nonempty");
        return false;
    }
    let first = name.chars().next().unwrap();
    if first == '_' {
        tracing::error!("Aleo names cannot begin with an underscore");
        return false;
    }
    if first.is_numeric() {
        tracing::error!("Aleo names cannot begin with a number");
        return false;
    }
    if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        tracing::error!("Aleo names can only contain ASCII alphanumeric characters and underscores.");
        return false;
    }
    if reserved_keywords().any(|kw| kw == name) {
        tracing::error!(
            "Aleo names cannot be a SnarkVM reserved keyword. Reserved keywords are: {}.",
            reserved_keywords().collect::<Vec<_>>().join(", ")
        );
        return false;
    }
    if name.contains("aleo") {
        tracing::error!("Aleo names cannot contain the keyword `aleo`.");
        return false;
    }
    true
}

/// Iterator over snarkVM's reserved + restricted program-name keywords.
/// See: <https://github.com/ProvableHQ/snarkVM/blob/046a2964f75576b2c4afbab9aa9eabc43ceb6dc3/synthesizer/program/src/lib.rs#L192>
pub fn reserved_keywords() -> impl Iterator<Item = &'static str> {
    use snarkvm::prelude::{Program, TestnetV0};
    let restricted = Program::<TestnetV0>::RESTRICTED_KEYWORDS.iter().flat_map(|(_, kws)| kws.iter().copied());
    Program::<TestnetV0>::KEYWORDS.iter().copied().chain(restricted)
}

/// Verify that a fetched program is valid Aleo instructions.
pub fn verify_valid_program(name: &str, program: &str) -> Result<(), Backtraced> {
    use snarkvm::prelude::{Program, TestnetV0};
    use std::str::FromStr as _;

    let program_size = program.len();
    if program_size > MAX_PROGRAM_SIZE {
        return Err(program_size_limit_exceeded(name, program_size, MAX_PROGRAM_SIZE));
    }

    match Program::<TestnetV0>::from_str(program) {
        Ok(_) => Ok(()),
        Err(_) => Err(snarkvm_parsing_error(name)),
    }
}

#[cfg(test)]
mod tests {
    use leo_package::extract_aleo_import_names;
    use snarkvm::prelude::{Program as SvmProgram, TestnetV0};

    #[test]
    fn extract_imports_matches_snarkvm() {
        // Parity check: the inline scanner in `leo-package` produces the same
        // set of import names snarkVM's full bytecode parser would, across
        // the corner cases wasm callers can hit (tab whitespace, inline
        // trailing comments, straddling block comments).
        let corpora: [&str; 4] = [
            // Happy path.
            "import foo.aleo;\nimport bar.aleo;\n\nprogram baz.aleo;\nfunction main:\n    input r0 as u32.public;\n    output r0 as u32.private;\n",
            // Tab whitespace and inline `// …` after a statement.
            "import\tfoo.aleo;\nimport bar.aleo; // tail\n\nprogram baz.aleo;\nfunction main:\n    input r0 as u32.public;\n    output r0 as u32.private;\n",
            // Block comment between imports.
            "import foo.aleo;\n/* between */ import bar.aleo;\nprogram baz.aleo;\nfunction main:\n    input r0 as u32.public;\n    output r0 as u32.private;\n",
            // No imports.
            "program baz.aleo;\nfunction main:\n    input r0 as u32.public;\n    output r0 as u32.private;\n",
        ];
        for src in corpora {
            let svm: SvmProgram<TestnetV0> = src.parse().unwrap_or_else(|e| panic!("invalid .aleo source: {e}\n{src}"));
            let svm_imports: Vec<String> = svm.imports().keys().map(|id| id.to_string()).collect();
            assert_eq!(extract_aleo_import_names(src), svm_imports, "scanner / snarkVM divergence on:\n{src}");
        }
    }
}
