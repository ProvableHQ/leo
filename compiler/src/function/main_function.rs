//! Enforces constraints on the main function of a compiled Leo program.

use crate::{
    errors::FunctionError,
    function::check_arguments_length,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_types::{Expression, Function, Input, Inputs};

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
        inputs: Inputs,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of inputs
        check_arguments_length(function.inputs.len(), inputs.len(), function.span.clone())?;

        // Iterate over main function inputs and allocate new passed-by variable values
        let mut input_variables = vec![];
        let mut seen = 0;
        for (i, input_model) in function.inputs.clone().into_iter().enumerate() {
            match input_model {
                Input::FunctionInput(input_model) => {
                    let name = input_model.identifier.name.clone();
                    let input_option = inputs.get(&name);
                    let input_value = self.allocate_main_function_input(
                        cs,
                        input_model.type_,
                        name.clone(),
                        input_option,
                        function.span.clone(),
                    )?;

                    // Store a new variable for every allocated main function input
                    let input_name = new_scope(function_name.clone(), input_model.identifier.name.clone());
                    self.store(input_name.clone(), input_value);

                    input_variables.push(Expression::Identifier(input_model.identifier));
                }
                Input::Registers => {
                    seen += 1;
                }
                Input::Record => {
                    seen += 1;
                }
                Input::State => {
                    seen += 1;
                }
                Input::StateLeaf => {
                    seen += 1;
                }
            }
        }

        self.enforce_function(cs, scope, function_name, function, input_variables)
    }
}
