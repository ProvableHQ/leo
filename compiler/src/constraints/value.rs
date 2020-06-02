//! The in memory stored value for a defined name in a resolved Leo program.

use crate::{
    errors::ValueError,
    types::{Circuit, Function, Identifier, Integer, IntegerType, Type},
    FieldType, GroupType,
};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::utilities::{
        boolean::Boolean, uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64,
        uint8::UInt8,
    },
};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstrainedCircuitMember<F: Field + PrimeField, G: GroupType<F>>(
    pub Identifier,
    pub ConstrainedValue<F, G>,
);

#[derive(Clone, PartialEq, Eq)]
pub enum ConstrainedValue<F: Field + PrimeField, G: GroupType<F>> {
    Integer(Integer),
    Field(FieldType<F>),
    Group(G),
    Boolean(Boolean),

    Array(Vec<ConstrainedValue<F, G>>),

    CircuitDefinition(Circuit),
    CircuitExpression(Identifier, Vec<ConstrainedCircuitMember<F, G>>),

    Function(Option<Identifier>, Function), // (optional circuit identifier, function definition)
    Return(Vec<ConstrainedValue<F, G>>),

    Mutable(Box<ConstrainedValue<F, G>>),
    Static(Box<ConstrainedValue<F, G>>),
    Unresolved(String),
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedValue<F, G> {
    pub(crate) fn from_other(
        value: String,
        other: &ConstrainedValue<F, G>,
    ) -> Result<Self, ValueError> {
        let other_type = other.to_type();

        ConstrainedValue::from_type(value, &other_type)
    }

    pub(crate) fn from_type(value: String, _type: &Type) -> Result<Self, ValueError> {
        match _type {
            Type::IntegerType(integer_type) => Ok(ConstrainedValue::Integer(match integer_type {
                IntegerType::U8 => Integer::U8(UInt8::constant(value.parse::<u8>()?)),
                IntegerType::U16 => Integer::U16(UInt16::constant(value.parse::<u16>()?)),
                IntegerType::U32 => Integer::U32(UInt32::constant(value.parse::<u32>()?)),
                IntegerType::U64 => Integer::U64(UInt64::constant(value.parse::<u64>()?)),
                IntegerType::U128 => Integer::U128(UInt128::constant(value.parse::<u128>()?)),
            })),
            Type::Field => Ok(ConstrainedValue::Field(FieldType::constant(value)?)),
            Type::Group => Ok(ConstrainedValue::Group(G::constant(value)?)),
            Type::Boolean => Ok(ConstrainedValue::Boolean(Boolean::Constant(
                value.parse::<bool>()?,
            ))),
            Type::Array(ref _type, _dimensions) => ConstrainedValue::from_type(value, _type),
            _ => Ok(ConstrainedValue::Unresolved(value)),
        }
    }

    pub(crate) fn to_type(&self) -> Type {
        match self {
            ConstrainedValue::Integer(integer) => Type::IntegerType(integer.get_type()),
            ConstrainedValue::Field(_field) => Type::Field,
            ConstrainedValue::Group(_group) => Type::Group,
            ConstrainedValue::Boolean(_bool) => Type::Boolean,
            _ => unimplemented!("to type only implemented for primitives"),
        }
    }

    pub(crate) fn resolve_type(&mut self, types: &Vec<Type>) -> Result<(), ValueError> {
        if let ConstrainedValue::Unresolved(ref string) = self {
            if !types.is_empty() {
                *self = ConstrainedValue::from_type(string.clone(), &types[0])?
            }
        }

        Ok(())
    }

    pub(crate) fn get_inner_mut(&mut self) {
        if let ConstrainedValue::Mutable(inner) = self {
            *self = *inner.clone()
        }
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> fmt::Display for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConstrainedValue::Integer(ref value) => write!(f, "{}", value),
            ConstrainedValue::Field(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Group(ref value) => write!(f, "{:?}", value),
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
            ConstrainedValue::CircuitExpression(ref identifier, ref members) => {
                write!(f, "{} {{", identifier)?;
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
                unimplemented!("cannot return circuit definition in program")
            }
            ConstrainedValue::Function(ref _circuit_option, ref function) => {
                write!(f, "{}", function)
            }
            ConstrainedValue::Mutable(ref value) => write!(f, "mut {}", value),
            ConstrainedValue::Static(ref value) => write!(f, "static {}", value),
            ConstrainedValue::Unresolved(ref value) => write!(f, "unresolved {}", value),
        }
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> fmt::Debug for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
