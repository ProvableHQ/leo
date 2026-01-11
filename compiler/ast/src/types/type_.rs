// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{
    ArrayType,
    CompositeType,
    FutureType,
    Identifier,
    IntegerType,
    Location,
    MappingType,
    OptionalType,
    Path,
    TupleType,
    VectorType,
};

use itertools::Itertools;
use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{
    LiteralType,
    Network,
    PlaintextType,
    PlaintextType::{Array, Literal, Struct},
};
use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// The `address` type.
    Address,
    /// The array type.
    Array(ArrayType),
    /// The `bool` type.
    Boolean,
    /// The composite type.
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
    /// A nullable type.
    Optional(OptionalType),
    /// The `scalar` type.
    Scalar,
    /// The `signature` type.
    Signature,
    /// The `string` type.
    String,
    /// A static tuple of at least one type.
    Tuple(TupleType),
    /// The vector type.
    Vector(VectorType),
    /// Numeric type which should be resolved to `Field`, `Group`, `Integer(_)`, or `Scalar`.
    Numeric,
    /// The `unit` type.
    Unit,
    /// Placeholder for a type that could not be resolved or was not well-formed.
    /// Will eventually lead to a compile error.
    #[default]
    Err,
}

impl Type {
    /// Are the types considered equal as far as the Leo user is concerned?
    ///
    /// In particular, any comparison involving an `Err` is `true`, and Futures which aren't explicit compare equal to
    /// other Futures.
    ///
    /// An array with an undetermined length (e.g., one that depends on a `const`) is considered equal to other arrays
    /// if their element types match. This allows const propagation to potentially resolve the length before type
    /// checking is performed again.
    ///
    /// Composite types are considered equal if their names and resolved program names match. If either side still has
    /// const generic arguments, they are treated as equal unconditionally since monomorphization and other passes of
    /// type-checking will handle mismatches later.
    ///
    /// Note: `is_record` won't be necessary when we support external structs.
    pub fn eq_user<F>(&self, other: &Type, is_record: &F) -> bool
    where
        F: Fn(&Location) -> bool,
    {
        match (self, other) {
            (Type::Err, _)
            | (_, Type::Err)
            | (Type::Address, Type::Address)
            | (Type::Boolean, Type::Boolean)
            | (Type::Field, Type::Field)
            | (Type::Group, Type::Group)
            | (Type::Scalar, Type::Scalar)
            | (Type::Signature, Type::Signature)
            | (Type::String, Type::String)
            | (Type::Unit, Type::Unit) => true,
            (Type::Array(left), Type::Array(right)) => {
                (match (left.length.as_u32(), right.length.as_u32()) {
                    (Some(l1), Some(l2)) => l1 == l2,
                    _ => {
                        // An array with an undetermined length (e.g., one that depends on a `const`) is considered
                        // equal to other arrays because their lengths _may_ eventually be proven equal.
                        true
                    }
                }) && left.element_type().eq_user(right.element_type(), is_record)
            }
            (Type::Identifier(left), Type::Identifier(right)) => left.name == right.name,
            (Type::Integer(left), Type::Integer(right)) => left == right,
            (Type::Mapping(left), Type::Mapping(right)) => {
                left.key.eq_user(&right.key, is_record) && left.value.eq_user(&right.value, is_record)
            }
            (Type::Optional(left), Type::Optional(right)) => left.inner.eq_user(&right.inner, is_record),
            (Type::Tuple(left), Type::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| left_type.eq_user(right_type, is_record)),
            (Type::Vector(left), Type::Vector(right)) => left.element_type.eq_user(&right.element_type, is_record),
            (Type::Composite(left), Type::Composite(right)) => {
                // If either composite still has const generic arguments, treat them as equal.
                // Type checking will run again after monomorphization.
                if !left.const_arguments.is_empty() || !right.const_arguments.is_empty() {
                    return true;
                }

                // Two composite types are the same if their programs and their _absolute_ paths match.
                match (&left.path.try_global_location(), &right.path.try_global_location()) {
                    (Some(l), Some(r)) => {
                        // If both a records, compare full locations.
                        // If neither are records (i.e. they are structs), only compare the paths.
                        // This will soon change when we support external structs.
                        if is_record(l) && is_record(r) {
                            l == r
                        } else if !is_record(l) && !is_record(r) {
                            l.path == r.path
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            (Type::Future(left), Type::Future(right)) if !left.is_explicit || !right.is_explicit => true,
            (Type::Future(left), Type::Future(right)) if left.inputs.len() == right.inputs.len() => left
                .inputs()
                .iter()
                .zip_eq(right.inputs().iter())
                .all(|(left_type, right_type)| left_type.eq_user(right_type, is_record)),
            _ => false,
        }
    }

    /// Returns `true` if the self `Type` is equal to the other `Type` in all aspects besides composite program of origin.
    ///
    /// In the case of futures, it also makes sure that if both are not explicit, they are equal.
    ///
    /// Flattens array syntax: `[[u8; 1]; 2] == [u8; (2, 1)] == true`
    ///
    /// Composite types are considered equal if their names match. If either side still has const generic arguments,
    /// they are treated as equal unconditionally since monomorphization and other passes of type-checking will handle
    /// mismatches later.
    pub fn eq_flat_relaxed(&self, other: &Self) -> bool {
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
                // Two arrays are equal if their element types are the same and if their lengths
                // are the same, assuming the lengths can be extracted as `u32`.
                (match (left.length.as_u32(), right.length.as_u32()) {
                    (Some(l1), Some(l2)) => l1 == l2,
                    _ => {
                        // An array with an undetermined length (e.g., one that depends on a `const`) is considered
                        // equal to other arrays because their lengths _may_ eventually be proven equal.
                        true
                    }
                }) && left.element_type().eq_flat_relaxed(right.element_type())
            }
            (Type::Identifier(left), Type::Identifier(right)) => left.matches(right),
            (Type::Integer(left), Type::Integer(right)) => left.eq(right),
            (Type::Mapping(left), Type::Mapping(right)) => {
                left.key.eq_flat_relaxed(&right.key) && left.value.eq_flat_relaxed(&right.value)
            }
            (Type::Optional(left), Type::Optional(right)) => left.inner.eq_flat_relaxed(&right.inner),
            (Type::Tuple(left), Type::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| left_type.eq_flat_relaxed(right_type)),
            (Type::Vector(left), Type::Vector(right)) => left.element_type.eq_flat_relaxed(&right.element_type),
            (Type::Composite(left), Type::Composite(right)) => {
                // If either composite still has const generic arguments, treat them as equal.
                // Type checking will run again after monomorphization.
                if !left.const_arguments.is_empty() || !right.const_arguments.is_empty() {
                    return true;
                }

                // Two composite types are the same if their _absolute_ paths match.
                // If the absolute paths are not available, then we really can't compare the two
                // types and we just return `false` to be conservative.
                match (&left.path.try_global_location(), &right.path.try_global_location()) {
                    (Some(l), Some(r)) => l.path == r.path,
                    _ => false,
                }
            }
            // Don't type check when type hasn't been explicitly defined.
            (Type::Future(left), Type::Future(right)) if !left.is_explicit || !right.is_explicit => true,
            (Type::Future(left), Type::Future(right)) if left.inputs.len() == right.inputs.len() => left
                .inputs()
                .iter()
                .zip_eq(right.inputs().iter())
                .all(|(left_type, right_type)| left_type.eq_flat_relaxed(right_type)),
            _ => false,
        }
    }

    pub fn from_snarkvm<N: Network>(t: &PlaintextType<N>, program: Symbol) -> Self {
        match t {
            Literal(lit) => (*lit).into(),
            Struct(s) => Type::Composite(CompositeType {
                path: {
                    let ident = Identifier::from(s);
                    Path::from(ident).to_global(Location::new(program, vec![ident.name]))
                },
                const_arguments: Vec::new(),
            }),
            Array(array) => Type::Array(ArrayType::from_snarkvm(array, program)),
        }
    }

    // Attempts to convert `self` to a snarkVM `PlaintextType`.
    pub fn to_snarkvm<N: Network>(&self) -> anyhow::Result<PlaintextType<N>> {
        match self {
            Type::Address => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Address)),
            Type::Boolean => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Boolean)),
            Type::Field => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Field)),
            Type::Group => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Group)),
            Type::Integer(int_type) => match int_type {
                IntegerType::U8 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::U8)),
                IntegerType::U16 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::U16)),
                IntegerType::U32 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::U32)),
                IntegerType::U64 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::U64)),
                IntegerType::U128 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::U128)),
                IntegerType::I8 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::I8)),
                IntegerType::I16 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::I16)),
                IntegerType::I32 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::I32)),
                IntegerType::I64 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::I64)),
                IntegerType::I128 => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::I128)),
            },
            Type::Scalar => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Scalar)),
            Type::Signature => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Signature)),
            Type::Array(array_type) => Ok(PlaintextType::<N>::Array(array_type.to_snarkvm()?)),
            _ => anyhow::bail!("Converting from type {self} to snarkVM type is not supported"),
        }
    }

    // A helper function to get the size in bits of the input type.
    pub fn size_in_bits<N: Network, F>(&self, is_raw: bool, get_composite: F) -> anyhow::Result<usize>
    where
        F: Fn(&snarkvm::prelude::Identifier<N>) -> anyhow::Result<snarkvm::prelude::StructType<N>>,
    {
        match is_raw {
            false => self.to_snarkvm::<N>()?.size_in_bits(&get_composite),
            true => self.to_snarkvm::<N>()?.size_in_bits_raw(&get_composite),
        }
    }

    /// Determines whether `self` can be coerced to the `expected` type.
    ///
    /// This method checks if the current type can be implicitly coerced to the expected type
    /// according to specific rules:
    /// - `Optional<T>` can be coerced to `Optional<T>`.
    /// - `T` can be coerced to `Optional<T>`.
    /// - Arrays `[T; N]` can be coerced to `[Optional<T>; N]` if lengths match or are unknown,
    ///   and element types are coercible.
    /// - Falls back to an equality check for other types.
    ///
    /// # Arguments
    /// * `expected` - The type to which `self` is being coerced.
    /// * `is_record` - Determines whether a given `Location` refers to a record or not.
    ///
    /// # Returns
    /// `true` if coercion is allowed; `false` otherwise.
    ///
    /// # Note: `is_record` won't be necessary when we support external structs.
    pub fn can_coerce_to<F>(&self, expected: &Type, is_record: &F) -> bool
    where
        F: Fn(&Location) -> bool,
    {
        use Type::*;

        match (self, expected) {
            // Allow Optional<T> → Optional<T>
            (Optional(actual_opt), Optional(expected_opt)) => {
                actual_opt.inner.can_coerce_to(&expected_opt.inner, is_record)
            }

            // Allow T → Optional<T>
            (a, Optional(opt)) => a.can_coerce_to(&opt.inner, is_record),

            // Allow [T; N] → [Optional<T>; N]
            (Array(a_arr), Array(e_arr)) => {
                let lengths_equal = match (a_arr.length.as_u32(), e_arr.length.as_u32()) {
                    (Some(l1), Some(l2)) => l1 == l2,
                    _ => true,
                };

                lengths_equal && a_arr.element_type().can_coerce_to(e_arr.element_type(), is_record)
            }

            // Fallback: check for exact match
            _ => self.eq_user(expected, &is_record),
        }
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(_))
    }

    pub fn is_mapping(&self) -> bool {
        matches!(self, Self::Mapping(_))
    }

    pub fn to_optional(&self) -> Type {
        Type::Optional(OptionalType { inner: Box::new(self.clone()) })
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Type::Unit => true,
            Type::Array(array_type) => {
                if let Some(length) = array_type.length.as_u32() {
                    length == 0
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl From<LiteralType> for Type {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Address => Type::Address,
            LiteralType::Boolean => Type::Boolean,
            LiteralType::Field => Type::Field,
            LiteralType::Group => Type::Group,
            LiteralType::U8 => Type::Integer(IntegerType::U8),
            LiteralType::U16 => Type::Integer(IntegerType::U16),
            LiteralType::U32 => Type::Integer(IntegerType::U32),
            LiteralType::U64 => Type::Integer(IntegerType::U64),
            LiteralType::U128 => Type::Integer(IntegerType::U128),
            LiteralType::I8 => Type::Integer(IntegerType::I8),
            LiteralType::I16 => Type::Integer(IntegerType::I16),
            LiteralType::I32 => Type::Integer(IntegerType::I32),
            LiteralType::I64 => Type::Integer(IntegerType::I64),
            LiteralType::I128 => Type::Integer(IntegerType::I128),
            LiteralType::Scalar => Type::Scalar,
            LiteralType::Signature => Type::Signature,
            LiteralType::String => Type::String,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Address => write!(f, "address"),
            Type::Array(ref array_type) => write!(f, "{array_type}"),
            Type::Boolean => write!(f, "bool"),
            Type::Field => write!(f, "field"),
            Type::Future(ref future_type) => write!(f, "{future_type}"),
            Type::Group => write!(f, "group"),
            Type::Identifier(ref variable) => write!(f, "{variable}"),
            Type::Integer(ref integer_type) => write!(f, "{integer_type}"),
            Type::Mapping(ref mapping_type) => write!(f, "{mapping_type}"),
            Type::Optional(ref optional_type) => write!(f, "{optional_type}"),
            Type::Scalar => write!(f, "scalar"),
            Type::Signature => write!(f, "signature"),
            Type::String => write!(f, "string"),
            Type::Composite(ref composite_type) => write!(f, "{composite_type}"),
            Type::Tuple(ref tuple) => write!(f, "{tuple}"),
            Type::Vector(ref vector_type) => write!(f, "{vector_type}"),
            Type::Numeric => write!(f, "numeric"),
            Type::Unit => write!(f, "()"),
            Type::Err => write!(f, "error"),
        }
    }
}
