// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{errors::FunctionError, Program};
use leo_asg::{AsgConvertError, Circuit, CircuitMember, InputCategory, Span};

use snarkvm_ir::Value;

use super::input_keyword::*;

impl<'a> Program<'a> {
    pub fn allocate_input_section(
        &mut self,
        name: &str,
        span: &Span,
        expected_type: &'a Circuit<'a>,
        origin: Vec<(String, leo_ast::Type)>,
    ) -> Result<Vec<Value>, FunctionError> {
        let mut value_out = vec![];
        let category = match name {
            RECORD_VARIABLE_NAME => InputCategory::StateRecord,
            REGISTERS_VARIABLE_NAME => InputCategory::Register,
            STATE_VARIABLE_NAME => InputCategory::PublicState,
            STATE_LEAF_VARIABLE_NAME => InputCategory::StateLeaf,
            _ => panic!("unknown input section: {}", name),
        };
        // Allocate each section definition as a circuit member value
        let section_members = expected_type.members.borrow();

        let mut names = Vec::with_capacity(origin.len());
        for (name, type_) in origin {
            let real_type = self.asg.scope.resolve_ast_type(&type_)?;

            if let Some(member) = section_members.get(&name) {
                let expected_type = match member {
                    CircuitMember::Variable(inner) => inner,
                    _ => continue, // present, but unused
                };
                if !real_type.is_assignable_from(expected_type) {
                    return Err(AsgConvertError::unexpected_type(
                        &real_type.to_string(),
                        Some(&expected_type.to_string()),
                        span,
                    )
                    .into());
                }
                value_out.push(Value::Ref(self.alloc_input(category, &*name, expected_type.clone())));
            } else {
                value_out.push(Value::Ref(self.alloc_input(category, &*name, real_type)));
            }
            names.push(name);
        }
        self.register_section_names(category, names);

        Ok(value_out)
    }
}
