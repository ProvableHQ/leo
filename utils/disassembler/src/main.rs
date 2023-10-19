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
    prelude::{Itertools, Network, RegisterType, ValueType},
    synthesizer::{
        program::{CommandTrait, InstructionTrait, Program, ProgramCore},
        Command,
        Instruction,
    },
};
use snarkvm::console::program::Identifier as IdentifierCore;
use leo_span::Symbol;
use std::{ops::Add, str::FromStr};

use leo_ast::{Identifier, ProgramId, Struct, Stub};
use leo_ast::Type::Identifier as IdentifierType;

type CurrentNetwork = Testnet3;

fn main() {
    let a = Symbol::intern("aleo");
}

// fn old() {
//     // let credits_aleo =
//     //     std::fs::read_to_string("/Users/evanschott/work/leo/utils/disassembler/src/tests/credits.aleo").unwrap();
//     // println!("{}", code_gen(credits_aleo));
//     let aleo_prog_1 = r"import credits.aleo;
//
// import battleship.aleo;
// import juice.aleo;
//
// program to_parse.aleo;
//
// function new_board_state:
//     input r0 as u64.private;
//     input r1 as address.private;
//     cast self.caller 0u64 0u64 r0 self.caller r1 false into r2 as board_state.record;
//     output r2 as board_state.record;
//
// closure add_up:
//     input r0 as u8;
//     input r1 as u8;
//     add r0 r1 into r2;
//     output r2 as u8;
//
// function transfer_public_to_private:
//     input r0 as address.private;
//     input r1 as u64.public;
//     cast r0 r1 into r2 as credits.record;
//     async transfer_public_to_private self.caller r1 into r3;
//     output r2 as credits.record;
//     output r3 as credits.aleo/transfer_public_to_private.future;
//
// finalize transfer_public_to_private:
//     input r0 as address.public;
//     input r1 as u64.public;
//     get.or_use account[r0] 0u64 into r2;
//     sub r2 r1 into r3;
//     set r3 into account[r0];
// ";
//     let program = Program::<CurrentNetwork>::from_str(aleo_prog_1);
//     match program {
//         Ok(p) => {
//             let disassembled = disassemble(p);
//             println!("{}", disassembled)
//         }
//         Err(e) => {
//             println!("{}", e);
//         }
//     }
//     // println!("{}", disassemble(Program::<CurrentNetwork>::from_str(aleo_prog_1).unwrap()));
// }

// fn disassemble<N: Network, Instruction: InstructionTrait<N>, Command: CommandTrait<N>>(
//     program: ProgramCore<N, Instruction, Command>,
// ) -> Stub {
//     dbg!(program.records());
//
//     let mut stub = Stub {
//         imports: Vec::new(),
//         stub_id: ProgramId::from(program.id()),
//         consts: Vec::new(),
//         structs: Vec::new(),
//         mappings: Vec::new(),
//         functions: Vec::new(),
//         span: Default::default(),
//     };
//
//
//     let structs = program.structs().iter().map(|(id,s)| {
//         (Identifier::from(id).name, Struct::from(s))
//     }).collect_vec();
//     let records = program.records().iter().map(|(id,s)| {
//         dbg!(id.clone());
//         dbg!(s);
//         println!("{}", id.clone());
//         let id_str = id.clone().to_string();
//         let id_sym = Symbol::intern(&id_str);
//         ((id_sym, Struct::from(s)))
//     }).collect_vec();
//
//
//     stub
//
//     // Stub {
//     //     imports: program.imports().into_iter().map(|(id, import)| ProgramId::from(id)).collect(),
//     //     stub_id: ProgramId::from(program.id()),
//     //     consts: Vec::new(),
//     //     structs: [structs,records].concat(),
//     //     mappings: Vec::new(),
//     //     functions: Vec::new(), // TODO: Add functions AND closures
//     //     span: Default::default(),
//     // }
// }

fn print_visibility<N: Network>(visibility: &ValueType<N>) -> String {
    match visibility {
        ValueType::Public(_) => "public ".to_string(),
        _ => "".to_string(),
    }
}

fn print_transition_input<N: Network>(input: &ValueType<N>, index: usize, len: usize) -> String {
    format!("{}a{}: {}", print_visibility(input), index + 1, print_transition_value(input, index, len))
}

fn print_transition_value<N: Network>(value: &ValueType<N>, index: usize, len: usize) -> String {
    let value_str = match value {
        ValueType::Constant(val) => format!("{}", val.to_string().replace("boolean", "bool")),
        ValueType::Public(val) => format!("{}", val.to_string().replace("boolean", "bool")),
        ValueType::Private(val) => format!("{}", val.to_string().replace("boolean", "bool")),
        ValueType::Record(id) => format!("{}", id.to_string()),
        ValueType::ExternalRecord(loc) => format!("{}.aleo/{}", loc.name(), loc.resource().to_string()),
        ValueType::Future(_) => panic!("Futures must be filtered out by this stage"),
    };
    let situational_comma = if index == len - 1 { "" } else { ", " };
    format!("{}{}", value_str, situational_comma)
}

fn print_transition_output<N: Network>(output: &ValueType<N>, index: usize, len: usize) -> String {
    let (left, right) = if len == 1 {
        (" -> ", "")
    } else if index == 0 {
        (" -> (", "")
    } else if index == len - 1 {
        ("", ")")
    } else {
        ("", "")
    };
    format!("{}{}{}", left, print_transition_value(output, index, len), right)
}

fn print_function_output<N: Network>(output: &RegisterType<N>, index: usize, len: usize) -> String {
    let (left, right) = if len == 1 {
        (" -> ", "")
    } else if index == 0 {
        (" -> (", "")
    } else if index == len - 1 {
        ("", ")")
    } else {
        ("", "")
    };
    format!("{}{}{}", left, print_function_value(output, index, len), right)
}

fn print_function_input<N: Network>(input: &RegisterType<N>, index: usize, len: usize) -> String {
    format!("a{}: {}", index + 1, print_function_value(input, index, len))
}

fn print_function_value<N: Network>(value: &RegisterType<N>, index: usize, len: usize) -> String {
    let value_str = match value {
        RegisterType::Plaintext(val) => format!("{}", val.to_string().replace("boolean", "bool")),
        RegisterType::Record(val) => format!("{}", val.to_string()),
        RegisterType::ExternalRecord(loc) => format!("{}.aleo/{}", loc.name(), loc.resource().to_string()),
        RegisterType::Future(_) => panic!("Futures must be filtered out by this stage"),
    };
    let situational_comma = if index == len - 1 { "" } else { ", " };
    format!("{}{}", value_str, situational_comma)
}

fn code_gen(input: String) -> String {
    // Parse a new program.
    let result = Program::<CurrentNetwork>::from_str(&input).unwrap();

    let mut output = format!("stub {}.aleo {{", result.id().name());

    // Write imports
    output.push_str(
        &result.imports().into_iter().map(|(name, _)| format!("\n    import {};\n", name)).collect::<String>(),
    );

    // Write records
    output.push_str(
        &result
            .records()
            .into_iter()
            .map(|(name, fields)| {
                format!("\n    record {} {{\n        owner: address,\n", name).add(
                    &(fields
                        .entries()
                        .into_iter()
                        .map(|(id, entry_type)| {
                            if entry_type.plaintext_type().to_string() == "boolean" {
                                return format!("        {}: bool,\n", id);
                            } else {
                                format!("        {}: {},\n", id, entry_type.plaintext_type().to_string())
                            }
                        })
                        .collect::<String>()
                        + "    }\n")
                        .to_string(),
                )
            })
            .collect::<String>(),
    );

    // Write transitions
    output.push_str(
        &result
            .functions()
            .into_iter()
            .map(|(name, value)| {
                let (inputs, outputs) = (value.inputs(), value.outputs().clone());

                // Can assume that last output is a future if the function has associated finalize
                let outputs_vec = if value.finalize_logic().is_some() {
                    outputs[..outputs.len() - 1].iter().collect_vec()
                } else {
                    outputs.iter().collect_vec()
                };
                let len = outputs_vec.len();

                format!("\n    transition {}(", name).add(
                    &(inputs
                        .into_iter()
                        .enumerate()
                        .map(|(index, input)| print_transition_input(&input.value_type(), index, inputs.len()))
                        .collect::<String>()
                        + ")")
                        .add(
                            &(outputs_vec
                                .into_iter()
                                .enumerate()
                                .map(|(index, output)| print_transition_output(&output.value_type(), index, len))
                                .collect::<String>()
                                + ";\n")
                                .to_string(),
                        ),
                )
            })
            .collect::<String>(),
    );

    // Write functions
    output.push_str(
        &result
            .closures()
            .into_iter()
            .map(|(name, value)| {
                let (inputs, outputs) = (value.inputs(), value.outputs().clone());
                let len = outputs.len();

                format!("\n    function {}(", name).add(
                    &(inputs
                        .into_iter()
                        .enumerate()
                        .map(|(index, input)| print_function_input(&input.register_type(), index, inputs.len()))
                        .collect::<String>()
                        + ")")
                        .add(
                            &(outputs
                                .into_iter()
                                .enumerate()
                                .map(|(index, output)| print_function_output(&output.register_type(), index, len))
                                .collect::<String>()
                                + ";\n")
                                .to_string(),
                        ),
                )
            })
            .collect::<String>(),
    );

    // Append final closing bracket
    output.push_str("}\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_gen_test() {
        let aleo_prog_1 = r"import credits.aleo;

import battleship.aleo;
import juice.aleo;

program to_parse.aleo;

record board_state:
    owner as address.private;
    hits_and_misses as u64.private;
    played_tiles as u64.private;
    ships as u64.private;
    player_1 as address.private;
    player_2 as address.private;
    game_started as boolean.private;

function new_board_state:
    input r0 as u64.private;
    input r1 as address.private;
    cast self.caller 0u64 0u64 r0 self.caller r1 false into r2 as board_state.record;
    output r2 as board_state.record;

closure add_up:
    input r0 as u8;
    input r1 as u8;
    add r0 r1 into r2;
    output r2 as u8;

function transfer_public_to_private:
    input r0 as address.private;
    input r1 as u64.public;
    cast r0 r1 into r2 as credits.record;
    async transfer_public_to_private self.caller r1 into r3;
    output r2 as credits.record;
    output r3 as credits.aleo/transfer_public_to_private.future;

finalize transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];
";
        let leo_stub_1: &str = r"stub to_parse.aleo {
    import credits.aleo;

    import battleship.aleo;

    import juice.aleo;

    record board_state {
        owner: address,
        hits_and_misses: u64,
        played_tiles: u64,
        ships: u64,
        player_1: address,
        player_2: address,
        game_started: bool,
    }

    transition new_board_state(a1: u64, a2: address) -> board_state;

    transition transfer_public_to_private(a1: address, public a2: u64) -> credits;

    function add_up(a1: u8, a2: u8) -> u8;
}
";
        let code_gen_1 = code_gen(aleo_prog_1.to_string());
        assert_eq!(code_gen_1, leo_stub_1);

    }
}
