// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{ArrayType, CompositeType, Type};

use itertools::Itertools;
use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An optional type. For example `u32?` where `inner` refers to `u32`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalType {
    pub inner: Box<Type>,
}

pub fn make_optional_struct_symbol(ty: &Type) -> Symbol {
    // Step 1: Extract a usable type name
    fn display_type(ty: &Type) -> String {
        match ty {
            Type::Address
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::Signature
            | Type::Boolean
            | Type::Integer(..) => format!("{ty}"),
            Type::Array(ArrayType { element_type, length }) => {
                format!("[{}; {length}]", display_type(element_type))
            }
            Type::Composite(CompositeType { path, .. }) => {
                format!("::{}", path.absolute_path().iter().format("::"))
            }

            Type::Tuple(_)
            | Type::Optional(_)
            | Type::Mapping(_)
            | Type::Numeric
            | Type::Identifier(_)
            | Type::Future(_)
            | Type::Vector(_)
            | Type::String
            | Type::Err
            | Type::Unit => {
                panic!("unexpected inner type in optional struct name")
            }
        }
    }

    // Step 3: Build symbol that ends with `?`.
    Symbol::intern(&format!("\"{}?\"", display_type(ty)))
}

impl fmt::Display for OptionalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}?", self.inner)
    }
}
