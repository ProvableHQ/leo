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
        for input_model in function.inputs.clone().into_iter() {
            let (identifier, value) = match input_model {
                Input::FunctionInput(input_model) => {
                    let name = input_model.identifier.name.clone();
                    let input_option = inputs
                        .get(&name)
                        .ok_or(FunctionError::input_not_found(name.clone(), function.span.clone()))?;
                    let input_value = self.allocate_main_function_input(
                        cs,
                        input_model.type_,
                        name.clone(),
                        input_option,
                        function.span.clone(),
                    )?;

                    (input_model.identifier, input_value)
                }
                Input::Registers(identifier) => {
                    let section = inputs.get_registers();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
                Input::Record(identifier) => {
                    let section = inputs.get_record();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
                Input::State(identifier) => {
                    let section = inputs.get_state();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
                Input::StateLeaf(identifier) => {
                    let section = inputs.get_state_leaf();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
            };

            // Store input as variable with {function_name}_{identifier_name}
            let input_name = new_scope(function_name.clone(), identifier.name.clone());

            // Store a new variable for every allocated main function input
            self.store(input_name, value);

            input_variables.push(Expression::Identifier(identifier));
        }

        self.enforce_function(cs, scope, function_name, function, input_variables)
    }
}
