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

use crate::TypeTable;

use leo_ast::{CoreFunction, Expression, ExpressionVisitor, Function, Node, StatementVisitor, Type};
use leo_errors::{StaticAnalyzerError, emitter::Handler};

/// Error if futures are used improperly.
///
/// This prevents, for instance, a bare call which creates an unused future.
pub fn future_check_function(function: &Function, type_table: &TypeTable, handler: &Handler) {
    let mut future_checker = FutureChecker { type_table, handler };
    future_checker.visit_block(&function.block);
}

#[derive(Clone, Copy, Debug, Default)]
enum Position {
    #[default]
    Misc,
    Await,
    TupleAccess,
    Return,
    FunctionArgument,
    LastTupleLiteral,
    Definition,
}

struct FutureChecker<'a> {
    type_table: &'a TypeTable,
    handler: &'a Handler,
}

impl<'a> FutureChecker<'a> {
    fn emit_err(&self, err: StaticAnalyzerError) {
        self.handler.emit_err(err);
    }
}

impl ExpressionVisitor for FutureChecker<'_> {
    type AdditionalInput = Position;
    type Output = ();

    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
        use Position::*;
        let is_call = matches!(input, Expression::Call(..));
        match self.type_table.get(&input.id()) {
            Some(Type::Future(..)) if is_call => {
                // A call producing a Future may appear in any of these positions.
                if !matches!(additional, Await | Return | FunctionArgument | LastTupleLiteral | Definition) {
                    self.emit_err(StaticAnalyzerError::misplaced_future(input.span()));
                }
            }
            Some(Type::Future(..)) => {
                // A Future expression that's not a call may appear in any of these positions.
                if !matches!(additional, Await | Return | FunctionArgument | LastTupleLiteral | TupleAccess) {
                    self.emit_err(StaticAnalyzerError::misplaced_future(input.span()));
                }
            }
            Some(Type::Tuple(tuple)) if !matches!(tuple.elements().last(), Some(Type::Future(_))) => {}
            Some(Type::Tuple(..)) if is_call => {
                // A call producing a Tuple ending in a Future may appear in any of these positions.
                if !matches!(additional, Return | Definition) {
                    self.emit_err(StaticAnalyzerError::misplaced_future(input.span()));
                }
            }
            Some(Type::Tuple(..)) => {
                // A Tuple ending in a Future that's not a call may appear in any of these positions.
                if !matches!(additional, Return | TupleAccess) {
                    self.emit_err(StaticAnalyzerError::misplaced_future(input.span()));
                }
            }
            _ => {}
        }

        match input {
            Expression::Access(access) => self.visit_access(access, &Position::Misc),
            Expression::Array(array) => self.visit_array(array, &Position::Misc),
            Expression::AssociatedConstant(associated_constant) => {
                self.visit_associated_constant(associated_constant, &Position::Misc)
            }
            Expression::AssociatedFunction(associated_function) => {
                self.visit_associated_function(associated_function, &Position::Misc)
            }
            Expression::Binary(binary) => self.visit_binary(binary, &Position::Misc),
            Expression::Call(call) => self.visit_call(call, &Position::Misc),
            Expression::Cast(cast) => self.visit_cast(cast, &Position::Misc),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, &Position::Misc),
            Expression::Err(err) => self.visit_err(err, &Position::Misc),
            Expression::Identifier(identifier) => self.visit_identifier(identifier, &Position::Misc),
            Expression::Literal(literal) => self.visit_literal(literal, &Position::Misc),
            Expression::Locator(locator) => self.visit_locator(locator, &Position::Misc),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, &Position::Misc),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::Unary(unary) => self.visit_unary(unary, &Position::Misc),
            Expression::Unit(unit) => self.visit_unit(unit, &Position::Misc),
        }
    }

    fn visit_access(&mut self, input: &leo_ast::AccessExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            leo_ast::AccessExpression::Array(array) => {
                self.visit_expression(&array.array, &Position::Misc);
                self.visit_expression(&array.index, &Position::Misc);
            }
            leo_ast::AccessExpression::Member(member) => {
                self.visit_expression(&member.inner, &Position::Misc);
            }
            leo_ast::AccessExpression::Tuple(tuple) => {
                self.visit_expression(&tuple.tuple, &Position::TupleAccess);
            }
        }

        Default::default()
    }

    fn visit_associated_function(
        &mut self,
        input: &leo_ast::AssociatedFunctionExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        let core_function = CoreFunction::from_symbols(input.variant.name, input.name.name)
            .expect("Typechecking guarantees that this function exists.");
        let position = if core_function == CoreFunction::FutureAwait { Position::Await } else { Position::Misc };
        input.arguments.iter().for_each(|arg| {
            self.visit_expression(arg, &position);
        });
    }

    fn visit_call(&mut self, input: &leo_ast::CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Position::FunctionArgument);
        });
        Default::default()
    }

    fn visit_tuple(&mut self, input: &leo_ast::TupleExpression, additional: &Self::AdditionalInput) -> Self::Output {
        let next_position = match additional {
            Position::Definition | Position::Return => Position::LastTupleLiteral,
            _ => Position::Misc,
        };
        let mut iter = input.elements.iter().peekable();
        while let Some(expr) = iter.next() {
            let position = if iter.peek().is_some() { &Position::Misc } else { &next_position };
            self.visit_expression(expr, position);
        }
        Default::default()
    }
}

impl StatementVisitor for FutureChecker<'_> {
    fn visit_definition(&mut self, input: &leo_ast::DefinitionStatement) {
        self.visit_expression(&input.value, &Position::Definition);
    }

    fn visit_return(&mut self, input: &leo_ast::ReturnStatement) {
        self.visit_expression(&input.expression, &Position::Return);
    }
}
