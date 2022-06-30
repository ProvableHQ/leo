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

//! This module contains Visitor trait implementations for the AST.
//! It implements default methods for each node to be made
//! given the type of node its visiting.

use crate::*;

pub trait InstructionVisitor<'a> {
    // TODO: Remove associated types if not necessary.
    type AdditionalInput: Default;
    type Output: Default;

    fn visit_literal(
        &mut self,
        _input: &'a LiteralExpression,
        _additional_input: &Self::AdditionalInput,
    ) -> Self::Output {
        Default::default()
    }

    fn visit_identifier(&mut self, _input: &'a Identifier, _additional_input: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_invalid_operand(&mut self, _additional_input: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_operand(&mut self, input: &'a Operand, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            Operand::Invalid => self.visit_invalid_operand(additional),
            Operand::Identifier(operand) => self.visit_identifier(operand, additional),
            Operand::Literal(operand) => self.visit_literal(operand, additional),
        }
    }

    fn visit_instruction(&mut self, input: &'a Instruction, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            Instruction::Add(inst) => self.visit_add_instruction(inst, additional),
            Instruction::And(inst) => self.visit_and_instruction(inst, additional),
            Instruction::Div(inst) => self.visit_div_instruction(inst, additional),
            Instruction::GreaterThan(inst) => self.visit_greater_than_instruction(inst, additional),
            Instruction::GreaterThanOrEqual(inst) => self.visit_greater_than_or_equal_instruction(inst, additional),
            Instruction::IsEqual(inst) => self.visit_is_equal_instruction(inst, additional),
            Instruction::IsNotEqual(inst) => self.visit_is_not_equal_instruction(inst, additional),
            Instruction::LessThan(inst) => self.visit_less_than_instruction(inst, additional),
            Instruction::LessThanOrEqual(inst) => self.visit_less_than_or_equal_instruction(inst, additional),
            Instruction::Mul(inst) => self.visit_mul_instruction(inst, additional),
            Instruction::Nop(inst) => self.visit_nop_instruction(inst, additional),
            Instruction::Not(inst) => self.visit_not_instruction(inst, additional),
            Instruction::Or(inst) => self.visit_or_instruction(inst, additional),
            Instruction::Sub(inst) => self.visit_sub_instruction(inst, additional),
            Instruction::Ternary(inst) => self.visit_ternary_instruction(inst, additional),
        }
    }

    fn visit_add_instruction(&mut self, input: &'a Add, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_and_instruction(&mut self, input: &'a And, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_div_instruction(&mut self, input: &'a Div, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_greater_than_instruction(
        &mut self,
        input: &'a GreaterThan,
        additional: &Self::AdditionalInput,
    ) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_greater_than_or_equal_instruction(
        &mut self,
        input: &'a GreaterThanOrEqual,
        additional: &Self::AdditionalInput,
    ) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_is_equal_instruction(&mut self, input: &'a IsEqual, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_is_not_equal_instruction(
        &mut self,
        input: &'a IsNotEqual,
        additional: &Self::AdditionalInput,
    ) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_less_than_instruction(&mut self, input: &'a LessThan, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_less_than_or_equal_instruction(
        &mut self,
        input: &'a LessThanOrEqual,
        additional: &Self::AdditionalInput,
    ) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_mul_instruction(&mut self, input: &'a Mul, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_nop_instruction(&mut self, _input: &'a Nop, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_not_instruction(&mut self, input: &'a Not, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.operand, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_or_instruction(&mut self, input: &'a Or, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_sub_instruction(&mut self, input: &'a Sub, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }

    fn visit_ternary_instruction(&mut self, input: &'a Ternary, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_operand(&input.first, additional);
        self.visit_operand(&input.second, additional);
        self.visit_operand(&input.third, additional);
        self.visit_identifier(&input.destination, additional);
        Default::default()
    }
}

pub trait ExpressionVisitor<'a> {
    type AdditionalInput: Default;
    type Output: Default;

    fn visit_expression(&mut self, input: &'a Expression, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            Expression::Access(expr) => self.visit_access(expr, additional),
            Expression::CircuitInit(expr) => self.visit_circuit_init(expr, additional),
            Expression::Identifier(expr) => self.visit_identifier(expr, additional),
            Expression::Literal(expr) => self.visit_literal(expr, additional),
            Expression::Binary(expr) => self.visit_binary(expr, additional),
            Expression::Unary(expr) => self.visit_unary(expr, additional),
            Expression::Ternary(expr) => self.visit_ternary(expr, additional),
            Expression::Call(expr) => self.visit_call(expr, additional),
            Expression::Err(expr) => self.visit_err(expr, additional),
        }
    }

    fn visit_access(&mut self, _input: &'a AccessExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_circuit_init(
        &mut self,
        _input: &'a CircuitInitExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        Default::default()
    }

    fn visit_identifier(&mut self, _input: &'a Identifier, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_literal(&mut self, _input: &'a LiteralExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.left, additional);
        self.visit_expression(&input.right, additional);
        Default::default()
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.receiver, additional);
        Default::default()
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, additional);
        self.visit_expression(&input.if_true, additional);
        self.visit_expression(&input.if_false, additional);
        Default::default()
    }

    fn visit_call(&mut self, input: &'a CallExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_err(&mut self, _input: &'a ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }
}

pub trait StatementVisitor<'a>: ExpressionVisitor<'a> + InstructionVisitor<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        match input {
            Statement::AssemblyBlock(stmt) => self.visit_assembly_block(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assembly_block(&mut self, input: &'a AssemblyBlock) {
        input.instructions.iter().for_each(|inst| {
            self.visit_instruction(inst, &Default::default());
        });
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        self.visit_expression(&input.expression, &Default::default());
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());
        self.visit_block(&input.block);
        if let Some(stmt) = input.next.as_ref() {
            self.visit_statement(stmt);
        }
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.visit_block(&input.block);
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                self.visit_expression(expr, &Default::default());
            }
            ConsoleFunction::Error(fmt) | ConsoleFunction::Log(fmt) => {
                fmt.parameters.iter().for_each(|expr| {
                    self.visit_expression(expr, &Default::default());
                });
            }
        };
    }

    fn visit_block(&mut self, input: &'a Block) {
        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));
    }
}

pub trait ProgramVisitor<'a>: StatementVisitor<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        input
            .functions
            .values()
            .for_each(|function| self.visit_function(function));

        input
            .circuits
            .values()
            .for_each(|function| self.visit_circuit(function));
    }

    fn visit_function(&mut self, input: &'a Function) {
        self.visit_block(&input.block);
    }

    fn visit_circuit(&mut self, _input: &'a Circuit) {}
}
