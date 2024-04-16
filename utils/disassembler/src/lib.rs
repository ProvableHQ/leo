// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use snarkvm::{
    prelude::{Itertools, Network, Testnet3},
    synthesizer::program::{CommandTrait, InstructionTrait, Program, ProgramCore},
};
use std::str::FromStr;
type CurrentNetwork = Testnet3;

use leo_ast::{Composite, FunctionStub, Identifier, Mapping, ProgramId, Stub};
use leo_errors::UtilError;

pub fn disassemble<N: Network, Instruction: InstructionTrait<N>, Command: CommandTrait<N>>(
    program: ProgramCore<N, Instruction, Command>,
) -> Stub {
    let program_id = ProgramId::from(program.id());
    Stub {
        imports: program.imports().into_iter().map(|(id, _)| ProgramId::from(id)).collect(),
        stub_id: program_id,
        consts: Vec::new(),
        structs: [
            program
                .structs()
                .iter()
                .map(|(id, s)| (Identifier::from(id).name, Composite::from_snarkvm(s)))
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
            .map(|(id, m)| (Identifier::from(id).name, Mapping::from_snarkvm(m)))
            .collect(),
        functions: [
            program
                .closures()
                .iter()
                .map(|(id, closure)| (Identifier::from(id).name, FunctionStub::from_closure(closure)))
                .collect_vec(),
            program
                .functions()
                .iter()
                .map(|(id, function)| {
                    (Identifier::from(id).name, FunctionStub::from_function_core(function, program_id.name.name))
                })
                .collect_vec(),
        ]
        .concat(),
        span: Default::default(),
    }
}

pub fn disassemble_from_str(program: &str) -> Result<Stub, UtilError> {
    match Program::<CurrentNetwork>::from_str(program) {
        Ok(p) => Ok(disassemble(p)),
        Err(_) => Err(UtilError::snarkvm_parsing_error(Default::default())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::symbol::create_session_if_not_set_then;
    use snarkvm::{prelude::Testnet3, synthesizer::program::Program};
    use std::fs;

    type CurrentNetwork = Testnet3;

    #[test]
    #[ignore]
    fn credits_test() {
        create_session_if_not_set_then(|_| {
            let program = Program::<CurrentNetwork>::credits();
            match program {
                Ok(p) => {
                    let disassembled = disassemble(p);
                    println!("{}", disassembled);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        });
    }
    #[test]
    #[ignore]
    fn array_test() {
        create_session_if_not_set_then(|_| {
            let program_from_file =
                fs::read_to_string("../tmp/.aleo/registry/testnet3/zk_bitwise_stack_v0_0_2.aleo").unwrap();
            let _program = disassemble_from_str(&program_from_file).unwrap();
        });
    }
}
