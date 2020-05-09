//! The in memory stored value for a defined name in a resolved Leo program.

use crate::{
    errors::ValueError,
    types::{FieldElement, Function, Struct, Type, Variable},
    Integer,
};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::utilities::boolean::Boolean,
};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstrainedStructMember<F: Field + PrimeField>(pub Variable<F>, pub ConstrainedValue<F>);

#[derive(Clone, PartialEq, Eq)]
pub enum ConstrainedValue<F: Field + PrimeField> {
    Integer(Integer),
    FieldElement(FieldElement<F>),
    Boolean(Boolean),
    Array(Vec<ConstrainedValue<F>>),
    StructDefinition(Struct<F>),
    StructExpression(Variable<F>, Vec<ConstrainedStructMember<F>>),
    Function(Function<F>),
    Return(Vec<ConstrainedValue<F>>), // add Null for function returns
}

impl<F: Field + PrimeField> ConstrainedValue<F> {
    pub(crate) fn expect_type(&self, _type: &Type<F>) -> Result<(), ValueError> {
        match (self, _type) {
            (ConstrainedValue::Integer(ref integer), Type::IntegerType(ref _type)) => {
                integer.expect_type(_type)?;
            }
            (ConstrainedValue::FieldElement(ref _f), Type::FieldElement) => {}
            (ConstrainedValue::Boolean(ref _b), Type::Boolean) => {}
            (ConstrainedValue::Array(ref arr), Type::Array(ref ty, ref len)) => {
                // check array lengths are equal
                if arr.len() != *len {
                    return Err(ValueError::ArrayLength(format!(
                        "Expected array {:?} to be length {}",
                        arr, len
                    )));
                }
                // check each value in array matches
                for value in arr {
                    value.expect_type(ty)?;
                }
            }
            (
                ConstrainedValue::StructExpression(ref actual_name, ref _members),
                Type::Struct(ref expected_name),
            ) => {
                if expected_name != actual_name {
                    return Err(ValueError::StructName(format!(
                        "Expected struct name {} got {}",
                        expected_name, actual_name
                    )));
                }
            }
            (ConstrainedValue::Return(ref values), ty) => {
                for value in values {
                    value.expect_type(ty)?;
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

impl<F: Field + PrimeField> fmt::Display for ConstrainedValue<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConstrainedValue::Integer(ref value) => write!(f, "{}", value),
            ConstrainedValue::FieldElement(ref value) => write!(f, "{}", value),
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
            ConstrainedValue::StructExpression(ref variable, ref members) => {
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
            ConstrainedValue::StructDefinition(ref _definition) => {
                unimplemented!("cannot return struct definition in program")
            }
            ConstrainedValue::Function(ref function) => write!(f, "{}();", function.function_name),
        }
    }
}

impl<F: Field + PrimeField> fmt::Debug for ConstrainedValue<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
