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

//! Enforces an assign statement in a compiled Leo program.

use crate::{arithmetic::*, errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{AssignOperation, AssignStatement, Span};

use snarkvm_models::{
    curves::PrimeField,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_assign_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: &Boolean,
        statement: &AssignStatement<'a>,
    ) -> Result<(), StatementError> {
        // Get the name of the variable we are assigning to
        let new_value = self.enforce_expression(cs, statement.value.get())?;
        let mut resolved_assignee = self.resolve_assign(cs, statement)?;

        if resolved_assignee.len() == 1 {
            let span = statement.span.clone().unwrap_or_default();

            Self::enforce_assign_operation(
                cs,
                indicator,
                format!("select {} {}:{}", new_value, &span.line_start, &span.col_start),
                &statement.operation,
                resolved_assignee[0],
                new_value,
                &span,
            )?;
        } else {
            match new_value {
                ConstrainedValue::Array(new_values) => {
                    let span = statement.span.clone().unwrap_or_default();

                    for (i, (old_ref, new_value)) in
                        resolved_assignee.into_iter().zip(new_values.into_iter()).enumerate()
                    {
                        Self::enforce_assign_operation(
                            cs,
                            indicator,
                            format!(
                                "select-splice {} {} {}:{}",
                                i, new_value, &span.line_start, &span.col_start
                            ),
                            &statement.operation,
                            old_ref,
                            new_value,
                            &span,
                        )?;
                    }
                }
                _ => {
                    return Err(StatementError::array_assign_range(
                        &statement.span.clone().unwrap_or_default(),
                    ));
                }
            };
        }

        Ok(())
    }

    fn enforce_assign_operation<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        condition: &Boolean,
        scope: String,
        operation: &AssignOperation,
        target: &mut ConstrainedValue<'a, F, G>,
        new_value: ConstrainedValue<'a, F, G>,
        span: &Span,
    ) -> Result<(), StatementError> {
        let new_value = match operation {
            AssignOperation::Assign => new_value,
            AssignOperation::Add => enforce_add(cs, target.clone(), new_value, span)?,
            AssignOperation::Sub => enforce_sub(cs, target.clone(), new_value, span)?,
            AssignOperation::Mul => enforce_mul(cs, target.clone(), new_value, span)?,
            AssignOperation::Div => enforce_div(cs, target.clone(), new_value, span)?,
            AssignOperation::Pow => enforce_pow(cs, target.clone(), new_value, span)?,
            _ => unimplemented!("unimplemented assign operator"),
        };
        let selected_value = ConstrainedValue::conditionally_select(cs.ns(|| scope), condition, &new_value, target)
            .map_err(|_| StatementError::select_fail(new_value.to_string(), target.to_string(), span))?;

        *target = selected_value;
        Ok(())
    }
}
