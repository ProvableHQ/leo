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

//! This module contains a Consumer trait for the AST.
//! Consumers are used to completely transform the AST without any restrictions on the output.

use crate::*;

/// A Consumer trait for expressions in the AST.
pub trait ExpressionConsumer {
    type Output;

    fn consume_expression(&mut self, input: Expression) -> Self::Output {
        match input {
            Expression::Access(access) => self.consume_access(access),
            Expression::Array(array) => self.consume_array(array),
            Expression::Binary(binary) => self.consume_binary(binary),
            Expression::Call(call) => self.consume_call(call),
            Expression::Cast(cast) => self.consume_cast(cast),
            Expression::Struct(struct_) => self.consume_struct_init(struct_),
            Expression::Err(err) => self.consume_err(err),
            Expression::Identifier(identifier) => self.consume_identifier(identifier),
            Expression::Literal(value) => self.consume_literal(value),
            Expression::Ternary(ternary) => self.consume_ternary(ternary),
            Expression::Tuple(tuple) => self.consume_tuple(tuple),
            Expression::Unary(unary) => self.consume_unary(unary),
            Expression::Unit(unit) => self.consume_unit(unit),
        }
    }

    fn consume_access(&mut self, _input: AccessExpression) -> Self::Output;

    fn consume_array(&mut self, _input: ArrayExpression) -> Self::Output;

    fn consume_binary(&mut self, _input: BinaryExpression) -> Self::Output;

    fn consume_call(&mut self, _input: CallExpression) -> Self::Output;

    fn consume_cast(&mut self, _input: CastExpression) -> Self::Output;

    fn consume_struct_init(&mut self, _input: StructExpression) -> Self::Output;

    fn consume_err(&mut self, _input: ErrExpression) -> Self::Output {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn consume_identifier(&mut self, _input: Identifier) -> Self::Output;

    fn consume_literal(&mut self, _input: Literal) -> Self::Output;

    fn consume_ternary(&mut self, _input: TernaryExpression) -> Self::Output;

    fn consume_tuple(&mut self, _input: TupleExpression) -> Self::Output;

    fn consume_unary(&mut self, _input: UnaryExpression) -> Self::Output;

    fn consume_unit(&mut self, _input: UnitExpression) -> Self::Output;
}

/// A Consumer trait for statements in the AST.
pub trait StatementConsumer {
    type Output;

    fn consume_statement(&mut self, input: Statement) -> Self::Output {
        match input {
            Statement::Assert(assert) => self.consume_assert(assert),
            Statement::Assign(stmt) => self.consume_assign(*stmt),
            Statement::Block(stmt) => self.consume_block(stmt),
            Statement::Conditional(stmt) => self.consume_conditional(stmt),
            Statement::Console(stmt) => self.consume_console(stmt),
            Statement::Const(stmt) => self.consume_const(stmt),
            Statement::Definition(stmt) => self.consume_definition(stmt),
            Statement::Expression(stmt) => self.consume_expression_statement(stmt),
            Statement::Iteration(stmt) => self.consume_iteration(*stmt),
            Statement::Return(stmt) => self.consume_return(stmt),
        }
    }

    fn consume_assert(&mut self, input: AssertStatement) -> Self::Output;

    fn consume_assign(&mut self, input: AssignStatement) -> Self::Output;

    fn consume_block(&mut self, input: Block) -> Self::Output;

    fn consume_conditional(&mut self, input: ConditionalStatement) -> Self::Output;

    fn consume_console(&mut self, input: ConsoleStatement) -> Self::Output;

    fn consume_const(&mut self, input: ConstDeclaration) -> Self::Output;

    fn consume_definition(&mut self, input: DefinitionStatement) -> Self::Output;

    fn consume_expression_statement(&mut self, input: ExpressionStatement) -> Self::Output;

    fn consume_iteration(&mut self, input: IterationStatement) -> Self::Output;

    fn consume_return(&mut self, input: ReturnStatement) -> Self::Output;
}

/// A Consumer trait for functions in the AST.
pub trait FunctionConsumer {
    type Output;

    fn consume_function(&mut self, input: Function) -> Self::Output;
}

/// A Consumer trait for structs in the AST.
pub trait StructConsumer {
    type Output;

    fn consume_struct(&mut self, input: Struct) -> Self::Output;
}

/// A Consumer trait for imported programs in the AST.
pub trait ImportConsumer {
    type Output;

    fn consume_import(&mut self, input: Program) -> Self::Output;
}

/// A Consumer trait for mappings in the AST.
pub trait MappingConsumer {
    type Output;

    fn consume_mapping(&mut self, input: Mapping) -> Self::Output;
}

/// A Consumer trait for program scopes in the AST.
pub trait ProgramScopeConsumer {
    type Output;

    fn consume_program_scope(&mut self, input: ProgramScope) -> Self::Output;
}

/// A Consumer trait for the program represented by the AST.
pub trait ProgramConsumer {
    type Output;
    fn consume_program(&mut self, input: Program) -> Self::Output;
}
