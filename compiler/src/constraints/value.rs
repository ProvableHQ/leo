//! The in memory stored value for a defined name in a resolved Leo program.

use crate::{
    errors::ValueError,
    types::{Circuit, FieldElement, Function, Type, Variable},
    Integer,
};

use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::utilities::boolean::Boolean,
};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstrainedCircuitMember<F: Field + PrimeField, G: Group>(
    pub Variable<F, G>,
    pub ConstrainedValue<F, G>,
);

#[derive(Clone, PartialEq, Eq)]
pub enum ConstrainedValue<F: Field + PrimeField, G: Group> {
    Integer(Integer),
    FieldElement(FieldElement<F>),
    GroupElement(G),
    Boolean(Boolean),
    Array(Vec<ConstrainedValue<F, G>>),
    CircuitDefinition(Circuit<F, G>),
    CircuitExpression(Variable<F, G>, Vec<ConstrainedCircuitMember<F, G>>),
    Function(Function<F, G>),
    Return(Vec<ConstrainedValue<F, G>>), // add Null for function returns
}

impl<F: Field + PrimeField, G: Group> ConstrainedValue<F, G> {
    pub(crate) fn expect_type(&self, _type: &Type<F, G>) -> Result<(), ValueError> {
        match (self, _type) {
            (ConstrainedValue::Integer(ref integer), Type::IntegerType(ref _type)) => {
                integer.expect_type(_type)?;
            }
            (ConstrainedValue::FieldElement(ref _f), Type::FieldElement) => {}
            (ConstrainedValue::GroupElement(ref _g), Type::GroupElement) => {}
            (ConstrainedValue::Boolean(ref _b), Type::Boolean) => {}
            (ConstrainedValue::Array(ref arr), Type::Array(ref _type, ref dimensions)) => {
                // check array lengths are equal
                if arr.len() != dimensions[0] {
                    return Err(ValueError::ArrayLength(format!(
                        "Expected array {:?} to be length {:?}",
                        arr, dimensions[0]
                    )));
                }

                // get next dimension of array if nested
                let next_type = _type.next_dimension(dimensions);

                // check each value in array matches
                for value in arr {
                    value.expect_type(&next_type)?;
                }
            }
            (
                ConstrainedValue::CircuitExpression(ref actual_name, ref _members),
                Type::Circuit(ref expected_name),
            ) => {
                if expected_name != actual_name {
                    return Err(ValueError::StructName(format!(
                        "Expected struct name {} got {}",
                        expected_name, actual_name
                    )));
                }
            }
            (ConstrainedValue::Return(ref values), _type) => {
                for value in values {
                    value.expect_type(_type)?;
                }
            }
            (value, _type) => {
                return Err(ValueError::TypeError(format!(
                    "expected type {}, got {}",
                    _type, value
                )))
            }
        }

        Ok(())
    }
}

impl<F: Field + PrimeField, G: Group> fmt::Display for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConstrainedValue::Integer(ref value) => write!(f, "{}", value),
            ConstrainedValue::FieldElement(ref value) => write!(f, "{}", value),
            ConstrainedValue::GroupElement(ref value) => write!(f, "{}", value),
            ConstrainedValue::Boolean(ref value) => write!(f, "{}", value.get_value().unwrap()),
            ConstrainedValue::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ConstrainedValue::CircuitExpression(ref variable, ref members) => {
                write!(f, "{} {{", variable)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.0, member.1)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            ConstrainedValue::Return(ref values) => {
                write!(f, "Program output: [")?;
                for (i, value) in values.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < values.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ConstrainedValue::CircuitDefinition(ref _definition) => {
                unimplemented!("cannot return struct definition in program")
            }
            ConstrainedValue::Function(ref function) => write!(f, "{}();", function.function_name),
        }
    }
}

impl<F: Field + PrimeField, G: Group> fmt::Debug for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
