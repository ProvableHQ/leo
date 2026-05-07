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

#[cfg(target_arch = "wasm32")]
extern crate self as snarkvm;

// Preserve this crate's existing `snarkvm::...` imports on WASM without pulling
// in the full native-oriented `snarkvm` dependency graph.
#[cfg(target_arch = "wasm32")]
mod snarkvm_wasm;
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub use snarkvm_wasm::{prelude, synthesizer};

use leo_ast::{AleoProgram, Composite, FunctionStub, Identifier, Mapping, NetworkName, ProgramId};
use leo_errors::UtilError;
use leo_span::Symbol;

use snarkvm::{
    prelude::{Itertools, Network},
    synthesizer::program::{Program, ProgramCore},
};

#[cfg(target_arch = "wasm32")]
use snarkvm::prelude::ValueType;

use std::{fmt, str::FromStr};

pub fn disassemble<N: Network>(program: ProgramCore<N>) -> AleoProgram {
    let program_id = ProgramId::from(program.id());
    AleoProgram {
        imports: program.imports().into_iter().map(|(id, _)| ProgramId::from(id)).collect(),
        stub_id: program_id,
        consts: Vec::new(),
        composites: [
            program
                .structs()
                .iter()
                .map(|(id, s)| (Identifier::from(id).name, Composite::from_snarkvm(s, program_id)))
                .collect_vec(),
            program
                .records()
                .iter()
                .map(|(id, s)| (Identifier::from(id).name, Composite::from_external_record(s, program_id)))
                .collect_vec(),
        ]
        .concat(),
        mappings: program
            .mappings()
            .into_iter()
            .map(|(id, m)| (Identifier::from(id).name, Mapping::from_snarkvm(m, program_id)))
            .collect(),
        functions: [
            program
                .closures()
                .iter()
                .map(|(id, closure)| (Identifier::from(id).name, FunctionStub::from_closure(closure, program_id)))
                .collect_vec(),
            program
                .functions()
                .iter()
                .map(|(id, function)| {
                    (Identifier::from(id).name, FunctionStub::from_function_core(function, program_id))
                })
                .collect_vec(),
            program
                .functions()
                .iter()
                .filter_map(|(id, function)| match function.finalize_logic() {
                    Some(_f) => {
                        let key_name = Symbol::intern(&format!(
                            "finalize/{}",
                            Symbol::intern(&Identifier::from(id).name.to_string())
                        ));
                        Some((key_name, FunctionStub::from_finalize(function, key_name, program_id)))
                    }
                    None => None,
                })
                .collect_vec(),
        ]
        .concat(),
        span: Default::default(),
    }
}

/// Parse-only disassembly. Performs grammar-level checks via `Program::from_str`
/// and converts to the leo AST. Use `disassemble_from_str_validated` (native) if
/// you have a `Process` and want the snarkVM semantic checks that reject the
/// class of malformed-but-parseable bytecode that `disassemble` panics on.
pub fn disassemble_from_str<N: Network>(name: impl fmt::Display, program: &str) -> Result<AleoProgram, UtilError> {
    let p = Program::<N>::from_str(program).map_err(|_| UtilError::snarkvm_parsing_error(&name))?;

    // WASM fallback: native callers should use `disassemble_from_str_validated`,
    // but `snarkvm::prelude::Process` isn't in the wasm dep set (only
    // `snarkvm-synthesizer-program` is shipped to wasm). Guard the one panic
    // site we have a known reproducer for (issue #29399).
    #[cfg(target_arch = "wasm32")]
    for (id, function) in p.functions().iter() {
        for input in function.inputs().iter() {
            if matches!(input.value_type(), ValueType::Future(_) | ValueType::DynamicFuture) {
                return Err(UtilError::snarkvm_validation_error(
                    &name,
                    format!("function `{id}` has a future-typed register input on a non-finalize function"),
                ));
            }
        }
    }

    Ok(disassemble(p))
}

/// Parse, validate via `Process::add_program`, and disassemble. Catches the class
/// of malformed-but-parseable bytecode that `disassemble` panics on (e.g. issue
/// #29399, where a non-finalize function declared a future-typed register input).
/// Pattern matches `crates/compiler/src/test_compiler.rs:62-66`.
///
/// `process` must already have all of `program`'s declared imports loaded —
/// snarkVM's `add_program` is contextual and rejects a program whose imports
/// aren't yet in the process. Callers that disassemble multiple related
/// dependencies should reuse the same process across calls in topological
/// dependency order so each program's imports are present when it's added.
#[cfg(not(target_arch = "wasm32"))]
pub fn disassemble_from_str_validated<N: Network>(
    name: impl fmt::Display,
    program: &str,
    process: &mut snarkvm::prelude::Process<N>,
) -> Result<AleoProgram, UtilError> {
    let p = Program::<N>::from_str(program).map_err(|_| UtilError::snarkvm_parsing_error(&name))?;
    process.add_program(&p).map_err(|e| UtilError::snarkvm_validation_error(&name, e))?;
    Ok(disassemble(p))
}

/// Disassembles Aleo bytecode using the snarkVM network selected by `network`.
/// Native: validates via a fresh `Process` per call. Note the fresh process has
/// no other programs loaded, so this rejects programs with imports — callers
/// that have a typed network at compile time and need import-using programs
/// should prefer `disassemble_from_str_validated` with a shared process loaded
/// in topological order. WASM: parse-only (Process unavailable in the wasm dep
/// set).
pub fn disassemble_from_str_for_network(
    name: impl fmt::Display,
    program: &str,
    network: NetworkName,
) -> Result<AleoProgram, UtilError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        match network {
            NetworkName::MainnetV0 => {
                let mut process = snarkvm::prelude::Process::<snarkvm::prelude::MainnetV0>::load()
                    .map_err(|e| UtilError::snarkvm_validation_error(&name, e))?;
                disassemble_from_str_validated(name, program, &mut process)
            }
            NetworkName::TestnetV0 => {
                let mut process = snarkvm::prelude::Process::<snarkvm::prelude::TestnetV0>::load()
                    .map_err(|e| UtilError::snarkvm_validation_error(&name, e))?;
                disassemble_from_str_validated(name, program, &mut process)
            }
            NetworkName::CanaryV0 => {
                let mut process = snarkvm::prelude::Process::<snarkvm::prelude::CanaryV0>::load()
                    .map_err(|e| UtilError::snarkvm_validation_error(&name, e))?;
                disassemble_from_str_validated(name, program, &mut process)
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        match network {
            NetworkName::MainnetV0 => disassemble_from_str::<snarkvm::prelude::MainnetV0>(name, program),
            NetworkName::TestnetV0 => disassemble_from_str::<snarkvm::prelude::TestnetV0>(name, program),
            NetworkName::CanaryV0 => disassemble_from_str::<snarkvm::prelude::CanaryV0>(name, program),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::create_session_if_not_set_then;
    use snarkvm::synthesizer::program::Program;
    use std::fs;

    type CurrentNetwork = snarkvm::prelude::MainnetV0;

    #[test]
    #[ignore]
    fn credits_test() {
        create_session_if_not_set_then(|_| {
            let program = Program::<CurrentNetwork>::credits();
            match program {
                Ok(p) => {
                    let disassembled = disassemble(p);
                    println!("{disassembled}");
                }
                Err(e) => {
                    println!("{e}");
                }
            }
        });
    }
    #[test]
    #[ignore]
    fn array_test() {
        create_session_if_not_set_then(|_| {
            let program_from_file =
                fs::read_to_string("../tmp/.aleo/registry/mainnet/zk_bitwise_stack_v0_0_2.aleo").unwrap();
            let _program =
                disassemble_from_str::<CurrentNetwork>("zk_bitwise_stack_v0_0_2", &program_from_file).unwrap();
        });
    }

    /// Regression for #29399: a dependency that declares a non-finalize function
    /// with a future-typed register input must be rejected with a clean error
    /// rather than panicking deep in `from_function_core`. Pre-fix this test
    /// would abort the test process with `Functions do not contain futures as inputs`.
    #[test]
    fn rejects_future_typed_register_input_without_panic() {
        create_session_if_not_set_then(|_| {
            let src = include_str!("tests/victim_future_input.aleo");
            let mut process = snarkvm::prelude::Process::<CurrentNetwork>::load().unwrap();
            let result = disassemble_from_str_validated::<CurrentNetwork>("victim", src, &mut process);
            assert!(result.is_err(), "expected disassembler to reject malformed bytecode, got Ok");
        });
    }
}
