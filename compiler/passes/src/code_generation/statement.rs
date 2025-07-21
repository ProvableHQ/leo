// Copyright (C) 2019-2025 Provable Inc.
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

use super::*;

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    ExpressionStatement,
    IterationStatement,
    Mode,
    ReturnStatement,
    Statement,
    Type,
};

use indexmap::IndexSet;
use itertools::Itertools as _;
use std::fmt::Write as _;

impl CodeGeneratingVisitor<'_> {
    fn visit_statement(&mut self, input: &Statement) -> String {
        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Const(_) => {
                panic!("`ConstStatement`s should not be in the AST at this phase of compilation.")
            }
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &AssertStatement) -> String {
        let mut generate_assert_instruction = |name: &str, left: &Expression, right: &Expression| {
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

    fn visit_return(&mut self, input: &ReturnStatement) -> String {
        if let Expression::Unit(..) = input.expression {
            // Skip empty return statements.
            return String::new();
        }

        let mut instructions = String::new();
        let mut operands = IndexSet::with_capacity(self.current_function.unwrap().output.len());

        if let Expression::Tuple(tuple) = &input.expression {
            // Now tuples only appear in return position, so let's handle this
            // ourselves.
            let outputs = &self.current_function.unwrap().output;
            assert_eq!(tuple.elements.len(), outputs.len());

            for (expr, output) in tuple.elements.iter().zip(outputs) {
                let (operand, op_instructions) = self.visit_expression(expr);
                instructions.push_str(&op_instructions);
                if self.internal_record_inputs.contains(&operand) || operands.contains(&operand) {
                    // We can't output an internal record we received as input.
                    // We also can't output the same value twice.
                    // Either way, clone it.
                    let (new_operand, new_instr) = self.clone_register(&operand, &output.type_);
                    instructions.push_str(&new_instr);
                    operands.insert(new_operand);
                } else {
                    operands.insert(operand);
                }
            }
        } else {
            // Not a tuple - only one output.
            let (operand, op_instructions) = self.visit_expression(&input.expression);
            if self.internal_record_inputs.contains(&operand) {
                // We can't output an internal record we received as input.
                let (new_operand, new_instr) =
                    self.clone_register(&operand, &self.current_function.unwrap().output_type);
                instructions.push_str(&new_instr);
                operands.insert(new_operand);
            } else {
                instructions = op_instructions;
                operands.insert(operand);
            }
        }

        for (operand, output) in operands.iter().zip(&self.current_function.unwrap().output) {
            // Transitions outputs with no mode are private.
            let visibility = match (self.variant.unwrap().is_transition(), output.mode) {
                (true, Mode::None) => Mode::Private,
                (_, mode) => mode,
            };
            if let Type::Future(_) = output.type_ {
                writeln!(
                    &mut instructions,
                    "    output {} as {}.aleo/{}.future;",
                    operand,
                    self.program_id.unwrap().name,
                    self.current_function.unwrap().identifier,
                )
                .unwrap();
            } else {
                writeln!(
                    &mut instructions,
                    "    output {} as {};",
                    operand,
                    self.visit_type_with_visibility(&output.type_, visibility)
                )
                .unwrap();
            }
        }

        instructions
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) -> String {
        match (&input.place, &input.value) {
            (DefinitionPlace::Single(identifier), _) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                self.variable_mapping.insert(identifier.name, operand);
                expression_instructions
            }
            (DefinitionPlace::Multiple(identifiers), Expression::Call(_)) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                // Add the destinations to the variable mapping.
                for (identifier, operand) in identifiers.iter().zip_eq(operand.split(' ')) {
                    self.variable_mapping.insert(identifier.name, operand.to_string());
                }
                expression_instructions
            }
            _ => panic!("Previous passes should have ensured that a definition with multiple identifiers is a `Call`."),
        }
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement) -> String {
        self.visit_expression(&input.expression).1
    }

    fn visit_assign(&mut self, _input: &AssignStatement) -> String {
        panic!("AssignStatement's should not exist in SSA form.")
    }

    fn visit_conditional(&mut self, _input: &ConditionalStatement) -> String {
        // Note that this unwrap is safe because we set the variant before traversing the function.
        if !self.variant.unwrap().is_async_function() {
            panic!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
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
            instructions.push_str(&format!("    position {end_then_label};\n"));

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

    fn visit_iteration(&mut self, _input: &IterationStatement) -> String {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    pub(crate) fn visit_block(&mut self, input: &Block) -> String {
        // For each statement in the block, visit it and add its instructions to the list.
        input.statements.iter().map(|stmt| self.visit_statement(stmt)).join("")
    }
}
