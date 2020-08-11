//! Enforces an assign statement in a compiled Leo program.

use crate::{
    assignee::resolve_assignee,
    errors::StatementError,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};
use leo_typed::{Assignee, Expression, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_assign_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        assignee: Assignee,
        expression: Expression,
        span: Span,
    ) -> Result<(), StatementError> {
        // Get the name of the variable we are assigning to
        let variable_name = resolve_assignee(function_scope.clone(), assignee.clone());

        // Evaluate new value
        let mut new_value =
            self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), &vec![], expression)?;

        // Mutate the old value into the new value
        match assignee {
            Assignee::Identifier(_identifier) => {
                let condition = indicator.unwrap_or(Boolean::Constant(true));
                let old_value = self.get_mutable_assignee(variable_name.clone(), span.clone())?;

                new_value.resolve_type(&vec![old_value.to_type(span.clone())?], span.clone())?;

                let name_unique = format!("select {} {}:{}", new_value, span.line, span.start);
                let selected_value =
                    ConstrainedValue::conditionally_select(cs.ns(|| name_unique), &condition, &new_value, old_value)
                        .map_err(|_| StatementError::select_fail(new_value.to_string(), old_value.to_string(), span))?;

                *old_value = selected_value;

                Ok(())
            }
            Assignee::Array(_assignee, range_or_expression) => self.assign_array(
                cs,
                file_scope,
                function_scope,
                indicator,
                variable_name,
                range_or_expression,
                new_value,
                span,
            ),
            Assignee::Tuple(_tuple, index) => self.assign_tuple(cs, indicator, variable_name, index, new_value, span),
            Assignee::CircuitField(_assignee, object_name) => {
                self.mutute_circuit_field(cs, indicator, variable_name, object_name, new_value, span)
            }
        }
    }
}
