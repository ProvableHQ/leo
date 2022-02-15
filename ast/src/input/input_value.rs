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

use crate::{CharValue, Expression, GroupValue, IntegerType, Node, SpreadOrExpression, Type, ValueExpression};

use leo_errors::{AstError, LeoError, ParserError, Result};

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Char(CharValue),
    Field(String),
    Group(GroupValue),
    Integer(IntegerType, String),
    Array(Vec<InputValue>),
    Tuple(Vec<InputValue>),
}

type InputData = (Type, Expression);

impl TryFrom<InputData> for InputValue {
    type Error = LeoError;
    fn try_from(value: InputData) -> Result<Self> {
        Ok(match value {
            (type_, Expression::Value(value)) => {
                match (type_, value) {
                    (Type::Address, ValueExpression::Address(value, _)) => Self::Address(value.to_string()),
                    (Type::Boolean, ValueExpression::Boolean(value, span)) => {
                        let bool_value = value.parse::<bool>().map_err(|_| ParserError::unexpected_eof(&span))?; // TODO: change error
                        Self::Boolean(bool_value)
                    }
                    (Type::Char, ValueExpression::Char(value)) => Self::Char(value),
                    (Type::Field, ValueExpression::Field(value, _) | ValueExpression::Implicit(value, _)) => {
                        Self::Field(value.to_string())
                    }
                    (Type::Group, ValueExpression::Group(value)) => Self::Group(*value),
                    (Type::IntegerType(type_), ValueExpression::Implicit(value, _)) => {
                        Self::Integer(type_, value.to_string())
                    }
                    (Type::IntegerType(expected), ValueExpression::Integer(actual, value, _)) => {
                        if expected == actual {
                            Self::Integer(expected, value.to_string())
                        } else {
                            todo!("make a decent error here");
                        }
                    }
                    (Type::Array(type_, _), ValueExpression::String(string, span)) => {
                        if !matches!(*type_, Type::Char) {
                            todo!("string can only be used for arrays of chars");
                        }

                        Self::Array(
                            string
                                .into_iter()
                                .map(|c| {
                                    Self::Char(CharValue {
                                        character: c,
                                        span: span.clone(),
                                    })
                                })
                                .collect(),
                        )
                    }
                    (x, y) => {
                        todo!("type mismatch, expected type {}, got {}", x, y);
                    }
                }
            }
            (Type::Array(type_, type_dimensions), Expression::ArrayInit(mut array_init)) => {
                // let mut dimensions = array_init.dimensions;
                // let expression = array_init.element;
                let span = array_init.span.clone();

                if type_dimensions != array_init.dimensions || array_init.dimensions.is_zero() {
                    return Err(AstError::invalid_array_dimension_size(&span).into());
                }

                if let Some(dimension) = array_init.dimensions.remove_first() {
                    if let Some(number) = dimension.as_specified() {
                        let size = number.value.parse::<usize>().unwrap();
                        let mut values = Vec::with_capacity(size);

                        // For when Dimensions are specified in a canonical way: [[u8; 3], 2];
                        // Else treat as math notation: [u8; (2, 3)];
                        if array_init.dimensions.len() == 0 {
                            for _ in 0..size {
                                values.push(InputValue::try_from((*type_.clone(), *array_init.element.clone()))?);
                            }
                        // Faking canonical array init is relatively easy: instead of using a straightforward
                        // recursion, with each iteration we manually modify ArrayInitExpression cutting off
                        // dimension by dimension.
                        } else {
                            for _ in 0..size {
                                values.push(InputValue::try_from((
                                    Type::Array(type_.clone(), array_init.dimensions.clone()),
                                    Expression::ArrayInit(array_init.clone()),
                                ))?);
                            }
                        };

                        Self::Array(values)
                    } else {
                        unreachable!("dimensions must be specified");
                    }
                } else {
                    unreachable!("dimensions are checked for zero");
                }
            }
            (Type::Tuple(types), Expression::TupleInit(tuple_init)) => {
                let size = tuple_init.elements.len();
                let mut elements = Vec::with_capacity(size);

                if size != types.len() {
                    todo!(
                        "tuple length mismatch, defined {} types, got {} values",
                        size,
                        types.len()
                    );
                }

                for (i, element) in tuple_init.elements.into_iter().enumerate() {
                    elements.push(Self::try_from((types[i].clone(), element))?);
                }

                Self::Tuple(elements)
            }
            (Type::Array(type_, _dimensions), Expression::ArrayInline(array_inline)) => {
                let mut elements = Vec::with_capacity(array_inline.elements.len());
                let _span = array_inline.span().clone(); // todo!: use for spanning the error

                for element in array_inline.elements.into_iter() {
                    if let SpreadOrExpression::Expression(value_expression) = element {
                        elements.push(Self::try_from((*type_.clone(), value_expression))?);
                    } else {
                        todo!("make error that only expression is allowed in inline, no spread please");
                    }
                }
                Self::Array(elements)
            }
            (_type_, expr) => {
                dbg!(&expr);
                todo!("forbidden expression in inputs");
            }
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
            InputValue::Array(ref array) => {
                let values = array.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "array [{}]", values)
            }
            InputValue::Tuple(ref tuple) => {
                let values = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "({})", values)
            }
        }
    }
}
