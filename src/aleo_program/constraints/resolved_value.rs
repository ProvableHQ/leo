use crate::aleo_program::types::{Function, Struct, StructMember, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{utilities::boolean::Boolean, utilities::uint32::UInt32};
use std::fmt;

#[derive(Clone)]
pub enum ResolvedValue<F: Field + PrimeField> {
    U32(UInt32),
    U32Array(Vec<UInt32>),
    FieldElement(F),
    FieldElementArray(Vec<F>),
    Boolean(Boolean),
    BooleanArray(Vec<Boolean>),
    StructDefinition(Struct<F>),
    StructExpression(Variable<F>, Vec<StructMember<F>>),
    Function(Function<F>),
    Return(Vec<ResolvedValue<F>>), // add Null for function returns
}

impl<F: Field + PrimeField> fmt::Display for ResolvedValue<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolvedValue::U32(ref value) => write!(f, "{}", value.value.unwrap()),
            ResolvedValue::U32Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e.value.unwrap())?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::FieldElement(ref value) => write!(f, "{}", value),
            ResolvedValue::FieldElementArray(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::Boolean(ref value) => write!(f, "{}", value.get_value().unwrap()),
            ResolvedValue::BooleanArray(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e.get_value().unwrap())?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::StructExpression(ref variable, ref members) => {
                write!(f, "{} {{", variable)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.variable, member.expression)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            ResolvedValue::Return(ref values) => {
                write!(f, "Return values : [")?;
                for (i, value) in values.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < values.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::StructDefinition(ref _definition) => {
                unimplemented!("cannot return struct definition in program")
            }
            ResolvedValue::Function(ref _function) => {
                unimplemented!("cannot return function definition in program")
            } // _ => unimplemented!("display not impl for value"),
        }
    }
}
