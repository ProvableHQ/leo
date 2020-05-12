//! Methods to enforce functions with arguments in
//! a resolved Leo program.

use crate::{
    constraints::{
        new_scope, new_scope_from_variable, new_variable_from_variables, ConstrainedProgram,
        ConstrainedValue,
    },
    errors::{FunctionError, ImportError},
    new_variable_from_variable,
    types::{Expression, Function, InputValue, Program, Type},
    InputModel, Variable,
};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    fn check_inputs_length(expected: usize, actual: usize) -> Result<(), FunctionError> {
        // Make sure we are given the correct number of arguments
        if expected != actual {
            Err(FunctionError::InputsLength(expected, actual))
        } else {
            Ok(())
        }
    }

    fn enforce_input(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function_name: String,
        input: Expression<F>,
    ) -> Result<ConstrainedValue<F>, FunctionError> {
        match input {
            Expression::Variable(variable) => Ok(self.enforce_variable(caller_scope, variable)?),
            expression => Ok(self.enforce_expression(cs, scope, function_name, expression)?),
        }
    }

    pub(crate) fn enforce_function(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function: Function<F>,
        inputs: Vec<Expression<F>>,
    ) -> Result<ConstrainedValue<F>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of inputs
        Self::check_inputs_length(function.inputs.len(), inputs.len())?;

        // Store input values as new variables in resolved program
        for (input_model, input_expression) in
            function.inputs.clone().iter().zip(inputs.into_iter())
        {
            // First evaluate input expression
            let input_value = self.enforce_input(
                cs,
                scope.clone(),
                caller_scope.clone(),
                function_name.clone(),
                input_expression,
            )?;

            // Check that input is correct type
            input_value.expect_type(&input_model._type)?;

            // Store input as variable with {function_name}_{input_name}
            let variable_name =
                new_scope_from_variable(function_name.clone(), &input_model.variable);
            self.store(variable_name, input_value);
        }

        // Evaluate function statements

        let mut return_values = ConstrainedValue::Return(vec![]);

        for statement in function.statements.iter() {
            if let Some(returned) = self.enforce_statement(
                cs,
                scope.clone(),
                function_name.clone(),
                statement.clone(),
                function.returns.clone(),
            )? {
                return_values = returned;
                break;
            }
        }

        Ok(return_values)
    }

    fn allocate_array(
        &mut self,
        cs: &mut CS,
        array_name: Variable<F>,
        array_private: bool,
        array_type: Type<F>,
        array_dimensions: Vec<usize>,
        input_value: Option<InputValue<F>>,
    ) -> Result<ConstrainedValue<F>, FunctionError> {
        let expected_length = array_dimensions[0];
        let mut array_value = vec![];

        match input_value {
            Some(InputValue::Array(arr)) => {
                // Check the dimension of the array
                Self::check_inputs_length(expected_length, arr.len())?;

                // Allocate each value in the current row
                for (i, value) in arr.into_iter().enumerate() {
                    let array_input_model = InputModel {
                        private: array_private,
                        _type: array_type.next_dimension(&array_dimensions),
                        variable: new_variable_from_variables(
                            &array_name,
                            &Variable::new(i.to_string()),
                        ),
                    };

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        array_input_model,
                        Some(value),
                    )?)
                }
            }
            None => {
                // Allocate all row values as none
                for i in 0..expected_length {
                    let array_input_model = InputModel {
                        private: array_private,
                        _type: array_type.next_dimension(&array_dimensions),
                        variable: new_variable_from_variables(
                            &array_name,
                            &Variable::new(i.to_string()),
                        ),
                    };

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        array_input_model,
                        None,
                    )?);
                }
            }
            _ => {
                return Err(FunctionError::InvalidArray(
                    input_value.unwrap().to_string(),
                ))
            }
        }

        Ok(ConstrainedValue::Array(array_value))
    }

    fn allocate_main_function_input(
        &mut self,
        cs: &mut CS,
        input_model: InputModel<F>,
        input_value: Option<InputValue<F>>,
    ) -> Result<ConstrainedValue<F>, FunctionError> {
        match input_model._type {
            Type::IntegerType(ref _integer_type) => {
                Ok(self.integer_from_parameter(cs, input_model, input_value)?)
            }
            Type::FieldElement => {
                Ok(self.field_element_from_input(cs, input_model, input_value)?)
            }
            Type::Boolean => Ok(self.bool_from_input(cs, input_model, input_value)?),
            Type::Array(_type, dimensions) => self.allocate_array(
                cs,
                input_model.variable,
                input_model.private,
                *_type,
                dimensions,
                input_value,
            ),
            _ => unimplemented!("main function input not implemented for type"),
        }
    }

    pub(crate) fn enforce_main_function(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Function<F>,
        inputs: Vec<Option<InputValue<F>>>,
    ) -> Result<ConstrainedValue<F>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of inputs
        Self::check_inputs_length(function.inputs.len(), inputs.len())?;

        // Iterate over main function inputs and allocate new passed-by variable values
        let mut input_variables = vec![];
        for (input_model, input_value) in
            function.inputs.clone().into_iter().zip(inputs.into_iter())
        {
            let variable = new_variable_from_variable(scope.clone(), &input_model.variable);
            let value = self.allocate_main_function_input(cs, input_model, input_value)?;

            // store a new variable for every allocated main function input
            self.store_variable(variable.clone(), value);

            input_variables.push(Expression::Variable(variable));
        }

        self.enforce_function(cs, scope, function_name, function, input_variables)
    }

    pub(crate) fn resolve_definitions(
        &mut self,
        cs: &mut CS,
        program: Program<F>,
    ) -> Result<(), ImportError> {
        let program_name = program.name.clone();

        // evaluate and store all imports
        program
            .imports
            .into_iter()
            .map(|import| self.enforce_import(cs, program_name.name.clone(), import))
            .collect::<Result<Vec<_>, ImportError>>()?;

        // evaluate and store all struct definitions
        program
            .structs
            .into_iter()
            .for_each(|(variable, struct_def)| {
                let resolved_struct_name =
                    new_variable_from_variables(&program_name.clone(), &variable);
                self.store_variable(
                    resolved_struct_name,
                    ConstrainedValue::StructDefinition(struct_def),
                );
            });

        // evaluate and store all function definitions
        program
            .functions
            .into_iter()
            .for_each(|(function_name, function)| {
                let resolved_function_name = new_scope(program_name.name.clone(), function_name.0);
                self.store(resolved_function_name, ConstrainedValue::Function(function));
            });

        Ok(())
    }
}
