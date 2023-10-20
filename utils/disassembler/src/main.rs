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
    console::network::Testnet3,
    prelude::{Itertools, Network},
    synthesizer::program::{CommandTrait, InstructionTrait, Program, ProgramCore},
};
use std::str::FromStr;

use leo_ast::{FunctionStub, Identifier, ProgramId, Struct, Stub};
use leo_span::symbol::create_session_if_not_set_then;

type CurrentNetwork = Testnet3;

fn main() {}

pub fn disassemble<N: Network, Instruction: InstructionTrait<N>, Command: CommandTrait<N>>(
    program: ProgramCore<N, Instruction, Command>,
) -> Stub {
    Stub {
        imports: program.imports().into_iter().map(|(id, _)| ProgramId::from(id)).collect(),
        stub_id: ProgramId::from(program.id()),
        consts: Vec::new(),
        structs: [
            program.structs().iter().map(|(id, s)| (Identifier::from(id).name, Struct::from(s))).collect_vec(),
            program.records().iter().map(|(id, s)| (Identifier::from(id).name, Struct::from(s))).collect_vec(),
        ]
        .concat(),
        mappings: Vec::new(),
        functions: [
            program
                .closures()
                .iter()
                .map(|(id, closure)| (Identifier::from(id).name, FunctionStub::from(closure)))
                .collect_vec(),
            program
                .functions()
                .iter()
                .map(|(id, function)| (Identifier::from(id).name, FunctionStub::from(function)))
                .collect_vec(),
        ]
        .concat(),
        span: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credits_test() {
        create_session_if_not_set_then(|_| {
            let aleo_prog_1 =
                std::fs::read_to_string("/Users/evanschott/work/leo/utils/disassembler/src/tests/credits.aleo")
                    .unwrap();
            let program = Program::<CurrentNetwork>::from_str(&*aleo_prog_1);
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
}
