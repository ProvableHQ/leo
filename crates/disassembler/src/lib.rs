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

pub fn disassemble_from_str<N: Network>(name: impl fmt::Display, program: &str) -> Result<AleoProgram, UtilError> {
    match Program::<N>::from_str(program) {
        Ok(p) => Ok(disassemble(p)),
        Err(_) => Err(UtilError::snarkvm_parsing_error(name)),
    }
}

/// Disassembles Aleo bytecode using the snarkVM network selected by `network`.
pub fn disassemble_from_str_for_network(
    name: impl fmt::Display,
    program: &str,
    network: NetworkName,
) -> Result<AleoProgram, UtilError> {
    match network {
        NetworkName::MainnetV0 => disassemble_from_str::<snarkvm::prelude::MainnetV0>(name, program),
        NetworkName::TestnetV0 => disassemble_from_str::<snarkvm::prelude::TestnetV0>(name, program),
        NetworkName::CanaryV0 => disassemble_from_str::<snarkvm::prelude::CanaryV0>(name, program),
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
}
