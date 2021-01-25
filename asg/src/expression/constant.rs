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

use crate::{
    AsgConvertError,
    ConstInt,
    ConstValue,
    Expression,
    ExpressionNode,
    FromAst,
    GroupValue,
    Node,
    PartialType,
    Scope,
    Span,
    Type,
};
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct Constant {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub value: ConstValue, // should not be compound constants
}

impl Node for Constant {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for Constant {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, _expr: &Arc<Expression>) {}

    fn get_type(&self) -> Option<Type> {
        self.value.get_type()
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        Some(self.value.clone())
    }
}

impl FromAst<leo_ast::ValueExpression> for Constant {
    fn from_ast(
        _scope: &Scope,
        value: &leo_ast::ValueExpression,
        expected_type: Option<PartialType>,
    ) -> Result<Constant, AsgConvertError> {
        use leo_ast::ValueExpression::*;
        Ok(match value {
            Address(value, span) => {
                match expected_type.map(PartialType::full).flatten() {
                    Some(Type::Address) | None => (),
                    Some(x) => {
                        return Err(AsgConvertError::unexpected_type(
                            &x.to_string(),
                            Some(&*Type::Address.to_string()),
                            span,
                        ));
                    }
                }
                Constant {
                    parent: RefCell::new(None),
                    span: Some(span.clone()),
                    value: ConstValue::Address(value.clone()),
                }
            }
            Boolean(value, span) => {
                match expected_type.map(PartialType::full).flatten() {
                    Some(Type::Boolean) | None => (),
                    Some(x) => {
                        return Err(AsgConvertError::unexpected_type(
                            &x.to_string(),
                            Some(&*Type::Boolean.to_string()),
                            span,
                        ));
                    }
                }
                Constant {
                    parent: RefCell::new(None),
                    span: Some(span.clone()),
                    value: ConstValue::Boolean(
                        value
                            .parse::<bool>()
                            .map_err(|_| AsgConvertError::invalid_boolean(&value, span))?,
                    ),
                }
            }
            Field(value, span) => {
                match expected_type.map(PartialType::full).flatten() {
                    Some(Type::Field) | None => (),
                    Some(x) => {
                        return Err(AsgConvertError::unexpected_type(
                            &x.to_string(),
                            Some(&*Type::Field.to_string()),
                            span,
                        ));
                    }
                }
                Constant {
                    parent: RefCell::new(None),
                    span: Some(span.clone()),
                    value: ConstValue::Field(value.parse().map_err(|_| AsgConvertError::invalid_int(&value, span))?),
                }
            }
            Group(value) => {
                match expected_type.map(PartialType::full).flatten() {
                    Some(Type::Group) | None => (),
                    Some(x) => {
                        return Err(AsgConvertError::unexpected_type(
                            &x.to_string(),
                            Some(&*Type::Group.to_string()),
                            value.span(),
                        ));
                    }
                }
                Constant {
                    parent: RefCell::new(None),
                    span: Some(value.span().clone()),
                    value: ConstValue::Group(match &**value {
                        leo_ast::GroupValue::Single(value, _) => GroupValue::Single(value.clone()),
                        leo_ast::GroupValue::Tuple(leo_ast::GroupTuple { x, y, .. }) => {
                            GroupValue::Tuple(x.into(), y.into())
                        }
                    }),
                }
            }
            Implicit(value, span) => match expected_type {
                None => return Err(AsgConvertError::unresolved_type("unknown", span)),
                Some(PartialType::Integer(Some(sub_type), _)) | Some(PartialType::Integer(None, Some(sub_type))) => {
                    Constant {
                        parent: RefCell::new(None),
                        span: Some(span.clone()),
                        value: ConstValue::Int(ConstInt::parse(&sub_type, value, span)?),
                    }
                }
                Some(PartialType::Type(Type::Field)) => Constant {
                    parent: RefCell::new(None),
                    span: Some(span.clone()),
                    value: ConstValue::Field(value.parse().map_err(|_| AsgConvertError::invalid_int(&value, span))?),
                },
                Some(PartialType::Type(Type::Address)) => Constant {
                    parent: RefCell::new(None),
                    span: Some(span.clone()),
                    value: ConstValue::Address(value.to_string()),
                },
                Some(x) => return Err(AsgConvertError::unexpected_type(&x.to_string(), Some("unknown"), span)),
            },
            Integer(int_type, value, span) => {
                match expected_type {
                    Some(PartialType::Integer(Some(sub_type), _)) if &sub_type == int_type => (),
                    Some(PartialType::Integer(None, Some(_))) => (),
                    None => (),
                    Some(x) => {
                        return Err(AsgConvertError::unexpected_type(
                            &x.to_string(),
                            Some(&*int_type.to_string()),
                            span,
                        ));
                    }
                }
                Constant {
                    parent: RefCell::new(None),
                    span: Some(span.clone()),
                    value: ConstValue::Int(ConstInt::parse(int_type, value, span)?),
                }
            }
        })
    }
}

impl Into<leo_ast::ValueExpression> for &Constant {
    fn into(self) -> leo_ast::ValueExpression {
        match &self.value {
            ConstValue::Address(value) => {
                leo_ast::ValueExpression::Address(value.clone(), self.span.clone().unwrap_or_default())
            }
            ConstValue::Boolean(value) => {
                leo_ast::ValueExpression::Boolean(value.to_string(), self.span.clone().unwrap_or_default())
            }
            ConstValue::Field(value) => {
                leo_ast::ValueExpression::Field(value.to_string(), self.span.clone().unwrap_or_default())
            }
            ConstValue::Group(value) => leo_ast::ValueExpression::Group(Box::new(match value {
                GroupValue::Single(single) => {
                    leo_ast::GroupValue::Single(single.clone(), self.span.clone().unwrap_or_default())
                }
                GroupValue::Tuple(left, right) => leo_ast::GroupValue::Tuple(leo_ast::GroupTuple {
                    x: left.into(),
                    y: right.into(),
                    span: self.span.clone().unwrap_or_default(),
                }),
            })),
            ConstValue::Int(int) => leo_ast::ValueExpression::Integer(
                int.get_int_type(),
                int.raw_value(),
                self.span.clone().unwrap_or_default(),
            ),
            ConstValue::Tuple(_) => unimplemented!(),
            ConstValue::Array(_) => unimplemented!(),
        }
    }
}
