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
use leo_ast::{InputValue, IntegerType, Span, Type};
use leo_gadgets::{
    arithmetic::*,
    bits::comparator::{ComparatorGadget, EvaluateLtGadget},
    signed_integer::*,
};

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
            select::CondSelectGadget,
            uint::*,
        },
    },
};
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
    pub fn new(
        expected_type: Option<Type>,
        actual_integer_type: &IntegerType,
        string: String,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        // Check expected type if given.
        if let Some(type_) = expected_type {
            // Check expected type is an integer.
            match type_ {
                Type::IntegerType(expected_integer_type) => {
                    // Check expected integer type == actual integer type
                    if expected_integer_type.ne(actual_integer_type) {
                        return Err(IntegerError::invalid_integer_type(
                            &expected_integer_type,
                            actual_integer_type,
                            span.to_owned(),
                        ));
                    }
                }
                type_ => return Err(IntegerError::invalid_type(&type_, span.to_owned())),
            }
        }

        // Return a new constant integer.
        Self::new_constant(actual_integer_type, string, span)
    }

    ///
    /// Returns a new integer value from an expression.
    ///
    /// The returned integer value is "constant" and is not allocated in the constraint system.
    ///
    pub fn new_constant(integer_type: &IntegerType, string: String, span: &Span) -> Result<Self, IntegerError> {
        match integer_type {
            IntegerType::U8 => {
                let number = string
                    .parse::<u8>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::U8(UInt8::constant(number)))
            }
            IntegerType::U16 => {
                let number = string
                    .parse::<u16>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::U16(UInt16::constant(number)))
            }
            IntegerType::U32 => {
                let number = string
                    .parse::<u32>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::U32(UInt32::constant(number)))
            }
            IntegerType::U64 => {
                let number = string
                    .parse::<u64>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::U64(UInt64::constant(number)))
            }
            IntegerType::U128 => {
                let number = string
                    .parse::<u128>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::U128(UInt128::constant(number)))
            }

            IntegerType::I8 => {
                let number = string
                    .parse::<i8>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::I8(Int8::constant(number)))
            }
            IntegerType::I16 => {
                let number = string
                    .parse::<i16>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::I16(Int16::constant(number)))
            }
            IntegerType::I32 => {
                let number = string
                    .parse::<i32>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::I32(Int32::constant(number)))
            }
            IntegerType::I64 => {
                let number = string
                    .parse::<i64>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::I64(Int64::constant(number)))
            }
            IntegerType::I128 => {
                let number = string
                    .parse::<i128>()
                    .map_err(|_| IntegerError::invalid_integer(string, span.to_owned()))?;

                Ok(Integer::I128(Int128::constant(number)))
            }
        }
    }

    pub fn get_bits(&self) -> Vec<Boolean> {
        let integer = self;
        match_integer!(integer => integer.get_bits())
    }

    pub fn get_value(&self) -> Option<String> {
        let integer = self;
        match_integer!(integer => integer.get_value())
    }

    pub fn to_usize(&self, span: &Span) -> Result<usize, IntegerError> {
        let unsigned_integer = self;
        let value_option: Option<String> = match_unsigned_integer!(unsigned_integer => unsigned_integer.get_value());

        let value = value_option.ok_or_else(|| IntegerError::invalid_index(span.to_owned()))?;
        let value_usize = value
            .parse::<usize>()
            .map_err(|_| IntegerError::invalid_integer(value, span.to_owned()))?;
        Ok(value_usize)
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
        integer_type: IntegerType,
        name: &str,
        option: Option<String>,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        Ok(match integer_type {
            IntegerType::U8 => {
                let u8_option = option.map(|s| {
                    s.parse::<u8>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });

                let u8_result = UInt8::alloc(cs.ns(|| format!("`{}: u8` {}:{}", name, span.line, span.start)), || {
                    u8_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(format!("{}: u8", name), span.to_owned()))?;

                Integer::U8(u8_result)
            }
            IntegerType::U16 => {
                let u16_option = option.map(|s| {
                    s.parse::<u16>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let u16_result = UInt16::alloc(
                    cs.ns(|| format!("`{}: u16` {}:{}", name, span.line, span.start)),
                    || u16_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: u16", name), span.to_owned()))?;

                Integer::U16(u16_result)
            }
            IntegerType::U32 => {
                let u32_option = option.map(|s| {
                    s.parse::<u32>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let u32_result = UInt32::alloc(
                    cs.ns(|| format!("`{}: u32` {}:{}", name, span.line, span.start)),
                    || u32_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: u32", name), span.to_owned()))?;

                Integer::U32(u32_result)
            }
            IntegerType::U64 => {
                let u64_option = option.map(|s| {
                    s.parse::<u64>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let u64_result = UInt64::alloc(
                    cs.ns(|| format!("`{}: u64` {}:{}", name, span.line, span.start)),
                    || u64_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: u64", name), span.to_owned()))?;

                Integer::U64(u64_result)
            }
            IntegerType::U128 => {
                let u128_option = option.map(|s| {
                    s.parse::<u128>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let u128_result = UInt128::alloc(
                    cs.ns(|| format!("`{}: u128` {}:{}", name, span.line, span.start)),
                    || u128_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: u128", name), span.to_owned()))?;

                Integer::U128(u128_result)
            }

            IntegerType::I8 => {
                let i8_option = option.map(|s| {
                    s.parse::<i8>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let i8_result = Int8::alloc(cs.ns(|| format!("`{}: i8` {}:{}", name, span.line, span.start)), || {
                    i8_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(format!("{}: i8", name), span.to_owned()))?;

                Integer::I8(i8_result)
            }
            IntegerType::I16 => {
                let i16_option = option.map(|s| {
                    s.parse::<i16>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let i16_result = Int16::alloc(
                    cs.ns(|| format!("`{}: i16` {}:{}", name, span.line, span.start)),
                    || i16_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: i16", name), span.to_owned()))?;

                Integer::I16(i16_result)
            }
            IntegerType::I32 => {
                let i32_option = option.map(|s| {
                    s.parse::<i32>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let i32_result = Int32::alloc(
                    cs.ns(|| format!("`{}: i32` {}:{}", name, span.line, span.start)),
                    || i32_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: i32", name), span.to_owned()))?;

                Integer::I32(i32_result)
            }
            IntegerType::I64 => {
                let i64_option = option.map(|s| {
                    s.parse::<i64>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let i64_result = Int64::alloc(
                    cs.ns(|| format!("`{}: i64` {}:{}", name, span.line, span.start)),
                    || i64_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: i64", name), span.to_owned()))?;

                Integer::I64(i64_result)
            }
            IntegerType::I128 => {
                let i128_option = option.map(|s| {
                    s.parse::<i128>()
                        .map_err(|_| IntegerError::invalid_integer(s, span.to_owned()))
                        .unwrap()
                });
                let i128_result = Int128::alloc(
                    cs.ns(|| format!("`{}: i128` {}:{}", name, span.line, span.start)),
                    || i128_option.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(|_| IntegerError::missing_integer(format!("{}: i128", name), span.to_owned()))?;

                Integer::I128(i128_result)
            }
        })
    }

    pub fn from_input<F: Field, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        integer_type: IntegerType,
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
                    return Err(IntegerError::invalid_integer(input.to_string(), span.to_owned()));
                }
            }
            None => None,
        };

        Self::allocate_type(cs, integer_type, name, option, span)
    }

    pub fn negate<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce -{} {}:{}", self, span.line, span.start);

        let a = self;

        let result = match_signed_integer!(a, span => a.neg(cs.ns(|| unique_namespace)));

        result.ok_or_else(|| IntegerError::negate_operation(span.to_owned()))
    }

    pub fn add<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} + {} {}:{}", self, other, span.line, span.start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.add(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("+".to_string(), span.to_owned()))
    }

    pub fn sub<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} - {} {}:{}", self, other, span.line, span.start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.sub(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("-".to_string(), span.to_owned()))
    }

    pub fn mul<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} * {} {}:{}", self, other, span.line, span.start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.mul(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("*".to_string(), span.to_owned()))
    }

    pub fn div<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} ÷ {} {}:{}", self, other, span.line, span.start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.div(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("÷".to_string(), span.to_owned()))
    }

    pub fn pow<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: &Span,
    ) -> Result<Self, IntegerError> {
        let unique_namespace = format!("enforce {} ** {} {}:{}", self, other, span.line, span.start);

        let a = self;
        let b = other;

        let result = match_integers_span!((a, b), span => a.pow(cs.ns(|| unique_namespace), &b));

        result.ok_or_else(|| IntegerError::binary_operation("**".to_string(), span.to_owned()))
    }
}

impl<F: Field + PrimeField> EvaluateEqGadget<F> for Integer {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        let a = self;
        let b = other;

        let result = match_integers!((a, b) => a.evaluate_equal(cs, b));

        result.ok_or(SynthesisError::Unsatisfiable)
    }
}

impl<F: Field + PrimeField> EvaluateLtGadget<F> for Integer {
    fn less_than<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        let a = self;
        let b = other;
        let result = match_integers!((a, b) => a.less_than(cs, b));

        result.ok_or(SynthesisError::Unsatisfiable)
    }
}

impl<F: Field + PrimeField> ComparatorGadget<F> for Integer {}

impl<F: Field + PrimeField> EqGadget<F> for Integer {}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for Integer {
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

impl<F: Field + PrimeField> CondSelectGadget<F> for Integer {
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
