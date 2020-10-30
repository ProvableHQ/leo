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
use crate::{Attribute, FunctionInputVariableType, Type};
use leo_typed::{Circuit, Function, Identifier};

use std::{
    fmt,
    hash::{Hash, Hasher},
};

/// Stores information for a user defined type.
///
/// User defined types include circuits and functions in a Leo program.
#[derive(Clone, Debug)]
pub struct UserDefinedType {
    pub identifier: Identifier,
    pub type_: Type,
    pub attribute: Option<Attribute>,
}

impl From<Circuit> for UserDefinedType {
    fn from(value: Circuit) -> Self {
        let identifier = value.circuit_name;

        UserDefinedType {
            identifier: identifier.clone(),
            type_: Type::Circuit(identifier),
            attribute: None,
        }
    }
}

impl From<Function> for UserDefinedType {
    fn from(value: Function) -> Self {
        let identifier = value.identifier;

        UserDefinedType {
            identifier: identifier.clone(),
            type_: Type::Function(identifier),
            attribute: None,
        }
    }
}

impl From<FunctionInputVariableType> for UserDefinedType {
    fn from(value: FunctionInputVariableType) -> Self {
        UserDefinedType {
            identifier: value.identifier,
            type_: value.type_,
            attribute: value.attribute,
        }
    }
}

impl fmt::Display for UserDefinedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl PartialEq for UserDefinedType {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}

impl Eq for UserDefinedType {}

impl Hash for UserDefinedType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
