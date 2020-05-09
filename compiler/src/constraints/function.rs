//! Methods to enforce functions with arguments in
//! a resolved Leo program.

use crate::{
    constraints::{
        new_scope, new_scope_from_variable, new_variable_from_variables, ConstrainedProgram,
        ConstrainedValue,
    },
    errors::{FunctionError, ImportError},
    types::{Expression, Function, InputValue, Program, Type},
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

        // Make sure we are given the correct number of arguments
        Self::check_inputs_length(function.inputs.len(), inputs.len())?;

        // Store argument values as new variables in resolved program
        for (input_model, input_expression) in
            function.inputs.clone().iter().zip(inputs.into_iter())
        {
            // Check that argument is correct type
            match input_model._type.clone() {
                Type::IntegerType(integer_type) => {
                    match self.enforce_input(
                        cs,
                        scope.clone(),
                        caller_scope.clone(),
                        function_name.clone(),
                        input_expression,
                    )? {
                        ConstrainedValue::Integer(number) => {
                            number.expect_type(&integer_type)?;
                            // Store argument as variable with {function_name}_{parameter name}
                            let variable_name = new_scope_from_variable(
                                function_name.clone(),
                                &input_model.variable,
                            );
                            self.store(variable_name, ConstrainedValue::Integer(number));
                        }
                        argument => {
                            return Err(FunctionError::InvalidInput(
                                integer_type.to_string(),
                                argument.to_string(),
                            ))
                        }
                    }
                }
                Type::FieldElement => {
                    match self.enforce_input(
                        cs,
                        scope.clone(),
                        caller_scope.clone(),
                        function_name.clone(),
                        input_expression,
                    )? {
                        ConstrainedValue::FieldElement(fe) => {
                            // Store argument as variable with {function_name}_{parameter name}
                            let variable_name = new_scope_from_variable(
                                function_name.clone(),
                                &input_model.variable,
                            );
                            self.store(variable_name, ConstrainedValue::FieldElement(fe));
                        }
                        argument => {
                            return Err(FunctionError::InvalidInput(
                                Type::<F>::FieldElement.to_string(),
                                argument.to_string(),
                            ))
                        }
                    }
                }
                Type::Boolean => {
                    match self.enforce_input(
                        cs,
                        scope.clone(),
                        caller_scope.clone(),
                        function_name.clone(),
                        input_expression,
                    )? {
                        ConstrainedValue::Boolean(bool) => {
                            // Store argument as variable with {function_name}_{parameter name}
                            let variable_name = new_scope_from_variable(
                                function_name.clone(),
                                &input_model.variable,
                            );
                            self.store(variable_name, ConstrainedValue::Boolean(bool));
                        }
                        argument => {
                            return Err(FunctionError::InvalidInput(
                                Type::<F>::Boolean.to_string(),
                                argument.to_string(),
                            ))
                        }
                    }
                }
                ty => return Err(FunctionError::UndefinedInput(ty.to_string())),
            }
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

    pub(crate) fn enforce_main_function(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Function<F>,
        inputs: Vec<Option<InputValue<F>>>,
    ) -> Result<ConstrainedValue<F>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of arguments
        Self::check_inputs_length(function.inputs.len(), inputs.len())?;

        // Iterate over main function inputs and allocate new passed-by variable values
        let mut input_variables = vec![];
        for (input_model, input_value) in
            function.inputs.clone().into_iter().zip(inputs.into_iter())
        {
            // append each variable to inputs vector
            let variable = match input_model._type {
                Type::IntegerType(ref _integer_type) => self.integer_from_parameter(
                    cs,
                    function_name.clone(),
                    input_model,
                    input_value,
                )?,
                Type::FieldElement => self.field_element_from_parameter(
                    cs,
                    function_name.clone(),
                    input_model,
                    input_value,
                )?,
                Type::Boolean => {
                    self.bool_from_parameter(cs, function_name.clone(), input_model, input_value)?
                }
                Type::Array(ref ty, _length) => match *ty.clone() {
                    Type::IntegerType(_type) => self.integer_array_from_parameter(
                        cs,
                        function_name.clone(),
                        input_model,
                        input_value,
                    )?,
                    Type::FieldElement => self.field_element_array_from_parameter(
                        cs,
                        function_name.clone(),
                        input_model,
                        input_value,
                    )?,
                    Type::Boolean => self.boolean_array_from_parameter(
                        cs,
                        function_name.clone(),
                        input_model,
                        input_value,
                    )?,
                    _type => return Err(FunctionError::UndefinedInput(_type.to_string())),
                },
                _type => return Err(FunctionError::UndefinedInput(_type.to_string())),
            };

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
