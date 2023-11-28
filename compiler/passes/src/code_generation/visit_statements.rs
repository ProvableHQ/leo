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

use crate::CodeGenerator;

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionStatement,
    IterationStatement,
    Mode,
    Output,
    ReturnStatement,
    Statement,
};

use itertools::Itertools;
use std::fmt::Write as _;

impl<'a> CodeGenerator<'a> {
    fn visit_statement(&mut self, input: &'a Statement) -> String {
        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Const(_) => {
                unreachable!("`ConstStatement`s should not be in the AST at this phase of compilation.")
            }
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &'a AssertStatement) -> String {
        let mut generate_assert_instruction = |name: &str, left: &'a Expression, right: &'a Expression| {
            let (left_operand, left_instructions) = self.visit_expression(left);
            let (right_operand, right_instructions) = self.visit_expression(right);
            let assert_instruction = format!("    {name} {left_operand} {right_operand};\n");

            // Concatenate the instructions.
            let mut instructions = left_instructions;
            instructions.push_str(&right_instructions);
            instructions.push_str(&assert_instruction);

            instructions
        };
        match &input.variant {
            AssertVariant::Assert(expr) => {
                let (operand, mut instructions) = self.visit_expression(expr);
                let assert_instruction = format!("    assert.eq {operand} true;\n");

                instructions.push_str(&assert_instruction);
                instructions
            }
            AssertVariant::AssertEq(left, right) => generate_assert_instruction("assert.eq", left, right),
            AssertVariant::AssertNeq(left, right) => generate_assert_instruction("assert.neq", left, right),
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) -> String {
        let mut outputs = match input.expression {
            // Skip empty return statements.
            Expression::Unit(_) => String::new(),
            _ => {
                let (operand, mut expression_instructions) = self.visit_expression(&input.expression);
                // Get the output type of the function.
                let output = if self.in_finalize {
                    // Note that the first unwrap is safe, since `current_function` is set in `visit_function`.
                    self.current_function.unwrap().finalize.as_ref().unwrap().output.iter()
                } else {
                    // Note that this unwrap is safe, since `current_function` is set in `visit_function`.
                    self.current_function.unwrap().output.iter()
                };
                // If the operand string is empty, initialize an empty vector.
                let operand_strings = match operand.is_empty() {
                    true => vec![],
                    false => operand.split(' ').collect_vec(),
                };
                let instructions = operand_strings
                    .iter()
                    .zip_eq(output)
                    .map(|(operand, output)| {
                        match output {
                            Output::Internal(output) => {
                                let visibility = if self.is_transition_function {
                                    match self.in_finalize {
                                        // If in finalize block, the default visibility is public.
                                        true => match output.mode {
                                            Mode::None => Mode::Public,
                                            mode => mode,
                                        },
                                        // If not in finalize block, the default visibility is private.
                                        false => match output.mode {
                                            Mode::None => Mode::Private,
                                            mode => mode,
                                        },
                                    }
                                } else {
                                    // Only program functions have visibilities associated with their outputs.
                                    Mode::None
                                };
                                format!(
                                    "    output {} as {};\n",
                                    operand,
                                    self.visit_type_with_visibility(&output.type_, visibility)
                                )
                            }
                            Output::External(output) => {
                                format!(
                                    "    output {} as {}.aleo/{}.record;\n",
                                    operand, output.program_name, output.record,
                                )
                            }
                        }
                    })
                    .join("");

                expression_instructions.push_str(&instructions);

                expression_instructions
            }
        };

        // Initialize storage for the instructions.
        let mut instructions = String::new();

        // If there are any futures or if the return instruction has `finalize_arguments`, then
        // create an `async` instruction that uses them.
        if !self.futures.is_empty() || input.finalize_arguments.is_some() {
            // Note that this unwrap is safe, since `current_function` is set in `visit_function`.
            let function_id = self.current_function.unwrap().name();
            let mut async_instruction = format!("    async {function_id}");
            // Add the futures to the async instruction.
            for (future_register, _) in self.futures.iter() {
                write!(async_instruction, " {}", future_register).expect("failed to write to string");
            }
            // Add the finalize arguments to the async instruction.
            if let Some(arguments) = &input.finalize_arguments {
                for argument in arguments.iter() {
                    let (argument, argument_instructions) = self.visit_expression(argument);
                    write!(async_instruction, " {argument}").expect("failed to write to string");
                    instructions.push_str(&argument_instructions);
                }
            }
            // Write the destination register.
            let destination_register = format!("r{}", self.next_register);
            writeln!(async_instruction, " into {};", destination_register).expect("failed to write to string");
            // Increment the register counter.
            self.next_register += 1;
            // Add the async instruction to the instructions.
            instructions.push_str(&async_instruction);

            // Add the destination register to the outputs.
            let program_id = match self.program_id {
                Some(program_id) => program_id,
                None => unreachable!("`program_id` should be set in `visit_function`"),
            };
            outputs
                .push_str(&format!("    output {} as {}/{}.future;\n", destination_register, program_id, function_id));
        }

        // Extend the instructions with the outputs.
        instructions.push_str(&outputs);

        instructions
    }

    fn visit_definition(&mut self, _input: &'a DefinitionStatement) -> String {
        // TODO: If SSA is made optional, then conditionally enable codegen for DefinitionStatement
        // let (operand, expression_instructions) = self.visit_expression(&input.value);
        // self.variable_mapping.insert(&input.variable_name.name, operand);
        // expression_instructions
        unreachable!("DefinitionStatement's should not exist in SSA form.")
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) -> String {
        self.visit_expression(&input.expression).1
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) -> String {
        match (&input.place, &input.value) {
            (Expression::Identifier(identifier), _) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                self.variable_mapping.insert(&identifier.name, operand);
                expression_instructions
            }
            (Expression::Tuple(tuple), Expression::Call(_)) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                // Split out the destinations from the tuple.
                let operands = operand.split(' ').collect::<Vec<_>>();
                // Add the destinations to the variable mapping.
                tuple.elements.iter().zip_eq(operands).for_each(|(element, operand)| {
                    match element {
                        Expression::Identifier(identifier) => {
                            self.variable_mapping.insert(&identifier.name, operand.to_string())
                        }
                        _ => {
                            unreachable!("Type checking ensures that tuple elements on the lhs are always identifiers.")
                        }
                    };
                });
                expression_instructions
            }
            _ => unimplemented!(
                "Code generation for the left-hand side of an assignment is only implemented for `Identifier`s."
            ),
        }
    }

    fn visit_conditional(&mut self, _input: &'a ConditionalStatement) -> String {
        // TODO: Once SSA is made optional, create a Leo error informing the user to enable the SSA pass.
        unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
    }

    fn visit_iteration(&mut self, _input: &'a IterationStatement) -> String {
        // TODO: Once loop unrolling is made optional, create a Leo error informing the user to enable the loop unrolling pass..
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn visit_console(&mut self, _: &'a ConsoleStatement) -> String {
        unreachable!("Parsing guarantees that `ConsoleStatement`s are not present in the AST.")
    }

    pub(crate) fn visit_block(&mut self, input: &'a Block) -> String {
        // For each statement in the block, visit it and add its instructions to the list.
        input.statements.iter().map(|stmt| self.visit_statement(stmt)).join("")
    }
}
