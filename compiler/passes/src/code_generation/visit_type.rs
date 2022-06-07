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

use leo_ast::Type;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_type(&mut self, input: &'a Type) -> String {
        match input {
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::String
            | Type::IntegerType(..) => format!("{}", input),
            Type::Identifier(_) => unimplemented!("Support for composite types is not yet implemented"),
            Type::Err => unreachable!("Error types should not exist at this phase of compilation"),
        }
    }
}
