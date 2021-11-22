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

use crate::{accesses::*, ConstValue, Expression, ExpressionNode, FromAst, Node, PartialType, Scope, Type};
use leo_errors::{Result, Span};

#[derive(Clone)]
pub enum AccessExpression<'a> {
    Array(ArrayAccess<'a>),
    ArrayRange(ArrayRangeAccess<'a>),
    Struct(StructAccess<'a>),
    Tuple(TupleAccess<'a>),
}

impl<'a> Node for AccessExpression<'a> {
    fn span(&self) -> Option<&Span> {
        use AccessExpression::*;

        match self {
            Array(access) => access.span(),
            ArrayRange(access) => access.span(),
            Struct(access) => access.span(),
            Tuple(access) => access.span(),
        }
    }
}

impl<'a> ExpressionNode<'a> for AccessExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        use AccessExpression::*;

        match self {
            Array(access) => access.set_parent(parent),
            ArrayRange(access) => access.set_parent(parent),
            Struct(access) => access.set_parent(parent),
            Tuple(access) => access.set_parent(parent),
        }
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        use AccessExpression::*;

        match self {
            Array(access) => access.get_parent(),
            ArrayRange(access) => access.get_parent(),
            Struct(access) => access.get_parent(),
            Tuple(access) => access.get_parent(),
        }
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        use AccessExpression::*;

        match self {
            Array(access) => access.enforce_parents(expr),
            ArrayRange(access) => access.enforce_parents(expr),
            Struct(access) => access.enforce_parents(expr),
            Tuple(access) => access.enforce_parents(expr),
        }
    }

    fn get_type(&'a self) -> Option<Type<'a>> {
        use AccessExpression::*;

        match self {
            Array(access) => access.get_type(),
            ArrayRange(access) => access.get_type(),
            Struct(access) => access.get_type(),
            Tuple(access) => access.get_type(),
        }
    }

    fn is_mut_ref(&self) -> bool {
        use AccessExpression::*;

        match self {
            Array(access) => access.is_mut_ref(),
            ArrayRange(access) => access.is_mut_ref(),
            Struct(access) => access.is_mut_ref(),
            Tuple(access) => access.is_mut_ref(),
        }
    }

    fn const_value(&'a self) -> Option<ConstValue> {
        use AccessExpression::*;

        match self {
            Array(access) => access.const_value(),
            ArrayRange(access) => access.const_value(),
            Struct(access) => access.const_value(),
            Tuple(access) => access.const_value(),
        }
    }

    fn is_consty(&self) -> bool {
        use AccessExpression::*;

        match self {
            Array(access) => access.is_consty(),
            ArrayRange(access) => access.is_consty(),
            Struct(access) => access.is_consty(),
            Tuple(access) => access.is_consty(),
        }
    }
}

impl<'a> FromAst<'a, leo_ast::AccessExpression> for AccessExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::AccessExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<AccessExpression<'a>> {
        use leo_ast::AccessExpression::*;

        match value {
            Array(access) => ArrayAccess::from_ast(scope, access, expected_type).map(AccessExpression::Array),
            ArrayRange(access) => {
                ArrayRangeAccess::from_ast(scope, access, expected_type).map(AccessExpression::ArrayRange)
            }
            Member(access) => StructAccess::from_ast(scope, access, expected_type).map(AccessExpression::Struct),
            Tuple(access) => TupleAccess::from_ast(scope, access, expected_type).map(AccessExpression::Tuple),
            Static(access) => StructAccess::from_ast(scope, access, expected_type).map(AccessExpression::Struct),
        }
    }
}

impl<'a> Into<leo_ast::Expression> for &AccessExpression<'a> {
    fn into(self) -> leo_ast::Expression {
        use AccessExpression::*;

        match self {
            Array(access) => leo_ast::Expression::Access(leo_ast::AccessExpression::Array(access.into())),
            ArrayRange(access) => leo_ast::Expression::Access(leo_ast::AccessExpression::ArrayRange(access.into())),
            Struct(access) => access.into(),
            Tuple(access) => leo_ast::Expression::Access(leo_ast::AccessExpression::Tuple(access.into())),
        }
    }
}
