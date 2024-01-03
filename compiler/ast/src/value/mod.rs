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

use crate::{GroupLiteral, Identifier, IntegerType, Literal, NodeID, Type};

use leo_errors::{type_name, FlattenError, LeoError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use std::{
    fmt::Display,
    num::ParseIntError,
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
                l: |l: $m_type, span| l.$method().ok_or_else(|| FlattenError::unary_overflow(l, $str, span))
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
                l: |l: $m_type, _| -> Result<$m_type> { Ok(l.$method()) }
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
        // TODO: This is temporary since the currently unused code is used in constant folding.
        #[allow(dead_code)]
        #[allow(clippy::redundant_closure_call)]
        pub(crate) fn $name(self, span: Span) -> Result<Self> {
            use Value::*;

            match self {
                $(
                    $type(v, _) => {
                        Ok($type($logic(v.into(), span)?, span))
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
                logic: |l: $m_lhs, r: $m_rhs, t, span| l.$method(r).ok_or_else(|| FlattenError::binary_overflow(l, $str, r, t, span))
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
                logic: |l: $m_lhs, r: $m_rhs, _, _| -> Result<$m_lhs> {Ok(l.$method(r))}
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
                logic: |l: $m_lhs, r: $m_rhs, _, _| -> Result<bool> {Ok(l.$method(&r))}
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
        // This is temporary since the currently unused code is used in constant folding.
        #[allow(dead_code)]
        #[allow(clippy::redundant_closure_call)]
        pub(crate) fn $name(self, other: Self, span: Span) -> Result<Self> {
            use Value::*;

            match (self, other) {
                $(
                    $(
                        ($lhs(types, _), $rhs(rhs, _)) => {
                            let rhs_type = type_name(&rhs);
                            let out = $logic(types, rhs.into(), rhs_type, span)?;
                            Ok($out(out, span))
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
    Address(String, Span),
    Boolean(bool, Span),
    Struct(Identifier, IndexMap<Symbol, Value>),
    Field(String, Span),
    Group(Box<GroupLiteral>),
    I8(i8, Span),
    I16(i16, Span),
    I32(i32, Span),
    I64(i64, Span),
    I128(i128, Span),
    U8(u8, Span),
    U16(u16, Span),
    U32(u32, Span),
    U64(u64, Span),
    U128(u128, Span),
    Scalar(String, Span),
    String(String, Span),
}

impl Value {
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

    // TODO: This is temporary since the currently unused code is used in constant folding.
    #[allow(dead_code)]
    pub(crate) fn is_supported_const_fold_type(&self) -> bool {
        use Value::*;
        matches!(
            self,
            Boolean(_, _)
                | I8(_, _)
                | I16(_, _)
                | I32(_, _)
                | I64(_, _)
                | I128(_, _)
                | U8(_, _)
                | U16(_, _)
                | U32(_, _)
                | U64(_, _)
                | U128(_, _)
        )
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Input(type_, ident) => write!(f, "input var {}: {type_}", ident.name),
            Address(val, _) => write!(f, "{val}"),
            Struct(val, _) => write!(f, "{}", val.name),
            Boolean(val, _) => write!(f, "{val}"),
            Field(val, _) => write!(f, "{val}"),
            Group(val) => write!(f, "{val}"),
            I8(val, _) => write!(f, "{val}"),
            I16(val, _) => write!(f, "{val}"),
            I32(val, _) => write!(f, "{val}"),
            I64(val, _) => write!(f, "{val}"),
            I128(val, _) => write!(f, "{val}"),
            U8(val, _) => write!(f, "{val}"),
            U16(val, _) => write!(f, "{val}"),
            U32(val, _) => write!(f, "{val}"),
            U64(val, _) => write!(f, "{val}"),
            U128(val, _) => write!(f, "{val}"),
            Scalar(val, _) => write!(f, "{val}"),
            String(val, _) => write!(f, "{val}"),
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
            I8(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I16(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I32(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I64(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I128(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U8(val, _) => Ok(*val as u128),
            U16(val, _) => Ok(*val as u128),
            U32(val, _) => Ok(*val as u128),
            U64(val, _) => Ok(*val as u128),
            U128(val, _) => Ok(*val),
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
            Address(_, _) => Type::Address,
            Boolean(_, _) => Type::Boolean,
            Struct(ident, _) => Type::Identifier(*ident),
            Field(_, _) => Type::Field,
            Group(_) => Type::Group,
            I8(_, _) => Type::Integer(IntegerType::I8),
            I16(_, _) => Type::Integer(IntegerType::I16),
            I32(_, _) => Type::Integer(IntegerType::I32),
            I64(_, _) => Type::Integer(IntegerType::I64),
            I128(_, _) => Type::Integer(IntegerType::I128),
            U8(_, _) => Type::Integer(IntegerType::U8),
            U16(_, _) => Type::Integer(IntegerType::U16),
            U32(_, _) => Type::Integer(IntegerType::U32),
            U64(_, _) => Type::Integer(IntegerType::U64),
            U128(_, _) => Type::Integer(IntegerType::U128),
            Scalar(_, _) => Type::Scalar,
            String(_, _) => Type::String,
        }
    }
}

impl TryFrom<&Literal> for Value {
    type Error = ParseIntError;

    /// Converts a literal to a value.
    fn try_from(literal: &Literal) -> Result<Self, Self::Error> {
        Ok(match literal {
            Literal::Address(string, span, _) => Self::Address(string.clone(), *span),
            Literal::Boolean(bool, span, _) => Self::Boolean(*bool, *span),
            Literal::Field(string, span, _) => Self::Field(string.clone(), *span),
            Literal::Group(group_literal) => Self::Group(group_literal.clone()),
            Literal::Scalar(string, span, _) => Self::Scalar(string.clone(), *span),
            Literal::String(string, span, _) => Self::String(string.clone(), *span),
            Literal::Integer(integer_type, raw_string, span, _) => {
                let string = raw_string.replace('_', "");
                match integer_type {
                    IntegerType::U8 => Self::U8(string.parse()?, *span),
                    IntegerType::U16 => Self::U16(string.parse()?, *span),
                    IntegerType::U32 => Self::U32(string.parse()?, *span),
                    IntegerType::U64 => Self::U64(string.parse()?, *span),
                    IntegerType::U128 => Self::U128(string.parse()?, *span),
                    IntegerType::I8 => Self::I8(string.parse()?, *span),
                    IntegerType::I16 => Self::I16(string.parse()?, *span),
                    IntegerType::I32 => Self::I32(string.parse()?, *span),
                    IntegerType::I64 => Self::I64(string.parse()?, *span),
                    IntegerType::I128 => Self::I128(string.parse()?, *span),
                }
            }
        })
    }
}

impl Literal {
    #[allow(unused)]
    fn from_value(v: Value, id: NodeID) -> Self {
        use Value::*;
        match v {
            Input(_, _) => todo!("We need to test if this is hittable"),
            Address(v, span) => Literal::Address(v, span, id),
            Boolean(v, span) => Literal::Boolean(v, span, id),
            Struct(_ident, _values) => todo!("We need to test if this is hittable"),
            Field(v, span) => Literal::Field(v, span, id),
            Group(v) => Literal::Group(v),
            I8(v, span) => Literal::Integer(IntegerType::I8, v.to_string(), span, id),
            I16(v, span) => Literal::Integer(IntegerType::I16, v.to_string(), span, id),
            I32(v, span) => Literal::Integer(IntegerType::I32, v.to_string(), span, id),
            I64(v, span) => Literal::Integer(IntegerType::I64, v.to_string(), span, id),
            I128(v, span) => Literal::Integer(IntegerType::I128, v.to_string(), span, id),
            U8(v, span) => Literal::Integer(IntegerType::U8, v.to_string(), span, id),
            U16(v, span) => Literal::Integer(IntegerType::U16, v.to_string(), span, id),
            U32(v, span) => Literal::Integer(IntegerType::U32, v.to_string(), span, id),
            U64(v, span) => Literal::Integer(IntegerType::U64, v.to_string(), span, id),
            U128(v, span) => Literal::Integer(IntegerType::U128, v.to_string(), span, id),
            Scalar(v, span) => Literal::Scalar(v, span, id),
            String(v, span) => Literal::String(v, span, id),
        }
    }
}
