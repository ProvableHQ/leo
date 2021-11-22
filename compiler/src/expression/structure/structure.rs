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

//! Enforces a struct expression in a compiled Leo program.

use crate::program::Program;
use leo_asg::{StructInitExpression, StructMember};
use leo_errors::{CompilerError, Result, Span};
use snarkvm_ir::Value;

impl<'a> Program<'a> {
    pub fn enforce_struct(&mut self, expr: &StructInitExpression<'a>, span: &Span) -> Result<Value> {
        let structure = expr.structure.get();
        let members = structure.members.borrow();

        let member_var_len = members
            .values()
            .filter(|x| matches!(x, StructMember::Variable(_)))
            .count();

        let mut resolved_members = vec![None; member_var_len];

        // type checking is already done in asg
        for (name, inner) in expr.values.iter() {
            let (index, _, target) = members
                .get_full(name.name.as_ref())
                .expect("illegal name in asg struct init expression");
            match target {
                StructMember::Variable(_type_) => {
                    let variable_value = self.enforce_expression(inner.get())?;
                    resolved_members[index] = Some(variable_value);
                }
                _ => return Err(CompilerError::expected_struct_member(name, span).into()),
            }
        }

        Ok(Value::Tuple(
            resolved_members
                .into_iter()
                .map(|x| x.expect("missing struct field"))
                .collect(),
        ))
    }
}
