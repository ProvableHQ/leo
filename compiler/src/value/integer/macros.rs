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

use snarkvm_gadgets::traits::utilities::{
    boolean::Boolean,
    int::{Int128, Int16, Int32, Int64, Int8},
    uint::{UInt128, UInt16, UInt32, UInt64, UInt8},
};
use std::{convert::TryInto, fmt::Debug};

pub trait IntegerTrait: Sized + Clone + Debug {
    fn get_value(&self) -> Option<String>;

    fn get_index(&self) -> Option<usize>;

    fn get_bits(&self) -> Vec<Boolean>;
}

macro_rules! integer_trait_impl {
    ($($gadget: ident)*) => ($(
        impl IntegerTrait for $gadget {
            fn get_value(&self) -> Option<String> {
                self.value.map(|num| num.to_string())
            }

            fn get_index(&self) -> Option<usize> {
                self.value.map(|num| num.try_into().ok()).flatten()
            }

            fn get_bits(&self) -> Vec<Boolean> {
                self.bits.clone()
            }
        }

    )*)
}

integer_trait_impl!(UInt8 UInt16 UInt32 UInt64 UInt128 Int8 Int16 Int32 Int64 Int128);

/// Useful macros to avoid duplicating `match` constructions.
#[macro_export]
macro_rules! match_integer {
    ($integer: ident => $expression: expr) => {
        match $integer {
            Integer::U8($integer) => $expression,
            Integer::U16($integer) => $expression,
            Integer::U32($integer) => $expression,
            Integer::U64($integer) => $expression,
            Integer::U128($integer) => $expression,

            Integer::I8($integer) => $expression,
            Integer::I16($integer) => $expression,
            Integer::I32($integer) => $expression,
            Integer::I64($integer) => $expression,
            Integer::I128($integer) => $expression,
        }
    };
}

#[macro_export]
macro_rules! match_unsigned_integer {
    ($integer: ident => $expression: expr) => {
        match $integer {
            Integer::U8($integer) => $expression,
            Integer::U16($integer) => $expression,
            Integer::U32($integer) => $expression,
            Integer::U64($integer) => $expression,
            Integer::U128($integer) => $expression,

            _ => None,
        }
    };
}

#[macro_export]
macro_rules! match_signed_integer {
    ($integer: ident, $span: ident => $expression: expr) => {
        match $integer {
            Integer::I8($integer) => Some(Integer::I8($expression.map_err(|e| IntegerError::signed(e, $span))?)),
            Integer::I16($integer) => Some(Integer::I16($expression.map_err(|e| IntegerError::signed(e, $span))?)),
            Integer::I32($integer) => Some(Integer::I32($expression.map_err(|e| IntegerError::signed(e, $span))?)),
            Integer::I64($integer) => Some(Integer::I64($expression.map_err(|e| IntegerError::signed(e, $span))?)),
            Integer::I128($integer) => Some(Integer::I128($expression.map_err(|e| IntegerError::signed(e, $span))?)),

            _ => None,
        }
    };
}

#[macro_export]
macro_rules! match_integers {
    (($a: ident, $b: ident) => $expression:expr) => {
        match ($a, $b) {
            (Integer::U8($a), Integer::U8($b)) => Some($expression?),
            (Integer::U16($a), Integer::U16($b)) => Some($expression?),
            (Integer::U32($a), Integer::U32($b)) => Some($expression?),
            (Integer::U64($a), Integer::U64($b)) => Some($expression?),
            (Integer::U128($a), Integer::U128($b)) => Some($expression?),

            (Integer::I8($a), Integer::I8($b)) => Some($expression?),
            (Integer::I16($a), Integer::I16($b)) => Some($expression?),
            (Integer::I32($a), Integer::I32($b)) => Some($expression?),
            (Integer::I64($a), Integer::I64($b)) => Some($expression?),
            (Integer::I128($a), Integer::I128($b)) => Some($expression?),
            (_, _) => None,
        }
    };
}

#[macro_export]
macro_rules! match_integers_span {
    (($a: ident, $b: ident), $span: ident => $expression:expr) => {
        match ($a, $b) {
            (Integer::U8($a), Integer::U8($b)) => {
                Some(Integer::U8($expression.map_err(|e| IntegerError::synthesis(e, $span))?))
            }
            (Integer::U16($a), Integer::U16($b)) => Some(Integer::U16(
                $expression.map_err(|e| IntegerError::synthesis(e, $span))?,
            )),
            (Integer::U32($a), Integer::U32($b)) => Some(Integer::U32(
                $expression.map_err(|e| IntegerError::synthesis(e, $span))?,
            )),
            (Integer::U64($a), Integer::U64($b)) => Some(Integer::U64(
                $expression.map_err(|e| IntegerError::synthesis(e, $span))?,
            )),
            (Integer::U128($a), Integer::U128($b)) => Some(Integer::U128(
                $expression.map_err(|e| IntegerError::synthesis(e, $span))?,
            )),

            (Integer::I8($a), Integer::I8($b)) => {
                Some(Integer::I8($expression.map_err(|e| IntegerError::signed(e, $span))?))
            }
            (Integer::I16($a), Integer::I16($b)) => {
                Some(Integer::I16($expression.map_err(|e| IntegerError::signed(e, $span))?))
            }
            (Integer::I32($a), Integer::I32($b)) => {
                Some(Integer::I32($expression.map_err(|e| IntegerError::signed(e, $span))?))
            }
            (Integer::I64($a), Integer::I64($b)) => {
                Some(Integer::I64($expression.map_err(|e| IntegerError::signed(e, $span))?))
            }
            (Integer::I128($a), Integer::I128($b)) => {
                Some(Integer::I128($expression.map_err(|e| IntegerError::signed(e, $span))?))
            }
            (_, _) => None,
        }
    };
}

macro_rules! allocate_type {
    ($rust_ty:ty, $gadget_ty:ty, $leo_ty:path, $cs:expr, $name:expr, $option:expr, $span:expr) => {{
        let option = $option.map(|s| {
            s.parse::<$rust_ty>()
                .map_err(|_| IntegerError::invalid_integer(s, $span))
                .unwrap()
        });

        let result = <$gadget_ty>::alloc(
            $cs.ns(|| {
                format!(
                    "`{}: {}` {}:{}",
                    $name,
                    stringify!($rust_ty),
                    $span.line_start,
                    $span.col_start
                )
            }),
            || option.ok_or(SynthesisError::AssignmentMissing),
        )
        .map_err(|_| IntegerError::missing_integer(format!("{}: {}", $name, stringify!($rust_ty)), $span))?;

        $leo_ty(result)
    }};
}
