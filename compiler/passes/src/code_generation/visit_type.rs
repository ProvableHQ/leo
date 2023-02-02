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

use crate::CodeGenerator;

use leo_ast::{Mode, Type};

impl<'a> CodeGenerator<'a> {
    fn visit_type(&mut self, input: &'a Type) -> String {
        match input {
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::String
            | Type::Integer(..) => format!("{input}"),
            Type::Identifier(ident) => format!("{ident}"),
            Type::Mapping(_) => {
                unreachable!("Mapping types are not supported at this phase of compilation")
            }
            Type::Tuple(_) => {
                unreachable!("Tuple types should not be visited at this phase of compilation")
            }
            Type::Err => unreachable!("Error types should not exist at this phase of compilation"),
            Type::Unit => unreachable!("Unit types are not supported at this phase of compilation"),
        }
    }

    pub(crate) fn visit_type_with_visibility(&mut self, type_: &'a Type, visibility: Mode) -> String {
        match type_ {
            // When the type is a record.
            // Note that this unwrap is safe because all composite types have been added to the mapping.
            Type::Identifier(identifier) if self.composite_mapping.get(&identifier.name).unwrap().0 => {
                format!("{identifier}.record")
            }
            _ => match visibility {
                Mode::None => self.visit_type(type_),
                _ => format!("{}.{visibility}", self.visit_type(type_)),
            },
        }
    }
}
