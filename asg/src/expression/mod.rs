// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! This module defines an expression node in an asg.
//!
//! Notable differences after conversion from an ast expression include:
//! 1. Storing variable references instead of variable identifiers - better history tracking and mutability
//! 2. Resolving constant values - optimizes execution of program circuit.

mod accesses;
pub use accesses::*;

mod array_inline;
pub use array_inline::*;

mod array_init;
pub use array_init::*;

mod binary;
pub use binary::*;

mod call;
pub use call::*;

mod circuit_init;
pub use circuit_init::*;

mod constant;
pub use constant::*;

mod ternary;
pub use ternary::*;

mod tuple_init;
pub use tuple_init::*;

mod unary;
pub use unary::*;

mod variable_ref;
pub use variable_ref::*;

mod cast;
pub use cast::*;

mod err;
pub use err::*;

use crate::{ConstValue, FromAst, Node, PartialType, Scope, Type};
use leo_errors::{Result, Span};

#[derive(Clone)]
pub enum Expression<'a> {
    VariableRef(VariableRef<'a>),
    Constant(Constant<'a>),
    Binary(BinaryExpression<'a>),
    Unary(UnaryExpression<'a>),
    Ternary(TernaryExpression<'a>),
    Cast(CastExpression<'a>),
    Access(AccessExpression<'a>),

    ArrayInline(ArrayInlineExpression<'a>),
    ArrayInit(ArrayInitExpression<'a>),

    TupleInit(TupleInitExpression<'a>),

    CircuitInit(CircuitInitExpression<'a>),

    Call(CallExpression<'a>),

    Err(ErrExpression<'a>),
}

impl<'a> Expression<'a> {
    pub fn ptr_eq(&self, other: &Expression<'a>) -> bool {
        std::ptr::eq(self as *const Expression<'a>, other as *const Expression<'a>)
    }

    pub fn get_id(&self) -> u32 {
        use Expression::*;
        match self {
            VariableRef(x) => x.id,
            Constant(x) => x.id,
            Binary(x) => x.id,
            Unary(x) => x.id,
            Ternary(x) => x.id,
            Cast(x) => x.id,
            Access(x) => x.get_id(),
            ArrayInline(x) => x.id,
            ArrayInit(x) => x.id,
            TupleInit(x) => x.id,
            CircuitInit(x) => x.id,
            Call(x) => x.id,
            Err(x) => x.id,
        }
    }
}

impl<'a> Node for Expression<'a> {
    fn span(&self) -> Option<&Span> {
        use Expression::*;
        match self {
            VariableRef(x) => x.span(),
            Constant(x) => x.span(),
            Binary(x) => x.span(),
            Unary(x) => x.span(),
            Ternary(x) => x.span(),
            Cast(x) => x.span(),
            Access(x) => x.span(),
            ArrayInline(x) => x.span(),
            ArrayInit(x) => x.span(),
            TupleInit(x) => x.span(),
            CircuitInit(x) => x.span(),
            Call(x) => x.span(),
            Err(x) => x.span(),
        }
    }
}

pub trait ExpressionNode<'a>: Node {
    fn set_parent(&self, parent: &'a Expression<'a>);
    fn get_parent(&self) -> Option<&'a Expression<'a>>;
    fn enforce_parents(&self, expr: &'a Expression<'a>);

    fn get_type(&'a self) -> Option<Type<'a>>;
    fn is_mut_ref(&self) -> bool;
    fn const_value(&'a self) -> Option<ConstValue>; // todo: memoize
    fn is_consty(&self) -> bool;
}

impl<'a> ExpressionNode<'a> for Expression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        use Expression::*;
        match self {
            VariableRef(x) => x.set_parent(parent),
            Constant(x) => x.set_parent(parent),
            Binary(x) => x.set_parent(parent),
            Unary(x) => x.set_parent(parent),
            Ternary(x) => x.set_parent(parent),
            Cast(x) => x.set_parent(parent),
            Access(x) => x.set_parent(parent),
            ArrayInline(x) => x.set_parent(parent),
            ArrayInit(x) => x.set_parent(parent),
            TupleInit(x) => x.set_parent(parent),
            CircuitInit(x) => x.set_parent(parent),
            Call(x) => x.set_parent(parent),
            Err(x) => x.set_parent(parent),
        }
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        use Expression::*;
        match self {
            VariableRef(x) => x.get_parent(),
            Constant(x) => x.get_parent(),
            Binary(x) => x.get_parent(),
            Unary(x) => x.get_parent(),
            Ternary(x) => x.get_parent(),
            Cast(x) => x.get_parent(),
            Access(x) => x.get_parent(),
            ArrayInline(x) => x.get_parent(),
            ArrayInit(x) => x.get_parent(),
            TupleInit(x) => x.get_parent(),
            CircuitInit(x) => x.get_parent(),
            Call(x) => x.get_parent(),
            Err(x) => x.get_parent(),
        }
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        use Expression::*;
        match self {
            VariableRef(x) => x.enforce_parents(expr),
            Constant(x) => x.enforce_parents(expr),
            Binary(x) => x.enforce_parents(expr),
            Unary(x) => x.enforce_parents(expr),
            Ternary(x) => x.enforce_parents(expr),
            Cast(x) => x.enforce_parents(expr),
            Access(x) => x.enforce_parents(expr),
            ArrayInline(x) => x.enforce_parents(expr),
            ArrayInit(x) => x.enforce_parents(expr),
            TupleInit(x) => x.enforce_parents(expr),
            CircuitInit(x) => x.enforce_parents(expr),
            Call(x) => x.enforce_parents(expr),
            Err(x) => x.enforce_parents(expr),
        }
    }

    fn get_type(&'a self) -> Option<Type<'a>> {
        use Expression::*;
        match self {
            VariableRef(x) => x.get_type(),
            Constant(x) => x.get_type(),
            Binary(x) => x.get_type(),
            Unary(x) => x.get_type(),
            Ternary(x) => x.get_type(),
            Cast(x) => x.get_type(),
            Access(x) => x.get_type(),
            ArrayInline(x) => x.get_type(),
            ArrayInit(x) => x.get_type(),
            TupleInit(x) => x.get_type(),
            CircuitInit(x) => x.get_type(),
            Call(x) => x.get_type(),
            Err(x) => x.get_type(),
        }
    }

    fn is_mut_ref(&self) -> bool {
        use Expression::*;
        match self {
            VariableRef(x) => x.is_mut_ref(),
            Constant(x) => x.is_mut_ref(),
            Binary(x) => x.is_mut_ref(),
            Unary(x) => x.is_mut_ref(),
            Ternary(x) => x.is_mut_ref(),
            Cast(x) => x.is_mut_ref(),
            Access(x) => x.is_mut_ref(),
            ArrayInline(x) => x.is_mut_ref(),
            ArrayInit(x) => x.is_mut_ref(),
            TupleInit(x) => x.is_mut_ref(),
            CircuitInit(x) => x.is_mut_ref(),
            Call(x) => x.is_mut_ref(),
            Err(x) => x.is_mut_ref(),
        }
    }

    fn const_value(&'a self) -> Option<ConstValue<'a>> {
        use Expression::*;
        match self {
            VariableRef(x) => x.const_value(),
            Constant(x) => x.const_value(),
            Binary(x) => x.const_value(),
            Unary(x) => x.const_value(),
            Ternary(x) => x.const_value(),
            Cast(x) => x.const_value(),
            Access(x) => x.const_value(),
            ArrayInline(x) => x.const_value(),
            ArrayInit(x) => x.const_value(),
            TupleInit(x) => x.const_value(),
            CircuitInit(x) => x.const_value(),
            Call(x) => x.const_value(),
            Err(x) => x.const_value(),
        }
    }

    fn is_consty(&self) -> bool {
        use Expression::*;
        match self {
            VariableRef(x) => x.is_consty(),
            Constant(x) => x.is_consty(),
            Binary(x) => x.is_consty(),
            Unary(x) => x.is_consty(),
            Ternary(x) => x.is_consty(),
            Cast(x) => x.is_consty(),
            Access(x) => x.is_consty(),
            ArrayInline(x) => x.is_consty(),
            ArrayInit(x) => x.is_consty(),
            TupleInit(x) => x.is_consty(),
            CircuitInit(x) => x.is_consty(),
            Call(x) => x.is_consty(),
            Err(x) => x.is_consty(),
        }
    }
}

impl<'a> FromAst<'a, leo_ast::Expression> for &'a Expression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::Expression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<Self> {
        use leo_ast::Expression::*;
        let expression = match value {
            Err(value) => scope
                .context
                .alloc_expression(ErrExpression::from_ast(scope, value, expected_type).map(Expression::Err)?),
            Identifier(identifier) => Self::from_ast(scope, identifier, expected_type)?,
            Value(value) => scope
                .context
                .alloc_expression(Constant::from_ast(scope, value, expected_type).map(Expression::Constant)?),
            Binary(binary) => scope
                .context
                .alloc_expression(BinaryExpression::from_ast(scope, binary, expected_type).map(Expression::Binary)?),
            Unary(unary) => scope
                .context
                .alloc_expression(UnaryExpression::from_ast(scope, unary, expected_type).map(Expression::Unary)?),
            Ternary(conditional) => scope.context.alloc_expression(
                TernaryExpression::from_ast(scope, conditional, expected_type).map(Expression::Ternary)?,
            ),
            Cast(cast) => scope
                .context
                .alloc_expression(CastExpression::from_ast(scope, cast, expected_type).map(Expression::Cast)?),
            Access(access) => scope
                .context
                .alloc_expression(AccessExpression::from_ast(scope, access, expected_type).map(Expression::Access)?),

            ArrayInline(array_inline) => scope.context.alloc_expression(
                ArrayInlineExpression::from_ast(scope, array_inline, expected_type).map(Expression::ArrayInline)?,
            ),
            ArrayInit(array_init) => scope.context.alloc_expression(
                ArrayInitExpression::from_ast(scope, array_init, expected_type).map(Expression::ArrayInit)?,
            ),

            TupleInit(tuple_init) => scope.context.alloc_expression(
                TupleInitExpression::from_ast(scope, tuple_init, expected_type).map(Expression::TupleInit)?,
            ),

            CircuitInit(circuit_init) => scope.context.alloc_expression(
                CircuitInitExpression::from_ast(scope, circuit_init, expected_type).map(Expression::CircuitInit)?,
            ),

            Call(call) => scope
                .context
                .alloc_expression(CallExpression::from_ast(scope, call, expected_type).map(Expression::Call)?),
        };
        expression.enforce_parents(expression);
        Ok(expression)
    }
}

impl<'a> Into<leo_ast::Expression> for &Expression<'a> {
    fn into(self) -> leo_ast::Expression {
        use Expression::*;
        match self {
            VariableRef(x) => leo_ast::Expression::Identifier(x.into()),
            Constant(x) => leo_ast::Expression::Value(x.into()),
            Binary(x) => leo_ast::Expression::Binary(x.into()),
            Unary(x) => leo_ast::Expression::Unary(x.into()),
            Ternary(x) => leo_ast::Expression::Ternary(x.into()),
            Cast(x) => leo_ast::Expression::Cast(x.into()),
            Access(x) => x.into(),
            ArrayInline(x) => leo_ast::Expression::ArrayInline(x.into()),
            ArrayInit(x) => leo_ast::Expression::ArrayInit(x.into()),
            TupleInit(x) => leo_ast::Expression::TupleInit(x.into()),
            CircuitInit(x) => leo_ast::Expression::CircuitInit(x.into()),
            Call(x) => leo_ast::Expression::Call(x.into()),
            Err(x) => leo_ast::Expression::Err(x.into()),
        }
    }
}
