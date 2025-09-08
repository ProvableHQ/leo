// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{
    CoreFunction,
    Expression,
    halt,
    interpreter_value::{Value, util::ExpectTc},
    tc_fail,
};
use leo_errors::{InterpreterHalt, Result};
use leo_span::{Span, Symbol};

use snarkvm::prelude::{CastLossy as _, Network as _, TestnetV0, ToBits};

use rand::Rng as _;
use rand_chacha::ChaCha20Rng;
use std::collections::HashMap;

/// A context in which we can evaluate core functions.
///
/// This is intended to be implemented by `Cursor`, which will be used during
/// execution of the interpreter, and by `Vec<Value>`, which will be used
/// during compile time evaluation for constant folding.
///
/// The default implementations for `rng`, `set_block_height`, and mapping lookup
/// do nothing, as those features will not be available during compile time
/// evaluation.
pub trait CoreFunctionHelper {
    fn pop_value_impl(&mut self) -> Option<Value>;

    fn pop_value(&mut self) -> Result<Value> {
        match self.pop_value_impl() {
            Some(v) => Ok(v),
            None => {
                Err(InterpreterHalt::new("value expected - this may be a bug in the Leo interpreter".to_string())
                    .into())
            }
        }
    }

    fn set_block_height(&mut self, _height: u32) {}

    fn lookup_mapping(&self, _program: Option<Symbol>, _name: Symbol) -> Option<&HashMap<Value, Value>> {
        None
    }

    fn lookup_mapping_mut(&mut self, _program: Option<Symbol>, _name: Symbol) -> Option<&mut HashMap<Value, Value>> {
        None
    }

    fn rng(&mut self) -> Option<&mut ChaCha20Rng> {
        None
    }
}

impl CoreFunctionHelper for Vec<Value> {
    fn pop_value_impl(&mut self) -> Option<Value> {
        self.pop()
    }
}

pub fn evaluate_core_function(
    helper: &mut dyn CoreFunctionHelper,
    core_function: CoreFunction,
    arguments: &[Expression],
    span: Span,
) -> Result<Option<Value>> {
    macro_rules! apply {
        ($func: expr, $value: ident, $to: ident) => {{
            let v = helper.pop_value()?;
            let bits = v.$to();
            Value::$value($func(&bits).expect_tc(span)?)
        }};
    }

    macro_rules! apply_cast {
        ($func: expr, $value: ident, $to: ident) => {{
            let v = helper.pop_value()?;
            let bits = v.$to();
            let group = $func(&bits).expect_tc(span)?;
            let x = group.to_x_coordinate();
            Value::$value(x.cast_lossy())
        }};
    }

    macro_rules! apply_cast_int {
        ($func: expr, $value: ident, $int_ty: ident, $to: ident) => {{
            let v = helper.pop_value()?;
            let bits = v.$to();
            let group = $func(&bits).expect_tc(span)?;
            let x = group.to_x_coordinate();
            let bits = x.to_bits_le();
            let mut result: $int_ty = 0;
            for bit in 0..std::cmp::min($int_ty::BITS as usize, bits.len()) {
                let setbit = (if bits[bit] { 1 } else { 0 }) << bit;
                result |= setbit;
            }
            Value::$value(result)
        }};
    }

    macro_rules! apply_cast2 {
        ($func: expr, $value: ident) => {{
            let Value::Scalar(randomizer) = helper.pop_value()? else {
                tc_fail!();
            };
            let v = helper.pop_value()?;
            let bits = v.to_bits_le();
            let group = $func(&bits, &randomizer).expect_tc(span)?;
            let x = group.to_x_coordinate();
            Value::$value(x.cast_lossy())
        }};
    }

    macro_rules! maybe_gen {
        () => {
            if let Some(rng) = helper.rng() {
                rng.r#gen()
            } else {
                return Ok(None);
            }
        };
    }

    let value = match core_function {
        CoreFunction::BHP256CommitToAddress => {
            apply_cast2!(TestnetV0::commit_to_group_bhp256, Address)
        }
        CoreFunction::BHP256CommitToField => {
            apply_cast2!(TestnetV0::commit_to_group_bhp256, Field)
        }
        CoreFunction::BHP256CommitToGroup => {
            apply_cast2!(TestnetV0::commit_to_group_bhp256, Group)
        }
        CoreFunction::BHP256HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_bhp256, Address, to_bits_le)
        }
        CoreFunction::BHP256HashToField => apply!(TestnetV0::hash_bhp256, Field, to_bits_le),
        CoreFunction::BHP256HashToGroup => apply!(TestnetV0::hash_to_group_bhp256, Group, to_bits_le),
        CoreFunction::BHP256HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, I8, i8, to_bits_le)
        }
        CoreFunction::BHP256HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, I16, i16, to_bits_le)
        }
        CoreFunction::BHP256HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, I32, i32, to_bits_le)
        }
        CoreFunction::BHP256HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, I64, i64, to_bits_le)
        }
        CoreFunction::BHP256HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, I128, i128, to_bits_le)
        }
        CoreFunction::BHP256HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, U8, u8, to_bits_le)
        }
        CoreFunction::BHP256HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, U16, u16, to_bits_le)
        }
        CoreFunction::BHP256HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, U32, u32, to_bits_le)
        }
        CoreFunction::BHP256HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, U64, u64, to_bits_le)
        }
        CoreFunction::BHP256HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp256, U128, u128, to_bits_le)
        }
        CoreFunction::BHP256HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_bhp256, Scalar, to_bits_le)
        }
        CoreFunction::BHP512CommitToAddress => {
            apply_cast2!(TestnetV0::commit_to_group_bhp512, Address)
        }
        CoreFunction::BHP512CommitToField => {
            apply_cast2!(TestnetV0::commit_to_group_bhp512, Field)
        }
        CoreFunction::BHP512CommitToGroup => {
            apply_cast2!(TestnetV0::commit_to_group_bhp512, Group)
        }
        CoreFunction::BHP512HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_bhp512, Address, to_bits_le)
        }
        CoreFunction::BHP512HashToField => apply!(TestnetV0::hash_bhp512, Field, to_bits_le),
        CoreFunction::BHP512HashToGroup => apply!(TestnetV0::hash_to_group_bhp512, Group, to_bits_le),
        CoreFunction::BHP512HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, I8, i8, to_bits_le)
        }
        CoreFunction::BHP512HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, I16, i16, to_bits_le)
        }
        CoreFunction::BHP512HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, I32, i32, to_bits_le)
        }
        CoreFunction::BHP512HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, I64, i64, to_bits_le)
        }
        CoreFunction::BHP512HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, I128, i128, to_bits_le)
        }
        CoreFunction::BHP512HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, U8, u8, to_bits_le)
        }
        CoreFunction::BHP512HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, U16, u16, to_bits_le)
        }
        CoreFunction::BHP512HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, U32, u32, to_bits_le)
        }
        CoreFunction::BHP512HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, U64, u64, to_bits_le)
        }
        CoreFunction::BHP512HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp512, U128, u128, to_bits_le)
        }
        CoreFunction::BHP512HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_bhp512, Scalar, to_bits_le)
        }
        CoreFunction::BHP768CommitToAddress => {
            apply_cast2!(TestnetV0::commit_to_group_bhp768, Address)
        }
        CoreFunction::BHP768CommitToField => {
            apply_cast2!(TestnetV0::commit_to_group_bhp768, Field)
        }
        CoreFunction::BHP768CommitToGroup => {
            apply_cast2!(TestnetV0::commit_to_group_bhp768, Group)
        }
        CoreFunction::BHP768HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_bhp768, Address, to_bits_le)
        }
        CoreFunction::BHP768HashToField => apply!(TestnetV0::hash_bhp768, Field, to_bits_le),
        CoreFunction::BHP768HashToGroup => apply!(TestnetV0::hash_to_group_bhp768, Group, to_bits_le),
        CoreFunction::BHP768HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, I8, i8, to_bits_le)
        }
        CoreFunction::BHP768HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, I16, i16, to_bits_le)
        }
        CoreFunction::BHP768HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, I32, i32, to_bits_le)
        }
        CoreFunction::BHP768HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, I64, i64, to_bits_le)
        }
        CoreFunction::BHP768HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, I128, i128, to_bits_le)
        }
        CoreFunction::BHP768HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, U8, u8, to_bits_le)
        }
        CoreFunction::BHP768HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, U16, u16, to_bits_le)
        }
        CoreFunction::BHP768HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, U32, u32, to_bits_le)
        }
        CoreFunction::BHP768HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, U64, u64, to_bits_le)
        }
        CoreFunction::BHP768HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp768, U128, u128, to_bits_le)
        }
        CoreFunction::BHP768HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_bhp768, Scalar, to_bits_le)
        }
        CoreFunction::BHP1024CommitToAddress => {
            apply_cast2!(TestnetV0::commit_to_group_bhp1024, Address)
        }
        CoreFunction::BHP1024CommitToField => {
            apply_cast2!(TestnetV0::commit_to_group_bhp1024, Field)
        }
        CoreFunction::BHP1024CommitToGroup => {
            apply_cast2!(TestnetV0::commit_to_group_bhp1024, Group)
        }
        CoreFunction::BHP1024HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_bhp1024, Address, to_bits_le)
        }
        CoreFunction::BHP1024HashToField => apply!(TestnetV0::hash_bhp1024, Field, to_bits_le),
        CoreFunction::BHP1024HashToGroup => apply!(TestnetV0::hash_to_group_bhp1024, Group, to_bits_le),
        CoreFunction::BHP1024HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I8, i8, to_bits_le)
        }
        CoreFunction::BHP1024HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I16, i16, to_bits_le)
        }
        CoreFunction::BHP1024HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I32, i32, to_bits_le)
        }
        CoreFunction::BHP1024HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I64, i64, to_bits_le)
        }
        CoreFunction::BHP1024HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I128, i128, to_bits_le)
        }
        CoreFunction::BHP1024HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U8, u8, to_bits_le)
        }
        CoreFunction::BHP1024HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U16, u16, to_bits_le)
        }
        CoreFunction::BHP1024HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U32, u32, to_bits_le)
        }
        CoreFunction::BHP1024HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U64, u64, to_bits_le)
        }
        CoreFunction::BHP1024HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U128, u128, to_bits_le)
        }
        CoreFunction::BHP1024HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_bhp1024, Scalar, to_bits_le)
        }
        CoreFunction::Keccak256HashToAddress => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            Address,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToField => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            Field,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToGroup => {
            apply!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                Group,
                to_bits_le
            )
        }
        CoreFunction::Keccak256HashToI8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            I8,
            i8,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToI16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            I16,
            i16,
            to_bits_le
        ),

        CoreFunction::Keccak256HashToI32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            I32,
            i32,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToI64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            I64,
            i64,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToI128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            I128,
            i128,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToU8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            U8,
            u8,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToU16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            U16,
            u16,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToU32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            U32,
            u32,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToU64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            U64,
            u64,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToU128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            U128,
            u128,
            to_bits_le
        ),
        CoreFunction::Keccak256HashToScalar => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
            Scalar,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToAddress => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            Address,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToField => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            Field,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToGroup => {
            apply!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                Group,
                to_bits_le
            )
        }
        CoreFunction::Keccak384HashToI8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            I8,
            i8,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToI16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            I16,
            i16,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToI32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            I32,
            i32,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToI64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            I64,
            i64,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToI128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            I128,
            i128,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToU8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            U8,
            u8,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToU16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            U16,
            u16,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToU32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            U32,
            u32,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToU64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            U64,
            u64,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToU128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            U128,
            u128,
            to_bits_le
        ),
        CoreFunction::Keccak384HashToScalar => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
            Scalar,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToAddress => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            Address,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToField => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            Field,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToGroup => {
            apply!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                Group,
                to_bits_le
            )
        }
        CoreFunction::Keccak512HashToI8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            I8,
            i8,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToI16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            I16,
            i16,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToI32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            I32,
            i32,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToI64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            I64,
            i64,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToI128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            I128,
            i128,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToU8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            U8,
            u8,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToU16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            U16,
            u16,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToU32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            U32,
            u32,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToU64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            U64,
            u64,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToU128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            U128,
            u128,
            to_bits_le
        ),
        CoreFunction::Keccak512HashToScalar => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
            Scalar,
            to_bits_le
        ),
        CoreFunction::Pedersen64CommitToAddress => {
            apply_cast2!(TestnetV0::commit_to_group_ped64, Address)
        }
        CoreFunction::Pedersen64CommitToField => {
            apply_cast2!(TestnetV0::commit_to_group_ped64, Field)
        }
        CoreFunction::Pedersen64CommitToGroup => {
            apply_cast2!(TestnetV0::commit_to_group_ped64, Group)
        }
        CoreFunction::Pedersen64HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_ped64, Address, to_bits_le)
        }
        CoreFunction::Pedersen64HashToField => apply!(TestnetV0::hash_ped64, Field, to_bits_le),
        CoreFunction::Pedersen64HashToGroup => apply!(TestnetV0::hash_to_group_ped64, Group, to_bits_le),
        CoreFunction::Pedersen64HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I8, i8, to_bits_le)
        }
        CoreFunction::Pedersen64HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I16, i16, to_bits_le)
        }
        CoreFunction::Pedersen64HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I32, i32, to_bits_le)
        }
        CoreFunction::Pedersen64HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I64, i64, to_bits_le)
        }
        CoreFunction::Pedersen64HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I128, i128, to_bits_le)
        }
        CoreFunction::Pedersen64HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U8, u8, to_bits_le)
        }
        CoreFunction::Pedersen64HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U16, u16, to_bits_le)
        }
        CoreFunction::Pedersen64HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U32, u32, to_bits_le)
        }
        CoreFunction::Pedersen64HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U64, u64, to_bits_le)
        }
        CoreFunction::Pedersen64HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U128, u128, to_bits_le)
        }
        CoreFunction::Pedersen64HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_ped64, Scalar, to_bits_le)
        }
        CoreFunction::Pedersen128HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_ped128, Address, to_bits_le)
        }
        CoreFunction::Pedersen128HashToField => {
            apply_cast!(TestnetV0::hash_to_group_ped128, Field, to_bits_le)
        }
        CoreFunction::Pedersen128HashToGroup => {
            apply_cast!(TestnetV0::hash_to_group_ped128, Group, to_bits_le)
        }
        CoreFunction::Pedersen128HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped128, I8, i8, to_bits_le)
        }
        CoreFunction::Pedersen128HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I16, i16, to_bits_le)
        }
        CoreFunction::Pedersen128HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped128, I32, i32, to_bits_le)
        }
        CoreFunction::Pedersen128HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, I64, i64, to_bits_le)
        }
        CoreFunction::Pedersen128HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped128, I128, i128, to_bits_le)
        }
        CoreFunction::Pedersen128HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped128, U8, u8, to_bits_le)
        }
        CoreFunction::Pedersen128HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U16, u16, to_bits_le)
        }
        CoreFunction::Pedersen128HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped128, U32, u32, to_bits_le)
        }
        CoreFunction::Pedersen128HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped64, U64, u64, to_bits_le)
        }
        CoreFunction::Pedersen128HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_ped128, U128, u128, to_bits_le)
        }
        CoreFunction::Pedersen128HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_ped128, Scalar, to_bits_le)
        }
        CoreFunction::Pedersen128CommitToAddress => {
            apply_cast2!(TestnetV0::commit_to_group_ped128, Address)
        }
        CoreFunction::Pedersen128CommitToField => {
            apply_cast2!(TestnetV0::commit_to_group_ped128, Field)
        }
        CoreFunction::Pedersen128CommitToGroup => {
            apply_cast2!(TestnetV0::commit_to_group_ped128, Group)
        }
        CoreFunction::Poseidon2HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_psd2, Address, to_fields)
        }
        CoreFunction::Poseidon2HashToField => {
            apply!(TestnetV0::hash_psd2, Field, to_fields)
        }
        CoreFunction::Poseidon2HashToGroup => {
            apply_cast!(TestnetV0::hash_to_group_psd2, Group, to_fields)
        }
        CoreFunction::Poseidon2HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, I8, i8, to_fields)
        }
        CoreFunction::Poseidon2HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, I16, i16, to_fields)
        }
        CoreFunction::Poseidon2HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, I32, i32, to_fields)
        }
        CoreFunction::Poseidon2HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, I64, i64, to_fields)
        }
        CoreFunction::Poseidon2HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, I128, i128, to_fields)
        }
        CoreFunction::Poseidon2HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, U8, u8, to_fields)
        }
        CoreFunction::Poseidon2HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, U16, u16, to_fields)
        }
        CoreFunction::Poseidon2HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, U32, u32, to_fields)
        }
        CoreFunction::Poseidon2HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, U64, u64, to_fields)
        }
        CoreFunction::Poseidon2HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd2, U128, u128, to_fields)
        }
        CoreFunction::Poseidon2HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_psd4, Scalar, to_fields)
        }
        CoreFunction::Poseidon4HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_psd4, Address, to_fields)
        }
        CoreFunction::Poseidon4HashToField => {
            apply!(TestnetV0::hash_psd4, Field, to_fields)
        }
        CoreFunction::Poseidon4HashToGroup => {
            apply_cast!(TestnetV0::hash_to_group_psd4, Group, to_fields)
        }
        CoreFunction::Poseidon4HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, I8, i8, to_fields)
        }
        CoreFunction::Poseidon4HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, I16, i16, to_fields)
        }
        CoreFunction::Poseidon4HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, I32, i32, to_fields)
        }
        CoreFunction::Poseidon4HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, I64, i64, to_fields)
        }
        CoreFunction::Poseidon4HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, I128, i128, to_fields)
        }
        CoreFunction::Poseidon4HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, U8, u8, to_fields)
        }
        CoreFunction::Poseidon4HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, U16, u16, to_fields)
        }
        CoreFunction::Poseidon4HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, U32, u32, to_fields)
        }
        CoreFunction::Poseidon4HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, U64, u64, to_fields)
        }
        CoreFunction::Poseidon4HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd4, U128, u128, to_fields)
        }
        CoreFunction::Poseidon4HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_psd4, Scalar, to_fields)
        }
        CoreFunction::Poseidon8HashToAddress => {
            apply_cast!(TestnetV0::hash_to_group_psd8, Address, to_fields)
        }
        CoreFunction::Poseidon8HashToField => {
            apply!(TestnetV0::hash_psd8, Field, to_fields)
        }
        CoreFunction::Poseidon8HashToGroup => {
            apply_cast!(TestnetV0::hash_to_group_psd8, Group, to_fields)
        }
        CoreFunction::Poseidon8HashToI8 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, I8, i8, to_fields)
        }
        CoreFunction::Poseidon8HashToI16 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, I16, i16, to_fields)
        }
        CoreFunction::Poseidon8HashToI32 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, I32, i32, to_fields)
        }
        CoreFunction::Poseidon8HashToI64 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, I64, i64, to_fields)
        }
        CoreFunction::Poseidon8HashToI128 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, I128, i128, to_fields)
        }
        CoreFunction::Poseidon8HashToU8 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, U8, u8, to_fields)
        }
        CoreFunction::Poseidon8HashToU16 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, U16, u16, to_fields)
        }
        CoreFunction::Poseidon8HashToU32 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, U32, u32, to_fields)
        }
        CoreFunction::Poseidon8HashToU64 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, U64, u64, to_fields)
        }
        CoreFunction::Poseidon8HashToU128 => {
            apply_cast_int!(TestnetV0::hash_to_group_psd8, U128, u128, to_fields)
        }
        CoreFunction::Poseidon8HashToScalar => {
            apply_cast!(TestnetV0::hash_to_group_psd8, Scalar, to_fields)
        }
        CoreFunction::SHA3_256HashToAddress => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            Address,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToField => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            Field,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToGroup => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            Group,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToI8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            I8,
            i8,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToI16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            I16,
            i16,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToI32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            I32,
            i32,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToI64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            I64,
            i64,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToI128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            I128,
            i128,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToU8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            U8,
            u8,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToU16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            U16,
            u16,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToU32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            U32,
            u32,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToU64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            U64,
            u64,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToU128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            U128,
            u128,
            to_bits_le
        ),
        CoreFunction::SHA3_256HashToScalar => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
            Scalar,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToAddress => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            Address,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToField => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            Field,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToGroup => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            Group,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToI8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            I8,
            i8,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToI16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            I16,
            i16,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToI32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            I32,
            i32,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToI64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            I64,
            i64,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToI128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            I128,
            i128,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToU8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            U8,
            u8,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToU16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            U16,
            u16,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToU32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            U32,
            u32,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToU64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            U64,
            u64,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToU128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            U128,
            u128,
            to_bits_le
        ),
        CoreFunction::SHA3_384HashToScalar => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
            Scalar,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToAddress => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            Address,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToField => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            Field,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToGroup => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            Group,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToI8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            I8,
            i8,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToI16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            I16,
            i16,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToI32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            I32,
            i32,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToI64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            I64,
            i64,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToI128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            I128,
            i128,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToU8 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            U8,
            u8,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToU16 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            U16,
            u16,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToU32 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            U32,
            u32,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToU64 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            U64,
            u64,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToU128 => apply_cast_int!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            U128,
            u128,
            to_bits_le
        ),
        CoreFunction::SHA3_512HashToScalar => apply_cast!(
            |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
            Scalar,
            to_bits_le
        ),
        CoreFunction::GroupToXCoordinate => {
            let Value::Group(g) = helper.pop_value()? else {
                tc_fail!();
            };
            Value::Field(g.to_x_coordinate())
        }
        CoreFunction::GroupToYCoordinate => {
            let Value::Group(g) = helper.pop_value()? else {
                tc_fail!();
            };
            Value::Field(g.to_y_coordinate())
        }
        CoreFunction::ChaChaRandAddress => Value::Address(maybe_gen!()),
        CoreFunction::ChaChaRandBool => Value::Bool(maybe_gen!()),
        CoreFunction::ChaChaRandField => Value::Field(maybe_gen!()),
        CoreFunction::ChaChaRandGroup => Value::Group(maybe_gen!()),
        CoreFunction::ChaChaRandI8 => Value::I8(maybe_gen!()),
        CoreFunction::ChaChaRandI16 => Value::I16(maybe_gen!()),
        CoreFunction::ChaChaRandI32 => Value::I32(maybe_gen!()),
        CoreFunction::ChaChaRandI64 => Value::I64(maybe_gen!()),
        CoreFunction::ChaChaRandI128 => Value::I128(maybe_gen!()),
        CoreFunction::ChaChaRandU8 => Value::U8(maybe_gen!()),
        CoreFunction::ChaChaRandU16 => Value::U16(maybe_gen!()),
        CoreFunction::ChaChaRandU32 => Value::U32(maybe_gen!()),
        CoreFunction::ChaChaRandU64 => Value::U64(maybe_gen!()),
        CoreFunction::ChaChaRandU128 => Value::U128(maybe_gen!()),
        CoreFunction::ChaChaRandScalar => Value::Scalar(maybe_gen!()),
        CoreFunction::CheatCodePrintMapping => {
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail!(),
            };
            if let Some(mapping) = helper.lookup_mapping(program, name) {
                // TODO: What is the appropriate way to print this to the console.
                // Print the name of the mapping.
                println!(
                    "Mapping: {}",
                    if let Some(program) = program { format!("{program}/{name}") } else { name.to_string() }
                );
                // Print the contents of the mapping.
                for (key, value) in mapping {
                    println!("  {key} -> {value}");
                }
            } else {
                tc_fail!();
            }
            Value::Unit
        }
        CoreFunction::CheatCodeSetBlockHeight => {
            let Value::U32(height) = helper.pop_value()? else {
                tc_fail!();
            };
            helper.set_block_height(height);
            Value::Unit
        }
        CoreFunction::MappingGet => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail!(),
            };
            match helper.lookup_mapping(program, name).and_then(|mapping| mapping.get(&key)) {
                Some(v) => v.clone(),
                None => halt!(span, "map lookup failure"),
            }
        }
        CoreFunction::MappingGetOrUse => {
            let use_value = helper.pop_value().expect_tc(span)?;
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail!(),
            };
            match helper.lookup_mapping(program, name).and_then(|mapping| mapping.get(&key)) {
                Some(v) => v.clone(),
                None => use_value,
            }
        }
        CoreFunction::MappingSet => {
            let value = helper.pop_value()?;
            let key = helper.pop_value()?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail!(),
            };
            if let Some(mapping) = helper.lookup_mapping_mut(program, name) {
                mapping.insert(key, value);
            } else {
                tc_fail!();
            }
            Value::Unit
        }
        CoreFunction::MappingRemove => {
            let key = helper.pop_value()?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail!(),
            };
            if let Some(mapping) = helper.lookup_mapping_mut(program, name) {
                mapping.remove(&key);
            } else {
                tc_fail!();
            }
            Value::Unit
        }
        CoreFunction::MappingContains => {
            let key = helper.pop_value()?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail!(),
            };
            if let Some(mapping) = helper.lookup_mapping_mut(program, name) {
                Value::Bool(mapping.contains_key(&key))
            } else {
                tc_fail!();
            }
        }
        CoreFunction::SignatureVerify => todo!(),
        CoreFunction::FutureAwait => panic!("await must be handled elsewhere"),
        CoreFunction::ProgramChecksum => {
            // TODO: This is a placeholder. The actual implementation should look up the program in the global context and get its checksum.
            return Ok(None);
        }
        CoreFunction::ProgramEdition => {
            // TODO: This is a placeholder. The actual implementation should look up the program in the global context and get its edition.
            return Ok(None);
        }
        CoreFunction::ProgramOwner => {
            // TODO: This is a placeholder. The actual implementation should look up the program in the global context and get its owner.
            return Ok(None);
        }
    };

    Ok(Some(value))
}
