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

    fn visit_access(&mut self, input: &'a AccessExpression, additional: &Self::AdditionalInput) -> Self::Output {
        match input {
            AccessExpression::AssociatedFunction(function) => {
                function.args.iter().for_each(|arg| {
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
