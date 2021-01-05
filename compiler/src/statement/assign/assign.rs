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
    arithmetic::*,
    errors::StatementError,
    new_scope,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};
use leo_ast::{AssignOperation, AssignStatement, AssigneeAccess, Span};

use snarkvm_models::{
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
        indicator: &Boolean,
        mut_self: bool,
        statement: AssignStatement,
    ) -> Result<(), StatementError> {
        // Get the name of the variable we are assigning to
        let mut new_value = self.enforce_expression(cs, file_scope, function_scope, None, statement.value)?;
        let mut resolved_assignee = self.resolve_assignee(
            cs,
            file_scope,
            function_scope,
            declared_circuit_reference,
            mut_self,
            statement.assignee.clone(),
        )?;

        if resolved_assignee.len() == 1 {
            new_value.resolve_type(Some(resolved_assignee[0].to_type(&statement.span)?), &statement.span)?;

            let span = statement.span.clone();

            Self::enforce_assign_operation(
                cs,
                indicator,
                format!("select {} {}:{}", new_value, &span.line, &span.start),
                &statement.operation,
                resolved_assignee[0],
                new_value,
                &span,
            )?;
        } else {
            match new_value {
                ConstrainedValue::Array(new_values) => {
                    let span = statement.span.clone();

                    for (i, (old_ref, new_value)) in
                        resolved_assignee.into_iter().zip(new_values.into_iter()).enumerate()
                    {
                        Self::enforce_assign_operation(
                            cs,
                            indicator,
                            format!("select-splice {} {} {}:{}", i, new_value, &span.line, &span.start),
                            &statement.operation,
                            old_ref,
                            new_value,
                            &span,
                        )?;
                    }
                }
                _ => return Err(StatementError::array_assign_range(statement.span)),
            };
        }

        // self re-store logic -- structure is already checked by enforce_assign_operation
        if statement.assignee.identifier.is_self() && mut_self {
            if let Some(AssigneeAccess::Member(member_name)) = statement.assignee.accesses.get(0) {
                let self_circuit_variable_name = new_scope(&statement.assignee.identifier.name, &member_name.name);
                let self_variable_name = new_scope(file_scope, &self_circuit_variable_name);
                // get circuit ref
                let target = match self.get(declared_circuit_reference) {
                    Some(ConstrainedValue::Mutable(value)) => &**value,
                    _ => unimplemented!(),
                };
                // get freshly assigned member ref, and clone it
                let source = match target {
                    ConstrainedValue::CircuitExpression(_circuit_name, members) => {
                        let matched_variable = members.iter().find(|member| &member.0 == member_name);

                        match matched_variable {
                            Some(member) => &member.1,
                            None => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
                .clone();
                self.store(self_variable_name, source);
            }
        }

        Ok(())
    }

    fn enforce_assign_operation<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        condition: &Boolean,
        scope: String,
        operation: &AssignOperation,
        target: &mut ConstrainedValue<F, G>,
        mut new_value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<(), StatementError> {
        new_value.resolve_type(Some(target.to_type(span)?), span)?;

        let new_value = match operation {
            AssignOperation::Assign => new_value,
            AssignOperation::Add => enforce_add(cs, target.clone(), new_value, span)?,
            AssignOperation::Sub => enforce_sub(cs, target.clone(), new_value, span)?,
            AssignOperation::Mul => enforce_mul(cs, target.clone(), new_value, span)?,
            AssignOperation::Div => enforce_div(cs, target.clone(), new_value, span)?,
            AssignOperation::Pow => enforce_pow(cs, target.clone(), new_value, span)?,
        };
        let selected_value = ConstrainedValue::conditionally_select(cs.ns(|| scope), condition, &new_value, target)
            .map_err(|_| StatementError::select_fail(new_value.to_string(), target.to_string(), span.clone()))?;

        *target = selected_value;
        Ok(())
    }
}
