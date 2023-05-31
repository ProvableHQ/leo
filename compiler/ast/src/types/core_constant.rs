// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::Type;
use leo_span::{sym, Symbol};

/// A core constant that maps directly to an AVM bytecode constant.
#[derive(Clone, PartialEq, Eq)]
pub enum CoreConstant {
    GroupGenerator,
}

impl CoreConstant {
    /// Returns a `CoreConstant` from the given type and constant symbols.
    pub fn from_symbols(type_: Symbol, constant: Symbol) -> Option<Self> {
        Some(match (type_, constant) {
            (sym::group, sym::GEN) => Self::GroupGenerator,
            _ => return None,
        })
    }

    /// Returns the `Type` of the `CoreConstant`.
    pub fn to_type(&self) -> Type {
        match self {
            Self::GroupGenerator => Type::Group,
        }
    }
}
