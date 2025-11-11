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
    Output,
    ReturnStatement,
    Statement,
    Type,
};

use indexmap::IndexMap;

impl CodeGeneratingVisitor<'_> {
    fn visit_statement(&mut self, input: &Statement) -> Vec<AleoStmt> {
        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => vec![self.visit_assign(stmt)],
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Const(_) => {
                panic!("`ConstStatement`s should not be in the AST at this phase of compilation.")
            }
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => vec![self.visit_iteration(stmt)],
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &AssertStatement) -> Vec<AleoStmt> {
        match &input.variant {
            AssertVariant::Assert(expr) => {
                let (operand, mut instructions) = self.visit_expression(expr);
                let operand = operand.expect("Trying to assert an empty expression.");
                instructions.push(AleoStmt::AssertEq(operand, AleoExpr::Bool(true)));
                instructions
            }
            AssertVariant::AssertEq(left, right) => {
                let (left, left_stmts) = self.visit_expression(left);
                let (right, right_stmts) = self.visit_expression(right);
                let left = left.expect("Trying to assert an empty expression.");
                let right = right.expect("Trying to assert an empty expression.");
                let assert_instruction = AleoStmt::AssertEq(left, right);

                // Concatenate the instructions.
                let mut instructions = left_stmts;
                instructions.extend(right_stmts);
                instructions.push(assert_instruction);
                instructions
            }
            AssertVariant::AssertNeq(left, right) => {
                let (left, left_stmts) = self.visit_expression(left);
                let (right, right_stmts) = self.visit_expression(right);
                let left = left.expect("Trying to assert an empty expression.");
                let right = right.expect("Trying to assert an empty expression.");
                let assert_instruction = AleoStmt::AssertNeq(left, right);

                // Concatenate the instructions.
                let mut instructions = left_stmts;
                instructions.extend(right_stmts);
                instructions.push(assert_instruction);
                instructions
            }
        }
    }

    fn visit_return(&mut self, input: &ReturnStatement) -> Vec<AleoStmt> {
        let mut instructions = vec![];
        let mut operands: IndexMap<AleoExpr, &Output> =
            IndexMap::with_capacity(self.current_function.unwrap().output.len());

        if let Expression::Tuple(tuple) = &input.expression {
            // Now tuples only appear in return position, so let's handle this
            // ourselves.
            let outputs = &self.current_function.unwrap().output;
            assert_eq!(tuple.elements.len(), outputs.len());

            for (expr, output) in tuple.elements.iter().zip_eq(outputs) {
                let (operand, op_instructions) = self.visit_expression(expr);
                instructions.extend(op_instructions);
                if let Some(operand) = operand {
                    if self.internal_record_inputs.contains(&operand) || operands.contains_key(&operand) {
                        // We can't output an internal record we received as input.
                        // We also can't output the same value twice.
                        // Either way, clone it.
                        let (new_operand, new_instr) = self.clone_register(&operand, &output.type_);
                        instructions.extend(new_instr);
                        operands.insert(new_operand, output);
                    } else {
                        operands.insert(operand, output);
                    }
                }
            }
        } else {
            // Not a tuple - only one output.
            let (operand, op_instructions) = self.visit_expression(&input.expression);
            if let Some(operand) = operand {
                let output = &self.current_function.unwrap().output[0];

                if self.internal_record_inputs.contains(&operand) {
                    // We can't output an internal record we received as input.
                    let (new_operand, new_instr) =
                        self.clone_register(&operand, &self.current_function.unwrap().output_type);
                    instructions.extend(new_instr);
                    operands.insert(new_operand, output);
                } else {
                    instructions = op_instructions;
                    operands.insert(operand, output);
                }
            }
        }

        for (operand, output) in operands.iter() {
            // Transitions outputs with no mode are private.
            let visibility = match (self.variant.unwrap().is_transition(), output.mode) {
                (true, Mode::None) => Some(AleoVisibility::Private),
                (_, mode) => AleoVisibility::maybe_from(mode),
            };
            if let Type::Future(_) = output.type_ {
                instructions.push(AleoStmt::Output(
                    operand.clone(),
                    AleoType::Future {
                        name: self.current_function.unwrap().identifier.to_string(),
                        program: self.program_id.unwrap().name.to_string(),
                    },
                    None,
                ));
            } else if output.type_.is_empty() {
                // do nothing
            } else {
                let (output_type, output_viz) = self.visit_type_with_visibility(&output.type_, visibility);
                instructions.push(AleoStmt::Output(operand.clone(), output_type, output_viz));
            }
        }

        instructions
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) -> Vec<AleoStmt> {
        match (&input.place, &input.value) {
            (DefinitionPlace::Single(identifier), _) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                if let Some(operand) = operand {
                    self.variable_mapping.insert(identifier.name, operand);
                }
                expression_instructions
            }
            (DefinitionPlace::Multiple(identifiers), Expression::Call(_)) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                let Some(AleoExpr::Tuple(elems)) = operand else {
                    panic!("Definition with multiple identifiers should yield a tuple")
                };
                // Add the destinations to the variable mapping.
                for (identifier, operand) in identifiers.iter().zip_eq(elems.iter()) {
                    self.variable_mapping.insert(identifier.name, operand.clone());
                }
                expression_instructions
            }
            _ => panic!("Previous passes should have ensured that a definition with multiple identifiers is a `Call`."),
        }
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement) -> Vec<AleoStmt> {
        self.visit_expression(&input.expression).1
    }

    fn visit_assign(&mut self, _input: &AssignStatement) -> AleoStmt {
        panic!("AssignStatement's should not exist in SSA form.")
    }

    fn visit_conditional(&mut self, _input: &ConditionalStatement) -> Vec<AleoStmt> {
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
            let condition = condition.expect("Trying to branch on an empty expression");
            instructions.push(AleoStmt::BranchEq(condition, AleoExpr::Bool(false), end_then_label.clone()));

            // Visit the `then` block.
            instructions.extend(self.visit_block(&_input.then));
            // If the `otherwise` block is present, add a branch instruction to jump to the end of the `otherwise` block.
            if has_otherwise {
                instructions.push(AleoStmt::BranchEq(
                    AleoExpr::Bool(true),
                    AleoExpr::Bool(true),
                    end_otherwise_label.clone(),
                ));
            }

            // Add a label for the end of the `then` block.
            instructions.push(AleoStmt::Position(end_then_label));

            // Visit the `otherwise` block.
            if let Some(else_block) = &_input.otherwise {
                // Visit the `otherwise` block.
                instructions.extend(self.visit_statement(else_block));
                // Add a label for the end of the `otherwise` block.
                instructions.push(AleoStmt::Position(end_otherwise_label));
            }

            // Decrement the conditional depth.
            self.conditional_depth -= 1;

            instructions
        }
    }

    fn visit_iteration(&mut self, _input: &IterationStatement) -> AleoStmt {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    pub(crate) fn visit_block(&mut self, input: &Block) -> Vec<AleoStmt> {
        // For each statement in the block, visit it and add its instructions to the list.
        input.statements.iter().flat_map(|stmt| self.visit_statement(stmt)).collect()
    }
}
