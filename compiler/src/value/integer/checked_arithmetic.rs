// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{errors::IntegerError, Integer};
use leo_typed::Span;

use std::convert::TryFrom;

macro_rules! match_checked_options {
    ($a: ident, $b: ident, $span: ident => $expression: expr) => {{
        match ($a, $b) {
            (Some($a), Some($b)) => Ok($expression.is_some()),
            (_, _) => Err(IntegerError::mismatched_option($span)),
        }
    }};
}

macro_rules! match_integers_value {
    ($a: ident, $b: ident, $span: ident => $expression: expr) => {{

        match ($a, $b) {
            // Unsigned integers
            (Integer::U8(a), Integer::U8(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::U16(a), Integer::U16(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::U32(a), Integer::U32(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::U64(a), Integer::U64(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::U128(a), Integer::U128(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },

            // Signed integers
            (Integer::I8(a), Integer::I8(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::I16(a), Integer::I16(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::I32(a), Integer::I32(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::I64(a), Integer::I64(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (Integer::I128(a), Integer::I128(b)) => {
                let $a = a.value;
                let $b = b.value;
                match_checked_options!($a, $b, $span => $expression)
            },
            (a, b) => Err(IntegerError::mismatched_types(a.get_type(), b.get_type(), $span))
        }
    }}
}

macro_rules! is_negative {
    ($v: ident, $e: ident) => {{
        if let Some(value) = $v {
            if value < 0 {
                return Err($e);
            }
        }
        Ok(())
    }};
}

macro_rules! u32_try_from {
    ($v: ident, $e: ident) => {{
        if let Some(value) = $v {
            let result = u32::try_from(value);
            if result.is_err() {
                return Err($e);
            }
        }
        Ok(())
    }};
}

impl Integer {
    /// Checks if integer addition will throw an error
    pub(crate) fn checked_add(&self, other: &Integer, span: Span) -> Result<(), IntegerError> {
        let a = self.clone();
        let b = other.clone();
        let s = span.clone();

        let result = match_integers_value!(a, b, s => a.checked_add(b))?;

        if !result {
            Err(IntegerError::overflow_integer(self, other, "+".to_owned(), span))
        } else {
            Ok(())
        }
    }

    /// Checks if integer subtraction will throw an error
    pub(crate) fn checked_sub(&self, other: &Integer, span: Span) -> Result<(), IntegerError> {
        let a = self.clone();
        let b = other.clone();
        let s = span.clone();

        let result = match_integers_value!(a, b, s => a.checked_sub(b))?;

        if !result {
            Err(IntegerError::overflow_integer(self, other, "-".to_owned(), span))
        } else {
            Ok(())
        }
    }

    /// Checks if integer multiplication will throw an error
    pub(crate) fn checked_mul(&self, other: &Integer, span: Span) -> Result<(), IntegerError> {
        let a = self.clone();
        let b = other.clone();
        let s = span.clone();

        let result = match_integers_value!(a, b, s => a.checked_mul(b))?;

        if !result {
            Err(IntegerError::overflow_integer(self, other, "*".to_owned(), span))
        } else {
            Ok(())
        }
    }

    /// Checks if integer division will throw an error
    pub(crate) fn checked_div(&self, other: &Integer, span: Span) -> Result<(), IntegerError> {
        let a = self.clone();
        let b = other.clone();
        let s = span.clone();

        let result = match_integers_value!(a, b, s => a.checked_div(b))?;

        if !result {
            Err(IntegerError::division_by_zero(span))
        } else {
            Ok(())
        }
    }

    /// Checks if integer exponentiation will throw an error
    pub(crate) fn checked_pow(&self, other: &Integer, span: Span) -> Result<(), IntegerError> {
        let operation = "**".to_owned();
        let rhs = other.clone();

        // If the rhs is negative, throw an signed integer overflow error since
        // raising a signed integer to a negative exponent results in a fraction
        let signed_overflow_error = IntegerError::overflow_signed_integer(self, other, operation.clone(), span.clone());
        let overflow_error = IntegerError::overflow_rust_pow(other, span.clone());
        match rhs {
            Integer::I8(int) => {
                let v = int.value;
                let res: Result<(), IntegerError> = is_negative!(v, signed_overflow_error);
                res?;
            }
            Integer::I16(int) => {
                let v = int.value;
                let res: Result<(), IntegerError> = is_negative!(v, signed_overflow_error);
                res?;
            }
            Integer::I32(int) => {
                let v = int.value;
                let res: Result<(), IntegerError> = is_negative!(v, signed_overflow_error);
                res?;
            }
            Integer::I64(int) => {
                let v1 = int.value;
                let v2 = int.value;

                let res: Result<(), IntegerError> = is_negative!(v1, signed_overflow_error);
                res?;

                // Check that the rhs can be type cast to a u32
                let u32_res: Result<(), IntegerError> = u32_try_from!(v2, overflow_error);
                u32_res?;
            }
            Integer::I128(int) => {
                let v1 = int.value;
                let v2 = int.value;

                let res: Result<(), IntegerError> = is_negative!(v1, signed_overflow_error);
                res?;

                // Check that the rhs can be type cast to a u32
                let u32_res: Result<(), IntegerError> = u32_try_from!(v2, overflow_error);
                u32_res?;
            }
            Integer::U64(uint) => {
                let v = uint.value;

                // Check that the rhs can be type cast to a u32
                let res: Result<(), IntegerError> = u32_try_from!(v, overflow_error);
                res?;
            }
            Integer::U128(uint) => {
                let v = uint.value;

                // Check that the rhs can be type cast to a u32
                let res: Result<(), IntegerError> = u32_try_from!(v, overflow_error);
                res?;
            }
            _ => {}
        }

        // Call native rust `checked_pow` function to check for overflow
        let a = self.clone();
        let b = other.clone();
        let s = span.clone();

        let result = match_integers_value!(a, b, s => a.checked_pow(b as u32))?;

        if !result {
            Err(IntegerError::overflow_integer(self, other, operation.clone(), span))
        } else {
            Ok(())
        }
    }
}
