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

use crate::ast::Rule;
use crate::values::AddressValue;
use crate::values::BooleanValue;
use crate::values::FieldValue;
use crate::values::GroupValue;
use crate::values::IntegerValue;
use crate::values::NumberValue;

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Address(AddressValue<'ast>),
    Boolean(BooleanValue<'ast>),
    Field(FieldValue<'ast>),
    Group(GroupValue<'ast>),
    Implicit(NumberValue<'ast>),
    Integer(IntegerValue<'ast>),
}

impl<'ast> Value<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            Value::Address(value) => &value.span,
            Value::Boolean(value) => &value.span,
            Value::Field(value) => &value.span,
            Value::Group(value) => &value.span,
            Value::Implicit(value) => &value.span(),
            Value::Integer(value) => &value.span(),
        }
    }
}

impl<'ast> fmt::Display for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Address(ref value) => write!(f, "{}", value),
            Value::Boolean(ref value) => write!(f, "{}", value),
            Value::Field(ref value) => write!(f, "{}", value),
            Value::Group(ref value) => write!(f, "{}", value),
            Value::Implicit(ref value) => write!(f, "{}", value),
            Value::Integer(ref value) => write!(f, "{}", value),
        }
    }
}
