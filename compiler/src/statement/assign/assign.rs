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

//! Enforces an assign statement in a compiled Leo program.

use crate::{
    assignee::resolve_assignee,
    errors::StatementError,
    new_scope,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};
use leo_ast::{Assignee, AssigneeAccess, Expression, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_assign_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        declared_circuit_reference: &str,
        indicator: Option<Boolean>,
        assignee: Assignee,
        expression: Expression,
        span: &Span,
    ) -> Result<(), StatementError> {
        // Get the name of the variable we are assigning to
        let variable_name = resolve_assignee(function_scope.to_string(), assignee.clone());

        // Evaluate new value
        let mut new_value = self.enforce_expression(cs, file_scope, function_scope, None, expression)?;

        // Mutate the old value into the new value
        if assignee.accesses.is_empty() {
            let condition = indicator.unwrap_or(Boolean::Constant(true));
            let old_value = self.get_mutable_assignee(&variable_name, span)?;

            new_value.resolve_type(Some(old_value.to_type(&span)?), span)?;

            let selected_value = ConstrainedValue::conditionally_select(
                cs.ns(|| format!("select {} {}:{}", new_value, span.line, span.start)),
                &condition,
                &new_value,
                old_value,
            )
            .map_err(|_| StatementError::select_fail(new_value.to_string(), old_value.to_string(), span.to_owned()))?;

            *old_value = selected_value;

            return Ok(());
        } else {
            match assignee.accesses[0].clone() {
                AssigneeAccess::Array(range_or_expression) => self.assign_array(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    &variable_name,
                    range_or_expression,
                    new_value,
                    span,
                ),
                AssigneeAccess::Tuple(index, span) => {
                    self.assign_tuple(cs, indicator, &variable_name, index, new_value, &span)
                }
                AssigneeAccess::Member(identifier) => {
                    // Mutate a circuit variable using the self keyword.
                    if assignee.identifier.is_self() {
                        let self_circuit_variable_name = new_scope(&assignee.identifier.name, &identifier.name);
                        let self_variable_name = new_scope(file_scope, &self_circuit_variable_name);
                        let value = self.mutate_circuit_variable(
                            cs,
                            indicator,
                            declared_circuit_reference,
                            identifier,
                            new_value,
                            span,
                        )?;

                        self.store(self_variable_name, value);
                    } else {
                        let _value =
                            self.mutate_circuit_variable(cs, indicator, &variable_name, identifier, new_value, span)?;
                    }

                    Ok(())
                }
            }
        }
    }
}
