// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! Enforces a tuple assignment statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn assign_tuple<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        name: String,
        index: usize,
        mut new_value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));

        // Modify the single value of the tuple in place
        match self.get_mutable_assignee(name, span.clone())? {
            ConstrainedValue::Tuple(old) => {
                new_value.resolve_type(Some(old[index].to_type(span.clone())?), span.clone())?;

                let name_unique = format!("select {} {}:{}", new_value, span.line, span.start);
                let selected_value =
                    ConstrainedValue::conditionally_select(cs.ns(|| name_unique), &condition, &new_value, &old[index])
                        .map_err(|_| {
                            StatementError::select_fail(new_value.to_string(), old[index].to_string(), span)
                        })?;

                old[index] = selected_value;
            }
            _ => return Err(StatementError::tuple_assign_index(span)),
        }

        Ok(())
    }
}
