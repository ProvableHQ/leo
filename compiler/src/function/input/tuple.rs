//! Allocates an array as a main function input parameter in a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_typed::{InputValue, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_tuple<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        name: String,
        types: Vec<Type>,
        input_value: Option<InputValue>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let mut tuple_values = vec![];

        match input_value {
            Some(InputValue::Tuple(values)) => {
                // Allocate each value in the tuple
                for (i, (value, type_)) in values.into_iter().zip(types.into_iter()).enumerate() {
                    let value_name = new_scope(name.clone(), i.to_string());

                    tuple_values.push(self.allocate_main_function_input(
                        cs,
                        type_,
                        value_name,
                        Some(value),
                        span.clone(),
                    )?)
                }
            }
            None => {
                // Allocate all tuple values as none
                for (i, type_) in types.into_iter().enumerate() {
                    let value_name = new_scope(name.clone(), i.to_string());

                    tuple_values.push(self.allocate_main_function_input(cs, type_, value_name, None, span.clone())?);
                }
            }
            _ => return Err(FunctionError::invalid_tuple(input_value.unwrap().to_string(), span)),
        }

        Ok(ConstrainedValue::Tuple(tuple_values))
    }
}
