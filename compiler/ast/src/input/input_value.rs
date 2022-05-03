// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{CharValue, Expression, GroupValue, IntegerType, Node, Type, UnaryOperation, ValueExpression};
use leo_errors::{InputError, LeoError, ParserError, Result};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Char(CharValue),
    Field(String),
    Group(GroupValue),
    Integer(IntegerType, String),
}

impl TryFrom<(Type, Expression)> for InputValue {
    type Error = LeoError;
    fn try_from(value: (Type, Expression)) -> Result<Self> {
        Ok(match value {
            (type_, Expression::Value(value)) => {
                match (type_, value) {
                    (Type::Address, ValueExpression::Address(value, _)) => Self::Address(value),
                    (Type::Boolean, ValueExpression::Boolean(value, span)) => {
                        let bool_value = value.parse::<bool>().map_err(|_| ParserError::unexpected_eof(&span))?; // TODO: change error
                        Self::Boolean(bool_value)
                    }
                    (Type::Char, ValueExpression::Char(value)) => Self::Char(value),
                    (Type::Field, ValueExpression::Field(value, _)) => Self::Field(value),
                    (Type::Group, ValueExpression::Group(value)) => Self::Group(*value),
                    (Type::IntegerType(expected), ValueExpression::Integer(actual, value, span)) => {
                        if expected == actual {
                            Self::Integer(expected, value)
                        } else {
                            return Err(InputError::unexpected_type(expected.to_string(), actual, &span).into());
                        }
                    }
                    (x, y) => {
                        return Err(InputError::unexpected_type(x, &y, &y.span()).into());
                    }
                }
            }
            (type_, Expression::Unary(unary)) if unary.op == UnaryOperation::Negate => {
                InputValue::try_from((type_, *unary.inner))?
            }
            (_type_, expr) => return Err(InputError::illegal_expression(&expr, &expr.span()).into()),
        })
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Address(ref address) => write!(f, "{}", address),
            InputValue::Boolean(ref boolean) => write!(f, "{}", boolean),
            InputValue::Char(ref character) => write!(f, "{}", character),
            InputValue::Group(ref group) => write!(f, "{}", group),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Integer(ref type_, ref number) => write!(f, "{}{:?}", number, type_),
        }
    }
}
