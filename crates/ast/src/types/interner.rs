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

//! Per-session arena assigning each canonical [`TypeKind`] a stable [`Type`] handle. Ground
//! variants are pre-interned at fixed ids ([`Type::ADDRESS`] etc.); id 0 is [`Type::ERR`], so
//! `Type::default()` is the error sentinel.

use crate::{Canonicalize, IntegerType, TypeKind};

use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use std::cell::RefCell;

/// Canonical type identity. `Copy`, id-equality. Ground variants are pre-interned at fixed
/// positions so `Type::ADDRESS`, `Type::U32`, etc. are `const`. For the source-shaped view,
/// see [`crate::TypeNode`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Type(u32);

impl Type {
    pub const ADDRESS: Type = Type(1);
    pub const BOOLEAN: Type = Type(2);
    pub const DYN_RECORD: Type = Type(11);
    // Every `TypeInterner::new()` pre-fills these in this exact order.
    pub const ERR: Type = Type(0);
    pub const FIELD: Type = Type(3);
    pub const GROUP: Type = Type(4);
    pub const I128: Type = Type(21);
    pub const I16: Type = Type(18);
    pub const I32: Type = Type(19);
    pub const I64: Type = Type(20);
    pub const I8: Type = Type(17);
    pub const IDENTIFIER: Type = Type(10);
    pub const NUMERIC: Type = Type(9);
    pub const NUM_PREINTERNED: u32 = 22;
    pub const SCALAR: Type = Type(5);
    pub const SIGNATURE: Type = Type(6);
    pub const STRING: Type = Type(7);
    pub const U128: Type = Type(16);
    pub const U16: Type = Type(13);
    pub const U32: Type = Type(14);
    pub const U64: Type = Type(15);
    pub const U8: Type = Type(12);
    pub const UNIT: Type = Type(8);

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// `IndexSet` of canonical [`TypeKind`] keys; the insertion index is the [`Type`].
pub struct TypeInterner {
    storage: RefCell<IndexSet<TypeKind, FxBuildHasher>>,
}

impl TypeInterner {
    /// The pre-intern order MUST match the `Type::*` consts; `assert_preintern_layout` guards it.
    pub fn new() -> Self {
        let preintern = [
            TypeKind::Err,
            TypeKind::Address,
            TypeKind::Boolean,
            TypeKind::Field,
            TypeKind::Group,
            TypeKind::Scalar,
            TypeKind::Signature,
            TypeKind::String,
            TypeKind::Unit,
            TypeKind::Numeric,
            TypeKind::Identifier,
            TypeKind::DynRecord,
            TypeKind::Integer(IntegerType::U8),
            TypeKind::Integer(IntegerType::U16),
            TypeKind::Integer(IntegerType::U32),
            TypeKind::Integer(IntegerType::U64),
            TypeKind::Integer(IntegerType::U128),
            TypeKind::Integer(IntegerType::I8),
            TypeKind::Integer(IntegerType::I16),
            TypeKind::Integer(IntegerType::I32),
            TypeKind::Integer(IntegerType::I64),
            TypeKind::Integer(IntegerType::I128),
        ];
        let mut storage: IndexSet<TypeKind, FxBuildHasher> = IndexSet::with_hasher(FxBuildHasher::default());
        for d in preintern {
            storage.insert(d);
        }
        assert_eq!(storage.len(), Type::NUM_PREINTERNED as usize);
        Self { storage: RefCell::new(storage) }
    }

    /// `&self`, not `&mut`, so callers can intern mid-expression without restructuring borrows.
    pub fn intern(&self, data: &TypeKind) -> Type {
        let canonical = data.clone().canonicalize();
        let mut s = self.storage.borrow_mut();
        if let Some(idx) = s.get_index_of(&canonical) {
            return Type(idx as u32);
        }
        let (idx, _) = s.insert_full(canonical);
        Type(idx as u32)
    }

    /// Resolve a canonical [`Type`] handle back to a cloned [`TypeKind`]. Panics if `ty`
    /// is not a valid handle for this interner (shouldn't happen in practice — see
    /// [`Type::default`] which uses the pre-interned `Type::ERR`).
    pub fn resolve(&self, ty: Type) -> TypeKind {
        self.storage.borrow().get_index(ty.index()).expect("Invalid Type handle").clone()
    }
}

impl Default for TypeInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_is_pre_interned_at_id_zero() {
        let i = TypeInterner::new();
        assert_eq!(i.intern(&TypeKind::Err), Type::ERR);
        assert_eq!(Type::ERR.index(), 0);
    }

    /// If this fails, the pre-intern order in `TypeInterner::new` and the `Type::*` consts
    /// have fallen out of sync.
    #[test]
    fn assert_preintern_layout() {
        let i = TypeInterner::new();
        assert_eq!(i.intern(&TypeKind::Err), Type::ERR);
        assert_eq!(i.intern(&TypeKind::Address), Type::ADDRESS);
        assert_eq!(i.intern(&TypeKind::Boolean), Type::BOOLEAN);
        assert_eq!(i.intern(&TypeKind::Field), Type::FIELD);
        assert_eq!(i.intern(&TypeKind::Group), Type::GROUP);
        assert_eq!(i.intern(&TypeKind::Scalar), Type::SCALAR);
        assert_eq!(i.intern(&TypeKind::Signature), Type::SIGNATURE);
        assert_eq!(i.intern(&TypeKind::String), Type::STRING);
        assert_eq!(i.intern(&TypeKind::Unit), Type::UNIT);
        assert_eq!(i.intern(&TypeKind::Numeric), Type::NUMERIC);
        assert_eq!(i.intern(&TypeKind::Identifier), Type::IDENTIFIER);
        assert_eq!(i.intern(&TypeKind::DynRecord), Type::DYN_RECORD);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::U8)), Type::U8);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::U16)), Type::U16);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::U32)), Type::U32);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::U64)), Type::U64);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::U128)), Type::U128);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::I8)), Type::I8);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::I16)), Type::I16);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::I32)), Type::I32);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::I64)), Type::I64);
        assert_eq!(i.intern(&TypeKind::Integer(IntegerType::I128)), Type::I128);
        assert_eq!(i.storage.borrow().len() as u32, Type::NUM_PREINTERNED);
    }

    #[test]
    fn intern_is_idempotent() {
        let i = TypeInterner::new();
        let a = i.intern(&TypeKind::Field);
        let b = i.intern(&TypeKind::Field);
        assert_eq!(a, b);
    }

    #[test]
    fn distinct_data_gets_distinct_ids() {
        let i = TypeInterner::new();
        let a = i.intern(&TypeKind::Field);
        let b = i.intern(&TypeKind::Boolean);
        assert_ne!(a, b);
    }

    #[test]
    fn default_id_is_err() {
        assert_eq!(Type::default(), Type::ERR);
    }
}
