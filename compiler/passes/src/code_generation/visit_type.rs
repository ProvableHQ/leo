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
    pub(crate) fn visit_type(&mut self, input: &'a Type) -> String {
        match input {
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::String
            | Type::IntegerType(..)
            | Type::Tuple(..) => format!("{}", input),
            Type::Identifier(ident) => {
                if let Some(type_) = self.composite_mapping.get(&ident.name) {
                    format!("{}.{}", ident.to_string().to_lowercase(), type_)
                } else {
                    unreachable!("All composite types should be known at this phase of compilation")
                }
            },
            Type::Err => unreachable!("Error types should not exist at this phase of compilation"),
        }
    }

    pub(crate) fn visit_type_with_visibility(&mut self, input: &'a Type, visibility: Option<ParamMode>) -> String {
        let mut return_type = self.visit_type(input);

        if let Type::Identifier(_) = input {
            // Do not append anything for record and circuit types.
        } else {
            // Append `.private` to return type.
            // todo: CAUTION private by default.
            write!(return_type, ".{}", visibility.unwrap_or(ParamMode::Private)).expect("failed to write to string");
        }

        return_type
    }
}
