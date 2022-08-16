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

use crate::CodeGenerator;

use leo_ast::{ParamMode, Type};
use std::fmt::Write as _;

impl<'a> CodeGenerator<'a> {
    fn visit_type(&mut self, input: &'a Type) -> String {
        match input {
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::String
            | Type::Integer(..) => format!("{}", input),
            Type::Identifier(ident) => {
                if let Some((_, type_)) = self.composite_mapping.get(&ident.name) {
                    format!("{}.{}", ident.to_string().to_lowercase(), type_)
                } else {
                    unreachable!("All composite types should be known at this phase of compilation")
                }
            }
            Type::Tuple(_) => {
                unreachable!("All composite types should be known at this phase of compilation")
            }
            Type::Err => unreachable!("Error types should not exist at this phase of compilation"),
        }
    }

    pub(crate) fn visit_type_with_visibility(&mut self, input: &'a Type, visibility: ParamMode) -> String {
        let mut type_string = self.visit_type(input);

        if let Type::Identifier(_) = input {
            // Do not append anything for record and circuit types.
        } else {
            // todo: CAUTION private by default.
            // Only program functions need a visibility associated with the input type.
            if self.is_program_function {
                // If a visibility is not provided in a program function, then it is private by default.
                let visibility = match visibility {
                    ParamMode::None => ParamMode::Private,
                    _ => visibility,
                };
                write!(type_string, ".{}", visibility).expect("failed to write to string");
            }
        }

        type_string
    }

    /// Returns one or more types equal to the number of return tuple members.
    pub(crate) fn visit_return_type(&mut self, input: &'a Type, visibility: ParamMode) -> Vec<String> {
        // Handle return tuples.
        if let Type::Tuple(types) = input {
            types
                .iter()
                .map(|type_| self.visit_type_with_visibility(type_, visibility))
                .collect()
        } else {
            vec![self.visit_type_with_visibility(input, visibility)]
        }
    }
}
