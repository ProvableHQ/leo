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

use leo_ast::{AleoProgram, Composite, FunctionStub, Identifier, Mapping, ProgramId};
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
                .map(|(id, s)| (Identifier::from(id).name, Composite::from_snarkvm(s, program_id.name.name)))
                .collect_vec(),
            program
                .records()
                .iter()
                .map(|(id, s)| (Identifier::from(id).name, Composite::from_external_record(s, program_id.name.name)))
                .collect_vec(),
        ]
        .concat(),
        mappings: program
            .mappings()
            .into_iter()
            .map(|(id, m)| (Identifier::from(id).name, Mapping::from_snarkvm(m, program_id.name.name)))
            .collect(),
        functions: [
            program
                .closures()
                .iter()
                .map(|(id, closure)| {
                    (Identifier::from(id).name, FunctionStub::from_closure(closure, program_id.name.name))
                })
                .collect_vec(),
            program
                .functions()
                .iter()
                .map(|(id, function)| {
                    (Identifier::from(id).name, FunctionStub::from_function_core(function, program_id.name.name))
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
                        Some((key_name, FunctionStub::from_finalize(function, key_name, program_id.name.name)))
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
