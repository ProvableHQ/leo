//! Conversion of integer declarations to constraints in Leo.

use crate::{errors::IntegerError, InputValue, IntegerType, Span};
use leo_ast::{types::IntegerType as AstIntegerType, values::NumberValue};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::{ConditionalEqGadget, EqGadget},
            select::CondSelectGadget,
            uint::{UInt, UInt128, UInt16, UInt32, UInt64, UInt8},
        },
    },
};

use snarkos_models::gadgets::utilities::eq::EvaluateEqGadget;
use std::fmt;

/// An integer type enum wrapping the integer value. Used only in expressions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Integer {
    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),
}

impl<'ast> Integer {
    pub fn from(number: NumberValue<'ast>, _type: AstIntegerType) -> Self {
        match _type {
            AstIntegerType::U8Type(_u8) => {
                Integer::U8(UInt8::constant(number.value.parse::<u8>().expect("unable to parse u8")))
            }
            AstIntegerType::U16Type(_u16) => Integer::U16(UInt16::constant(
                number.value.parse::<u16>().expect("unable to parse u16"),
            )),
            AstIntegerType::U32Type(_u32) => Integer::U32(UInt32::constant(
                number.value.parse::<u32>().expect("unable to parse integers.u32"),
            )),
            AstIntegerType::U64Type(_u64) => Integer::U64(UInt64::constant(
                number.value.parse::<u64>().expect("unable to parse u64"),
            )),
            AstIntegerType::U128Type(_u128) => Integer::U128(UInt128::constant(
                number.value.parse::<u128>().expect("unable to parse u128"),
            )),
        }
    }

    pub fn from_implicit(number: String) -> Self {
        Integer::U128(UInt128::constant(number.parse::<u128>().expect("unable to parse u128")))
    }
}

impl Integer {
    pub fn get_value(&self) -> Option<u128> {
        match self {
            Integer::U8(u8) => u8.value.map(|v| v as u128),
            Integer::U16(u16) => u16.value.map(|v| v as u128),
            Integer::U32(u32) => u32.value.map(|v| v as u128),
            Integer::U64(u64) => u64.value.map(|v| v as u128),
            Integer::U128(u128) => u128.value.map(|v| v as u128),
        }
    }

    pub fn to_usize(&self) -> usize {
        self.get_value().unwrap() as usize
    }

    pub fn get_type(&self) -> IntegerType {
        match self {
            Integer::U8(_u8) => IntegerType::U8,
            Integer::U16(_u16) => IntegerType::U16,
            Integer::U32(_u32) => IntegerType::U32,
            Integer::U64(_u64) => IntegerType::U64,
            Integer::U128(_u128) => IntegerType::U128,
        }
    }

    pub fn allocate_type<F: Field, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        integer_type: IntegerType,
        name: String,
        option: Option<u128>,
        span: Span,
    ) -> Result<Self, IntegerError> {
        Ok(match integer_type {
            IntegerType::U8 => {
                let u8_name = format!("{}: u8", name);
                let u8_name_unique = format!("`{}` {}:{}", u8_name, span.line, span.start);
                let u8_option = option.map(|integer| integer as u8);
                let u8_result = UInt8::alloc(cs.ns(|| u8_name_unique), || {
                    u8_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(u8_name, span))?;

                Integer::U8(u8_result)
            }
            IntegerType::U16 => {
                let u16_name = format!("{}: u16", name);
                let u16_name_unique = format!("`{}` {}:{}", u16_name, span.line, span.start);
                let u16_option = option.map(|integer| integer as u16);
                let u16_result = UInt16::alloc(cs.ns(|| u16_name_unique), || {
                    u16_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(u16_name, span))?;

                Integer::U16(u16_result)
            }
            IntegerType::U32 => {
                let u32_name = format!("{}: u32", name);
                let u32_name_unique = format!("`{}` {}:{}", u32_name, span.line, span.start);
                let u32_option = option.map(|integer| integer as u32);
                let u32_result = UInt32::alloc(cs.ns(|| u32_name_unique), || {
                    u32_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(u32_name, span))?;

                Integer::U32(u32_result)
            }
            IntegerType::U64 => {
                let u64_name = format!("{}: u64", name);
                let u64_name_unique = format!("`{}` {}:{}", u64_name, span.line, span.start);
                let u64_option = option.map(|integer| integer as u64);
                let u64_result = UInt64::alloc(cs.ns(|| u64_name_unique), || {
                    u64_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(u64_name, span))?;

                Integer::U64(u64_result)
            }
            IntegerType::U128 => {
                let u128_name = format!("{}: u128", name);
                let u128_name_unique = format!("`{}` {}:{}", u128_name, span.line, span.start);
                let u128_option = option.map(|integer| integer as u128);
                let u128_result = UInt128::alloc(cs.ns(|| u128_name_unique), || {
                    u128_option.ok_or(SynthesisError::AssignmentMissing)
                })
                .map_err(|_| IntegerError::missing_integer(u128_name, span))?;

                Integer::U128(u128_result)
            }
        })
    }

    pub fn from_input<F: Field, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        integer_type: IntegerType,
        name: String,
        integer_value: Option<InputValue>,
        span: Span,
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

    pub fn add<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: Span,
    ) -> Result<Self, IntegerError> {
        Ok(match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                let unique_namespace = format!(
                    "enforce {} + {} {}:{}",
                    left_u8.value.unwrap(),
                    right_u8.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = UInt8::addmany(cs.ns(|| unique_namespace), &[left_u8, right_u8])
                    .map_err(|e| IntegerError::cannot_enforce(format!("+"), e, span))?;

                Integer::U8(result)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                let unique_namespace = format!(
                    "enforce {} + {} {}:{}",
                    left_u16.value.unwrap(),
                    right_u16.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = UInt16::addmany(cs.ns(|| unique_namespace), &[left_u16, right_u16])
                    .map_err(|e| IntegerError::cannot_enforce(format!("+"), e, span))?;

                Integer::U16(result)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                let unique_namespace = format!(
                    "enforce {} + {} {}:{}",
                    left_u32.value.unwrap(),
                    right_u32.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = UInt32::addmany(cs.ns(|| unique_namespace), &[left_u32, right_u32])
                    .map_err(|e| IntegerError::cannot_enforce(format!("+"), e, span))?;

                Integer::U32(result)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                let unique_namespace = format!(
                    "enforce {} + {} {}:{}",
                    left_u64.value.unwrap(),
                    right_u64.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = UInt64::addmany(cs.ns(|| unique_namespace), &[left_u64, right_u64])
                    .map_err(|e| IntegerError::cannot_enforce(format!("+"), e, span))?;

                Integer::U64(result)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                let unique_namespace = format!(
                    "enforce {} + {} {}:{}",
                    left_u128.value.unwrap(),
                    right_u128.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = UInt128::addmany(cs.ns(|| unique_namespace), &[left_u128, right_u128])
                    .map_err(|e| IntegerError::cannot_enforce(format!("+"), e, span))?;

                Integer::U128(result)
            }
            (_, _) => return Err(IntegerError::cannot_evaluate(format!("+"), span)),
        })
    }

    pub fn sub<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: Span,
    ) -> Result<Self, IntegerError> {
        Ok(match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                let unique_namespace = format!(
                    "enforce {} - {} {}:{}",
                    left_u8.value.unwrap(),
                    right_u8.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u8
                    .sub(cs.ns(|| unique_namespace), &right_u8)
                    .map_err(|e| IntegerError::cannot_enforce(format!("-"), e, span))?;

                Integer::U8(result)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                let unique_namespace = format!(
                    "enforce {} - {} {}:{}",
                    left_u16.value.unwrap(),
                    right_u16.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u16
                    .sub(cs.ns(|| unique_namespace), &right_u16)
                    .map_err(|e| IntegerError::cannot_enforce(format!("-"), e, span))?;

                Integer::U16(result)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                let unique_namespace = format!(
                    "enforce {} - {} {}:{}",
                    left_u32.value.unwrap(),
                    right_u32.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u32
                    .sub(cs.ns(|| unique_namespace), &right_u32)
                    .map_err(|e| IntegerError::cannot_enforce(format!("-"), e, span))?;

                Integer::U32(result)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                let unique_namespace = format!(
                    "enforce {} - {} {}:{}",
                    left_u64.value.unwrap(),
                    right_u64.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u64
                    .sub(cs.ns(|| unique_namespace), &right_u64)
                    .map_err(|e| IntegerError::cannot_enforce(format!("-"), e, span))?;

                Integer::U64(result)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                let unique_namespace = format!(
                    "enforce {} - {} {}:{}",
                    left_u128.value.unwrap(),
                    right_u128.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u128
                    .sub(cs.ns(|| unique_namespace), &right_u128)
                    .map_err(|e| IntegerError::cannot_enforce(format!("-"), e, span))?;

                Integer::U128(result)
            }
            (_, _) => return Err(IntegerError::cannot_evaluate(format!("-"), span)),
        })
    }

    pub fn mul<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: Span,
    ) -> Result<Self, IntegerError> {
        Ok(match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                let unique_namespace = format!(
                    "enforce {} * {} {}:{}",
                    left_u8.value.unwrap(),
                    right_u8.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u8
                    .mul(cs.ns(|| unique_namespace), &right_u8)
                    .map_err(|e| IntegerError::cannot_enforce(format!("*"), e, span))?;

                Integer::U8(result)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                let unique_namespace = format!(
                    "enforce {} * {} {}:{}",
                    left_u16.value.unwrap(),
                    right_u16.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u16
                    .mul(cs.ns(|| unique_namespace), &right_u16)
                    .map_err(|e| IntegerError::cannot_enforce(format!("*"), e, span))?;

                Integer::U16(result)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                let unique_namespace = format!(
                    "enforce {} * {} {}:{}",
                    left_u32.value.unwrap(),
                    right_u32.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u32
                    .mul(cs.ns(|| unique_namespace), &right_u32)
                    .map_err(|e| IntegerError::cannot_enforce(format!("*"), e, span))?;

                Integer::U32(result)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                let unique_namespace = format!(
                    "enforce {} * {} {}:{}",
                    left_u64.value.unwrap(),
                    right_u64.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u64
                    .mul(cs.ns(|| unique_namespace), &right_u64)
                    .map_err(|e| IntegerError::cannot_enforce(format!("*"), e, span))?;

                Integer::U64(result)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                let unique_namespace = format!(
                    "enforce {} * {} {}:{}",
                    left_u128.value.unwrap(),
                    right_u128.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u128
                    .mul(cs.ns(|| unique_namespace), &right_u128)
                    .map_err(|e| IntegerError::cannot_enforce(format!("*"), e, span))?;

                Integer::U128(result)
            }
            (_, _) => return Err(IntegerError::cannot_evaluate(format!("*"), span)),
        })
    }

    pub fn div<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: Span,
    ) -> Result<Self, IntegerError> {
        Ok(match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                let unique_namespace = format!(
                    "enforce {} ÷ {} {}:{}",
                    left_u8.value.unwrap(),
                    right_u8.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u8
                    .div(cs.ns(|| unique_namespace), &right_u8)
                    .map_err(|e| IntegerError::cannot_enforce(format!("÷"), e, span))?;

                Integer::U8(result)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                let unique_namespace = format!(
                    "enforce {} ÷ {} {}:{}",
                    left_u16.value.unwrap(),
                    right_u16.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u16
                    .div(cs.ns(|| unique_namespace), &right_u16)
                    .map_err(|e| IntegerError::cannot_enforce(format!("÷"), e, span))?;

                Integer::U16(result)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                let unique_namespace = format!(
                    "enforce {} ÷ {} {}:{}",
                    left_u32.value.unwrap(),
                    right_u32.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u32
                    .div(cs.ns(|| unique_namespace), &right_u32)
                    .map_err(|e| IntegerError::cannot_enforce(format!("÷"), e, span))?;

                Integer::U32(result)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                let unique_namespace = format!(
                    "enforce {} ÷ {} {}:{}",
                    left_u64.value.unwrap(),
                    right_u64.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u64
                    .div(cs.ns(|| unique_namespace), &right_u64)
                    .map_err(|e| IntegerError::cannot_enforce(format!("÷"), e, span))?;

                Integer::U64(result)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                let unique_namespace = format!(
                    "enforce {} ÷ {} {}:{}",
                    left_u128.value.unwrap(),
                    right_u128.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u128
                    .div(cs.ns(|| unique_namespace), &right_u128)
                    .map_err(|e| IntegerError::cannot_enforce(format!("÷"), e, span))?;

                Integer::U128(result)
            }
            (_, _) => return Err(IntegerError::cannot_evaluate(format!("÷"), span)),
        })
    }

    pub fn pow<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: Self,
        span: Span,
    ) -> Result<Self, IntegerError> {
        Ok(match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                let unique_namespace = format!(
                    "enforce {} ** {} {}:{}",
                    left_u8.value.unwrap(),
                    right_u8.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u8
                    .pow(cs.ns(|| unique_namespace), &right_u8)
                    .map_err(|e| IntegerError::cannot_enforce(format!("**"), e, span))?;

                Integer::U8(result)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                let unique_namespace = format!(
                    "enforce {} ** {} {}:{}",
                    left_u16.value.unwrap(),
                    right_u16.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u16
                    .pow(cs.ns(|| unique_namespace), &right_u16)
                    .map_err(|e| IntegerError::cannot_enforce(format!("**"), e, span))?;

                Integer::U16(result)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                let unique_namespace = format!(
                    "enforce {} ** {} {}:{}",
                    left_u32.value.unwrap(),
                    right_u32.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u32
                    .pow(cs.ns(|| unique_namespace), &right_u32)
                    .map_err(|e| IntegerError::cannot_enforce(format!("**"), e, span))?;

                Integer::U32(result)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                let unique_namespace = format!(
                    "enforce {} ** {} {}:{}",
                    left_u64.value.unwrap(),
                    right_u64.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u64
                    .pow(cs.ns(|| unique_namespace), &right_u64)
                    .map_err(|e| IntegerError::cannot_enforce(format!("**"), e, span))?;

                Integer::U64(result)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                let unique_namespace = format!(
                    "enforce {} ** {} {}:{}",
                    left_u128.value.unwrap(),
                    right_u128.value.unwrap(),
                    span.line,
                    span.start
                );
                let result = left_u128
                    .pow(cs.ns(|| unique_namespace), &right_u128)
                    .map_err(|e| IntegerError::cannot_enforce(format!("**"), e, span))?;

                Integer::U128(result)
            }
            (_, _) => return Err(IntegerError::cannot_evaluate(format!("**"), span)),
        })
    }
}

impl<F: Field + PrimeField> EvaluateEqGadget<F> for Integer {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => left_u8.evaluate_equal(cs, right_u8),
            (Integer::U16(left_u16), Integer::U16(right_u16)) => left_u16.evaluate_equal(cs, right_u16),
            (Integer::U32(left_u32), Integer::U32(right_u32)) => left_u32.evaluate_equal(cs, right_u32),
            (Integer::U64(left_u64), Integer::U64(right_u64)) => left_u64.evaluate_equal(cs, right_u64),
            (Integer::U128(left_u128), Integer::U128(right_u128)) => left_u128.evaluate_equal(cs, right_u128),
            (_, _) => Err(SynthesisError::AssignmentMissing),
        }
    }
}

impl<F: Field + PrimeField> EqGadget<F> for Integer {}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for Integer {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => left_u8.conditional_enforce_equal(cs, right_u8, condition),
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                left_u16.conditional_enforce_equal(cs, right_u16, condition)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                left_u32.conditional_enforce_equal(cs, right_u32, condition)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                left_u64.conditional_enforce_equal(cs, right_u64, condition)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                left_u128.conditional_enforce_equal(cs, right_u128, condition)
            }
            (_, _) => Err(SynthesisError::AssignmentMissing),
        }
    }

    fn cost() -> usize {
        <UInt128 as ConditionalEqGadget<F>>::cost()
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
            (Integer::U8(u8_first), Integer::U8(u8_second)) => {
                Ok(Integer::U8(UInt8::conditionally_select(cs, cond, u8_first, u8_second)?))
            }
            (Integer::U16(u16_first), Integer::U16(u18_second)) => Ok(Integer::U16(UInt16::conditionally_select(
                cs, cond, u16_first, u18_second,
            )?)),
            (Integer::U32(u32_first), Integer::U32(u32_second)) => Ok(Integer::U32(UInt32::conditionally_select(
                cs, cond, u32_first, u32_second,
            )?)),
            (Integer::U64(u64_first), Integer::U64(u64_second)) => Ok(Integer::U64(UInt64::conditionally_select(
                cs, cond, u64_first, u64_second,
            )?)),
            (Integer::U128(u128_first), Integer::U128(u128_second)) => Ok(Integer::U128(
                UInt128::conditionally_select(cs, cond, u128_first, u128_second)?,
            )),
            (_, _) => Err(SynthesisError::Unsatisfiable), // types do not match
        }
    }

    fn cost() -> usize {
        unimplemented!("Cannot calculate cost.")
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let option = match self {
            Integer::U8(u8) => u8.value.map(|num| num as usize),
            Integer::U16(u16) => u16.value.map(|num| num as usize),
            Integer::U32(u32) => u32.value.map(|num| num as usize),
            Integer::U64(u64) => u64.value.map(|num| num as usize),
            Integer::U128(u128) => u128.value.map(|num| num as usize),
        };
        match option {
            Some(number) => write!(f, "{}{}", number, self.get_type()),
            None => write!(f, "[input]{}", self.get_type()),
        }
    }
}
