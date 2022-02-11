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

use crate::{
    CharValue, Expression, GroupValue, Identifier, IntegerType, Node, SpreadOrExpression, Type, ValueExpression,
};
use indexmap::IndexMap;
use leo_errors::{AstError, LeoError, ParserError, Result};
use pest::Parser;

#[derive(Clone, Debug)]
pub struct Input {
    values: IndexMap<Identifier, IndexMap<Identifier, (Type, Expression)>>,
}

impl Input {
    pub fn new(values: IndexMap<Identifier, IndexMap<Identifier, (Type, Expression)>>) -> Self {
        for (ident, definitions) in values.iter() {
            // println!("ident: {}", ident);
            for (_var, (type_, value)) in definitions.iter() {
                let _ = InputValue::create(type_.clone(), value.clone())
                    .map_err(|_| ParserError::unexpected_eof(value.span()));
            }
        }

        Input { values }
    }
}

impl InputValue {
    pub fn create(type_: Type, expression: Expression) -> Result<Self> {
        dbg!(InputValue::try_from((type_, expression))?);
        Ok(InputValue::Boolean(true))
    }
}

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

    // Should be removed later
    Implicit(String),
}

impl InputValue {
    fn verify_type(self, type_: Type) -> Result<Self> {
        Ok(self)
    }
}

impl TryFrom<(Type, Expression)> for InputValue {
    type Error = LeoError;
    fn try_from(value: (Type, Expression)) -> Result<Self> {
        Ok(match value {
            (type_, Expression::Value(value)) => {
                match value {
                    ValueExpression::Address(value, _) => Self::Address(value.to_string()),
                    ValueExpression::Boolean(value, span) => {
                        let bool_value = value.parse::<bool>().map_err(|_| ParserError::unexpected_eof(&span))?; // TODO: change error
                        Self::Boolean(bool_value)
                    }
                    ValueExpression::Char(value) => Self::Char(value),
                    ValueExpression::Field(value, _) => Self::Field(value.to_string()),
                    ValueExpression::Group(value) => Self::Group(*value),
                    ValueExpression::Implicit(value, _) => Self::Implicit(value.to_string()),
                    ValueExpression::Integer(type_, value, _) => Self::Integer(type_, value.to_string()),
                    ValueExpression::String(string, span) => Self::Array(
                        string
                            .into_iter()
                            .map(|c| {
                                Self::Char(CharValue {
                                    character: c,
                                    span: span.clone(),
                                })
                            })
                            .collect(),
                    ),
                }
                .verify_type(type_)?
            }
            (Type::Array(type_, type_dimensions), Expression::ArrayInit(array_init)) => {
                let mut dimensions = array_init.dimensions;
                let expression = array_init.element;
                let span = array_init.span.clone();

                if type_dimensions != dimensions || dimensions.is_zero() {
                    return Err(AstError::invalid_array_dimension_size(&span).into());
                }

                if dimensions.len() > 1 {
                    while let Some(dimension) = dimensions.remove_first() {
                        if let Some(number) = dimension.as_specified() {
                            let size = number.value.parse::<usize>().unwrap();
                            // let elements = Vec::with_capacity(size);
                            // for i in 0..size {
                            //     // elements.push()
                            // }
                        } else {
                            return Err(AstError::invalid_array_dimension_size(&span).into());
                        }
                    }
                } else {
                }

                // let elements = Vec::with_capacity(dimensions.len());

                dbg!(dimensions);
                dbg!(expression);

                // TBD

                Self::Boolean(false).verify_type(*type_)?
            }
            (type_, Expression::TupleInit(tuple_init)) => {
                let mut elements = Vec::with_capacity(tuple_init.elements.len());
                for element in tuple_init.elements.into_iter() {
                    elements.push(Self::try_from((type_.clone(), element))?);
                }
                Self::Tuple(elements).verify_type(type_)?
            }
            (type_, Expression::ArrayInline(array_inline)) => {
                let mut elements = Vec::with_capacity(array_inline.elements.len());
                let span = array_inline.span().clone();
                for element in array_inline.elements.into_iter() {
                    if let SpreadOrExpression::Expression(value_expression) = element {
                        elements.push(Self::try_from((type_.clone(), value_expression))?);
                    } else {
                        return Err(ParserError::unexpected_eof(&span).into());
                    }
                }
                Self::Array(elements).verify_type(type_)?
            }
            (_type_, expr) => {
                dbg!(&expr);
                return Err(ParserError::unexpected_eof(expr.span()).into());
            }
        })
    }
}
