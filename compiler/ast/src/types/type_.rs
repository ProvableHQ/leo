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
    MappingType,
    OptionalType,
    Path,
    TupleType,
};

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
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn eq_user(&self, other: &Type) -> bool {
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
                }) && left.element_type().eq_user(right.element_type())
            }
            (Type::Identifier(left), Type::Identifier(right)) => left.name == right.name,
            (Type::Integer(left), Type::Integer(right)) => left == right,
            (Type::Mapping(left), Type::Mapping(right)) => {
                left.key.eq_user(&right.key) && left.value.eq_user(&right.value)
            }
            (Type::Optional(left), Type::Optional(right)) => left.inner.eq_user(&right.inner),
            (Type::Tuple(left), Type::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| left_type.eq_user(right_type)),
            (Type::Composite(left), Type::Composite(right)) => {
                // If either composite still has const generic arguments, treat them as equal.
                // Type checking will run again after monomorphization.
                if !left.const_arguments.is_empty() || !right.const_arguments.is_empty() {
                    return true;
                }

                // Two composite types are the same if their programs and their _absolute_ paths match.
                (left.program == right.program)
                    && match (&left.path.try_absolute_path(), &right.path.try_absolute_path()) {
                        (Some(l), Some(r)) => l == r,
                        _ => false,
                    }
            }
            (Type::Future(left), Type::Future(right)) if !left.is_explicit || !right.is_explicit => true,
            (Type::Future(left), Type::Future(right)) if left.inputs.len() == right.inputs.len() => left
                .inputs()
                .iter()
                .zip_eq(right.inputs().iter())
                .all(|(left_type, right_type)| left_type.eq_user(right_type)),
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
            (Type::Composite(left), Type::Composite(right)) => {
                // If either composite still has const generic arguments, treat them as equal.
                // Type checking will run again after monomorphization.
                if !left.const_arguments.is_empty() || !right.const_arguments.is_empty() {
                    return true;
                }

                // Two composite types are the same if their _absolute_ paths match.
                // If the absolute paths are not available, then we really can't compare the two
                // types and we just return `false` to be conservative.
                match (&left.path.try_absolute_path(), &right.path.try_absolute_path()) {
                    (Some(l), Some(r)) => l == r,
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
            Struct(s) => Type::Composite(CompositeType {
                path: {
                    let ident = Identifier::from(s);
                    Path::from(ident).with_absolute_path(Some(vec![ident.name]))
                },
                const_arguments: Vec::new(),
                program,
            }),
            Array(array) => Type::Array(ArrayType::from_snarkvm(array, program)),
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
    ///
    /// # Returns
    /// `true` if coercion is allowed; `false` otherwise.
    pub fn can_coerce_to(&self, expected: &Type) -> bool {
        use Type::*;

        match (self, expected) {
            // Allow Optional<T> → Optional<T>
            (Optional(actual_opt), Optional(expected_opt)) => actual_opt.inner.can_coerce_to(&expected_opt.inner),

            // Allow T → Optional<T>
            (a, Optional(opt)) => a.can_coerce_to(&opt.inner),

            // Allow [T; N] → [Optional<T>; N]
            (Array(a_arr), Array(e_arr)) => {
                let lengths_equal = match (a_arr.length.as_u32(), e_arr.length.as_u32()) {
                    (Some(l1), Some(l2)) => l1 == l2,
                    _ => true,
                };

                lengths_equal && a_arr.element_type().can_coerce_to(e_arr.element_type())
            }

            // Fallback: check for exact match
            _ => self.eq_user(expected),
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
            Type::Composite(ref struct_type) => write!(f, "{struct_type}"),
            Type::Tuple(ref tuple) => write!(f, "{tuple}"),
            Type::Numeric => write!(f, "numeric"),
            Type::Unit => write!(f, "()"),
            Type::Err => write!(f, "error"),
        }
    }
}
