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
    ast::Rule,
    types::{BooleanType, CharType, FieldType, GroupType, IntegerType},
};

use crate::types::AddressType;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_data))]
pub enum DataType {
    Address(AddressType),
    Boolean(BooleanType),
    Char(CharType),
    Field(FieldType),
    Group(GroupType),
    Integer(IntegerType),
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::Address(_) => write!(f, "address"),
            DataType::Boolean(_) => write!(f, "bool"),
            DataType::Char(_) => write!(f, "char"),
            DataType::Field(_) => write!(f, "field"),
            DataType::Group(_) => write!(f, "group"),
            DataType::Integer(ref integer) => write!(f, "{}", integer),
        }
    }
}
