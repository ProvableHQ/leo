//! The in memory stored value for a defined name in a resolved Leo program.

use crate::{errors::ValueError, FieldType, GroupType};
use leo_types::{Circuit, Function, Identifier, Integer, IntegerType, Type};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::ConditionalEqGadget,
            select::CondSelectGadget,
            uint::{UInt128, UInt16, UInt32, UInt64, UInt8},
        },
    },
};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstrainedCircuitMember<F: Field + PrimeField, G: GroupType<F>>(pub Identifier, pub ConstrainedValue<F, G>);

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
    pub(crate) fn from_other(value: String, other: &ConstrainedValue<F, G>) -> Result<Self, ValueError> {
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
            Type::Boolean => Ok(ConstrainedValue::Boolean(Boolean::Constant(value.parse::<bool>()?))),
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

    /// Expect both `self` and `other` to resolve to the same type
    pub(crate) fn resolve_types(&mut self, other: &mut Self, types: &Vec<Type>) -> Result<(), ValueError> {
        if !types.is_empty() {
            self.resolve_type(types)?;
            return other.resolve_type(types);
        }

        match (&self, &other) {
            (ConstrainedValue::Unresolved(_), ConstrainedValue::Unresolved(_)) => Ok(()),
            (ConstrainedValue::Unresolved(_), _) => self.resolve_type(&vec![other.to_type()]),
            (_, ConstrainedValue::Unresolved(_)) => other.resolve_type(&vec![self.to_type()]),
            _ => Ok(()),
        }
    }

    pub(crate) fn get_inner_mut(&mut self) {
        if let ConstrainedValue::Mutable(inner) = self {
            *self = *inner.clone()
        }
    }

    pub(crate) fn allocate_value<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Self, ValueError> {
        Ok(match self {
            ConstrainedValue::Boolean(ref boolean) => {
                let option = boolean.get_value();
                let allocated = Boolean::alloc(cs, || option.ok_or(SynthesisError::AssignmentMissing))?;

                ConstrainedValue::Boolean(allocated)
            }
            ConstrainedValue::Integer(ref integer) => {
                let integer_type = integer.get_type();
                let option = integer.get_value();
                let name = format!("clone {}", integer);
                let allocated = Integer::allocate_type(&mut cs, integer_type, name, option)?;

                ConstrainedValue::Integer(allocated)
            }
            ConstrainedValue::Field(ref field) => {
                let option = field.get_value().map(|v| format!("{}", v));
                let allocated = FieldType::alloc(cs, || option.ok_or(SynthesisError::AssignmentMissing))?;

                ConstrainedValue::Field(allocated)
            }
            ConstrainedValue::Group(ref group) => {
                let string = format!("{}", group);
                let allocated = G::alloc(cs, || Ok(string))?;

                ConstrainedValue::Group(allocated)
            }
            ConstrainedValue::Array(ref array) => {
                let allocated = array
                    .iter()
                    .enumerate()
                    .map(|(i, value)| value.allocate_value(cs.ns(|| format!("allocate {}", i))))
                    .collect::<Result<Vec<ConstrainedValue<F, G>>, ValueError>>()?;

                ConstrainedValue::Array(allocated)
            }
            _ => unimplemented!("cannot allocate non-primitive value"),
        })
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
            ConstrainedValue::Function(ref _circuit_option, ref function) => write!(f, "{}", function),
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

impl<F: Field + PrimeField, G: GroupType<F>> ConditionalEqGadget<F> for ConstrainedValue<F, G> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => bool_1.conditional_enforce_equal(
                cs.ns(|| format!("{} == {}", self.to_string(), other.to_string())),
                bool_2,
                &condition,
            ),
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => num_1.conditional_enforce_equal(
                cs.ns(|| format!("{} == {}", self.to_string(), other.to_string())),
                num_2,
                &condition,
            ),
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => field_1.conditional_enforce_equal(
                cs.ns(|| format!("{} == {}", self.to_string(), other.to_string())),
                field_2,
                &condition,
            ),
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => group_1.conditional_enforce_equal(
                cs.ns(|| format!("{} == {}", self.to_string(), other.to_string())),
                group_2,
                &condition,
            ),
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                for (i, (left, right)) in arr_1.into_iter().zip(arr_2.into_iter()).enumerate() {
                    left.conditional_enforce_equal(
                        cs.ns(|| format!("array[{}] equal {} == {}", i, left.to_string(), right.to_string())),
                        right,
                        &condition,
                    )?;
                }
                Ok(())
            }
            (_, _) => return Err(SynthesisError::Unsatisfiable),
        }
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> CondSelectGadget<F> for ConstrainedValue<F, G> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        Ok(match (first, second) {
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                ConstrainedValue::Boolean(Boolean::conditionally_select(
                    cs.ns(|| format!("if cond ? {} else {}", first.to_string(), second.to_string())),
                    cond,
                    bool_1,
                    bool_2,
                )?)
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                ConstrainedValue::Integer(Integer::conditionally_select(
                    cs.ns(|| format!("if cond ? {} else {}", first.to_string(), second.to_string())),
                    cond,
                    num_1,
                    num_2,
                )?)
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                ConstrainedValue::Field(FieldType::conditionally_select(
                    cs.ns(|| format!("if cond ? {} else {}", first.to_string(), second.to_string())),
                    cond,
                    field_1,
                    field_2,
                )?)
            }
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => {
                ConstrainedValue::Group(G::conditionally_select(
                    cs.ns(|| format!("if cond ? {} else {}", first.to_string(), second.to_string())),
                    cond,
                    group_1,
                    group_2,
                )?)
            }
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                let mut array = vec![];
                for (i, (first, second)) in arr_1.into_iter().zip(arr_2.into_iter()).enumerate() {
                    array.push(Self::conditionally_select(
                        cs.ns(|| {
                            format!(
                                "array[{}] = if cond ? {} else {}",
                                i,
                                first.to_string(),
                                second.to_string()
                            )
                        }),
                        cond,
                        first,
                        second,
                    )?);
                }
                ConstrainedValue::Array(array)
            }
            (_, _) => return Err(SynthesisError::Unsatisfiable),
        })
    }

    fn cost() -> usize {
        unimplemented!() //lower bound 1, upper bound 128 or length of static array
    }
}
