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

use crate::*;

/// A Visitor trait for statements in the AST.
pub trait StatementVisitor<'a>: ExpressionVisitor<'a> + InstructionVisitor<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        match input {
            Statement::AssemblyBlock(stmt) => self.visit_assembly_block(stmt),
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Decrement(stmt) => self.visit_decrement(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Increment(stmt) => self.visit_increment(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assembly_block(&mut self, input: &'a AssemblyBlock) {
        input.instructions.iter().for_each(|inst| self.visit_instruction(inst));
    }

    fn visit_assert(&mut self, input: &'a AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => self.visit_expression(expr, &Default::default()),
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default())
            }
        };
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_block(&mut self, input: &'a Block) {
        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());
        self.visit_block(&input.then);
        if let Some(stmt) = input.otherwise.as_ref() {
            self.visit_statement(stmt);
        }
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                self.visit_expression(expr, &Default::default());
            }
            ConsoleFunction::AssertEq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default());
            }
            ConsoleFunction::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default());
            }
        };
    }

    fn visit_decrement(&mut self, input: &'a DecrementStatement) {
        self.visit_expression(&input.amount, &Default::default());
        self.visit_expression(&input.index, &Default::default());
        self.visit_identifier(&input.mapping, &Default::default());
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) {
        self.visit_expression(&input.expression, &Default::default());
    }

    fn visit_increment(&mut self, input: &'a IncrementStatement) {
        self.visit_expression(&input.amount, &Default::default());
        self.visit_expression(&input.index, &Default::default());
        self.visit_identifier(&input.mapping, &Default::default());
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.visit_block(&input.block);
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        self.visit_expression(&input.expression, &Default::default());
        if let Some(arguments) = &input.finalize_arguments {
            arguments.iter().for_each(|argument| {
                self.visit_expression(argument, &Default::default());
            })
        }
    }
}
