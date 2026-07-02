// Copyright (C) 2019-2026 Provable Inc.
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
    ProgramId,
    TupleType,
    Type,
    TypeInterner,
    VectorType,
};

use itertools::Itertools;
use leo_span::Span;
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{
    LiteralType,
    Network,
    PlaintextType,
    PlaintextType::{Array, ExternalStruct, Literal, Struct},
};
use std::fmt;

/// AST-level type annotation: the source-shaped `kind` with its cached canonical [`Type`]
/// handle. `kind` and `type_` are private to preserve the invariant
/// `type_ == interner.intern(&kind)`, which only [`TypeNode::new`] can establish.
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct TypeNode {
    kind: TypeKind,
    pub span: Span,
    #[serde(default, skip)]
    type_: Type,
}

impl TypeNode {
    pub fn new(interner: &TypeInterner, kind: TypeKind, span: Span) -> Self {
        let type_ = interner.intern(&kind);
        Self { kind, span, type_ }
    }

    /// Escape hatch that skips interning. `type_` is `Type::ERR`; the value must reach an
    /// interner before it can be compared or hashed. `PartialEq` / `Hash` `assert!` this.
    pub fn unchecked(kind: TypeKind, span: Span) -> Self {
        Self { kind, span, type_: Type::default() }
    }

    pub fn kind(&self) -> &TypeKind {
        &self.kind
    }

    pub fn ty(&self) -> Type {
        self.type_
    }

    pub fn into_parts(self) -> (TypeKind, Span, Type) {
        (self.kind, self.span, self.type_)
    }

    /// Bypass the interner. The caller is responsible for `type_ == interner.intern(&kind)`;
    /// only safe when `kind` is being canonicalized (span/id scrub) without shape change.
    pub fn from_parts(kind: TypeKind, span: Span, type_: Type) -> Self {
        Self { kind, span, type_ }
    }

    /// Fast path via the canonical handle; falls back to structural [`TypeKind::types_equivalent`]
    /// for equivalences that don't imply identity — implicit `Future`, undetermined array length,
    /// pending const-generic arguments.
    pub fn types_equivalent(&self, other: &TypeNode) -> bool {
        if self.type_ != Type::ERR && self.type_ == other.type_ {
            return true;
        }
        self.kind.types_equivalent(&other.kind)
    }
}

impl Default for TypeNode {
    fn default() -> Self {
        Self::unchecked(TypeKind::Err, Span::default())
    }
}

impl PartialEq for TypeNode {
    fn eq(&self, other: &Self) -> bool {
        assert!(
            self.kind == TypeKind::Err || self.type_ != Type::ERR,
            "TypeNode with non-Err kind must have been interned before equality use",
        );
        assert!(
            other.kind == TypeKind::Err || other.type_ != Type::ERR,
            "TypeNode with non-Err kind must have been interned before equality use",
        );
        self.type_ == other.type_
    }
}

impl std::hash::Hash for TypeNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        assert!(
            self.kind == TypeKind::Err || self.type_ != Type::ERR,
            "TypeNode with non-Err kind must have been interned before hashing",
        );
        self.type_.hash(state);
    }
}

impl fmt::Display for TypeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
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
    /// The `identifier` type.
    Identifier,
    /// The `dyn record` type.
    DynRecord,
    /// A reference to a built in type.
    Ident(Identifier),
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

impl TypeKind {
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
    pub fn types_equivalent(&self, other: &TypeKind) -> bool {
        match (self, other) {
            (TypeKind::Err, _)
            | (_, TypeKind::Err)
            | (TypeKind::Address, TypeKind::Address)
            | (TypeKind::Boolean, TypeKind::Boolean)
            | (TypeKind::Field, TypeKind::Field)
            | (TypeKind::Group, TypeKind::Group)
            | (TypeKind::Scalar, TypeKind::Scalar)
            | (TypeKind::Signature, TypeKind::Signature)
            | (TypeKind::String, TypeKind::String)
            | (TypeKind::Identifier, TypeKind::Identifier)
            | (TypeKind::DynRecord, TypeKind::DynRecord)
            | (TypeKind::Unit, TypeKind::Unit) => true,
            (TypeKind::Array(left), TypeKind::Array(right)) => {
                (match (left.length.as_u32(), right.length.as_u32()) {
                    (Some(l1), Some(l2)) => l1 == l2,
                    _ => {
                        // An array with an undetermined length (e.g., one that depends on a `const`) is considered
                        // equal to other arrays because their lengths _may_ eventually be proven equal.
                        true
                    }
                }) && left.element_type().types_equivalent(right.element_type())
            }
            (TypeKind::Ident(left), TypeKind::Ident(right)) => left.name == right.name,
            (TypeKind::Integer(left), TypeKind::Integer(right)) => left == right,
            (TypeKind::Mapping(left), TypeKind::Mapping(right)) => {
                left.key.types_equivalent(&right.key) && left.value.types_equivalent(&right.value)
            }
            (TypeKind::Optional(left), TypeKind::Optional(right)) => left.inner.types_equivalent(&right.inner),
            (TypeKind::Tuple(left), TypeKind::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| left_type.types_equivalent(right_type)),
            (TypeKind::Vector(left), TypeKind::Vector(right)) => {
                left.element_type.types_equivalent(&right.element_type)
            }
            (TypeKind::Composite(left), TypeKind::Composite(right)) => {
                // If either composite still has const generic arguments, treat them as equal;
                // monomorphization and a subsequent type-checking pass will handle mismatches.
                if !left.const_arguments.is_empty() || !right.const_arguments.is_empty() {
                    return true;
                }

                // Two composite types are the same if their global locations match.
                match (&left.path.try_global_location(), &right.path.try_global_location()) {
                    (Some(l), Some(r)) => l == r,
                    _ => false,
                }
            }

            (TypeKind::Future(left), TypeKind::Future(right)) if !left.is_explicit || !right.is_explicit => true,
            (TypeKind::Future(left), TypeKind::Future(right)) if left.inputs.len() == right.inputs.len() => left
                .inputs()
                .iter()
                .zip_eq(right.inputs().iter())
                .all(|(left_type, right_type)| left_type.types_equivalent(right_type)),
            _ => false,
        }
    }

    pub fn from_snarkvm<N: Network>(t: &PlaintextType<N>, program_id: ProgramId) -> Self {
        match t {
            Literal(lit) => (*lit).into(),
            Struct(s) => TypeKind::Composite(CompositeType {
                path: {
                    let ident = Identifier::from(s);
                    Path::from(ident).to_global(Location::new(program_id.as_symbol(), vec![ident.name]))
                },
                const_arguments: Vec::new(),
            }),
            ExternalStruct(l) => TypeKind::Composite(CompositeType {
                path: {
                    let external_program = ProgramId::from(l.program_id());
                    let name = Identifier::from(l.resource());
                    Path::from(name)
                        .with_user_program(external_program)
                        .to_global(Location::new(external_program.as_symbol(), vec![name.name]))
                },
                const_arguments: Vec::new(),
            }),
            Array(array) => TypeKind::Array(ArrayType::from_snarkvm(array, program_id)),
        }
    }

    // Attempts to convert `self` to a snarkVM `PlaintextType`.
    pub fn to_snarkvm<N: Network>(&self) -> anyhow::Result<PlaintextType<N>> {
        match self {
            TypeKind::Address => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Address)),
            TypeKind::Boolean => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Boolean)),
            TypeKind::Field => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Field)),
            TypeKind::Group => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Group)),
            TypeKind::Integer(int_type) => match int_type {
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
            TypeKind::Scalar => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Scalar)),
            TypeKind::Signature => Ok(PlaintextType::Literal(snarkvm::prelude::LiteralType::Signature)),
            TypeKind::Array(array_type) => Ok(PlaintextType::<N>::Array(array_type.to_snarkvm()?)),
            _ => anyhow::bail!("Converting from type {self} to snarkVM type is not supported"),
        }
    }

    // A helper function to get the size in bits of the input type.
    pub fn size_in_bits<N: Network, F0, F1>(
        &self,
        is_raw: bool,
        get_structs: F0,
        get_external_structs: F1,
    ) -> anyhow::Result<usize>
    where
        F0: Fn(&snarkvm::prelude::Identifier<N>) -> anyhow::Result<snarkvm::prelude::StructType<N>>,
        F1: Fn(&snarkvm::prelude::Locator<N>) -> anyhow::Result<snarkvm::prelude::StructType<N>>,
    {
        match is_raw {
            false => self.to_snarkvm::<N>()?.size_in_bits(&get_structs, &get_external_structs),
            true => self.to_snarkvm::<N>()?.size_in_bits_raw(&get_structs, &get_external_structs),
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
    pub fn can_coerce_to(&self, expected: &TypeKind) -> bool {
        use TypeKind::*;

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
            _ => self.types_equivalent(expected),
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

    pub fn to_optional(&self) -> TypeKind {
        TypeKind::Optional(OptionalType { inner: Box::new(self.clone()) })
    }

    pub fn is_empty(&self) -> bool {
        match self {
            TypeKind::Unit => true,
            TypeKind::Array(array_type) => {
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

impl From<LiteralType> for TypeKind {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Identifier => TypeKind::Identifier,
            LiteralType::Address => TypeKind::Address,
            LiteralType::Boolean => TypeKind::Boolean,
            LiteralType::Field => TypeKind::Field,
            LiteralType::Group => TypeKind::Group,
            LiteralType::U8 => TypeKind::Integer(IntegerType::U8),
            LiteralType::U16 => TypeKind::Integer(IntegerType::U16),
            LiteralType::U32 => TypeKind::Integer(IntegerType::U32),
            LiteralType::U64 => TypeKind::Integer(IntegerType::U64),
            LiteralType::U128 => TypeKind::Integer(IntegerType::U128),
            LiteralType::I8 => TypeKind::Integer(IntegerType::I8),
            LiteralType::I16 => TypeKind::Integer(IntegerType::I16),
            LiteralType::I32 => TypeKind::Integer(IntegerType::I32),
            LiteralType::I64 => TypeKind::Integer(IntegerType::I64),
            LiteralType::I128 => TypeKind::Integer(IntegerType::I128),
            LiteralType::Scalar => TypeKind::Scalar,
            LiteralType::Signature => TypeKind::Signature,
            LiteralType::String => TypeKind::String,
        }
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeKind::Address => write!(f, "address"),
            TypeKind::Identifier => write!(f, "identifier"),
            TypeKind::DynRecord => write!(f, "dyn record"),
            TypeKind::Array(ref array_type) => write!(f, "{array_type}"),
            TypeKind::Boolean => write!(f, "bool"),
            TypeKind::Field => write!(f, "field"),
            TypeKind::Future(ref future_type) => write!(f, "{future_type}"),
            TypeKind::Group => write!(f, "group"),
            TypeKind::Ident(ref variable) => write!(f, "{variable}"),
            TypeKind::Integer(ref integer_type) => write!(f, "{integer_type}"),
            TypeKind::Mapping(ref mapping_type) => write!(f, "{mapping_type}"),
            TypeKind::Optional(ref optional_type) => write!(f, "{optional_type}"),
            TypeKind::Scalar => write!(f, "scalar"),
            TypeKind::Signature => write!(f, "signature"),
            TypeKind::String => write!(f, "string"),
            TypeKind::Composite(ref composite_type) => write!(f, "{composite_type}"),
            TypeKind::Tuple(ref tuple) => write!(f, "{tuple}"),
            TypeKind::Vector(ref vector_type) => write!(f, "{vector_type}"),
            TypeKind::Numeric => write!(f, "numeric"),
            TypeKind::Unit => write!(f, "()"),
            TypeKind::Err => write!(f, "error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "TypeNode with non-Err kind must have been interned")]
    fn eq_panics_on_unchecked_non_err_kind() {
        let interner = TypeInterner::new();
        let unchecked = TypeNode::unchecked(TypeKind::Address, Span::default());
        let interned = TypeNode::new(&interner, TypeKind::Boolean, Span::default());
        let _ = unchecked == interned;
    }

    #[test]
    #[should_panic(expected = "TypeNode with non-Err kind must have been interned")]
    fn hash_panics_on_unchecked_non_err_kind() {
        use std::hash::Hash;

        let unchecked = TypeNode::unchecked(TypeKind::Address, Span::default());
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        unchecked.hash(&mut hasher);
    }

    #[test]
    fn eq_accepts_defaulted_err_nodes() {
        // Default (`Err`, `Type::ERR`) values must remain comparable — the invariant
        // is scoped to *non-Err* kinds.
        let a = TypeNode::default();
        let b = TypeNode::default();
        assert_eq!(a, b);
    }
}
