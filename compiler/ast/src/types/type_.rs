// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{common, ArrayType, CompositeType, FutureType, Identifier, IntegerType, MappingType, TupleType};

use itertools::Itertools;
use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{
    Network,
    PlaintextType,
    PlaintextType::{Array, Literal, Struct},
};
use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// The `address` type.
    Address,
    /// The array type.
    Array(ArrayType),
    /// The `bool` type.
    Boolean,
    /// The `struct` type.
    Composite(CompositeType),
    /// The `field` type.
    Field,
    /// The `future` type.
    Future(FutureType),
    /// The `group` type.
    Group,
    /// A reference to a built in type.
    Identifier(Identifier),
    /// An integer type.
    Integer(IntegerType),
    /// A mapping type.
    Mapping(MappingType),
    /// The `scalar` type.
    Scalar,
    /// The `signature` type.
    Signature,
    /// The `string` type.
    String,
    /// A static tuple of at least one type.
    Tuple(TupleType),
    /// The `unit` type.
    Unit,
    /// Placeholder for a type that could not be resolved or was not well-formed.
    /// Will eventually lead to a compile error.
    Err,
}

impl Type {
    ///
    /// Returns `true` if the self `Type` is equal to the other `Type`.
    ///
    /// Flattens array syntax: `[[u8; 1]; 2] == [u8; (2, 1)] == true`
    ///
    pub fn eq_flat(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Address, Type::Address)
            | (Type::Boolean, Type::Boolean)
            | (Type::Field, Type::Field)
            | (Type::Group, Type::Group)
            | (Type::Scalar, Type::Scalar)
            | (Type::Signature, Type::Signature)
            | (Type::String, Type::String)
            | (Type::Unit, Type::Unit) => true,
            (Type::Array(left), Type::Array(right)) => {
                left.element_type().eq_flat(right.element_type()) && left.length() == right.length()
            }
            (Type::Identifier(left), Type::Identifier(right)) => left.name == right.name,
            (Type::Integer(left), Type::Integer(right)) => left.eq(right),
            (Type::Mapping(left), Type::Mapping(right)) => {
                left.key.eq_flat(&right.key) && left.value.eq_flat(&right.value)
            }
            (Type::Tuple(left), Type::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| left_type.eq_flat(right_type)),
            (Type::Composite(left), Type::Composite(right)) => {
                left.id.name == right.id.name && left.program == right.program
            }
            (Type::Future(left), Type::Future(right))
                if left.inputs.len() == right.inputs.len() && left.location.is_some() && right.location.is_some() =>
            {
                left.location == right.location
                    && left
                        .inputs()
                        .iter()
                        .zip_eq(right.inputs().iter())
                        .all(|(left_type, right_type)| left_type.eq_flat(right_type))
            }
            _ => false,
        }
    }

    ///
    /// Returns `true` if the self `Type` is equal to the other `Type` in all aspects besides composite program of origin.
    ///
    /// Flattens array syntax: `[[u8; 1]; 2] == [u8; (2, 1)] == true`
    ///
    pub fn eq_flat_relax_composite(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Address, Type::Address)
            | (Type::Boolean, Type::Boolean)
            | (Type::Field, Type::Field)
            | (Type::Group, Type::Group)
            | (Type::Scalar, Type::Scalar)
            | (Type::Signature, Type::Signature)
            | (Type::String, Type::String)
            | (Type::Unit, Type::Unit) => true,
            (Type::Array(left), Type::Array(right)) => {
                left.element_type().eq_flat_relax_composite(right.element_type()) && left.length() == right.length()
            }
            (Type::Identifier(left), Type::Identifier(right)) => left.matches(right),
            (Type::Integer(left), Type::Integer(right)) => left.eq(right),
            (Type::Mapping(left), Type::Mapping(right)) => {
                left.key.eq_flat_relax_composite(&right.key) && left.value.eq_flat(&right.value)
            }
            (Type::Tuple(left), Type::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| left_type.eq_flat_relax_composite(right_type)),
            (Type::Composite(left), Type::Composite(right)) => left.id.name == right.id.name,
            // Don't type check when type hasn't been explicitly defined.
            (Type::Future(left), Type::Future(right)) if !left.is_explicit || !right.is_explicit => true,
            (Type::Future(left), Type::Future(right)) if left.inputs.len() == right.inputs.len() => left
                .inputs()
                .iter()
                .zip_eq(right.inputs().iter())
                .all(|(left_type, right_type)| left_type.eq_flat_relax_composite(right_type)),
            _ => false,
        }
    }

    pub fn from_snarkvm<N: Network>(t: &PlaintextType<N>, program: Option<Symbol>) -> Self {
        match t {
            Literal(lit) => match lit {
                snarkvm::prelude::LiteralType::Address => Type::Address,
                snarkvm::prelude::LiteralType::Boolean => Type::Boolean,
                snarkvm::prelude::LiteralType::Field => Type::Field,
                snarkvm::prelude::LiteralType::Group => Type::Group,
                snarkvm::prelude::LiteralType::U8 => Type::Integer(IntegerType::U8),
                snarkvm::prelude::LiteralType::U16 => Type::Integer(IntegerType::U16),
                snarkvm::prelude::LiteralType::U32 => Type::Integer(IntegerType::U32),
                snarkvm::prelude::LiteralType::U64 => Type::Integer(IntegerType::U64),
                snarkvm::prelude::LiteralType::U128 => Type::Integer(IntegerType::U128),
                snarkvm::prelude::LiteralType::I8 => Type::Integer(IntegerType::I8),
                snarkvm::prelude::LiteralType::I16 => Type::Integer(IntegerType::I16),
                snarkvm::prelude::LiteralType::I32 => Type::Integer(IntegerType::I32),
                snarkvm::prelude::LiteralType::I64 => Type::Integer(IntegerType::I64),
                snarkvm::prelude::LiteralType::I128 => Type::Integer(IntegerType::I128),
                snarkvm::prelude::LiteralType::Scalar => Type::Scalar,
                snarkvm::prelude::LiteralType::Signature => Type::Signature,
                snarkvm::prelude::LiteralType::String => Type::String,
            },
            Struct(s) => Type::Composite(CompositeType { id: common::Identifier::from(s), program }),
            Array(array) => Type::Array(ArrayType::from_snarkvm(array, program)),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Address => write!(f, "address"),
            Type::Array(ref array_type) => write!(f, "{array_type}"),
            Type::Boolean => write!(f, "boolean"),
            Type::Field => write!(f, "field"),
            Type::Future(ref future_type) => write!(f, "{future_type}"),
            Type::Group => write!(f, "group"),
            Type::Identifier(ref variable) => write!(f, "{variable}"),
            Type::Integer(ref integer_type) => write!(f, "{integer_type}"),
            Type::Mapping(ref mapping_type) => write!(f, "{mapping_type}"),
            Type::Scalar => write!(f, "scalar"),
            Type::Signature => write!(f, "signature"),
            Type::String => write!(f, "string"),
            Type::Composite(ref struct_type) => write!(f, "{}", struct_type.id.name),
            Type::Tuple(ref tuple) => write!(f, "{tuple}"),
            Type::Unit => write!(f, "()"),
            Type::Err => write!(f, "error"),
        }
    }
}
