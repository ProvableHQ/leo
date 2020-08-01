//! Allocates a main function input parameter in a compiled Leo program.

use crate::{
    address::Address,
    errors::FunctionError,
    program::ConstrainedProgram,
    value::{
        boolean::input::bool_from_input,
        field::input::field_from_input,
        group::input::group_from_input,
        ConstrainedValue,
    },
    GroupType,
    Integer,
};

use leo_typed::{InputValue, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_main_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        _type: Type,
        name: String,
        input_value: Option<InputValue>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        match _type {
            Type::Address => Ok(Address::from_input(cs, name, input_value, span)?),
            Type::Boolean => Ok(bool_from_input(cs, name, input_value, span)?),
            Type::Field => Ok(field_from_input(cs, name, input_value, span)?),
            Type::Group => Ok(group_from_input(cs, name, input_value, span)?),
            Type::IntegerType(integer_type) => Ok(ConstrainedValue::Integer(Integer::from_input(
                cs,
                integer_type,
                name,
                input_value,
                span,
            )?)),
            Type::Array(_type, dimensions) => self.allocate_array(cs, name, *_type, dimensions, input_value, span),
            _ => unimplemented!("main function input not implemented for type"),
        }
    }
}
