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

use crate::{errors::StatementError, parse_index, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_ast::{PositiveNumber, Span};

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
        indicator: &Boolean,
        name: &str,
        index: PositiveNumber,
        mut new_value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<(), StatementError> {
        // Parse the index.
        let index_usize = parse_index(&index, &span)?;

        // Modify the single value of the tuple in place
        match self.get_mutable_assignee(name, &span)? {
            ConstrainedValue::Tuple(old) => {
                new_value.resolve_type(Some(old[index_usize].to_type(&span)?), &span)?;

                let selected_value = ConstrainedValue::conditionally_select(
                    cs.ns(|| format!("select {} {}:{}", new_value, span.line, span.start)),
                    indicator,
                    &new_value,
                    &old[index_usize],
                )
                .map_err(|_| {
                    StatementError::select_fail(new_value.to_string(), old[index_usize].to_string(), span.to_owned())
                })?;

                old[index_usize] = selected_value;
            }
            _ => return Err(StatementError::tuple_assign_index(span.to_owned())),
        }

        Ok(())
    }
}
