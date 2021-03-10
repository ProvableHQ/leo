// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! This module contains the reducer which iterates through ast nodes - converting them into
//! asg nodes and saving relevant information.

use crate::{
    reducer::ReconstructingReducer,
    Circuit,
    CircuitMember,
    Function,
    FunctionInput,
    FunctionInputVariable,
    Identifier,
    Type,
};

pub struct Canonicalizer;

impl Canonicalizer {
    fn is_self(&self, identifier: &Identifier) -> bool {
        match identifier.name.as_str() {
            "Self" => true,
            _ => false,
        }
    }

    fn is_self_keyword(&self, function_inputs: &Vec<FunctionInput>) -> bool {
        for function_input in function_inputs {
            match function_input {
                FunctionInput::SelfKeyword(_) => return true,
                _ => {}
            }
        }

        false
    }

    fn is_self_type(&self, type_option: Option<&Type>) -> bool {
        match type_option {
            Some(type_) => match type_ {
                Type::SelfType => true,
                _ => false,
            },
            None => false,
        }
    }

    fn canonicalize_function_input(&self, function_input: &FunctionInput, circuit_name: &Identifier) -> FunctionInput {
        match function_input {
            FunctionInput::SelfKeyword(self_keyword) => {
                return FunctionInput::Variable(FunctionInputVariable {
                    identifier: circuit_name.clone(),
                    const_: false,
                    mutable: false,
                    type_: Type::Circuit(circuit_name.clone()),
                    span: self_keyword.span.clone(),
                });
            }
            FunctionInput::MutSelfKeyword(mut_self_keyword) => {
                return FunctionInput::Variable(FunctionInputVariable {
                    identifier: circuit_name.clone(),
                    const_: false,
                    mutable: true,
                    type_: Type::Circuit(circuit_name.clone()),
                    span: mut_self_keyword.span.clone(),
                });
            }
            _ => {}
        }

        function_input.clone()
    }

    fn canonicalize_circuit_member(&self, circuit_member: &CircuitMember, circuit_name: &Identifier) -> CircuitMember {
        match circuit_member {
            CircuitMember::CircuitVariable(_, _) => {}
            CircuitMember::CircuitFunction(function) => {
                let input = function.input.clone();
                let mut output = function.output.clone();

                // probably shouldn't do this its self not Self
                // if self.is_self_keyword(&input) {
                //     input = input
                //         .iter()
                //         .map(|function_input| self.canonicalize_function_input(function_input, circuit_name))
                //         .collect();
                // }

                if self.is_self_type(output.as_ref()) {
                    output = Some(Type::Circuit(circuit_name.clone()));
                }

                return CircuitMember::CircuitFunction(Function {
                    annotations: function.annotations.clone(),
                    identifier: function.identifier.clone(),
                    input,
                    output,
                    block: function.block.clone(),
                    span: function.span.clone(),
                });
            }
        }

        circuit_member.clone()
    }
}

impl ReconstructingReducer for Canonicalizer {
    fn reduce_circuit(
        &mut self,
        _: &Circuit,
        circuit_name: Identifier,
        members: Vec<CircuitMember>,
    ) -> Option<Circuit> {
        let new_circuit = Circuit {
            circuit_name: circuit_name.clone(),
            members: members
                .iter()
                .map(|member| self.canonicalize_circuit_member(member, &circuit_name))
                .collect(),
        };

        Some(new_circuit)
    }

    // TODO make all self/Self outside of circuit error out
}
