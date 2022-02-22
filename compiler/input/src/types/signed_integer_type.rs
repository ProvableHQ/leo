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

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_integer_signed))]
pub enum SignedIntegerType {
    I8Type(I8Type),
    I16Type(I16Type),
    I32Type(I32Type),
    I64Type(I64Type),
    I128Type(I128Type),
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i8))]
pub struct I8Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i16))]
pub struct I16Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i32))]
pub struct I32Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i64))]
pub struct I64Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i128))]
pub struct I128Type {}

impl fmt::Display for SignedIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SignedIntegerType::I8Type(_) => write!(f, "i8"),
            SignedIntegerType::I16Type(_) => write!(f, "i16"),
            SignedIntegerType::I32Type(_) => write!(f, "i32"),
            SignedIntegerType::I64Type(_) => write!(f, "i64"),
            SignedIntegerType::I128Type(_) => write!(f, "i128"),
        }
    }
}
