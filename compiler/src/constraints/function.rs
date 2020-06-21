//! Methods to enforce functions with arguments in
//! a resolved Leo program.

use crate::{
    constraints::{new_scope, ConstrainedProgram, ConstrainedValue},
    errors::{FunctionError, ImportError},
    field_from_input,
    group_from_input,
    GroupType,
};
use leo_types::{Expression, Function, InputValue, Integer, Program, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn check_arguments_length(expected: usize, actual: usize, span: Span) -> Result<(), FunctionError> {
        // Make sure we are given the correct number of arguments
        if expected != actual {
            Err(FunctionError::arguments_length(expected, actual, span))
        } else {
            Ok(())
        }
    }

    fn enforce_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function_name: String,
        expected_types: Vec<Type>,
        input: Expression,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        // Evaluate the function input value as pass by value from the caller or
        // evaluate as an expression in the current function scope
        match input {
            Expression::Identifier(identifier) => {
                Ok(self.evaluate_identifier(caller_scope, function_name, &expected_types, identifier)?)
            }
            expression => Ok(self.enforce_expression(cs, scope, function_name, &expected_types, expression)?),
        }
    }

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
        Self::check_arguments_length(function.inputs.len(), inputs.len(), function.span.clone())?;

        // Store input values as new variables in resolved program
        for (input_model, input_expression) in function.inputs.clone().iter().zip(inputs.into_iter()) {
            // First evaluate input expression
            let mut input_value = self.enforce_input(
                cs,
                scope.clone(),
                caller_scope.clone(),
                function_name.clone(),
                vec![input_model._type.clone()],
                input_expression,
            )?;

            if input_model.mutable {
                input_value = ConstrainedValue::Mutable(Box::new(input_value))
            }

            // Store input as variable with {function_name}_{input_name}
            let input_program_identifier = new_scope(function_name.clone(), input_model.identifier.name.clone());
            self.store(input_program_identifier, input_value);
        }

        // Evaluate function statements

        let mut return_values = ConstrainedValue::Return(vec![]);

        for statement in function.statements.iter() {
            if let Some(returned) = self.enforce_statement(
                cs,
                scope.clone(),
                function_name.clone(),
                None,
                statement.clone(),
                function.returns.clone(),
            )? {
                return_values = returned;
                break;
            }
        }

        if let ConstrainedValue::Return(ref returns) = return_values {
            Self::check_arguments_length(function.returns.len(), returns.len(), function.span.clone())?;
        }

        Ok(return_values)
    }

    fn allocate_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        name: String,
        array_type: Type,
        array_dimensions: Vec<usize>,
        input_value: Option<InputValue>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let expected_length = array_dimensions[0];
        let mut array_value = vec![];

        match input_value {
            Some(InputValue::Array(arr)) => {
                // Check the dimension of the array
                Self::check_arguments_length(expected_length, arr.len(), span.clone())?;

                // Allocate each value in the current row
                for (i, value) in arr.into_iter().enumerate() {
                    let value_name = new_scope(name.clone(), i.to_string());
                    let value_type = array_type.outer_dimension(&array_dimensions);

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        value_type,
                        value_name,
                        Some(value),
                        span.clone(),
                    )?)
                }
            }
            None => {
                // Allocate all row values as none
                for i in 0..expected_length {
                    let value_name = new_scope(name.clone(), i.to_string());
                    let value_type = array_type.outer_dimension(&array_dimensions);

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        value_type,
                        value_name,
                        None,
                        span.clone(),
                    )?);
                }
            }
            _ => return Err(FunctionError::invalid_array(input_value.unwrap().to_string(), span)),
        }

        Ok(ConstrainedValue::Array(array_value))
    }

    fn allocate_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        _type: Type,
        name: String,
        input_value: Option<InputValue>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        match _type {
            Type::IntegerType(integer_type) => Ok(ConstrainedValue::Integer(Integer::from_input(
                cs,
                integer_type,
                name,
                input_value,
                span,
            )?)),
            Type::Field => Ok(field_from_input(cs, name, input_value, span)?),
            Type::Group => Ok(group_from_input(cs, name, input_value, span)?),
            Type::Boolean => Ok(self.bool_from_input(cs, name, input_value, span)?),
            Type::Array(_type, dimensions) => self.allocate_array(cs, name, *_type, dimensions, input_value, span),
            _ => unimplemented!("main function input not implemented for type"),
        }
    }

    pub(crate) fn enforce_main_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Function,
        inputs: Vec<Option<InputValue>>,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of inputs
        Self::check_arguments_length(function.inputs.len(), inputs.len(), function.span.clone())?;

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

    pub(crate) fn resolve_definitions(&mut self, program: Program) -> Result<(), ImportError> {
        let program_name = program.name.clone();

        // evaluate and store all imports
        program
            .imports
            .into_iter()
            .map(|import| self.enforce_import(program_name.clone(), import))
            .collect::<Result<Vec<_>, ImportError>>()?;

        // evaluate and store all circuit definitions
        program.circuits.into_iter().for_each(|(identifier, circuit)| {
            let resolved_circuit_name = new_scope(program_name.clone(), identifier.to_string());
            self.store(resolved_circuit_name, ConstrainedValue::CircuitDefinition(circuit));
        });

        // evaluate and store all function definitions
        program.functions.into_iter().for_each(|(function_name, function)| {
            let resolved_function_name = new_scope(program_name.clone(), function_name.to_string());
            self.store(resolved_function_name, ConstrainedValue::Function(None, function));
        });

        Ok(())
    }
}
