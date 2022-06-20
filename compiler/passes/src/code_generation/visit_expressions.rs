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
            Expression::Access(_expr) => todo!(),
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Call(expr) => self.visit_call(expr),
            Expression::CircuitInit(_expr) => todo!(),
            Expression::Err(expr) => self.visit_err(expr),
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Ternary(expr) => self.visit_ternary(expr),
            Expression::Unary(expr) => self.visit_unary(expr),
            Expression::Value(expr) => self.visit_value(expr),
        }
    }

    fn visit_identifier(&mut self, input: &'a Identifier) -> (String, String) {
        (self.variable_mapping.get(&input.name).unwrap().clone(), String::new())
    }

    fn visit_value(&mut self, input: &'a ValueExpression) -> (String, String) {
        (format!("{}", input), String::new())
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression) -> (String, String) {
        let (left_operand, left_instructions) = self.visit_expression(&input.left);
        let (right_operand, right_instructions) = self.visit_expression(&input.right);

        let opcode = match input.op {
            BinaryOperation::Add => String::from("add"),
            BinaryOperation::AddWrapped => String::from("add.w"),
            BinaryOperation::And => String::from("and"),
            BinaryOperation::BitwiseAnd => String::from("and"),
            BinaryOperation::Div => String::from("div"),
            BinaryOperation::DivWrapped => String::from("div.w"),
            BinaryOperation::Eq => String::from("is.eq"),
            BinaryOperation::Gte => String::from("gte"),
            BinaryOperation::Gt => String::from("gt"),
            BinaryOperation::Lte => String::from("lte"),
            BinaryOperation::Lt => String::from("lt"),
            BinaryOperation::Mul => String::from("mul"),
            BinaryOperation::MulWrapped => String::from("mul.w"),
            BinaryOperation::Nand => String::from("nand"),
            BinaryOperation::Neq => String::from("is.neq"),
            BinaryOperation::Nor => String::from("nor"),
            BinaryOperation::Or => String::from("or"),
            BinaryOperation::BitwiseOr => String::from("or"),
            BinaryOperation::Pow => String::from("pow"),
            BinaryOperation::PowWrapped => String::from("pow.w"),
            BinaryOperation::Shl => String::from("shl"),
            BinaryOperation::ShlWrapped => String::from("shl.w"),
            BinaryOperation::Shr => String::from("shr"),
            BinaryOperation::ShrWrapped => String::from("shr.w"),
            BinaryOperation::Sub => String::from("sub"),
            BinaryOperation::SubWrapped => String::from("sub.w"),
            BinaryOperation::Xor => String::from("xor"),
        };

        let destination_register = format!("r{}", self.next_register);
        let binary_instruction = format!(
            "    {} {} {} into {};\n",
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
        let (expression_operand, expression_instructions) = self.visit_expression(&input.receiver);

        let opcode = match input.op {
            UnaryOperation::Abs => String::from("abs"),
            UnaryOperation::AbsWrapped => String::from("abs.w"),
            UnaryOperation::Double => String::from("double"),
            UnaryOperation::Inverse => String::from("inv"),
            UnaryOperation::Not => String::from("not"),
            UnaryOperation::Negate => String::from("neg"),
            UnaryOperation::Square => String::from("square"),
            UnaryOperation::SquareRoot => String::from("sqrt"),
        };

        let destination_register = format!("r{}", self.next_register);
        let unary_instruction = format!("    {} {} into {};\n", opcode, expression_operand, destination_register);

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = expression_instructions;
        instructions.push_str(&unary_instruction);

        (destination_register, instructions)
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression) -> (String, String) {
        let (condition_operand, condition_instructions) = self.visit_expression(&input.condition);
        let (if_true_operand, if_true_instructions) = self.visit_expression(&input.if_true);
        let (if_false_operand, if_false_instructions) = self.visit_expression(&input.if_false);

        let destination_register = format!("r{}", self.next_register);
        let ternary_instruction = format!(
            "    ternary {} {} {} into r{};\n",
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

    fn visit_call(&mut self, _input: &'a CallExpression) -> (String, String) {
        unreachable!("`CallExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_err(&mut self, _input: &'a ErrExpression) -> (String, String) {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }
}
