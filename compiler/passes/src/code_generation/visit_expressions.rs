// Copyright (C) 2019-2022 Aleo Systems Inc.
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
    BinaryExpression, BinaryOperation, CallExpression, ErrExpression, Expression, Identifier, TernaryExpression,
    UnaryExpression, UnaryOperation, ValueExpression,
};

/// Implement the necessary methods to visit nodes in the AST.
// Note: We opt for this option instead of using `Visitor` and `Director` because this pass requires
// a post-order traversal of the AST. This is sufficient since this implementation is intended to be
// a prototype. The production implementation will require a redesign of `Director`.
impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_expression(&mut self, input: &'a Expression) -> (String, String) {
        match input {
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Value(expr) => self.visit_value(expr),
            Expression::Unary(expr) => self.visit_unary(expr),
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Ternary(expr) => self.visit_ternary(expr),
            Expression::Call(expr) => self.visit_call(expr),
            Expression::Err(expr) => self.visit_err(expr),
        }
    }

    fn visit_identifier(&mut self, input: &'a Identifier) -> (String, String) {
        (self.variable_mapping.get(input).unwrap().clone(), String::new())
    }

    fn visit_value(&mut self, input: &'a ValueExpression) -> (String, String) {
        (format!("{}", input), String::new())
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression) -> (String, String) {
        let (left_operand, left_instructions) = self.visit_expression(&input.left);
        let (right_operand, right_instructions) = self.visit_expression(&input.right);

        let opcode = match input.op {
            BinaryOperation::Add => String::from("add"),
            BinaryOperation::Sub => String::from("sub"),
            BinaryOperation::Mul => String::from("mul"),
            BinaryOperation::Div => String::from("div"),
            BinaryOperation::Pow => String::from("pow"),
            BinaryOperation::Or => String::from("or"),
            BinaryOperation::And => String::from("and"),
            BinaryOperation::Eq => String::from("eq"),
            BinaryOperation::Ne => String::from("neq"),
            BinaryOperation::Ge => String::from("ge"),
            BinaryOperation::Gt => String::from("gt"),
            BinaryOperation::Le => String::from("le"),
            BinaryOperation::Lt => String::from("lt"),
        };

        let destination_register = format!("r{}", self.next_register);
        let binary_instruction = format!(
            "{} {} {} into {}",
            opcode, left_operand, right_operand, destination_register
        );

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = left_instructions;
        instructions.push_str(&right_instructions);
        instructions.push_str(&binary_instruction);

        (destination_register, instructions)
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression) -> (String, String) {
        let (expression_operand, expression_instructions) = self.visit_expression(&input.inner);

        let opcode = match input.op {
            UnaryOperation::Not => String::from("not"),
            UnaryOperation::Negate => String::from("neg"),
        };

        let destination_register = format!("r{}", self.next_register);
        let unary_instruction = format!("{} {} into {};", opcode, expression_operand, destination_register);

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = expression_instructions;
        instructions.push_str(&unary_instruction);

        (destination_register, instructions)
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression) -> (String, String) {
        let (condition_operand, mut condition_instructions) = self.visit_expression(&input.condition);
        let (if_true_operand, if_true_instructions) = self.visit_expression(&input.if_true);
        let (if_false_operand, if_false_instructions) = self.visit_expression(&input.if_false);

        let destination_register = format!("r{}", self.next_register);
        let ternary_instruction = format!(
            "ternary {} {} {} into r{};",
            condition_operand, if_true_operand, if_false_operand, destination_register
        );

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = condition_instructions;
        instructions.push_str(&if_true_instructions);
        instructions.push_str(&if_false_instructions);
        instructions.push_str(&ternary_instruction);

        (destination_register, instructions)
    }

    fn visit_call(&mut self, input: &'a CallExpression) -> (String, String) {
        unreachable!("`CallExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_err(&mut self, input: &'a ErrExpression) -> (String, String) {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }
}
