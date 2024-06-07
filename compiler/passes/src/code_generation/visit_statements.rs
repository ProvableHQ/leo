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
    ReturnStatement,
    Statement,
    Type,
};

use itertools::Itertools;

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
        let outputs = match input.expression {
            // Skip empty return statements.
            Expression::Unit(_) => String::new(),
            _ => {
                let (operand, mut expression_instructions) = self.visit_expression(&input.expression);
                // Get the output type of the function.
                let output = self.current_function.unwrap().output.iter();
                // If the operand string is empty, initialize an empty vector.
                let operand_strings = match operand.is_empty() {
                    true => vec![],
                    false => operand.split(' ').collect_vec(),
                };

                let mut future_output = String::new();
                let mut instructions = operand_strings
                    .iter()
                    .zip_eq(output)
                    .map(|(operand, output)| {
                        // Transitions outputs with no mode are private.
                        // Note that this unwrap is safe because we set the variant before traversing the function.
                        let visibility = match (self.variant.unwrap().is_transition(), output.mode) {
                            (true, Mode::None) => Mode::Private,
                            (_, mode) => mode,
                        };

                        if let Type::Future(_) = output.type_ {
                            future_output = format!(
                                "    output {} as {}.aleo/{}.future;\n",
                                operand,
                                self.program_id.unwrap().name,
                                self.current_function.unwrap().identifier,
                            );
                            String::new()
                        } else {
                            format!(
                                "    output {} as {};\n",
                                operand,
                                self.visit_type_with_visibility(&output.type_, visibility)
                            )
                        }
                    })
                    .join("");

                // Insert future output at the end.
                instructions.push_str(&future_output);

                expression_instructions.push_str(&instructions);

                expression_instructions
            }
        };

        outputs
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
        // Note that this unwrap is safe because we set the variant before traversing the function.
        if !self.variant.unwrap().is_async_function() {
            unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
        } else {
            // Construct a label for the end of the `then` block.
            let end_then_label = format!("end_then_{}_{}", self.conditional_depth, self.next_label);
            self.next_label += 1;
            // Construct a label for the end of the `otherwise` block if it exists.
            let (has_otherwise, end_otherwise_label) = {
                match _input.otherwise.is_some() {
                    true => {
                        // Construct a label for the end of the `otherwise` block.
                        let end_otherwise_label =
                            { format!("end_otherwise_{}_{}", self.conditional_depth, self.next_label) };
                        self.next_label += 1;
                        (true, end_otherwise_label)
                    }
                    false => (false, String::new()),
                }
            };

            // Increment the conditional depth.
            self.conditional_depth += 1;

            // Create a `branch` instruction.
            let (condition, mut instructions) = self.visit_expression(&_input.condition);
            instructions.push_str(&format!("    branch.eq {condition} false to {end_then_label};\n"));

            // Visit the `then` block.
            instructions.push_str(&self.visit_block(&_input.then));
            // If the `otherwise` block is present, add a branch instruction to jump to the end of the `otherwise` block.
            if has_otherwise {
                instructions.push_str(&format!("    branch.eq true true to {end_otherwise_label};\n"));
            }

            // Add a label for the end of the `then` block.
            instructions.push_str(&format!("    position {};\n", end_then_label));

            // Visit the `otherwise` block.
            if let Some(else_block) = &_input.otherwise {
                // Visit the `otherwise` block.
                instructions.push_str(&self.visit_statement(else_block));
                // Add a label for the end of the `otherwise` block.
                instructions.push_str(&format!("    position {end_otherwise_label};\n"));
            }

            // Decrement the conditional depth.
            self.conditional_depth -= 1;

            instructions
        }
    }

    fn visit_iteration(&mut self, _input: &'a IterationStatement) -> String {
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
