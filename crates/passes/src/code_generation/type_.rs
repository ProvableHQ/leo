// Copyright (C) 2019-2026 Provable Inc.
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

use leo_ast::{IntegerType, Interface, Mode, TypeKind};

impl CodeGeneratingVisitor<'_> {
    pub fn visit_type(&self, input: &TypeKind) -> AleoType {
        match input {
            TypeKind::Address => AleoType::Address,
            TypeKind::Field => AleoType::Field,
            TypeKind::Group => AleoType::Group,
            TypeKind::Scalar => AleoType::Scalar,
            TypeKind::Signature => AleoType::Signature,
            TypeKind::String => AleoType::String,
            TypeKind::Identifier => AleoType::Identifier,
            TypeKind::DynRecord => AleoType::DynamicRecord,

            TypeKind::Integer(int) => match int {
                IntegerType::U8 => AleoType::U8,
                IntegerType::U16 => AleoType::U16,
                IntegerType::U32 => AleoType::U32,
                IntegerType::U64 => AleoType::U64,
                IntegerType::U128 => AleoType::U128,
                IntegerType::I8 => AleoType::I8,
                IntegerType::I16 => AleoType::I16,
                IntegerType::I32 => AleoType::I32,
                IntegerType::I64 => AleoType::I64,
                IntegerType::I128 => AleoType::I128,
            },
            TypeKind::Ident(id) => AleoType::Ident { name: id.to_string() },
            TypeKind::Composite(composite) => {
                let composite_location = composite.path.expect_global_location();
                let this_program_name = self.program_id.unwrap().as_symbol();
                let program_name = composite_location.program;
                // Use legalize_composite_name so library types get a unique, collision-free name
                // (prefixed with the library name), while local types use the path directly.
                let composite_name = self.legalize_composite_name(composite_location);
                if program_name == this_program_name || self.state.symbol_table.is_library(program_name) {
                    // Library composites are inlined into the consuming program, so they
                    // are emitted without a program qualifier, just like local types.
                    AleoType::Ident { name: composite_name }
                } else {
                    AleoType::Location { program: program_name.to_string(), name: composite_name }
                }
            }
            TypeKind::Boolean => AleoType::Boolean,
            TypeKind::Array(array_type) => AleoType::Array {
                inner: Box::new(self.visit_type(array_type.element_type())),
                len: array_type.length.as_u32().expect("length should be known at this point"),
            },
            TypeKind::Future(..) => {
                panic!("Future types should not be visited at this phase of compilation")
            }
            TypeKind::Optional(_) => {
                panic!("Optional types are not supported at this phase of compilation")
            }
            TypeKind::Mapping(_) => {
                panic!("Mapping types are not supported at this phase of compilation")
            }
            TypeKind::Tuple(_) => {
                panic!("Tuple types should not be visited at this phase of compilation")
            }
            TypeKind::Vector(_) => {
                panic!("Vector types should not be visited at this phase of compilation")
            }
            TypeKind::Numeric => panic!("`Numeric` types should not exist at this phase of compilation"),
            TypeKind::Err => panic!("Error types should not exist at this phase of compilation"),
            TypeKind::Unit => panic!("Unit types are not supported at this phase of compilation"),
        }
    }

    pub fn visit_type_with_visibility(
        &self,
        type_: &TypeKind,
        visibility: Option<AleoVisibility>,
    ) -> (AleoType, Option<AleoVisibility>) {
        // Dynamic records have no visibility qualifier.
        if matches!(type_, TypeKind::DynRecord) {
            return (AleoType::DynamicRecord, None);
        }

        // If the type is a record, handle it separately.
        if let TypeKind::Composite(composite) = type_ {
            let composite_location = composite.path.expect_global_location();
            let this_program_name = self.program_id.unwrap().as_symbol();
            let program_name = composite_location.program;
            if self.state.symbol_table.lookup_record(this_program_name, composite_location).is_some() {
                let [record_name] = &composite_location.path[..] else {
                    panic!("Absolute paths to records can only have a single segment at this stage.")
                };
                if program_name == this_program_name {
                    return (AleoType::Record { name: record_name.to_string(), program: None }, None);
                } else {
                    return (
                        AleoType::Record { name: record_name.to_string(), program: Some(program_name.to_string()) },
                        None,
                    );
                }
            }
        }

        (self.visit_type(type_), visibility)
    }

    /// Maps a dynamic call input type to an AVM type. `dyn record` and interface record types
    /// become `DynamicRecord`; all others use the provided visibility.
    pub fn dynamic_call_input_type(
        &self,
        type_: &TypeKind,
        visibility: Option<AleoVisibility>,
        interface: Option<&Interface>,
    ) -> (AleoType, Option<AleoVisibility>) {
        if matches!(type_, TypeKind::DynRecord) || interface.is_some_and(|i| i.is_record_type(type_)) {
            return (AleoType::DynamicRecord, None);
        }
        (self.visit_type(type_), visibility)
    }

    /// Maps a dynamic call output type to an AVM type. Futures become `DynamicFuture`, `dyn record`
    /// and interface record types become `DynamicRecord`; all others use the provided mode.
    pub fn dynamic_call_output_type(
        &self,
        type_: &TypeKind,
        mode: Mode,
        interface: Option<&Interface>,
    ) -> (AleoType, Option<AleoVisibility>) {
        if matches!(type_, TypeKind::Future(..)) {
            (AleoType::DynamicFuture, None)
        } else if matches!(type_, TypeKind::DynRecord) || interface.is_some_and(|i| i.is_record_type(type_)) {
            (AleoType::DynamicRecord, None)
        } else {
            let viz = AleoVisibility::maybe_from(mode).or(Some(AleoVisibility::Private));
            self.visit_type_with_visibility(type_, viz)
        }
    }
}
