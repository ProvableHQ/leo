//! The in memory stored value for a defined name in a resolved Leo program.

use crate::{
    allocate_bool,
    allocate_field,
    allocate_group,
    errors::ValueError,
    new_bool_constant,
    FieldType,
    GroupType,
};
use leo_types::{Circuit, Function, Identifier, Integer, Span, Type};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::ConditionalEqGadget, select::CondSelectGadget},
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
    pub(crate) fn from_other(value: String, other: &ConstrainedValue<F, G>, span: Span) -> Result<Self, ValueError> {
        let other_type = other.to_type(span.clone())?;

        ConstrainedValue::from_type(value, &other_type, span)
    }

    pub(crate) fn from_type(value: String, _type: &Type, span: Span) -> Result<Self, ValueError> {
        match _type {
            Type::IntegerType(integer_type) => Ok(ConstrainedValue::Integer(Integer::new_constant(
                integer_type,
                value,
                span,
            )?)),
            Type::Field => Ok(ConstrainedValue::Field(FieldType::constant(value, span)?)),
            Type::Group => Ok(ConstrainedValue::Group(G::constant(value, span)?)),
            Type::Boolean => Ok(ConstrainedValue::Boolean(new_bool_constant(value, span)?)),
            Type::Array(ref _type, _dimensions) => ConstrainedValue::from_type(value, _type, span),
            _ => Ok(ConstrainedValue::Unresolved(value)),
        }
    }

    pub(crate) fn to_type(&self, span: Span) -> Result<Type, ValueError> {
        Ok(match self {
            ConstrainedValue::Integer(integer) => Type::IntegerType(integer.get_type()),
            ConstrainedValue::Field(_field) => Type::Field,
            ConstrainedValue::Group(_group) => Type::Group,
            ConstrainedValue::Boolean(_bool) => Type::Boolean,
            value => return Err(ValueError::implicit(value.to_string(), span)),
        })
    }

    pub(crate) fn resolve_type(&mut self, types: &Vec<Type>, span: Span) -> Result<(), ValueError> {
        if let ConstrainedValue::Unresolved(ref string) = self {
            if !types.is_empty() {
                *self = ConstrainedValue::from_type(string.clone(), &types[0], span)?
            }
        }

        Ok(())
    }

    /// Expect both `self` and `other` to resolve to the same type
    pub(crate) fn resolve_types(&mut self, other: &mut Self, types: &Vec<Type>, span: Span) -> Result<(), ValueError> {
        if !types.is_empty() {
            self.resolve_type(types, span.clone())?;
            return other.resolve_type(types, span);
        }

        match (&self, &other) {
            (ConstrainedValue::Unresolved(_), ConstrainedValue::Unresolved(_)) => Ok(()),
            (ConstrainedValue::Unresolved(_), _) => self.resolve_type(&vec![other.to_type(span.clone())?], span),
            (_, ConstrainedValue::Unresolved(_)) => other.resolve_type(&vec![self.to_type(span.clone())?], span),
            _ => Ok(()),
        }
    }

    pub(crate) fn get_inner_mut(&mut self) {
        if let ConstrainedValue::Mutable(inner) = self {
            *self = *inner.clone()
        }
    }

    pub(crate) fn allocate_value<CS: ConstraintSystem<F>>(&mut self, mut cs: CS, span: Span) -> Result<(), ValueError> {
        match self {
            // allocated values
            ConstrainedValue::Boolean(boolean) => {
                let option = boolean.get_value();
                let name = option.map(|b| b.to_string()).unwrap_or(format!("[allocated]"));

                *boolean = allocate_bool(&mut cs, name, option, span)?;
            }
            ConstrainedValue::Integer(integer) => {
                let integer_type = integer.get_type();
                let option = integer.get_value();
                let name = option.map(|n| n.to_string()).unwrap_or(format!("[allocated]"));

                *integer = Integer::allocate_type(&mut cs, integer_type, name, option, span)?;
            }
            ConstrainedValue::Field(field) => {
                let option = field.get_value().map(|v| format!("{}", v));
                let name = option.clone().map(|f| f.to_string()).unwrap_or(format!("[allocated]"));

                *field = allocate_field(&mut cs, name, option, span)?;
            }
            ConstrainedValue::Group(group) => {
                let name = format!("{}", group); // may need to implement u256 -> decimal formatting
                let option = Some(name.clone());

                *group = allocate_group(&mut cs, name, option, span)?;
            }
            // value wrappers
            ConstrainedValue::Array(array) => {
                array
                    .iter_mut()
                    .enumerate()
                    .map(|(i, value)| {
                        let unique_name = format!("allocate array member {} {}:{}", i, span.line, span.start);

                        value.allocate_value(cs.ns(|| unique_name), span.clone())
                    })
                    .collect::<Result<(), ValueError>>()?;
            }
            ConstrainedValue::CircuitExpression(_id, members) => {
                members
                    .iter_mut()
                    .enumerate()
                    .map(|(i, member)| {
                        let unique_name = format!("allocate circuit member {} {}:{}", i, span.line, span.start);

                        member.1.allocate_value(cs.ns(|| unique_name), span.clone())
                    })
                    .collect::<Result<(), ValueError>>()?;
            }
            ConstrainedValue::Return(array) => {
                array
                    .iter_mut()
                    .enumerate()
                    .map(|(i, value)| {
                        let unique_name = format!("allocate return member {} {}:{}", i, span.line, span.start);

                        value.allocate_value(cs.ns(|| unique_name), span.clone())
                    })
                    .collect::<Result<(), ValueError>>()?;
            }
            ConstrainedValue::Mutable(value) => {
                value.allocate_value(cs, span)?;
            }
            ConstrainedValue::Static(value) => {
                value.allocate_value(cs, span)?;
            }
            // empty wrappers
            ConstrainedValue::CircuitDefinition(_) => {}
            ConstrainedValue::Function(_, _) => {}
            ConstrainedValue::Unresolved(value) => {
                return Err(ValueError::implicit(value.to_string(), span));
            }
        }

        Ok(())
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> fmt::Display for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConstrainedValue::Integer(ref value) => write!(f, "{}", value),
            ConstrainedValue::Field(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Group(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Boolean(ref value) => write!(
                f,
                "{}",
                value
                    .get_value()
                    .map(|v| v.to_string())
                    .unwrap_or(format!("[allocated]"))
            ),
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
            ConstrainedValue::CircuitDefinition(ref circuit) => write!(f, "circuit {{ {} }}", circuit.circuit_name),
            ConstrainedValue::Function(ref _circuit_option, ref function) => {
                write!(f, "function {{ {}() }}", function.function_name)
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
            (ConstrainedValue::Mutable(first), _) => Self::conditionally_select(cs, cond, first, second)?,
            (_, ConstrainedValue::Mutable(second)) => Self::conditionally_select(cs, cond, first, second)?,
            (_, _) => return Err(SynthesisError::Unsatisfiable),
        })
    }

    fn cost() -> usize {
        unimplemented!() //lower bound 1, upper bound 128 or length of static array
    }
}
