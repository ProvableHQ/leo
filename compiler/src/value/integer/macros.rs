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

use leo_gadgets::signed_integer::*;

use snarkvm_gadgets::traits::boolean::Boolean;
use snarkvm_gadgets::traits::uint::UInt128;
use snarkvm_gadgets::traits::uint::UInt16;
use snarkvm_gadgets::traits::uint::UInt32;
use snarkvm_gadgets::traits::uint::UInt64;
use snarkvm_gadgets::traits::uint::UInt8;
use std::fmt::Debug;

pub trait IntegerTrait: Sized + Clone + Debug {
    fn get_value(&self) -> Option<String>;

    fn get_bits(&self) -> Vec<Boolean>;
}

macro_rules! integer_trait_impl {
    ($($gadget: ident)*) => ($(
        impl IntegerTrait for $gadget {
            fn get_value(&self) -> Option<String> {
                self.value.map(|num| num.to_string())
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
            Integer::I8($integer) => Some(Integer::I8(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            Integer::I16($integer) => Some(Integer::I16(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            Integer::I32($integer) => Some(Integer::I32(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            Integer::I64($integer) => Some(Integer::I64(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            Integer::I128($integer) => Some(Integer::I128(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),

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
            (Integer::U8($a), Integer::U8($b)) => Some(Integer::U8(
                $expression.map_err(|e| IntegerError::synthesis(e, $span.to_owned()))?,
            )),
            (Integer::U16($a), Integer::U16($b)) => Some(Integer::U16(
                $expression.map_err(|e| IntegerError::synthesis(e, $span.to_owned()))?,
            )),
            (Integer::U32($a), Integer::U32($b)) => Some(Integer::U32(
                $expression.map_err(|e| IntegerError::synthesis(e, $span.to_owned()))?,
            )),
            (Integer::U64($a), Integer::U64($b)) => Some(Integer::U64(
                $expression.map_err(|e| IntegerError::synthesis(e, $span.to_owned()))?,
            )),
            (Integer::U128($a), Integer::U128($b)) => Some(Integer::U128(
                $expression.map_err(|e| IntegerError::synthesis(e, $span.to_owned()))?,
            )),

            (Integer::I8($a), Integer::I8($b)) => Some(Integer::I8(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            (Integer::I16($a), Integer::I16($b)) => Some(Integer::I16(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            (Integer::I32($a), Integer::I32($b)) => Some(Integer::I32(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            (Integer::I64($a), Integer::I64($b)) => Some(Integer::I64(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            (Integer::I128($a), Integer::I128($b)) => Some(Integer::I128(
                $expression.map_err(|e| IntegerError::signed(e, $span.to_owned()))?,
            )),
            (_, _) => None,
        }
    };
}
