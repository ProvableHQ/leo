//! Enforces a definition statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, GroupType};
use leo_typed::{Declare, Expression, Span, Variable};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        declare: Declare,
        variable: Variable,
        expression: Expression,
        span: Span,
    ) -> Result<(), StatementError> {
        let mut expected_types = vec![];
        if let Some(ref _type) = variable._type {
            expected_types.push(_type.clone());
        }
        let mut value = self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            expression,
        )?;

        match declare {
            Declare::Let => value.allocate_value(cs, span)?,
            Declare::Const => {
                if variable.mutable {
                    return Err(StatementError::immutable_assign(variable.to_string(), span));
                }
            }
        }

        self.store_definition(function_scope, variable, value);

        Ok(())
    }
}
