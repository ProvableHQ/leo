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

use super::*;

use leo_ast::{CompositeType, Location, Mode, Type};

impl CodeGeneratingVisitor<'_> {
    pub fn visit_type(input: &Type) -> String {
        match input {
            Type::Address
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::Signature
            | Type::String
            | Type::Future(..)
            | Type::Identifier(..)
            | Type::Integer(..) => format!("{input}"),
            Type::Composite(CompositeType { path, .. }) => {
                Self::legalize_path(&path.absolute_path()).expect("path format cannot be legalized at this point")
            }
            Type::Boolean => {
                // Leo calls this just `bool`, which isn't what we need.
                "boolean".into()
            }
            Type::Array(array_type) => {
                format!(
                    "[{}; {}u32]",
                    Self::visit_type(array_type.element_type()),
                    array_type.length.as_u32().expect("length should be known at this point")
                )
            }
            Type::Optional(_) => {
                panic!("Optional types are not supported at this phase of compilation")
            }
            Type::Mapping(_) => {
                panic!("Mapping types are not supported at this phase of compilation")
            }
            Type::Tuple(_) => {
                panic!("Tuple types should not be visited at this phase of compilation")
            }
            Type::Numeric => panic!("`Numeric` types should not exist at this phase of compilation"),
            Type::Err => panic!("Error types should not exist at this phase of compilation"),
            Type::Unit => panic!("Unit types are not supported at this phase of compilation"),
        }
    }

    pub fn visit_type_with_visibility(&self, type_: &Type, visibility: Mode) -> String {
        // If the type is a record, handle it separately.
        if let Type::Composite(composite) = type_ {
            let name = composite.path.absolute_path();
            let this_program_name = self.program_id.unwrap().name.name;
            let program_name = composite.program.unwrap_or(this_program_name);
            if self.state.symbol_table.lookup_record(&Location::new(program_name, name.to_vec())).is_some() {
                let [record_name] = &name[..] else {
                    panic!("Absolute paths to records can only have a single segment at this stage.")
                };
                if program_name == this_program_name {
                    return format!("{record_name}.record");
                } else {
                    return format!("{program_name}.aleo/{record_name}.record");
                }
            }
        }

        if let Mode::None = visibility {
            Self::visit_type(type_)
        } else {
            format!("{}.{visibility}", Self::visit_type(type_))
        }
    }
}
