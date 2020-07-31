//! Enforces constraints on the main function of a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    GroupType,
    OutputBytes,
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
    ) -> Result<OutputBytes, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());
        let registers = inputs.get_registers();

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
                    let section = inputs.get_registers().values();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
                Input::Record(identifier) => {
                    let section = inputs.get_record().values();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
                Input::State(identifier) => {
                    let section = inputs.get_state().values();
                    let value = self.allocate_input_section(cs, identifier.clone(), section)?;

                    (identifier, value)
                }
                Input::StateLeaf(identifier) => {
                    let section = inputs.get_state_leaf().values();
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

        let span = function.span.clone();
        let result_value = self.enforce_function(cs, scope, function_name, function, input_variables)?;
        let output_bytes = OutputBytes::new_from_constrained_value(registers, result_value, span)?;

        Ok(output_bytes)
    }
}
