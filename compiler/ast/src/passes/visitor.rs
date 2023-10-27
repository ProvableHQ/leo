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

//! This module contains Visitor trait implementations for the AST.
//! It implements default methods for each node to be made
//! given the type of node its visiting.

use crate::*;

/// A Visitor trait for expressions in the AST.
pub trait ExpressionVisitor<'a> {
    type AdditionalInput: Default;
    type Output: Default;

    fn visit_expression(&mut self, input: &'a Expression, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            Expression::Access(access) => self.visit_access(access, additional),
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Identifier(identifier) => self.visit_identifier(identifier, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, additional),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::Unary(unary) => self.visit_unary(unary, additional),
            Expression::Unit(unit) => self.visit_unit(unit, additional),
        }
    }

    fn visit_access(&mut self, input: &'a AccessExpression, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            AccessExpression::Array(array) => {
                self.visit_expression(&array.array, additional);
                self.visit_expression(&array.index, additional);
            }
            AccessExpression::AssociatedFunction(function) => {
                function.arguments.iter().for_each(|arg| {
                    self.visit_expression(arg, &Default::default());
                });
            }
            AccessExpression::Member(member) => {
                self.visit_expression(&member.inner, additional);
            }
            AccessExpression::Tuple(tuple) => {
                self.visit_expression(&tuple.tuple, additional);
            }
            _ => {}
        }

        Default::default()
    }

    fn visit_array(&mut self, input: &'a ArrayExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.left, additional);
        self.visit_expression(&input.right, additional);
        Default::default()
    }

    fn visit_call(&mut self, input: &'a CallExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_cast(&mut self, input: &'a CastExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.expression, additional);
        Default::default()
    }

    fn visit_struct_init(&mut self, _input: &'a StructExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_err(&mut self, _input: &'a ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_identifier(&mut self, _input: &'a Identifier, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_literal(&mut self, _input: &'a Literal, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, additional);
        self.visit_expression(&input.if_true, additional);
        self.visit_expression(&input.if_false, additional);
        Default::default()
    }

    fn visit_tuple(&mut self, input: &'a TupleExpression, additional: &Self::AdditionalInput) -> Self::Output {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, additional: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.receiver, additional);
        Default::default()
    }

    fn visit_unit(&mut self, _input: &'a UnitExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }
}

/// A Visitor trait for statements in the AST.
pub trait StatementVisitor<'a>: ExpressionVisitor<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Const(stmt) => self.visit_const(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
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

    fn visit_const(&mut self, input: &'a ConstDeclaration) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) {
        self.visit_expression(&input.expression, &Default::default());
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

/// A Visitor trait for the program represented by the AST.
pub trait ProgramVisitor<'a>: StatementVisitor<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        input.imports.values().for_each(|import| self.visit_import(&import.0));

        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &'a ProgramScope) {
        input.structs.iter().for_each(|(_, c)| (self.visit_struct(c)));

        input.mappings.iter().for_each(|(_, c)| (self.visit_mapping(c)));

        input.functions.iter().for_each(|(_, c)| (self.visit_function(c)));

        input.consts.iter().for_each(|(_, c)| (self.visit_const(c)));
    }

    fn visit_import(&mut self, input: &'a Program) {
        self.visit_program(input)
    }

    fn visit_struct(&mut self, _input: &'a Struct) {}

    fn visit_mapping(&mut self, _input: &'a Mapping) {}

    fn visit_function(&mut self, input: &'a Function) {
        self.visit_block(&input.block);
        if let Some(finalize) = &input.finalize {
            self.visit_block(&finalize.block);
        }
    }
}
