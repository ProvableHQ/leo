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
    Block,
    CallExpression,
    Circuit,
    CircuitInitExpression,
    CircuitMember,
    CircuitMemberAccessExpression,
    CircuitStaticFunctionAccessExpression,
    DefinitionStatement,
    Expression,
    ExpressionStatement,
    Function,
    FunctionInput,
    FunctionInputVariable,
    Identifier,
    ReturnStatement,
    Statement,
    Type,
};

pub struct Canonicalizer;

impl Canonicalizer {
    fn _is_self(&self, identifier: &Identifier) -> bool {
        matches!(identifier.name.as_str(), "Self")
    }

    fn _is_self_keyword(&self, function_inputs: &[FunctionInput]) -> bool {
        for function_input in function_inputs {
            if let FunctionInput::SelfKeyword(_) = function_input {
                return true;
            }
        }

        false
    }

    fn is_self_type(&self, type_option: Option<&Type>) -> bool {
        matches!(type_option, Some(Type::SelfType))
    }

    fn _canonicalize_function_input(&self, function_input: &FunctionInput, circuit_name: &Identifier) -> FunctionInput {
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

    fn canonicalize_expression(&self, expression: &Expression, circuit_name: &Identifier) -> Expression {
        match expression {
            Expression::CircuitInit(circuit_init) => {
                return Expression::CircuitInit(CircuitInitExpression {
                    name: circuit_name.clone(),
                    members: circuit_init.members.clone(),
                    span: circuit_init.span.clone(),
                });
            }
            Expression::CircuitMemberAccess(circuit_member_access) => {
                return Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                    circuit: Box::new(self.canonicalize_expression(&circuit_member_access.circuit, circuit_name)),
                    name: circuit_member_access.name.clone(),
                    span: circuit_member_access.span.clone(),
                });
            }
            Expression::CircuitStaticFunctionAccess(circuit_static_func_access) => {
                return Expression::CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression {
                    circuit: Box::new(self.canonicalize_expression(&circuit_static_func_access.circuit, circuit_name)),
                    name: circuit_static_func_access.name.clone(),
                    span: circuit_static_func_access.span.clone(),
                });
            }
            Expression::Call(call) => {
                return Expression::Call(CallExpression {
                    function: Box::new(self.canonicalize_expression(&call.function, circuit_name)),
                    arguments: call.arguments.clone(),
                    span: call.span.clone(),
                });
            }
            _ => {}
        }

        expression.clone()
    }

    fn canonicalize_statement(&self, statement: &Statement, circuit_name: &Identifier) -> Statement {
        // What Statements could have Self appear

        match statement {
            Statement::Return(return_statement) => {
                return Statement::Return(ReturnStatement {
                    expression: self.canonicalize_expression(&return_statement.expression, circuit_name),
                    span: return_statement.span.clone(),
                });
            }
            Statement::Definition(definition) => {
                let mut type_ = definition.type_.clone();

                if self.is_self_type(type_.as_ref()) {
                    type_ = Some(Type::Circuit(circuit_name.clone()));
                }

                return Statement::Definition(DefinitionStatement {
                    declaration_type: definition.declaration_type.clone(),
                    variable_names: definition.variable_names.clone(),
                    type_,
                    value: self.canonicalize_expression(&definition.value, circuit_name),
                    span: definition.span.clone(),
                });
            }
            Statement::Expression(expression) => {
                return Statement::Expression(ExpressionStatement {
                    expression: self.canonicalize_expression(&expression.expression, circuit_name),
                    span: expression.span.clone(),
                });
            }
            Statement::Block(block) => {
                return Statement::Block(Block {
                    statements: block
                        .statements
                        .iter()
                        .map(|block_statement| self.canonicalize_statement(&block_statement, circuit_name))
                        .collect(),
                    span: block.span.clone(),
                });
            }
            _ => {}
        }

        statement.clone()
    }

    fn canonicalize_circuit_member(&self, circuit_member: &CircuitMember, circuit_name: &Identifier) -> CircuitMember {
        match circuit_member {
            CircuitMember::CircuitVariable(_, _) => {}
            CircuitMember::CircuitFunction(function) => {
                let input = function.input.clone();
                let mut output = function.output.clone();
                let block = Block {
                    statements: function
                        .block
                        .statements
                        .iter()
                        .map(|statement| self.canonicalize_statement(statement, circuit_name))
                        .collect(),
                    span: function.block.span.clone(),
                };

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
                    block,
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
