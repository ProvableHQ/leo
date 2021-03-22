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

//! Conversion of integer declarations to constraints in Leo.
use crate::{errors::IntegerError, IntegerTrait};
use leo_asg::{ConstInt, IntegerType, Span};
use leo_ast::InputValue;

use snarkvm_fields::{Field, PrimeField};
use snarkvm_gadgets::traits::utilities::{
    alloc::AllocGadget,
    arithmetic::{Add, Div, Mul, Neg, Pow, Sub},
    bits::comparator::{ComparatorGadget, EvaluateLtGadget},
    boolean::Boolean,
    eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
    int::{Int128, Int16, Int32, Int64, Int8},
    select::CondSelectGadget,
    uint::{Sub as UIntSub, *},
};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};
use std::fmt;

/// An integer type enum wrapping the integer value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Integer {
    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),

    I8(Int8),
    I16(Int16),
    I32(Int32),
    I64(Int64),
    I128(Int128),
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let integer = self;
        let option = match_integer!(integer => integer.get_value());
        match option {
            Some(number) => write!(f, "{}", number),
            None => write!(f, "[input]{}", self.get_type()),
        }
    }
}

impl Integer {
    ///
    /// Returns a new integer from an expression.
    ///
    /// Checks that the expression is equal to the expected type if given.
    ///
    pub fn new(value: &ConstInt) -> Self {
        match value {
            ConstInt::U8(i) => Integer::U8(UInt8::constant(*i)),
            ConstInt::U16(i) => Integer::U16(UInt16::constant(*i)),
            ConstInt::U32(i) => Integer::U32(UInt32::constant(*i)),
            ConstInt::U64(i) => Integer::U64(UInt64::constant(*i)),
            ConstInt::U128(i) => Integer::U128(UInt128::constant(*i)),
            ConstInt::I8(i) => Integer::I8(Int8::constant(*i)),
            ConstInt::I16(i) => Integer::I16(Int16::constant(*i)),
            ConstInt::I32(i) => Integer::I32(Int32::constant(*i)),
            ConstInt::I64(i) => Integer::I64(Int64::constant(*i)),
            ConstInt::I128(i) => Integer::I128(Int128::constant(*i)),
        }
    }

    pub fn get_bits(&self) -> Vec<Boolean> {
        let integer = self;
        match_integer!(integer => integer.to_bits_le())
    }

    // pub fn get_bits_typed(&self) -> (Vec<Boolean>, IntegerType) {
    //     let integer = self;
    //     (match_integer!(integer => integer.to_bits_le()), self.get_type())
    // }

    // pub fn from_bits_typed(type_: &IntegerType, bits: &[Boolean]) -> Integer {
    //     match type_ {
    //         IntegerType::U8 => Integer::U8(UInt8::from_bits_le(bits)),
    //         IntegerType::U16 => Integer::U16(UInt16::from_bits_le(bits)),
    //         IntegerType::U32 => Integer::U32(UInt32::from_bits_le(bits)),
    //         IntegerType::U64 => Integer::U64(UInt64::from_bits_le(bits)),
    //         IntegerType::U128 => Integer::U128(UInt128::from_bits_le(bits)),
    //         IntegerType::I8 => Integer::I8(Int8::from_bits_le(bits)),
    //         IntegerType::I16 => Integer::I16(Int16::from_bits_le(bits)),
    //         IntegerType::I32 => Integer::I32(Int32::from_bits_le(bits)),
    //         IntegerType::I64 => Integer::I64(Int64::from_bits_le(bits)),
    //         IntegerType::I128 => Integer::I128(Int128::from_bits_le(bits)),
    //     }
    // }

    pub fn get_value(&self) -> Option<String> {
        let integer = self;
        match_integer!(integer => integer.get_value())
    }

    pub fn to_usize(&self) -> Option<usize> {
        let unsigned_integer = self;
        match_unsigned_integer!(unsigned_integer => unsigned_integer.get_index())
    }

    pub fn get_type(&self) -> IntegerType {
        match self {
            Integer::U8(_u8) => IntegerType::U8,
            Integer::U16(_u16) => IntegerType::U16,
            Integer::U32(_u32) => IntegerType::U32,
            Integer::U64(_u64) => IntegerType::U64,
            Integer::U128(_u128) => IntegerType::U128,

            Integer::I8(_u8) => IntegerType::I8,
            Integer::I16(_u16) => IntegerType::I16,
            Integer::I32(_u32) => IntegerType::I32,
            Integer::I64(_u64) => IntegerType::I64,
            Integer::I128(_u128) => IntegerType::I128,
        }
    }

    pub fn allocate_type<F: Field, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        integer_type: &IntegerType,
        name: &str,
        option: Option<String>,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        Ok(match integer_type {
            IntegerType::U8 => allocate_type!(u8, UInt8, Integer::U8, cs, name, option, span),
            IntegerType::U16 => allocate_type!(u16, UInt16, Integer::U16, cs, name, option, span),
            IntegerType::U32 => allocate_type!(u32, UInt32, Integer::U32, cs, name, option, span),
            IntegerType::U64 => allocate_type!(u64, UInt64, Integer::U64, cs, name, option, span),
            IntegerType::U128 => allocate_type!(u128, UInt128, Integer::U128, cs, name, option, span),

            IntegerType::I8 => allocate_type!(i8, Int8, Integer::I8, cs, name, option, span),
            IntegerType::I16 => allocate_type!(i16, Int16, Integer::I16, cs, name, option, span),
            IntegerType::I32 => allocate_type!(i32, Int32, Integer::I32, cs, name, option, span),
            IntegerType::I64 => allocate_type!(i64, Int64, Integer::I64, cs, name, option, span),
            IntegerType::I128 => allocate_type!(i128, Int128, Integer::I128, cs, name, option, span),
        })
    }

    pub fn from_input<F: Field, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        integer_type: &IntegerType,
        name: &str,
        integer_value: Option<InputValue>,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        // Check that the input value is the correct type
        let option = match integer_value {
            Some(input) => {
                if let InputValue::Integer(_type_, number) = input {
                    Some(number)
                } else {
                    return Err(IntegerError::invalid_integer(input.to_string(), span));
                }
            }
            None => None,
        };

        Self::allocate_type(cs, integer_type, name, option, span)
    }

    pub fn negate<F: PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce -{} {}:{}", self, span.line_start, span.col_start);

        let a = self;

        let result = match_signed_integer!(a, span => a.neg(cs.ns(|| unique_namespace)));

        result.ok_or_else(|| IntegerError::negate_operation(span))
    }

    pub fn add<F: PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} + {} {}:{}", self, other, span.line_start, span.col_start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.add(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("+".to_string(), span))
    }

    pub fn sub<F: PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} - {} {}:{}", self, other, span.line_start, span.col_start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.sub(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("-".to_string(), span))
    }

    pub fn mul<F: PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} * {} {}:{}", self, other, span.line_start, span.col_start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.mul(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("*".to_string(), span))
    }

    pub fn div<F: PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} ÷ {} {}:{}", self, other, span.line_start, span.col_start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.div(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("÷".to_string(), span))
    }

    pub fn pow<F: PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} ** {} {}:{}", self, other, span.line_start, span.col_start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.pow(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("**".to_string(), span))
    }
}

impl<F: PrimeField> EvaluateEqGadget<F> for Integer {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        let a = self;
        let b = other;

        let result = match_integers!((a, b) => a.evaluate_equal(cs, b));

        result.ok_or(SynthesisError::Unsatisfiable)
    }
}

impl<F: PrimeField> EvaluateLtGadget<F> for Integer {
    fn less_than<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        let a = self;
        let b = other;
        let result = match_integers!((a, b) => a.less_than(cs, b));

        result.ok_or(SynthesisError::Unsatisfiable)
    }
}

impl<F: PrimeField> ComparatorGadget<F> for Integer {}

impl<F: PrimeField> EqGadget<F> for Integer {}

impl<F: PrimeField> ConditionalEqGadget<F> for Integer {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        let a = self;
        let b = other;

        let result = match_integers!((a, b) => a.conditional_enforce_equal(cs, b, condition));

        result.ok_or(SynthesisError::Unsatisfiable)
    }

    fn cost() -> usize {
        unimplemented!() // cannot determine which integer we are enforcing
    }
}

impl<F: PrimeField> CondSelectGadget<F> for Integer {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        match (first, second) {
            (Integer::U8(a), Integer::U8(b)) => Ok(Integer::U8(UInt8::conditionally_select(cs, cond, a, b)?)),
            (Integer::U16(a), Integer::U16(b)) => Ok(Integer::U16(UInt16::conditionally_select(cs, cond, a, b)?)),
            (Integer::U32(a), Integer::U32(b)) => Ok(Integer::U32(UInt32::conditionally_select(cs, cond, a, b)?)),
            (Integer::U64(a), Integer::U64(b)) => Ok(Integer::U64(UInt64::conditionally_select(cs, cond, a, b)?)),
            (Integer::U128(a), Integer::U128(b)) => Ok(Integer::U128(UInt128::conditionally_select(cs, cond, a, b)?)),
            (Integer::I8(a), Integer::I8(b)) => Ok(Integer::I8(Int8::conditionally_select(cs, cond, a, b)?)),
            (Integer::I16(a), Integer::I16(b)) => Ok(Integer::I16(Int16::conditionally_select(cs, cond, a, b)?)),
            (Integer::I32(a), Integer::I32(b)) => Ok(Integer::I32(Int32::conditionally_select(cs, cond, a, b)?)),
            (Integer::I64(a), Integer::I64(b)) => Ok(Integer::I64(Int64::conditionally_select(cs, cond, a, b)?)),
            (Integer::I128(a), Integer::I128(b)) => Ok(Integer::I128(Int128::conditionally_select(cs, cond, a, b)?)),

            (_, _) => Err(SynthesisError::Unsatisfiable), // types do not match
        }
    }

    fn cost() -> usize {
        unimplemented!() // cannot determine which integer we are enforcing
    }
}
