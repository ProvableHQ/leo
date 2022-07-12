// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{GroupLiteral, Identifier, Literal, Type};

use leo_errors::{type_name, FlattenError, LeoError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

// TODO: Consider refactoring this module to use the console implementations from snarkVM.

// This is temporary since the currently unused code is used in constant folding.
#[allow(dead_code)]

// Macro for making implementing unary operations over appropriate types easier.
macro_rules! implement_const_unary {
    (
        @overflowing
        name: $name:ident,
        method: $method:ident,
        string: $str:expr,
        patterns: [$([$type:ident, $m_type:ty]),+]
    ) => {
        implement_const_unary!{
            name: $name,
            patterns: [$([
                t: $type,
                l: |l: $m_type| l.$method().ok_or_else(|| FlattenError::unary_overflow(l, $str, Default::default()))
            ]),+]
        }
    };

    (
        @non-overflowing
        name: $name:ident,
        method: $method:ident,
        patterns: [$([$type:ident, $m_type:ty]),+]
    ) => {
        implement_const_unary!{
            name: $name,
            patterns: [$([
                t: $type,
                l: |l: $m_type| -> Result<$m_type> { Ok(l.$method()) }
            ]),+]
        }
    };

    (
        name: $name:ident,
        patterns: [$([
            t: $type:ident,
            l: $logic:expr
        ]),+]
    ) => {
        pub fn $name(self) -> Result<Self> {
            use Value::*;

            match self {
                $(
                    $type(v) => {
                        Ok($type($logic(v.into())?))
                    },
                )+
                // Unreachable because type checking should have already caught this and errored out.
                s => unreachable!("Const operation not supported {}.{}()", type_name(&s), stringify!($name))
            }
        }
    };
}

// Macro for making implementing binary operations over appropriate types easier.
macro_rules! implement_const_binary {
    // for overflowing operations that can overflow
    (
        @overflowing
        name: $name:ident,
        method: $method:ident,
        string: $str:expr,
        patterns: [$(
            // lhs, rhs, out, method left, method right
            [$lhs:ident, [$($rhs:ident),+], $out:ident, $m_lhs:ty, $m_rhs:ty]
        ),+]
    ) => {
        implement_const_binary!{
            name: $name,
            patterns: [$([
                types: $lhs, [$($rhs),+], $out,
                logic: |l: $m_lhs, r: $m_rhs, t| l.$method(r).ok_or_else(|| FlattenError::binary_overflow(l, $str, r, t, Default::default()))
            ]),+]
        }
    };

    // for wrapping math operations
    (
        @non-overflowing
        name: $name:ident,
        method: $method:ident,
        patterns: [$(
            // lhs, rhs, out, method left, method right, method output
            [$lhs:ident, [$($rhs:ident),+], $out:ident, $m_lhs:ty, $m_rhs:ty]
        ),+]
    ) => {
        implement_const_binary!{
            name: $name,
            patterns: [$([
                types: $lhs, [$($rhs),+], $out,
                logic: |l: $m_lhs, r: $m_rhs, _| -> Result<$m_lhs> {Ok(l.$method(r))}
            ]),+]
        }
    };

    // for cmp operations
    (
        @cmp
        name: $name:ident,
        method: $method:ident,
        string: $str:expr,
        patterns: [$(
            // lhs, rhs, out, method left, method right, method output
            [$lhs:ident, [$($rhs:ident),+], $out:ident, $m_lhs:ty, $m_rhs:ty]
        ),+]
    ) => {
        implement_const_binary!{
            name: $name,
            patterns: [$([
                types: $lhs, [$($rhs),+], $out,
                logic: |l: $m_lhs, r: $m_rhs, _| -> Result<bool> {Ok(l.$method(&r))}
            ]),+]
        }
    };

    (
        name: $name:ident,
        patterns: [$([
            types: $lhs:ident, [$($rhs:ident),+], $out:ident,
            logic: $logic:expr
        ]),+]
    ) => {
        pub fn $name(self, other: Self) -> Result<Self> {
            use Value::*;

            match (self, other) {
                $(
                    $(
                        ($lhs(types), $rhs(rhs)) => {
                            let rhs_type = type_name(&rhs);
                            let out = $logic(types, rhs.into(), rhs_type)?;
                            Ok($out(out))
                        },
                    )+
                )+
                // Unreachable because type checking should have already caught this and errored out.
                (s, o) => unreachable!("Const operation not supported {}.{}({})", type_name(&s), stringify!($name), type_name(&o))
            }
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Input(Type, Identifier),
    Address(String),
    Boolean(bool),
    Circuit(Identifier, IndexMap<Symbol, Value>),
    Field(String),
    Group(Box<GroupLiteral>),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Scalar(String),
    String(String),
}

impl Value {
    pub fn is_supported_const_fold_type(&self) -> bool {
        use Value::*;
        matches!(
            self,
            Boolean(..)
                | I8(..)
                | I16(..)
                | I32(..)
                | I64(..)
                | I128(..)
                | U8(..)
                | U16(..)
                | U32(..)
                | U64(..)
                | U128(..)
        )
    }

    implement_const_unary!(
        @overflowing
        name: abs,
        method: checked_abs,
        string: "abs",
        patterns: [
            [I8, i8],
            [I16, i16],
            [I32, i32],
            [I64, i64],
            [I128, i128]
        ]
    );

    implement_const_unary!(
        @non-overflowing
        name: abs_wrapped,
        method: wrapping_abs,
        patterns: [
            [I8, i8],
            [I16, i16],
            [I32, i32],
            [I64, i64],
            [I128, i128]
        ]
    );

    implement_const_unary!(
        @overflowing
        name: neg,
        method: checked_neg,
        string: "neg",
        patterns: [
            // [Field, Field],
            // [Group, Group],
            [I8, i8],
            [I16, i16],
            [I32, i32],
            [I64, i64],
            [I128, i128]
        ]
    );

    implement_const_unary!(
        @non-overflowing
        name: not,
        method: not,
        patterns: [
            [Boolean, bool],
            [I8, i8],
            [I16, i16],
            [I32, i32],
            [I64, i64],
            [I128, i128],
            [U8, u8],
            [U16, u16],
            [U32, u32],
            [U64, u64],
            [U128, u128]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: add,
        method: checked_add,
        string: "+",
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],

        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: add_wrapped,
        method: wrapping_add,
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],

        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: bitand,
        method: bitand,
        patterns: [
            [Boolean, [Boolean], Boolean, bool, bool],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: div,
        method: checked_div,
        string: "/",
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: div_wrapped,
        method: wrapping_div,
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],
        ]
    );

    implement_const_binary!(
        @cmp
        name: eq,
        method: eq,
        string: "==",
        patterns: [
            [Boolean, [Boolean], Boolean, bool, bool],
            [I8, [I8], Boolean, i8, i8],
            [I16, [I16], Boolean, i16, i16],
            [I32, [I32], Boolean, i32, i32],
            [I64, [I64], Boolean, i64, i64],
            [I128, [I128], Boolean, i128, i128],
            [U8, [U8], Boolean, u8, u8],
            [U16, [U16], Boolean, u16, u16],
            [U32, [U32], Boolean, u32, u32],
            [U64, [U64], Boolean, u64, u64],
            [U128, [U128], Boolean, u128, u128]
        ]
    );

    implement_const_binary!(
        @cmp
        name: ge,
        method: ge,
        string: ">=",
        patterns: [
            [I8, [I8], Boolean, i8, i8],
            [I16, [I16], Boolean, i16, i16],
            [I32, [I32], Boolean, i32, i32],
            [I64, [I64], Boolean, i64, i64],
            [I128, [I128], Boolean, i128, i128],
            [U8, [U8], Boolean, u8, u8],
            [U16, [U16], Boolean, u16, u16],
            [U32, [U32], Boolean, u32, u32],
            [U64, [U64], Boolean, u64, u64],
            [U128, [U128], Boolean, u128, u128]
        ]
    );

    implement_const_binary!(
        @cmp
        name: gt,
        method: gt,
        string: ">",
        patterns: [
            [I8, [I8], Boolean, i8, i8],
            [I16, [I16], Boolean, i16, i16],
            [I32, [I32], Boolean, i32, i32],
            [I64, [I64], Boolean, i64, i64],
            [I128, [I128], Boolean, i128, i128],
            [U8, [U8], Boolean, u8, u8],
            [U16, [U16], Boolean, u16, u16],
            [U32, [U32], Boolean, u32, u32],
            [U64, [U64], Boolean, u64, u64],
            [U128, [U128], Boolean, u128, u128]
        ]
    );

    implement_const_binary!(
        @cmp
        name: le,
        method: le,
        string: "<=",
        patterns: [
            [I8, [I8], Boolean, i8, i8],
            [I16, [I16], Boolean, i16, i16],
            [I32, [I32], Boolean, i32, i32],
            [I64, [I64], Boolean, i64, i64],
            [I128, [I128], Boolean, i128, i128],
            [U8, [U8], Boolean, u8, u8],
            [U16, [U16], Boolean, u16, u16],
            [U32, [U32], Boolean, u32, u32],
            [U64, [U64], Boolean, u64, u64],
            [U128, [U128], Boolean, u128, u128]
        ]
    );

    implement_const_binary!(
        @cmp
        name: lt,
        method: lt,
        string: "<",
        patterns: [
            [I8, [I8], Boolean, i8, i8],
            [I16, [I16], Boolean, i16, i16],
            [I32, [I32], Boolean, i32, i32],
            [I64, [I64], Boolean, i64, i64],
            [I128, [I128], Boolean, i128, i128],
            [U8, [U8], Boolean, u8, u8],
            [U16, [U16], Boolean, u16, u16],
            [U32, [U32], Boolean, u32, u32],
            [U64, [U64], Boolean, u64, u64],
            [U128, [U128], Boolean, u128, u128]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: mul,
        method: checked_mul,
        string: "*",
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: mul_wrapped,
        method: wrapping_mul,
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: bitor,
        method: bitor,
        patterns: [
            [Boolean, [Boolean], Boolean, bool, bool],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: pow,
        method: checked_pow,
        string: "**",
        patterns: [
            [I8, [U8, U16, U32], I8, i8, u32],
            [I16, [U8, U16, U32], I16, i16, u32],
            [I32, [U8, U16, U32], I32, i32, u32],
            [I64, [U8, U16, U32], I64, i64, u32],
            [I128, [U8, U16, U32], I128, i128, u32],
            [U8, [U8, U16, U32], U8, u8, u32],
            [U16, [U8, U16, U32], U16, u16, u32],
            [U32, [U8, U16, U32], U32, u32, u32],
            [U64, [U8, U16, U32], U64, u64, u32],
            [U128, [U8, U16, U32], U128, u128, u32]
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: pow_wrapped,
        method: wrapping_pow,
        patterns: [
            [I8, [U8, U16, U32], I8, i8, u32],
            [I16, [U8, U16, U32], I16, i16, u32],
            [I32, [U8, U16, U32], I32, i32, u32],
            [I64, [U8, U16, U32], I64, i64, u32],
            [I128, [U8, U16, U32], I128, i128, u32],
            [U8, [U8, U16, U32], U8, u8, u32],
            [U16, [U8, U16, U32], U16, u16, u32],
            [U32, [U8, U16, U32], U32, u32, u32],
            [U64, [U8, U16, U32], U64, u64, u32],
            [U128, [U8, U16, U32], U128, u128, u32]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: shl,
        method: checked_shl,
        string: "<<",
        patterns: [
            [I8, [U8, U16, U32], I8, i8, u32],
            [I16, [U8, U16, U32], I16, i16, u32],
            [I32, [U8, U16, U32], I32, i32, u32],
            [I64, [U8, U16, U32], I64, i64, u32],
            [I128, [U8, U16, U32], I128, i128, u32],
            [U8, [U8, U16, U32], U8, u8, u32],
            [U16, [U8, U16, U32], U16, u16, u32],
            [U32, [U8, U16, U32], U32, u32, u32],
            [U64, [U8, U16, U32], U64, u64, u32],
            [U128, [U8, U16, U32], U128, u128, u32]
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: shl_wrapped,
        method: wrapping_shl,
        patterns: [
            [I8, [U8, U16, U32], I8, i8, u32],
            [I16, [U8, U16, U32], I16, i16, u32],
            [I32, [U8, U16, U32], I32, i32, u32],
            [I64, [U8, U16, U32], I64, i64, u32],
            [I128, [U8, U16, U32], I128, i128, u32],
            [U8, [U8, U16, U32], U8, u8, u32],
            [U16, [U8, U16, U32], U16, u16, u32],
            [U32, [U8, U16, U32], U32, u32, u32],
            [U64, [U8, U16, U32], U64, u64, u32],
            [U128, [U8, U16, U32], U128, u128, u32]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: shr,
        method: checked_shr,
        string: ">>",
        patterns: [
            [I8, [U8, U16, U32], I8, i8, u32],
            [I16, [U8, U16, U32], I16, i16, u32],
            [I32, [U8, U16, U32], I32, i32, u32],
            [I64, [U8, U16, U32], I64, i64, u32],
            [I128, [U8, U16, U32], I128, i128, u32],
            [U8, [U8, U16, U32], U8, u8, u32],
            [U16, [U8, U16, U32], U16, u16, u32],
            [U32, [U8, U16, U32], U32, u32, u32],
            [U64, [U8, U16, U32], U64, u64, u32],
            [U128, [U8, U16, U32], U128, u128, u32]
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: shr_wrapped,
        method: wrapping_shr,
        patterns: [
            [I8, [U8, U16, U32], I8, i8, u32],
            [I16, [U8, U16, U32], I16, i16, u32],
            [I32, [U8, U16, U32], I32, i32, u32],
            [I64, [U8, U16, U32], I64, i64, u32],
            [I128, [U8, U16, U32], I128, i128, u32],
            [U8, [U8, U16, U32], U8, u8, u32],
            [U16, [U8, U16, U32], U16, u16, u32],
            [U32, [U8, U16, U32], U32, u32, u32],
            [U64, [U8, U16, U32], U64, u64, u32],
            [U128, [U8, U16, U32], U128, u128, u32]
        ]
    );

    implement_const_binary!(
        @overflowing
        name: sub,
        method: checked_sub,
        string: "-",
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: sub_wrapped,
        method: wrapping_sub,
        patterns: [
            // [Field, [Field], Field, _, _],
            // [Group, [Group], Group, _, _],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
            //[Scalar, [Scalar], Scalar, _, _],
        ]
    );

    implement_const_binary!(
        @non-overflowing
        name: xor,
        method: bitxor,
        patterns: [
            [Boolean, [Boolean], Boolean, bool, bool],
            [I8, [I8], I8, i8, i8],
            [I16, [I16], I16, i16, i16],
            [I32, [I32], I32, i32, i32],
            [I64, [I64], I64, i64, i64],
            [I128, [I128], I128, i128, i128],
            [U8, [U8], U8, u8, u8],
            [U16, [U16], U16, u16, u16],
            [U32, [U32], U32, u32, u32],
            [U64, [U64], U64, u64, u64],
            [U128, [U128], U128, u128, u128]
        ]
    );
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Input(type_, ident) => write!(f, "input var {}: {type_}", ident.name),
            Address(val) => write!(f, "{val}"),
            Circuit(val, _) => write!(f, "{}", val.name),
            Boolean(val) => write!(f, "{val}"),
            Field(val) => write!(f, "{val}"),
            Group(val) => write!(f, "{val}"),
            I8(val) => write!(f, "{val}"),
            I16(val) => write!(f, "{val}"),
            I32(val) => write!(f, "{val}"),
            I64(val) => write!(f, "{val}"),
            I128(val) => write!(f, "{val}"),
            U8(val) => write!(f, "{val}"),
            U16(val) => write!(f, "{val}"),
            U32(val) => write!(f, "{val}"),
            U64(val) => write!(f, "{val}"),
            U128(val) => write!(f, "{val}"),
            Scalar(val) => write!(f, "{val}"),
            String(val) => write!(f, "{val}"),
        }
    }
}

impl TryFrom<Value> for i128 {
    type Error = LeoError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}

impl TryFrom<&Value> for i128 {
    type Error = LeoError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        use Value::*;
        match value {
            U8(val, span) => {
                i128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U16(val, span) => {
                i128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U32(val, span) => {
                i128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U64(val, span) => {
                i128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U128(val, span) => {
                i128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I8(val, _) => Ok(*val as i128),
            I16(val, _) => Ok(*val as i128),
            I32(val, _) => Ok(*val as i128),
            I64(val, _) => Ok(*val as i128),
            I128(val, _) => Ok(*val),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<Value> for u128 {
    type Error = LeoError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}

impl TryFrom<&Value> for u128 {
    type Error = LeoError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        use Value::*;
        match value {
            I8(val) => u128::try_from(*val)
                .map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), Default::default()).into()),
            I16(val) => u128::try_from(*val)
                .map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), Default::default()).into()),
            I32(val) => u128::try_from(*val)
                .map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), Default::default()).into()),
            I64(val) => u128::try_from(*val)
                .map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), Default::default()).into()),
            I128(val) => u128::try_from(*val)
                .map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), Default::default()).into()),
            U8(val) => Ok(*val as u128),
            U16(val) => Ok(*val as u128),
            U32(val) => Ok(*val as u128),
            U64(val) => Ok(*val as u128),
            U128(val) => Ok(*val),
            _ => unreachable!(),
        }
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<Value> for Type {
    fn from(v: Value) -> Self {
        v.as_ref().into()
    }
}

impl From<&Value> for Type {
    fn from(v: &Value) -> Self {
        use Value::*;
        match v {
            Input(type_, _) => type_.clone(),
            Address(..) => Type::Address,
            Boolean(..) => Type::Boolean,
            Circuit(ident, _) => Type::Identifier(*ident),
            Field(..) => Type::Field,
            Group(..) => Type::Group,
            I8(..) => Type::I8,
            I16(..) => Type::I16,
            I32(..) => Type::I32,
            I64(..) => Type::I64,
            I128(..) => Type::I128,
            U8(..) => Type::U8,
            U16(..) => Type::U16,
            U32(..) => Type::U32,
            U64(..) => Type::U64,
            U128(..) => Type::U128,
            Scalar(..) => Type::Scalar,
            String(..) => Type::String,
        }
    }
}

// TODO: Consider making this `Option<Value>` instead of `Value`.
impl From<&Literal> for Value {
    /// Converts a literal to a value.
    /// This should only be invoked on literals that are known to be valid.
    fn from(literal: &Literal) -> Self {
        match literal {
            Literal::Address(string, _) => Self::Address(string.clone()),
            Literal::Boolean(bool, _) => Self::Boolean(*bool),
            Literal::Field(string, _) => Self::Field(string.clone()),
            Literal::Group(group_literal) => Self::Group(group_literal.clone()),
            Literal::Scalar(string, _) => Self::Scalar(string.clone()),
            Literal::String(string, _) => Self::String(string.clone()),
            Literal::I8(string, _) => Self::I8(string.parse::<i8>().unwrap()),
            Literal::I16(string, _) => Self::I16(string.parse::<i16>().unwrap()),
            Literal::I32(string, _) => Self::I32(string.parse::<i32>().unwrap()),
            Literal::I64(string, _) => Self::I64(string.parse::<i64>().unwrap()),
            Literal::I128(string, _) => Self::I128(string.parse::<i128>().unwrap()),
            Literal::U8(string, _) => Self::U8(string.parse::<u8>().unwrap()),
            Literal::U16(string, _) => Self::U16(string.parse::<u16>().unwrap()),
            Literal::U32(string, _) => Self::U32(string.parse::<u32>().unwrap()),
            Literal::U64(string, _) => Self::U64(string.parse::<u64>().unwrap()),
            Literal::U128(string, _) => Self::U128(string.parse::<u128>().unwrap()),
        }
    }
}

impl From<Value> for Literal {
    fn from(v: Value) -> Self {
        use Value::*;
        match v {
            Input(_, _) => todo!("We need to test if this is hittable"),
            Address(v) => Literal::Address(v, Default::default()),
            Boolean(v) => Literal::Boolean(v, Default::default()),
            Circuit(_ident, _values) => todo!("We need to test if this is hittable"),
            Field(v) => Literal::Field(v, Default::default()),
            Group(v) => Literal::Group(v),
            I8(v) => Literal::I8(v.to_string(), Default::default()),
            I16(v) => Literal::I16(v.to_string(), Default::default()),
            I32(v) => Literal::I32(v.to_string(), Default::default()),
            I64(v) => Literal::I64(v.to_string(), Default::default()),
            I128(v) => Literal::I128(v.to_string(), Default::default()),
            U8(v) => Literal::U8(v.to_string(), Default::default()),
            U16(v) => Literal::U16(v.to_string(), Default::default()),
            U32(v) => Literal::U32(v.to_string(), Default::default()),
            U64(v) => Literal::U64(v.to_string(), Default::default()),
            U128(v) => Literal::U128(v.to_string(), Default::default()),
            Scalar(v) => Literal::Scalar(v, Default::default()),
            String(v) => Literal::String(v, Default::default()),
        }
    }
}
