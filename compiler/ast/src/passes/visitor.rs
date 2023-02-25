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
    type ExpressionOutput: Default;

    fn visit_expression(
        &mut self,
        input: &'a Expression,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        match input {
            Expression::Access(access) => self.visit_access(access, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
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

    fn visit_access(
        &mut self,
        input: &'a AccessExpression,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        match input {
            AccessExpression::AssociatedFunction(function) => self.visit_associated_function(function, additional),
            AccessExpression::Member(member) => self.visit_member_access(member, additional),
            AccessExpression::Tuple(tuple) => self.visit_tuple_access(tuple, additional),
        };
        Default::default()
    }

    fn visit_associated_function(
        &mut self,
        input: &'a AssociatedFunction,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        input.args.iter().for_each(|arg| {
            self.visit_expression(arg, additional);
        });
        Default::default()
    }

    fn visit_binary(
        &mut self,
        input: &'a BinaryExpression,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        self.visit_expression(&input.left, additional);
        self.visit_expression(&input.right, additional);
        Default::default()
    }

    fn visit_call(&mut self, input: &'a CallExpression, additional: &Self::AdditionalInput) -> Self::ExpressionOutput {
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_struct_init(
        &mut self,
        _input: &'a StructExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        Default::default()
    }

    fn visit_err(&mut self, _input: &'a ErrExpression, _additional: &Self::AdditionalInput) -> Self::ExpressionOutput {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_identifier(
        &mut self,
        _input: &'a Identifier,
        _additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        Default::default()
    }

    fn visit_literal(&mut self, _input: &'a Literal, _additional: &Self::AdditionalInput) -> Self::ExpressionOutput {
        Default::default()
    }

    fn visit_member_access(
        &mut self,
        input: &'a MemberAccess,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        self.visit_expression(&input.inner, additional);
        Default::default()
    }

    fn visit_ternary(
        &mut self,
        input: &'a TernaryExpression,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        self.visit_expression(&input.condition, additional);
        self.visit_expression(&input.if_true, additional);
        self.visit_expression(&input.if_false, additional);
        Default::default()
    }

    fn visit_tuple(
        &mut self,
        input: &'a TupleExpression,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        input.elements.iter().for_each(|expr| {
            self.visit_expression(expr, additional);
        });
        Default::default()
    }

    fn visit_tuple_access(
        &mut self,
        input: &'a TupleAccess,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        self.visit_expression(&input.tuple, additional);
        Default::default()
    }

    fn visit_unary(
        &mut self,
        input: &'a UnaryExpression,
        additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        self.visit_expression(&input.receiver, additional);
        Default::default()
    }

    fn visit_unit(
        &mut self,
        _input: &'a UnitExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::ExpressionOutput {
        Default::default()
    }
}

/// A Visitor trait for statements in the AST.
pub trait StatementVisitor<'a>: ExpressionVisitor<'a> {
    type StatementOutput: Default;

    fn visit_statement(&mut self, input: &'a Statement) -> Self::StatementOutput {
        match input {
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

    fn visit_assert(&mut self, input: &'a AssertStatement) -> Self::StatementOutput {
        match &input.variant {
            AssertVariant::Assert(expr) => self.visit_expression(expr, &Default::default()),
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default())
            }
        };
        Default::default()
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) -> Self::StatementOutput {
        self.visit_expression(&input.value, &Default::default());
        Default::default()
    }

    fn visit_block(&mut self, input: &'a Block) -> Self::StatementOutput {
        input.statements.iter().for_each(|stmt| {
            self.visit_statement(stmt);
        });
        Default::default()
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) -> Self::StatementOutput {
        self.visit_expression(&input.condition, &Default::default());
        self.visit_block(&input.then);
        if let Some(stmt) = input.otherwise.as_ref() {
            self.visit_statement(stmt);
        }
        Default::default()
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) -> Self::StatementOutput {
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
        Default::default()
    }

    fn visit_decrement(&mut self, input: &'a DecrementStatement) -> Self::StatementOutput {
        self.visit_expression(&input.amount, &Default::default());
        self.visit_expression(&input.index, &Default::default());
        self.visit_identifier(&input.mapping, &Default::default());
        Default::default()
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) -> Self::StatementOutput {
        self.visit_expression(&input.value, &Default::default());
        Default::default()
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) -> Self::StatementOutput {
        self.visit_expression(&input.expression, &Default::default());
        Default::default()
    }

    fn visit_increment(&mut self, input: &'a IncrementStatement) -> Self::StatementOutput {
        self.visit_expression(&input.amount, &Default::default());
        self.visit_expression(&input.index, &Default::default());
        self.visit_identifier(&input.mapping, &Default::default());
        Default::default()
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) -> Self::StatementOutput {
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        self.visit_block(&input.block);
        Default::default()
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) -> Self::StatementOutput {
        self.visit_expression(&input.expression, &Default::default());
        if let Some(arguments) = &input.finalize_arguments {
            arguments.iter().for_each(|argument| {
                self.visit_expression(argument, &Default::default());
            })
        }
        Default::default()
    }
}

/// A Visitor trait for the program represented by the AST.
pub trait ProgramVisitor<'a>: StatementVisitor<'a> {
    type ProgramOutput: Default;

    fn visit_program(&mut self, input: &'a Program) -> Self::ProgramOutput {
        input.imports.values().for_each(|import| {
            self.visit_import(&import.0);
        });

        input.program_scopes.values().for_each(|scope| {
            self.visit_program_scope(scope);
        });

        Default::default()
    }

    fn visit_program_scope(&mut self, input: &'a ProgramScope) -> Self::ProgramOutput {
        input.structs.values().for_each(|function| {
            self.visit_struct(function);
        });

        input.mappings.values().for_each(|mapping| {
            self.visit_mapping(mapping);
        });

        input.functions.values().for_each(|function| {
            self.visit_function(function);
        });

        Default::default()
    }

    fn visit_import(&mut self, input: &'a Program) -> Self::ProgramOutput {
        self.visit_program(input);
        Default::default()
    }

    fn visit_struct(&mut self, _input: &'a Struct) -> Self::ProgramOutput {
        Default::default()
    }

    fn visit_mapping(&mut self, _input: &'a Mapping) -> Self::ProgramOutput {
        Default::default()
    }

    fn visit_function(&mut self, input: &'a Function) -> Self::ProgramOutput {
        self.visit_block(&input.block);
        if let Some(finalize) = &input.finalize {
            self.visit_block(&finalize.block);
        }
        Default::default()
    }
}
