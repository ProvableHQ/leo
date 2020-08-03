//! Enforces an iteration statement in a compiled Leo program.

use crate::{
    errors::StatementError,
    new_scope,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    Integer,
};
use leo_typed::{Expression, Identifier, Span, Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, uint::UInt32},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_iteration_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        index: Identifier,
        start: Expression,
        stop: Expression,
        statements: Vec<Statement>,
        return_types: Vec<Type>,
        span: Span,
    ) -> Result<Vec<(Option<Boolean>, ConstrainedValue<F, G>)>, StatementError> {
        let mut results = vec![];

        let from = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), start, span.clone())?;
        let to = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), stop, span.clone())?;

        for i in from..to {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope(function_scope.clone(), index.to_string());

            self.store(
                index_name,
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(i as u32))),
            );

            // Evaluate statements and possibly return early
            let name_unique = format!("for loop iteration {} {}:{}", i, span.line, span.start);
            let mut result = self.evaluate_branch(
                &mut cs.ns(|| name_unique),
                file_scope.clone(),
                function_scope.clone(),
                indicator,
                statements.clone(),
                return_types.clone(),
            )?;

            results.append(&mut result);
        }

        Ok(results)
    }
}
