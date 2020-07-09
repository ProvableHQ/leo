//! The in memory stored value for a defined name in a compiled Leo program.

use crate::{
    boolean::input::{allocate_bool, new_bool_constant},
    errors::{ExpressionError, FieldError, ValueError},
    is_in_scope,
    new_scope,
    Address,
    FieldType,
    GroupType,
    Integer,
};
use leo_types::{Circuit, Function, Identifier, Span, Type};

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
    // Data types
    Address(Address),
    Boolean(Boolean),
    Field(FieldType<F>),
    Group(G),
    Integer(Integer),

    // Arrays
    Array(Vec<ConstrainedValue<F, G>>),

    // Circuits
    CircuitDefinition(Circuit),
    CircuitExpression(Identifier, Vec<ConstrainedCircuitMember<F, G>>),

    // Functions
    Function(Option<Identifier>, Function), // (optional circuit identifier, function definition)
    Return(Vec<ConstrainedValue<F, G>>),

    // Modifiers
    Mutable(Box<ConstrainedValue<F, G>>),
    Static(Box<ConstrainedValue<F, G>>),
    Unresolved(String),

    // Imports
    Import(String, Box<ConstrainedValue<F, G>>),
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedValue<F, G> {
    pub(crate) fn from_other(value: String, other: &ConstrainedValue<F, G>, span: Span) -> Result<Self, ValueError> {
        let other_type = other.to_type(span.clone())?;

        ConstrainedValue::from_type(value, &other_type, span)
    }

    pub(crate) fn from_type(value: String, _type: &Type, span: Span) -> Result<Self, ValueError> {
        match _type {
            // Data types
            Type::Address => Ok(ConstrainedValue::Address(Address::new(value, span)?)),
            Type::Boolean => Ok(ConstrainedValue::Boolean(new_bool_constant(value, span)?)),
            Type::Field => Ok(ConstrainedValue::Field(FieldType::constant(value, span)?)),
            Type::Group => Ok(ConstrainedValue::Group(G::constant(value, span)?)),
            Type::IntegerType(integer_type) => Ok(ConstrainedValue::Integer(Integer::new_constant(
                integer_type,
                value,
                span,
            )?)),

            // Data type wrappers
            Type::Array(ref _type, _dimensions) => ConstrainedValue::from_type(value, _type, span),
            _ => Ok(ConstrainedValue::Unresolved(value)),
        }
    }

    pub(crate) fn to_type(&self, span: Span) -> Result<Type, ValueError> {
        Ok(match self {
            // Data types
            ConstrainedValue::Address(_address) => Type::Address,
            ConstrainedValue::Boolean(_bool) => Type::Boolean,
            ConstrainedValue::Field(_field) => Type::Field,
            ConstrainedValue::Group(_group) => Type::Group,
            ConstrainedValue::Integer(integer) => Type::IntegerType(integer.get_type()),

            // Data type wrappers
            ConstrainedValue::Array(types) => {
                let array_type = types[0].to_type(span.clone())?;
                let count = types.len();

                // nested array type
                if let Type::Array(inner_type, inner_dimensions) = &array_type {
                    let mut dimensions = inner_dimensions.clone();
                    dimensions.push(count);
                    return Ok(Type::Array(inner_type.clone(), dimensions));
                }

                Type::Array(Box::new(array_type), vec![count])
            }
            ConstrainedValue::CircuitExpression(id, _members) => Type::Circuit(id.clone()),
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

    pub(crate) fn extract_function(self, scope: String, span: Span) -> Result<(String, Function), ExpressionError> {
        match self {
            ConstrainedValue::Function(circuit_identifier, function) => {
                let mut outer_scope = scope.clone();
                // If this is a circuit function, evaluate inside the circuit scope
                if let Some(identifier) = circuit_identifier {
                    // avoid creating recursive scope
                    if !is_in_scope(&scope, &identifier.name.to_string()) {
                        outer_scope = new_scope(scope, identifier.name.to_string());
                    }
                }

                Ok((outer_scope, function.clone()))
            }
            ConstrainedValue::Import(import_scope, function) => function.extract_function(import_scope, span),
            value => return Err(ExpressionError::undefined_function(value.to_string(), span)),
        }
    }

    pub(crate) fn extract_circuit(self, span: Span) -> Result<Circuit, ExpressionError> {
        match self {
            ConstrainedValue::CircuitDefinition(circuit) => Ok(circuit),
            ConstrainedValue::Import(_import_scope, circuit) => circuit.extract_circuit(span),
            value => return Err(ExpressionError::undefined_circuit(value.to_string(), span)),
        }
    }

    pub(crate) fn get_inner_mut(&mut self) {
        if let ConstrainedValue::Mutable(inner) = self {
            *self = *inner.clone()
        }
    }

    pub(crate) fn allocate_value<CS: ConstraintSystem<F>>(&mut self, mut cs: CS, span: Span) -> Result<(), ValueError> {
        match self {
            // Data types
            ConstrainedValue::Address(_address) => {
                // allow `let address()` even though addresses are constant
            }
            ConstrainedValue::Boolean(boolean) => {
                let option = boolean.get_value();
                let name = option.map(|b| b.to_string()).unwrap_or(format!("[allocated]"));

                *boolean = allocate_bool(&mut cs, name, option, span)?;
            }
            ConstrainedValue::Field(field) => {
                let gadget = field
                    .allocated(cs.ns(|| format!("allocate field {}:{}", span.line, span.start)))
                    .map_err(|error| ValueError::FieldError(FieldError::synthesis_error(error, span)))?;

                *field = FieldType::Allocated(gadget)
            }
            ConstrainedValue::Group(group) => {
                *group = group.to_allocated(cs, span)?;
            }
            ConstrainedValue::Integer(integer) => {
                let integer_type = integer.get_type();
                let option = integer.get_value();
                let name = option.map(|n| n.to_string()).unwrap_or(format!("[allocated]"));

                *integer = Integer::allocate_type(&mut cs, integer_type, name, option, span)?;
            }

            // Data type wrappers
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

            // Empty wrappers that are unreachable
            ConstrainedValue::CircuitDefinition(_) => {}
            ConstrainedValue::Function(_, _) => {}
            ConstrainedValue::Import(_, _) => {}

            // Cannot allocate an unresolved value
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
            // Data types
            ConstrainedValue::Address(ref value) => write!(f, "{}", value),
            ConstrainedValue::Boolean(ref value) => write!(
                f,
                "{}",
                value
                    .get_value()
                    .map(|v| v.to_string())
                    .unwrap_or(format!("[allocated]"))
            ),
            ConstrainedValue::Field(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Group(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Integer(ref value) => write!(f, "{}", value),

            // Data type wrappers
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
            ConstrainedValue::Import(_, ref value) => write!(f, "{}", value),
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
            (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
                address_1.conditional_enforce_equal(cs, address_2, condition)
            }
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                bool_1.conditional_enforce_equal(cs, bool_2, condition)
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                field_1.conditional_enforce_equal(cs, field_2, condition)
            }
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => {
                group_1.conditional_enforce_equal(cs, group_2, condition)
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                num_1.conditional_enforce_equal(cs, num_2, condition)
            }
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                for (i, (left, right)) in arr_1.into_iter().zip(arr_2.into_iter()).enumerate() {
                    left.conditional_enforce_equal(cs.ns(|| format!("array[{}]", i)), right, condition)?;
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
            (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
                ConstrainedValue::Address(Address::conditionally_select(cs, cond, address_1, address_2)?)
            }
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                ConstrainedValue::Boolean(Boolean::conditionally_select(cs, cond, bool_1, bool_2)?)
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                ConstrainedValue::Field(FieldType::conditionally_select(cs, cond, field_1, field_2)?)
            }
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => {
                ConstrainedValue::Group(G::conditionally_select(cs, cond, group_1, group_2)?)
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                ConstrainedValue::Integer(Integer::conditionally_select(cs, cond, num_1, num_2)?)
            }
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                let mut array = vec![];

                for (i, (first, second)) in arr_1.into_iter().zip(arr_2.into_iter()).enumerate() {
                    array.push(Self::conditionally_select(
                        cs.ns(|| format!("array[{}]", i)),
                        cond,
                        first,
                        second,
                    )?);
                }

                ConstrainedValue::Array(array)
            }
            (ConstrainedValue::Function(identifier_1, function_1), ConstrainedValue::Function(_, _)) => {
                // This is a no-op. functions cannot hold circuit values
                // However, we must return a result here
                ConstrainedValue::Function(identifier_1.clone(), function_1.clone())
            }
            (
                ConstrainedValue::CircuitExpression(identifier, members_1),
                ConstrainedValue::CircuitExpression(_identifier, members_2),
            ) => {
                let mut members = vec![];

                for (i, (first, second)) in members_1.into_iter().zip(members_2.into_iter()).enumerate() {
                    members.push(ConstrainedCircuitMember::conditionally_select(
                        cs.ns(|| format!("circuit member[{}]", i)),
                        cond,
                        first,
                        second,
                    )?);
                }

                ConstrainedValue::CircuitExpression(identifier.clone(), members)
            }
            (ConstrainedValue::Return(returns_1), ConstrainedValue::Return(returns_2)) => {
                let mut returns = vec![];

                for (i, (first, second)) in returns_1.into_iter().zip(returns_2.into_iter()).enumerate() {
                    returns.push(Self::conditionally_select(
                        cs.ns(|| format!("return[{}]", i)),
                        cond,
                        first,
                        second,
                    )?);
                }

                ConstrainedValue::Return(returns)
            }
            (ConstrainedValue::Static(first), ConstrainedValue::Static(second)) => {
                let value = Self::conditionally_select(cs, cond, first, second)?;

                ConstrainedValue::Static(Box::new(value))
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

impl<F: Field + PrimeField, G: GroupType<F>> CondSelectGadget<F> for ConstrainedCircuitMember<F, G> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        // identifiers will be the same
        let value = ConstrainedValue::conditionally_select(cs, cond, &first.1, &second.1)?;

        Ok(ConstrainedCircuitMember(first.0.clone(), value))
    }

    fn cost() -> usize {
        unimplemented!()
    }
}
