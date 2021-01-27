// Copyright (C) 2019-2020 Aleo Systems Inc.
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

mod variable_ref;
pub use variable_ref::*;
mod constant;
pub use constant::*;
mod binary;
pub use binary::*;
mod unary;
pub use unary::*;
mod ternary;
pub use ternary::*;
mod array_inline;
pub use array_inline::*;
mod array_init;
pub use array_init::*;
mod array_access;
pub use array_access::*;
mod array_range_access;
pub use array_range_access::*;
mod tuple_init;
pub use tuple_init::*;
mod tuple_access;
pub use tuple_access::*;
mod circuit_init;
pub use circuit_init::*;
mod circuit_access;
pub use circuit_access::*;
mod call;
pub use call::*;

use crate::{AsgConvertError, ConstValue, FromAst, Node, PartialType, Scope, Span, Type};
use std::sync::{Arc, Weak};

pub enum Expression {
    VariableRef(VariableRef),
    Constant(Constant),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),

    ArrayInline(ArrayInlineExpression),
    ArrayInit(ArrayInitExpression),
    ArrayAccess(ArrayAccessExpression),
    ArrayRangeAccess(ArrayRangeAccessExpression),

    TupleInit(TupleInitExpression),
    TupleAccess(TupleAccessExpression),

    CircuitInit(CircuitInitExpression),
    CircuitAccess(CircuitAccessExpression),

    Call(CallExpression),
}

impl Node for Expression {
    fn span(&self) -> Option<&Span> {
        use Expression::*;
        match self {
            VariableRef(x) => x.span(),
            Constant(x) => x.span(),
            Binary(x) => x.span(),
            Unary(x) => x.span(),
            Ternary(x) => x.span(),
            ArrayInline(x) => x.span(),
            ArrayInit(x) => x.span(),
            ArrayAccess(x) => x.span(),
            ArrayRangeAccess(x) => x.span(),
            TupleInit(x) => x.span(),
            TupleAccess(x) => x.span(),
            CircuitInit(x) => x.span(),
            CircuitAccess(x) => x.span(),
            Call(x) => x.span(),
        }
    }
}

pub trait ExpressionNode: Node {
    fn set_parent(&self, parent: Weak<Expression>);
    fn get_parent(&self) -> Option<Arc<Expression>>;
    fn enforce_parents(&self, expr: &Arc<Expression>);

    fn get_type(&self) -> Option<Type>;
    fn is_mut_ref(&self) -> bool;
    fn const_value(&self) -> Option<ConstValue>; // todo: memoize
    fn is_consty(&self) -> bool;
}

impl ExpressionNode for Expression {
    fn set_parent(&self, parent: Weak<Expression>) {
        use Expression::*;
        match self {
            VariableRef(x) => x.set_parent(parent),
            Constant(x) => x.set_parent(parent),
            Binary(x) => x.set_parent(parent),
            Unary(x) => x.set_parent(parent),
            Ternary(x) => x.set_parent(parent),
            ArrayInline(x) => x.set_parent(parent),
            ArrayInit(x) => x.set_parent(parent),
            ArrayAccess(x) => x.set_parent(parent),
            ArrayRangeAccess(x) => x.set_parent(parent),
            TupleInit(x) => x.set_parent(parent),
            TupleAccess(x) => x.set_parent(parent),
            CircuitInit(x) => x.set_parent(parent),
            CircuitAccess(x) => x.set_parent(parent),
            Call(x) => x.set_parent(parent),
        }
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        use Expression::*;
        match self {
            VariableRef(x) => x.get_parent(),
            Constant(x) => x.get_parent(),
            Binary(x) => x.get_parent(),
            Unary(x) => x.get_parent(),
            Ternary(x) => x.get_parent(),
            ArrayInline(x) => x.get_parent(),
            ArrayInit(x) => x.get_parent(),
            ArrayAccess(x) => x.get_parent(),
            ArrayRangeAccess(x) => x.get_parent(),
            TupleInit(x) => x.get_parent(),
            TupleAccess(x) => x.get_parent(),
            CircuitInit(x) => x.get_parent(),
            CircuitAccess(x) => x.get_parent(),
            Call(x) => x.get_parent(),
        }
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        use Expression::*;
        match self {
            VariableRef(x) => x.enforce_parents(expr),
            Constant(x) => x.enforce_parents(expr),
            Binary(x) => x.enforce_parents(expr),
            Unary(x) => x.enforce_parents(expr),
            Ternary(x) => x.enforce_parents(expr),
            ArrayInline(x) => x.enforce_parents(expr),
            ArrayInit(x) => x.enforce_parents(expr),
            ArrayAccess(x) => x.enforce_parents(expr),
            ArrayRangeAccess(x) => x.enforce_parents(expr),
            TupleInit(x) => x.enforce_parents(expr),
            TupleAccess(x) => x.enforce_parents(expr),
            CircuitInit(x) => x.enforce_parents(expr),
            CircuitAccess(x) => x.enforce_parents(expr),
            Call(x) => x.enforce_parents(expr),
        }
    }

    fn get_type(&self) -> Option<Type> {
        use Expression::*;
        match self {
            VariableRef(x) => x.get_type(),
            Constant(x) => x.get_type(),
            Binary(x) => x.get_type(),
            Unary(x) => x.get_type(),
            Ternary(x) => x.get_type(),
            ArrayInline(x) => x.get_type(),
            ArrayInit(x) => x.get_type(),
            ArrayAccess(x) => x.get_type(),
            ArrayRangeAccess(x) => x.get_type(),
            TupleInit(x) => x.get_type(),
            TupleAccess(x) => x.get_type(),
            CircuitInit(x) => x.get_type(),
            CircuitAccess(x) => x.get_type(),
            Call(x) => x.get_type(),
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
            ArrayInline(x) => x.is_mut_ref(),
            ArrayInit(x) => x.is_mut_ref(),
            ArrayAccess(x) => x.is_mut_ref(),
            ArrayRangeAccess(x) => x.is_mut_ref(),
            TupleInit(x) => x.is_mut_ref(),
            TupleAccess(x) => x.is_mut_ref(),
            CircuitInit(x) => x.is_mut_ref(),
            CircuitAccess(x) => x.is_mut_ref(),
            Call(x) => x.is_mut_ref(),
        }
    }

    fn const_value(&self) -> Option<ConstValue> {
        use Expression::*;
        match self {
            VariableRef(x) => x.const_value(),
            Constant(x) => x.const_value(),
            Binary(x) => x.const_value(),
            Unary(x) => x.const_value(),
            Ternary(x) => x.const_value(),
            ArrayInline(x) => x.const_value(),
            ArrayInit(x) => x.const_value(),
            ArrayAccess(x) => x.const_value(),
            ArrayRangeAccess(x) => x.const_value(),
            TupleInit(x) => x.const_value(),
            TupleAccess(x) => x.const_value(),
            CircuitInit(x) => x.const_value(),
            CircuitAccess(x) => x.const_value(),
            Call(x) => x.const_value(),
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
            ArrayInline(x) => x.is_consty(),
            ArrayInit(x) => x.is_consty(),
            ArrayAccess(x) => x.is_consty(),
            ArrayRangeAccess(x) => x.is_consty(),
            TupleInit(x) => x.is_consty(),
            TupleAccess(x) => x.is_consty(),
            CircuitInit(x) => x.is_consty(),
            CircuitAccess(x) => x.is_consty(),
            Call(x) => x.is_consty(),
        }
    }
}

impl FromAst<leo_ast::Expression> for Arc<Expression> {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::Expression,
        expected_type: Option<PartialType>,
    ) -> Result<Self, AsgConvertError> {
        use leo_ast::Expression::*;
        let expression = match value {
            Identifier(identifier) => Self::from_ast(scope, identifier, expected_type)?,
            Value(value) => Arc::new(Constant::from_ast(scope, value, expected_type).map(Expression::Constant)?),
            Binary(binary) => {
                Arc::new(BinaryExpression::from_ast(scope, binary, expected_type).map(Expression::Binary)?)
            }
            Unary(unary) => Arc::new(UnaryExpression::from_ast(scope, unary, expected_type).map(Expression::Unary)?),
            Ternary(conditional) => {
                Arc::new(TernaryExpression::from_ast(scope, conditional, expected_type).map(Expression::Ternary)?)
            }

            ArrayInline(array_inline) => Arc::new(
                ArrayInlineExpression::from_ast(scope, array_inline, expected_type).map(Expression::ArrayInline)?,
            ),
            ArrayInit(array_init) => {
                Arc::new(ArrayInitExpression::from_ast(scope, array_init, expected_type).map(Expression::ArrayInit)?)
            }
            ArrayAccess(array_access) => Arc::new(
                ArrayAccessExpression::from_ast(scope, array_access, expected_type).map(Expression::ArrayAccess)?,
            ),
            ArrayRangeAccess(array_range_access) => Arc::new(
                ArrayRangeAccessExpression::from_ast(scope, array_range_access, expected_type)
                    .map(Expression::ArrayRangeAccess)?,
            ),

            TupleInit(tuple_init) => {
                Arc::new(TupleInitExpression::from_ast(scope, tuple_init, expected_type).map(Expression::TupleInit)?)
            }
            TupleAccess(tuple_access) => Arc::new(
                TupleAccessExpression::from_ast(scope, tuple_access, expected_type).map(Expression::TupleAccess)?,
            ),

            CircuitInit(circuit_init) => Arc::new(
                CircuitInitExpression::from_ast(scope, circuit_init, expected_type).map(Expression::CircuitInit)?,
            ),
            CircuitMemberAccess(circuit_member) => Arc::new(
                CircuitAccessExpression::from_ast(scope, circuit_member, expected_type)
                    .map(Expression::CircuitAccess)?,
            ),
            CircuitStaticFunctionAccess(circuit_member) => Arc::new(
                CircuitAccessExpression::from_ast(scope, circuit_member, expected_type)
                    .map(Expression::CircuitAccess)?,
            ),

            Call(call) => Arc::new(CallExpression::from_ast(scope, call, expected_type).map(Expression::Call)?),
        };
        expression.enforce_parents(&expression);
        Ok(expression)
    }
}

impl Into<leo_ast::Expression> for &Expression {
    fn into(self) -> leo_ast::Expression {
        use Expression::*;
        match self {
            VariableRef(x) => leo_ast::Expression::Identifier(x.into()),
            Constant(x) => leo_ast::Expression::Value(x.into()),
            Binary(x) => leo_ast::Expression::Binary(x.into()),
            Unary(x) => leo_ast::Expression::Unary(x.into()),
            Ternary(x) => leo_ast::Expression::Ternary(x.into()),
            ArrayInline(x) => leo_ast::Expression::ArrayInline(x.into()),
            ArrayInit(x) => leo_ast::Expression::ArrayInit(x.into()),
            ArrayAccess(x) => leo_ast::Expression::ArrayAccess(x.into()),
            ArrayRangeAccess(x) => leo_ast::Expression::ArrayRangeAccess(x.into()),
            TupleInit(x) => leo_ast::Expression::TupleInit(x.into()),
            TupleAccess(x) => leo_ast::Expression::TupleAccess(x.into()),
            CircuitInit(x) => leo_ast::Expression::CircuitInit(x.into()),
            CircuitAccess(x) => x.into(),
            Call(x) => leo_ast::Expression::Call(x.into()),
        }
    }
}
