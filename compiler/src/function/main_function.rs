//! Enforces constraints on the main function of a compiled Leo program.

use crate::{
    errors::FunctionError,
    function::check_arguments_length,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_types::{Expression, Function, InputValue};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Function,
        inputs: Vec<Option<InputValue>>,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of inputs
        check_arguments_length(function.inputs.len(), inputs.len(), function.span.clone())?;

        // Iterate over main function inputs and allocate new passed-by variable values
        let mut input_variables = vec![];
        for (input_model, input_option) in function.inputs.clone().into_iter().zip(inputs.into_iter()) {
            let input_value = self.allocate_main_function_input(
                cs,
                input_model._type,
                input_model.identifier.name.clone(),
                input_option,
                function.span.clone(),
            )?;

            // Store a new variable for every allocated main function input
            let input_name = new_scope(function_name.clone(), input_model.identifier.name.clone());
            self.store(input_name.clone(), input_value);

            input_variables.push(Expression::Identifier(input_model.identifier));
        }

        self.enforce_function(cs, scope, function_name, function, input_variables)
    }
}
