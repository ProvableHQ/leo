// Copyright (C) 2019-2021 Aleo Systems Inc.
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
use leo_ast::{Identifier, IntegerType, Span, Type as UnresolvedType};

use serde::{Deserialize, Serialize};
use std::{
    cmp::{Eq, PartialEq},
    fmt,
};

/// A type in a Leo program.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    Array(Box<Type>),
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

            UnresolvedType::Array(type_, _) => {
                let array_type = Type::new(table, *type_, span)?;

                Type::Array(Box::new(array_type))
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
                    .ok_or_else(|| TypeError::undefined_circuit(identifier))?;

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
            UnresolvedType::Array(type_, _) => {
                let array_type = Type::new_from_circuit(table, *type_, circuit_name, span)?;
                Type::Array(Box::new(array_type))
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

    /// Returns a list of signed integer types.
    pub const fn signed_integer_types() -> [Type; 5] {
        [
            Type::IntegerType(IntegerType::I8),
            Type::IntegerType(IntegerType::I16),
            Type::IntegerType(IntegerType::I32),
            Type::IntegerType(IntegerType::I64),
            Type::IntegerType(IntegerType::I128),
        ]
    }

    /// Returns a list of unsigned integer types.
    pub const fn unsigned_integer_types() -> [Type; 5] {
        [
            Type::IntegerType(IntegerType::U8),
            Type::IntegerType(IntegerType::U16),
            Type::IntegerType(IntegerType::U32),
            Type::IntegerType(IntegerType::U64),
            Type::IntegerType(IntegerType::U128),
        ]
    }

    /// Returns a list of positive integer types.
    pub fn negative_integer_types() -> Vec<Type> {
        let field_group = [Type::Field, Type::Group];

        let mut types = Vec::new();

        types.extend_from_slice(&field_group);
        types.extend_from_slice(&Self::signed_integer_types());

        types
    }

    /// Returns a list of integer types.
    pub fn integer_types() -> Vec<Type> {
        let mut types = Vec::new();

        types.extend_from_slice(&Self::unsigned_integer_types());
        types.extend_from_slice(&Self::negative_integer_types());

        types
    }

    /// Returns a list of possible index types (u8, u16, u32).
    pub fn index_types() -> Vec<Type> {
        let index_types = [
            Type::IntegerType(IntegerType::U8),
            Type::IntegerType(IntegerType::U16),
            Type::IntegerType(IntegerType::U32),
        ];

        let mut types = Vec::new();

        types.extend_from_slice(&index_types);

        types
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
            Type::Array(self_type) => {
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

            Type::Array(type_) => write!(f, "[{}]", *type_),
            Type::Tuple(tuple) => {
                let tuple_string = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

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

            (Type::Array(array1), Type::Array(array2)) => {
                // Get both array element types before comparison.
                let array1_element = get_array_element_type(array1);
                let array2_element = get_array_element_type(array2);

                // Check that both arrays have the same element type.
                array1_element.eq(array2_element)
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
/// Returns the data type of the array element.
///
/// If the given `type_` is an array, call `get_array_element_type()` on the array element type.
/// If the given `type_` is any other type, return the `type_`.
///
pub fn get_array_element_type(type_: &Type) -> &Type {
    if let Type::Array(element_type) = type_ {
        get_array_element_type(element_type)
    } else {
        type_
    }
}
