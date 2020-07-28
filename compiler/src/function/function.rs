//! Enforces constraints on a function in a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_types::{Expression, Function, Input, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn check_arguments_length(expected: usize, actual: usize, span: Span) -> Result<(), FunctionError> {
    // Make sure we are given the correct number of arguments
    if expected != actual {
        Err(FunctionError::arguments_length(expected, actual, span))
    } else {
        Ok(())
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function: Function,
        inputs: Vec<Expression>,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of inputs
        check_arguments_length(function.inputs.len(), inputs.len(), function.span.clone())?;

        // Store input values as new variables in resolved program
        for (input_model, input_expression) in function.inputs.clone().iter().zip(inputs.into_iter()) {
            if let Input::FunctionInput(input_model) = input_model {
                // First evaluate input expression
                let mut input_value = self.enforce_input(
                    cs,
                    scope.clone(),
                    caller_scope.clone(),
                    function_name.clone(),
                    vec![input_model.type_.clone()],
                    input_expression,
                )?;

                if input_model.mutable {
                    input_value = ConstrainedValue::Mutable(Box::new(input_value))
                }

                // Store input as variable with {function_name}_{input_name}
                let input_program_identifier = new_scope(function_name.clone(), input_model.identifier.name.clone());
                self.store(input_program_identifier, input_value);
            } else {
                println!("function input model {}", input_model);
                println!("function input option {}", input_expression)
            }
        }

        // Evaluate every statement in the function and save all potential results
        let mut results = vec![];

        for statement in function.statements.iter() {
            let mut result = self.enforce_statement(
                cs,
                scope.clone(),
                function_name.clone(),
                None,
                statement.clone(),
                function.returns.clone(),
            )?;

            results.append(&mut result);
        }

        // Conditionally select a result based on returned indicators
        let mut return_values = ConstrainedValue::Return(vec![]);

        Self::conditionally_select_result(cs, &mut return_values, results, function.span.clone())?;

        if let ConstrainedValue::Return(ref returns) = return_values {
            if function.returns.len() != returns.len() {
                return Err(FunctionError::return_arguments_length(
                    function.returns.len(),
                    returns.len(),
                    function.span.clone(),
                ));
            }
        }

        Ok(return_values)
    }
}
