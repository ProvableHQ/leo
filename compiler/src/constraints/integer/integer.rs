//! Methods to enforce constraints on integers in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    types::{Integer, ParameterModel, ParameterValue, Type, Variable},
    IntegerType,
};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            boolean::Boolean, uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64,
            uint8::UInt8,
        },
    },
};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum ConstrainedInteger {
    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),
}

impl ConstrainedInteger {
    pub(crate) fn get_value(&self) -> usize {
        match self {
            ConstrainedInteger::U8(u8) => u8.value.unwrap() as usize,
            ConstrainedInteger::U16(u16) => u16.value.unwrap() as usize,
            ConstrainedInteger::U32(u32) => u32.value.unwrap() as usize,
            ConstrainedInteger::U64(u64) => u64.value.unwrap() as usize,
            ConstrainedInteger::U128(u128) => u128.value.unwrap() as usize,
        }
    }

    pub(crate) fn get_type(&self) -> IntegerType {
        match self {
            ConstrainedInteger::U8(u8) => IntegerType::U8,
            ConstrainedInteger::U16(u16) => IntegerType::U16,
            ConstrainedInteger::U32(u32) => IntegerType::U32,
            ConstrainedInteger::U64(u64) => IntegerType::U64,
            ConstrainedInteger::U128(u128) => IntegerType::U128,
        }
    }

    pub(crate) fn expect_type(&self, integer_type: &IntegerType) {
        match (self, integer_type) {
            (ConstrainedInteger::U8(_u8), IntegerType::U8) => {}
            (ConstrainedInteger::U16(_u16), IntegerType::U16) => {}
            (ConstrainedInteger::U32(_u32), IntegerType::U32) => {}
            (ConstrainedInteger::U64(_u64), IntegerType::U64) => {}
            (ConstrainedInteger::U128(_u128), IntegerType::U128) => {}
            (actual, expected) => {
                unimplemented!("expected integer type {}, got {}", expected, actual)
            }
        }
    }
}

impl fmt::Display for ConstrainedInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstrainedInteger::U8(u8) => write!(f, "{}u8", u8.value.unwrap()),
            ConstrainedInteger::U16(u16) => write!(f, "{}u16", u16.value.unwrap()),
            ConstrainedInteger::U32(u32) => write!(f, "{}u32", u32.value.unwrap()),
            ConstrainedInteger::U64(u64) => write!(f, "{}u64", u64.value.unwrap()),
            ConstrainedInteger::U128(u128) => write!(f, "{}u128", u128.value.unwrap()),
        }
    }
}

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn get_integer_constant(integer: Integer) -> ConstrainedValue<F> {
        ConstrainedValue::Integer(match integer {
            Integer::U8(u8_value) => ConstrainedInteger::U8(UInt8::constant(u8_value)),
            Integer::U16(u16_value) => ConstrainedInteger::U16(UInt16::constant(u16_value)),
            Integer::U32(u32_value) => ConstrainedInteger::U32(UInt32::constant(u32_value)),
            Integer::U64(u64_value) => ConstrainedInteger::U64(UInt64::constant(u64_value)),
            Integer::U128(u128_value) => ConstrainedInteger::U128(UInt128::constant(u128_value)),
        })
    }

    pub(crate) fn evaluate_integer_eq(
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) -> ConstrainedValue<F> {
        ConstrainedValue::Boolean(Boolean::Constant(match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                left_u8.eq(&right_u8)
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                left_u16.eq(&right_u16)
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                left_u32.eq(&right_u32)
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                left_u64.eq(&right_u64)
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                left_u128.eq(&right_u128)
            }
            (left, right) => unimplemented!(
                "cannot evaluate integer equality between {} == {}",
                left,
                right
            ),
        }))
    }

    pub(crate) fn integer_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: ParameterModel<F>,
        parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        let integer_type = match &parameter_model._type {
            Type::IntegerType(integer_type) => integer_type,
            _type => unimplemented!("expected integer parameter, got {}", _type),
        };

        match integer_type {
            IntegerType::U8 => self.u8_from_parameter(cs, scope, parameter_model, parameter_value),
            IntegerType::U16 => {
                self.u16_from_parameter(cs, scope, parameter_model, parameter_value)
            }
            IntegerType::U32 => {
                self.u32_from_parameter(cs, scope, parameter_model, parameter_value)
            }
            IntegerType::U64 => {
                self.u64_from_parameter(cs, scope, parameter_model, parameter_value)
            }
            IntegerType::U128 => {
                self.u128_from_parameter(cs, scope, parameter_model, parameter_value)
            }
        }
    }

    pub(crate) fn integer_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _parameter_model: ParameterModel<F>,
        _parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce integer array as parameter")
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     let number = if parameter.private {
        //         UInt32::alloc(cs.ns(|| name), Some(argument)).unwrap()
        //     } else {
        //         UInt32::alloc_input(cs.ns(|| name), Some(argument)).unwrap()
        //     };
        //
        //     array_value.push(number);
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::U32Array(array_value));
        //
        // parameter_variable
    }

    pub(crate) fn enforce_integer_eq(
        cs: &mut CS,
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) {
        match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                Self::enforce_u8_eq(cs, left_u8, right_u8)
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                Self::enforce_u16_eq(cs, left_u16, right_u16)
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                Self::enforce_u32_eq(cs, left_u32, right_u32)
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                Self::enforce_u64_eq(cs, left_u64, right_u64)
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                Self::enforce_u128_eq(cs, left_u128, right_u128)
            }
            (left, right) => unimplemented!(
                "cannot enforce integer equality between {} == {}",
                left,
                right
            ),
        }
    }

    pub(crate) fn enforce_integer_add(
        cs: &mut CS,
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) -> ConstrainedValue<F> {
        ConstrainedValue::Integer(match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                ConstrainedInteger::U8(Self::enforce_u8_add(cs, left_u8, right_u8))
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                ConstrainedInteger::U16(Self::enforce_u16_add(cs, left_u16, right_u16))
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                ConstrainedInteger::U32(Self::enforce_u32_add(cs, left_u32, right_u32))
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                ConstrainedInteger::U64(Self::enforce_u64_add(cs, left_u64, right_u64))
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                ConstrainedInteger::U128(Self::enforce_u128_add(cs, left_u128, right_u128))
            }
            (left, right) => unimplemented!(
                "cannot enforce integer addition between {} + {}",
                left,
                right
            ),
        })
    }
    pub(crate) fn enforce_integer_sub(
        cs: &mut CS,
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) -> ConstrainedValue<F> {
        ConstrainedValue::Integer(match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                ConstrainedInteger::U8(Self::enforce_u8_sub(cs, left_u8, right_u8))
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                ConstrainedInteger::U16(Self::enforce_u16_sub(cs, left_u16, right_u16))
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                ConstrainedInteger::U32(Self::enforce_u32_sub(cs, left_u32, right_u32))
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                ConstrainedInteger::U64(Self::enforce_u64_sub(cs, left_u64, right_u64))
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                ConstrainedInteger::U128(Self::enforce_u128_sub(cs, left_u128, right_u128))
            }
            (left, right) => unimplemented!(
                "cannot enforce integer subtraction between {} - {}",
                left,
                right
            ),
        })
    }
    pub(crate) fn enforce_integer_mul(
        cs: &mut CS,
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) -> ConstrainedValue<F> {
        ConstrainedValue::Integer(match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                ConstrainedInteger::U8(Self::enforce_u8_mul(cs, left_u8, right_u8))
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                ConstrainedInteger::U16(Self::enforce_u16_mul(cs, left_u16, right_u16))
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                ConstrainedInteger::U32(Self::enforce_u32_mul(cs, left_u32, right_u32))
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                ConstrainedInteger::U64(Self::enforce_u64_mul(cs, left_u64, right_u64))
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                ConstrainedInteger::U128(Self::enforce_u128_mul(cs, left_u128, right_u128))
            }
            (left, right) => unimplemented!(
                "cannot enforce integer multiplication between {} * {}",
                left,
                right
            ),
        })
    }
    pub(crate) fn enforce_integer_div(
        cs: &mut CS,
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) -> ConstrainedValue<F> {
        ConstrainedValue::Integer(match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                ConstrainedInteger::U8(Self::enforce_u8_div(cs, left_u8, right_u8))
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                ConstrainedInteger::U16(Self::enforce_u16_div(cs, left_u16, right_u16))
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                ConstrainedInteger::U32(Self::enforce_u32_div(cs, left_u32, right_u32))
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                ConstrainedInteger::U64(Self::enforce_u64_div(cs, left_u64, right_u64))
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                ConstrainedInteger::U128(Self::enforce_u128_div(cs, left_u128, right_u128))
            }
            (left, right) => unimplemented!(
                "cannot enforce integer division between {} / {}",
                left,
                right
            ),
        })
    }
    pub(crate) fn enforce_integer_pow(
        cs: &mut CS,
        left: ConstrainedInteger,
        right: ConstrainedInteger,
    ) -> ConstrainedValue<F> {
        ConstrainedValue::Integer(match (left, right) {
            (ConstrainedInteger::U8(left_u8), ConstrainedInteger::U8(right_u8)) => {
                ConstrainedInteger::U8(Self::enforce_u8_pow(cs, left_u8, right_u8))
            }
            (ConstrainedInteger::U16(left_u16), ConstrainedInteger::U16(right_u16)) => {
                ConstrainedInteger::U16(Self::enforce_u16_pow(cs, left_u16, right_u16))
            }
            (ConstrainedInteger::U32(left_u32), ConstrainedInteger::U32(right_u32)) => {
                ConstrainedInteger::U32(Self::enforce_u32_pow(cs, left_u32, right_u32))
            }
            (ConstrainedInteger::U64(left_u64), ConstrainedInteger::U64(right_u64)) => {
                ConstrainedInteger::U64(Self::enforce_u64_pow(cs, left_u64, right_u64))
            }
            (ConstrainedInteger::U128(left_u128), ConstrainedInteger::U128(right_u128)) => {
                ConstrainedInteger::U128(Self::enforce_u128_pow(cs, left_u128, right_u128))
            }
            (left, right) => unimplemented!(
                "cannot enforce integer exponentiation between {} ** {}",
                left,
                right
            ),
        })
    }
}
