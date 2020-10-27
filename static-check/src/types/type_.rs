// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.
use crate::{SymbolTable, TypeError, TypeVariable};
use leo_typed::{Identifier, IntegerType, Span, Type as UnresolvedType};

use serde::{Deserialize, Serialize};
use std::{
    cmp::{Eq, PartialEq},
    fmt,
};

/// A type in a Leo program.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    Array(Box<Type>, Vec<usize>),
    Tuple(Vec<Type>),

    // User defined types
    Circuit(Identifier),
    Function(Identifier),

    // Unknown type variables
    TypeVariable(TypeVariable),
}

impl Type {
    ///
    /// Return a new type from the given unresolved type.
    ///
    /// Performs a lookup in the given symbol table if the type is user-defined.
    ///
    pub fn new(table: &SymbolTable, type_: UnresolvedType, span: Span) -> Result<Self, TypeError> {
        Ok(match type_ {
            UnresolvedType::Address => Type::Address,
            UnresolvedType::Boolean => Type::Boolean,
            UnresolvedType::Field => Type::Field,
            UnresolvedType::Group => Type::Group,
            UnresolvedType::IntegerType(integer) => Type::IntegerType(integer),

            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::new(table, *type_, span)?;

                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::new(table, type_, span.clone()))
                    .collect::<Result<Vec<_>, _>>()?;

                Type::Tuple(tuple_types)
            }

            UnresolvedType::Circuit(identifier) => {
                // Lookup the circuit type in the symbol table
                let circuit_type = table
                    .get_circuit_type(&identifier.name)
                    .ok_or(TypeError::undefined_circuit(identifier))?;

                Type::Circuit(circuit_type.identifier.clone())
            }

            UnresolvedType::SelfType => {
                // Throw an error for using `Self` outside of a circuit
                return Err(TypeError::self_not_available(span));
            }
        })
    }

    ///
    /// Return a new type from the given unresolved type.
    ///
    /// If this type is SelfType, return the circuit's type.
    ///
    pub fn new_from_circuit(
        table: &SymbolTable,
        type_: UnresolvedType,
        circuit_name: Identifier,
        span: Span,
    ) -> Result<Self, TypeError> {
        Ok(match type_ {
            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::new_from_circuit(table, *type_, circuit_name, span)?;
                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::new_from_circuit(table, type_, circuit_name.clone(), span.clone()))
                    .collect::<Result<Vec<_>, _>>()?;

                Type::Tuple(tuple_types)
            }
            UnresolvedType::SelfType => Type::Circuit(circuit_name),
            // The unresolved type does not depend on the current circuit definition
            unresolved => Type::new(table, unresolved, span)?,
        })
    }

    ///
    /// Returns `Ok` if the given expected type is `Some` and expected type == actual type.
    ///
    pub fn check_type(expected_option: &Option<Self>, actual: &Type, span: Span) -> Result<(), TypeError> {
        if let Some(expected) = expected_option {
            if expected.ne(actual) {
                return Err(TypeError::mismatched_types(expected, actual, span));
            }
        }
        Ok(())
    }

    ///
    /// Returns `Ok` if self is an expected integer type `Type::IntegerType`.
    ///
    pub fn check_type_integer(&self, span: Span) -> Result<(), TypeError> {
        match self {
            Type::IntegerType(_) => Ok(()),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_integer(type_, span)),
        }
    }

    ///
    /// Returns array element type and dimensions if self is an expected array type `Type::Array`.
    ///
    pub fn get_type_array(&self, span: Span) -> Result<(&Type, &Vec<usize>), TypeError> {
        match self {
            Type::Array(element_type, dimensions) => Ok((element_type, dimensions)),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_array(type_, span)),
        }
    }

    ///
    /// Returns tuple element types if self is an expected tuple type `Type::Tuple`.
    ///
    pub fn get_type_tuple(&self, span: Span) -> Result<&Vec<Type>, TypeError> {
        match self {
            Type::Tuple(types) => Ok(types),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_tuple(type_, span)),
        }
    }

    ///
    /// Returns circuit identifier if self is an expected circuit type `Type::Circuit`.
    ///
    pub fn get_type_circuit(&self, span: Span) -> Result<&Identifier, TypeError> {
        match self {
            Type::Circuit(identifier) => Ok(identifier),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_circuit(type_, span)),
        }
    }

    ///
    /// Returns function identifier if self is an expected function type `Type::Function`.
    ///
    pub fn get_type_function(&self, span: Span) -> Result<&Identifier, TypeError> {
        match self {
            Type::Function(identifier) => Ok(identifier),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_function(type_, span)),
        }
    }

    /// Returns a list of signed integer types.
    pub fn signed_integer_types() -> Vec<Type> {
        vec![
            Type::IntegerType(IntegerType::I8),
            Type::IntegerType(IntegerType::I16),
            Type::IntegerType(IntegerType::I32),
            Type::IntegerType(IntegerType::I64),
            Type::IntegerType(IntegerType::I128),
        ]
    }

    /// Returns a list of unsigned integer types.
    pub fn unsigned_integer_types() -> Vec<Type> {
        vec![
            Type::IntegerType(IntegerType::U8),
            Type::IntegerType(IntegerType::U16),
            Type::IntegerType(IntegerType::U32),
            Type::IntegerType(IntegerType::U64),
            Type::IntegerType(IntegerType::U128),
        ]
    }

    /// Returns a list of positive integer types.
    pub fn negative_integer_types() -> Vec<Type> {
        let mut types = vec![Type::Field, Type::Group];

        types.append(&mut Self::signed_integer_types());

        types
    }

    /// Returns a list of integer types.
    pub fn integer_types() -> Vec<Type> {
        let mut types = Self::negative_integer_types();

        types.append(&mut Self::unsigned_integer_types());

        types
    }

    /// Returns a list of possible index types (u8, u16, u32).
    pub fn index_types() -> Vec<Type> {
        vec![
            Type::IntegerType(IntegerType::U8),
            Type::IntegerType(IntegerType::U16),
            Type::IntegerType(IntegerType::U32),
        ]
    }

    ///
    /// Replaces self with the given type if self is equal to the given `TypeVariable`.
    ///
    pub fn substitute(&mut self, variable: &TypeVariable, type_: &Type) {
        match self {
            Type::TypeVariable(self_variable) => {
                if self_variable == variable {
                    *self = type_.to_owned()
                }
            }
            Type::Array(self_type, _) => {
                self_type.substitute(variable, type_);
            }
            Type::Tuple(types) => types
                .iter_mut()
                .for_each(|tuple_type| tuple_type.substitute(variable, type_)),
            _ => {}
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::IntegerType(integer_type) => write!(f, "{}", integer_type),

            Type::Array(type_, dimensions) => {
                let dimensions_string = dimensions
                    .iter()
                    .map(|dimension| format!("{}", dimension))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "[{}; ({})]", *type_, dimensions_string)
            }
            Type::Tuple(tuple) => {
                let tuple_string = tuple.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ");

                write!(f, "({})", tuple_string)
            }

            Type::Circuit(identifier) => write!(f, "circuit {}", identifier),
            Type::Function(identifier) => write!(f, "function {}", identifier),
            Type::TypeVariable(type_variable) => write!(f, "{}", type_variable),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Address, Type::Address) => true,
            (Type::Boolean, Type::Boolean) => true,
            (Type::Field, Type::Field) => true,
            (Type::Group, Type::Group) => true,
            (Type::IntegerType(integer_type1), Type::IntegerType(integer_type2)) => integer_type1.eq(integer_type2),

            (Type::Array(type1, dimensions1), Type::Array(type2, dimensions2)) => {
                // Flatten both array types before comparison.
                let (type1_flat, dimensions1_flat) = flatten_array_type(type1, dimensions1.to_owned());
                let (type2_flat, dimensions2_flat) = flatten_array_type(type2, dimensions2.to_owned());

                // Element types and dimensions must match
                type1_flat.eq(type2_flat) && dimensions1_flat.eq(&dimensions2_flat)
            }

            (Type::Tuple(types1), Type::Tuple(types2)) => types1.eq(types2),
            (Type::Circuit(identifier1), Type::Circuit(identifier2)) => identifier1.eq(identifier2),
            (Type::Function(identifier1), Type::Function(identifier2)) => identifier1.eq(identifier2),
            (Type::TypeVariable(variable1), Type::TypeVariable(variable2)) => variable1.eq(variable2),
            _ => false,
        }
    }
}

impl Eq for Type {}

///
/// Returns the data type of the array element and vector of dimensions.
///
/// Will flatten an array type `[[[u8; 1]; 2]; 3]` into `[u8; (3, 2, 1)]`.
///
pub fn flatten_array_type(type_: &Type, mut dimensions: Vec<usize>) -> (&Type, Vec<usize>) {
    if let Type::Array(element_type, element_dimensions) = type_ {
        dimensions.append(&mut element_dimensions.to_owned());
        flatten_array_type(element_type, dimensions)
    } else {
        (type_, dimensions)
    }
}
