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

use std::collections::HashMap;

use rand::Rng as _;
use rand_chacha::ChaCha20Rng;

use crate::{
    CoreFunction,
    Expression,
    interpreter_value::{ExpectTc, Value},
    tc_fail2,
};
use leo_errors::{InterpreterHalt, Result};
use leo_span::{Span, Symbol};

use super::*;

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

    fn mapping_get(&self, program: Option<Symbol>, name: Symbol, key: &Value) -> Option<Value> {
        self.lookup_mapping(program, name).and_then(|map| map.get(key).cloned())
    }

    fn mapping_set(&mut self, program: Option<Symbol>, name: Symbol, key: Value, value: Value) -> Option<()> {
        self.lookup_mapping_mut(program, name).map(|map| {
            map.insert(key, value);
        })
    }

    fn mapping_remove(&mut self, program: Option<Symbol>, name: Symbol, key: &Value) -> Option<()> {
        self.lookup_mapping_mut(program, name).map(|map| {
            map.remove(key);
        })
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
    use snarkvm::{
        prelude::LiteralType,
        synthesizer::program::{CommitVariant, HashVariant},
    };

    let dohash = |helper: &mut dyn CoreFunctionHelper, variant: HashVariant, typ: LiteralType| -> Result<Value> {
        let input = helper.pop_value()?.try_into().expect_tc(span)?;
        let value = snarkvm::synthesizer::program::evaluate_hash(
            variant,
            &input,
            &snarkvm::prelude::PlaintextType::Literal(typ),
        )?;
        Ok(value.into())
    };

    let docommit = |helper: &mut dyn CoreFunctionHelper, variant: CommitVariant, typ: LiteralType| -> Result<Value> {
        let randomizer: Scalar = helper.pop_value()?.try_into().expect_tc(span)?;
        let input: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let value = snarkvm::synthesizer::program::evaluate_commit(variant, &input, &randomizer, typ)?;
        Ok(value.into())
    };

    macro_rules! random {
        ($ty: ident) => {{
            let Some(rng) = helper.rng() else {
                return Ok(None);
            };
            let value: $ty = rng.r#gen();
            value.into()
        }};
    }

    let value = match core_function {
        CoreFunction::BHP256CommitToAddress => docommit(helper, CommitVariant::CommitBHP256, LiteralType::Address)?,
        CoreFunction::BHP256CommitToField => docommit(helper, CommitVariant::CommitBHP256, LiteralType::Field)?,
        CoreFunction::BHP256CommitToGroup => docommit(helper, CommitVariant::CommitBHP256, LiteralType::Group)?,
        CoreFunction::BHP256HashToAddress => dohash(helper, HashVariant::HashBHP256, LiteralType::Address)?,
        CoreFunction::BHP256HashToField => dohash(helper, HashVariant::HashBHP256, LiteralType::Field)?,
        CoreFunction::BHP256HashToGroup => dohash(helper, HashVariant::HashBHP256, LiteralType::Group)?,
        CoreFunction::BHP256HashToI8 => dohash(helper, HashVariant::HashBHP256, LiteralType::I8)?,
        CoreFunction::BHP256HashToI16 => dohash(helper, HashVariant::HashBHP256, LiteralType::I16)?,
        CoreFunction::BHP256HashToI32 => dohash(helper, HashVariant::HashBHP256, LiteralType::I32)?,
        CoreFunction::BHP256HashToI64 => dohash(helper, HashVariant::HashBHP256, LiteralType::I64)?,
        CoreFunction::BHP256HashToI128 => dohash(helper, HashVariant::HashBHP256, LiteralType::I128)?,
        CoreFunction::BHP256HashToU8 => dohash(helper, HashVariant::HashBHP256, LiteralType::U8)?,
        CoreFunction::BHP256HashToU16 => dohash(helper, HashVariant::HashBHP256, LiteralType::U16)?,
        CoreFunction::BHP256HashToU32 => dohash(helper, HashVariant::HashBHP256, LiteralType::U32)?,
        CoreFunction::BHP256HashToU64 => dohash(helper, HashVariant::HashBHP256, LiteralType::U64)?,
        CoreFunction::BHP256HashToU128 => dohash(helper, HashVariant::HashBHP256, LiteralType::U128)?,
        CoreFunction::BHP256HashToScalar => dohash(helper, HashVariant::HashBHP256, LiteralType::Scalar)?,
        CoreFunction::BHP512CommitToAddress => docommit(helper, CommitVariant::CommitBHP512, LiteralType::Address)?,
        CoreFunction::BHP512CommitToField => docommit(helper, CommitVariant::CommitBHP512, LiteralType::Field)?,
        CoreFunction::BHP512CommitToGroup => docommit(helper, CommitVariant::CommitBHP512, LiteralType::Group)?,
        CoreFunction::BHP512HashToAddress => dohash(helper, HashVariant::HashBHP512, LiteralType::Address)?,
        CoreFunction::BHP512HashToField => dohash(helper, HashVariant::HashBHP512, LiteralType::Field)?,
        CoreFunction::BHP512HashToGroup => dohash(helper, HashVariant::HashBHP512, LiteralType::Group)?,
        CoreFunction::BHP512HashToI8 => dohash(helper, HashVariant::HashBHP512, LiteralType::I8)?,
        CoreFunction::BHP512HashToI16 => dohash(helper, HashVariant::HashBHP512, LiteralType::I16)?,
        CoreFunction::BHP512HashToI32 => dohash(helper, HashVariant::HashBHP512, LiteralType::I32)?,
        CoreFunction::BHP512HashToI64 => dohash(helper, HashVariant::HashBHP512, LiteralType::I64)?,
        CoreFunction::BHP512HashToI128 => dohash(helper, HashVariant::HashBHP512, LiteralType::I128)?,
        CoreFunction::BHP512HashToU8 => dohash(helper, HashVariant::HashBHP512, LiteralType::U8)?,
        CoreFunction::BHP512HashToU16 => dohash(helper, HashVariant::HashBHP512, LiteralType::U16)?,
        CoreFunction::BHP512HashToU32 => dohash(helper, HashVariant::HashBHP512, LiteralType::U32)?,
        CoreFunction::BHP512HashToU64 => dohash(helper, HashVariant::HashBHP512, LiteralType::U64)?,
        CoreFunction::BHP512HashToU128 => dohash(helper, HashVariant::HashBHP512, LiteralType::U128)?,
        CoreFunction::BHP512HashToScalar => dohash(helper, HashVariant::HashBHP512, LiteralType::Scalar)?,
        CoreFunction::BHP768CommitToAddress => docommit(helper, CommitVariant::CommitBHP768, LiteralType::Address)?,
        CoreFunction::BHP768CommitToField => docommit(helper, CommitVariant::CommitBHP768, LiteralType::Field)?,
        CoreFunction::BHP768CommitToGroup => docommit(helper, CommitVariant::CommitBHP768, LiteralType::Group)?,
        CoreFunction::BHP768HashToAddress => dohash(helper, HashVariant::HashBHP768, LiteralType::Address)?,
        CoreFunction::BHP768HashToField => dohash(helper, HashVariant::HashBHP768, LiteralType::Field)?,
        CoreFunction::BHP768HashToGroup => dohash(helper, HashVariant::HashBHP768, LiteralType::Group)?,
        CoreFunction::BHP768HashToI8 => dohash(helper, HashVariant::HashBHP768, LiteralType::I8)?,
        CoreFunction::BHP768HashToI16 => dohash(helper, HashVariant::HashBHP768, LiteralType::I16)?,
        CoreFunction::BHP768HashToI32 => dohash(helper, HashVariant::HashBHP768, LiteralType::I32)?,
        CoreFunction::BHP768HashToI64 => dohash(helper, HashVariant::HashBHP768, LiteralType::I64)?,
        CoreFunction::BHP768HashToI128 => dohash(helper, HashVariant::HashBHP768, LiteralType::I128)?,
        CoreFunction::BHP768HashToU8 => dohash(helper, HashVariant::HashBHP768, LiteralType::U8)?,
        CoreFunction::BHP768HashToU16 => dohash(helper, HashVariant::HashBHP768, LiteralType::U16)?,
        CoreFunction::BHP768HashToU32 => dohash(helper, HashVariant::HashBHP768, LiteralType::U32)?,
        CoreFunction::BHP768HashToU64 => dohash(helper, HashVariant::HashBHP768, LiteralType::U64)?,
        CoreFunction::BHP768HashToU128 => dohash(helper, HashVariant::HashBHP768, LiteralType::U128)?,
        CoreFunction::BHP768HashToScalar => dohash(helper, HashVariant::HashBHP768, LiteralType::Scalar)?,
        CoreFunction::BHP1024CommitToAddress => docommit(helper, CommitVariant::CommitBHP1024, LiteralType::Address)?,
        CoreFunction::BHP1024CommitToField => docommit(helper, CommitVariant::CommitBHP1024, LiteralType::Field)?,
        CoreFunction::BHP1024CommitToGroup => docommit(helper, CommitVariant::CommitBHP1024, LiteralType::Group)?,
        CoreFunction::BHP1024HashToAddress => dohash(helper, HashVariant::HashBHP1024, LiteralType::Address)?,
        CoreFunction::BHP1024HashToField => dohash(helper, HashVariant::HashBHP1024, LiteralType::Field)?,
        CoreFunction::BHP1024HashToGroup => dohash(helper, HashVariant::HashBHP1024, LiteralType::Group)?,
        CoreFunction::BHP1024HashToI8 => dohash(helper, HashVariant::HashBHP1024, LiteralType::I8)?,
        CoreFunction::BHP1024HashToI16 => dohash(helper, HashVariant::HashBHP1024, LiteralType::I16)?,
        CoreFunction::BHP1024HashToI32 => dohash(helper, HashVariant::HashBHP1024, LiteralType::I32)?,
        CoreFunction::BHP1024HashToI64 => dohash(helper, HashVariant::HashBHP1024, LiteralType::I64)?,
        CoreFunction::BHP1024HashToI128 => dohash(helper, HashVariant::HashBHP1024, LiteralType::I128)?,
        CoreFunction::BHP1024HashToU8 => dohash(helper, HashVariant::HashBHP1024, LiteralType::U8)?,
        CoreFunction::BHP1024HashToU16 => dohash(helper, HashVariant::HashBHP1024, LiteralType::U16)?,
        CoreFunction::BHP1024HashToU32 => dohash(helper, HashVariant::HashBHP1024, LiteralType::U32)?,
        CoreFunction::BHP1024HashToU64 => dohash(helper, HashVariant::HashBHP1024, LiteralType::U64)?,
        CoreFunction::BHP1024HashToU128 => dohash(helper, HashVariant::HashBHP1024, LiteralType::U128)?,
        CoreFunction::BHP1024HashToScalar => dohash(helper, HashVariant::HashBHP1024, LiteralType::Scalar)?,
        CoreFunction::Keccak256HashToAddress => dohash(helper, HashVariant::HashKeccak256, LiteralType::Address)?,
        CoreFunction::Keccak256HashToField => dohash(helper, HashVariant::HashKeccak256, LiteralType::Field)?,
        CoreFunction::Keccak256HashToGroup => dohash(helper, HashVariant::HashKeccak256, LiteralType::Group)?,
        CoreFunction::Keccak256HashToI8 => dohash(helper, HashVariant::HashKeccak256, LiteralType::I8)?,
        CoreFunction::Keccak256HashToI16 => dohash(helper, HashVariant::HashKeccak256, LiteralType::I16)?,
        CoreFunction::Keccak256HashToI32 => dohash(helper, HashVariant::HashKeccak256, LiteralType::I32)?,
        CoreFunction::Keccak256HashToI64 => dohash(helper, HashVariant::HashKeccak256, LiteralType::I64)?,
        CoreFunction::Keccak256HashToI128 => dohash(helper, HashVariant::HashKeccak256, LiteralType::I128)?,
        CoreFunction::Keccak256HashToU8 => dohash(helper, HashVariant::HashKeccak256, LiteralType::U8)?,
        CoreFunction::Keccak256HashToU16 => dohash(helper, HashVariant::HashKeccak256, LiteralType::U16)?,
        CoreFunction::Keccak256HashToU32 => dohash(helper, HashVariant::HashKeccak256, LiteralType::U32)?,
        CoreFunction::Keccak256HashToU64 => dohash(helper, HashVariant::HashKeccak256, LiteralType::U64)?,
        CoreFunction::Keccak256HashToU128 => dohash(helper, HashVariant::HashKeccak256, LiteralType::U128)?,
        CoreFunction::Keccak256HashToScalar => dohash(helper, HashVariant::HashKeccak256, LiteralType::Scalar)?,
        CoreFunction::Keccak384HashToAddress => dohash(helper, HashVariant::HashKeccak384, LiteralType::Address)?,
        CoreFunction::Keccak384HashToField => dohash(helper, HashVariant::HashKeccak384, LiteralType::Field)?,
        CoreFunction::Keccak384HashToGroup => dohash(helper, HashVariant::HashKeccak384, LiteralType::Group)?,
        CoreFunction::Keccak384HashToI8 => dohash(helper, HashVariant::HashKeccak384, LiteralType::I8)?,
        CoreFunction::Keccak384HashToI16 => dohash(helper, HashVariant::HashKeccak384, LiteralType::I16)?,
        CoreFunction::Keccak384HashToI32 => dohash(helper, HashVariant::HashKeccak384, LiteralType::I32)?,
        CoreFunction::Keccak384HashToI64 => dohash(helper, HashVariant::HashKeccak384, LiteralType::I64)?,
        CoreFunction::Keccak384HashToI128 => dohash(helper, HashVariant::HashKeccak384, LiteralType::I128)?,
        CoreFunction::Keccak384HashToU8 => dohash(helper, HashVariant::HashKeccak384, LiteralType::U8)?,
        CoreFunction::Keccak384HashToU16 => dohash(helper, HashVariant::HashKeccak384, LiteralType::U16)?,
        CoreFunction::Keccak384HashToU32 => dohash(helper, HashVariant::HashKeccak384, LiteralType::U32)?,
        CoreFunction::Keccak384HashToU64 => dohash(helper, HashVariant::HashKeccak384, LiteralType::U64)?,
        CoreFunction::Keccak384HashToU128 => dohash(helper, HashVariant::HashKeccak384, LiteralType::U128)?,
        CoreFunction::Keccak384HashToScalar => dohash(helper, HashVariant::HashKeccak384, LiteralType::Scalar)?,
        CoreFunction::Keccak512HashToAddress => dohash(helper, HashVariant::HashKeccak512, LiteralType::Address)?,
        CoreFunction::Keccak512HashToField => dohash(helper, HashVariant::HashKeccak512, LiteralType::Field)?,
        CoreFunction::Keccak512HashToGroup => dohash(helper, HashVariant::HashKeccak512, LiteralType::Group)?,
        CoreFunction::Keccak512HashToI8 => dohash(helper, HashVariant::HashKeccak512, LiteralType::I8)?,
        CoreFunction::Keccak512HashToI16 => dohash(helper, HashVariant::HashKeccak512, LiteralType::I16)?,
        CoreFunction::Keccak512HashToI32 => dohash(helper, HashVariant::HashKeccak512, LiteralType::I32)?,
        CoreFunction::Keccak512HashToI64 => dohash(helper, HashVariant::HashKeccak512, LiteralType::I64)?,
        CoreFunction::Keccak512HashToI128 => dohash(helper, HashVariant::HashKeccak512, LiteralType::I128)?,
        CoreFunction::Keccak512HashToU8 => dohash(helper, HashVariant::HashKeccak512, LiteralType::U8)?,
        CoreFunction::Keccak512HashToU16 => dohash(helper, HashVariant::HashKeccak512, LiteralType::U16)?,
        CoreFunction::Keccak512HashToU32 => dohash(helper, HashVariant::HashKeccak512, LiteralType::U32)?,
        CoreFunction::Keccak512HashToU64 => dohash(helper, HashVariant::HashKeccak512, LiteralType::U64)?,
        CoreFunction::Keccak512HashToU128 => dohash(helper, HashVariant::HashKeccak512, LiteralType::U128)?,
        CoreFunction::Keccak512HashToScalar => dohash(helper, HashVariant::HashKeccak512, LiteralType::Scalar)?,
        CoreFunction::Pedersen64CommitToAddress => docommit(helper, CommitVariant::CommitPED64, LiteralType::Address)?,
        CoreFunction::Pedersen64CommitToField => docommit(helper, CommitVariant::CommitPED64, LiteralType::Field)?,
        CoreFunction::Pedersen64CommitToGroup => docommit(helper, CommitVariant::CommitPED64, LiteralType::Group)?,
        CoreFunction::Pedersen64HashToAddress => dohash(helper, HashVariant::HashPED64, LiteralType::Address)?,
        CoreFunction::Pedersen64HashToField => dohash(helper, HashVariant::HashPED64, LiteralType::Field)?,
        CoreFunction::Pedersen64HashToGroup => dohash(helper, HashVariant::HashPED64, LiteralType::Group)?,
        CoreFunction::Pedersen64HashToI8 => dohash(helper, HashVariant::HashPED64, LiteralType::I8)?,
        CoreFunction::Pedersen64HashToI16 => dohash(helper, HashVariant::HashPED64, LiteralType::I16)?,
        CoreFunction::Pedersen64HashToI32 => dohash(helper, HashVariant::HashPED64, LiteralType::I32)?,
        CoreFunction::Pedersen64HashToI64 => dohash(helper, HashVariant::HashPED64, LiteralType::I64)?,
        CoreFunction::Pedersen64HashToI128 => dohash(helper, HashVariant::HashPED64, LiteralType::I128)?,
        CoreFunction::Pedersen64HashToU8 => dohash(helper, HashVariant::HashPED64, LiteralType::U8)?,
        CoreFunction::Pedersen64HashToU16 => dohash(helper, HashVariant::HashPED64, LiteralType::U16)?,
        CoreFunction::Pedersen64HashToU32 => dohash(helper, HashVariant::HashPED64, LiteralType::U32)?,
        CoreFunction::Pedersen64HashToU64 => dohash(helper, HashVariant::HashPED64, LiteralType::U64)?,
        CoreFunction::Pedersen64HashToU128 => dohash(helper, HashVariant::HashPED64, LiteralType::U128)?,
        CoreFunction::Pedersen64HashToScalar => dohash(helper, HashVariant::HashPED64, LiteralType::Scalar)?,
        CoreFunction::Pedersen128CommitToAddress => {
            docommit(helper, CommitVariant::CommitPED128, LiteralType::Address)?
        }
        CoreFunction::Pedersen128CommitToField => docommit(helper, CommitVariant::CommitPED128, LiteralType::Field)?,
        CoreFunction::Pedersen128CommitToGroup => docommit(helper, CommitVariant::CommitPED128, LiteralType::Group)?,
        CoreFunction::Pedersen128HashToAddress => dohash(helper, HashVariant::HashPED128, LiteralType::Address)?,
        CoreFunction::Pedersen128HashToField => dohash(helper, HashVariant::HashPED128, LiteralType::Field)?,
        CoreFunction::Pedersen128HashToGroup => dohash(helper, HashVariant::HashPED128, LiteralType::Group)?,
        CoreFunction::Pedersen128HashToI8 => dohash(helper, HashVariant::HashPED128, LiteralType::I8)?,
        CoreFunction::Pedersen128HashToI16 => dohash(helper, HashVariant::HashPED128, LiteralType::I16)?,
        CoreFunction::Pedersen128HashToI32 => dohash(helper, HashVariant::HashPED128, LiteralType::I32)?,
        CoreFunction::Pedersen128HashToI64 => dohash(helper, HashVariant::HashPED128, LiteralType::I64)?,
        CoreFunction::Pedersen128HashToI128 => dohash(helper, HashVariant::HashPED128, LiteralType::I128)?,
        CoreFunction::Pedersen128HashToU8 => dohash(helper, HashVariant::HashPED128, LiteralType::U8)?,
        CoreFunction::Pedersen128HashToU16 => dohash(helper, HashVariant::HashPED128, LiteralType::U16)?,
        CoreFunction::Pedersen128HashToU32 => dohash(helper, HashVariant::HashPED128, LiteralType::U32)?,
        CoreFunction::Pedersen128HashToU64 => dohash(helper, HashVariant::HashPED128, LiteralType::U64)?,
        CoreFunction::Pedersen128HashToU128 => dohash(helper, HashVariant::HashPED128, LiteralType::U128)?,
        CoreFunction::Pedersen128HashToScalar => dohash(helper, HashVariant::HashPED128, LiteralType::Scalar)?,
        CoreFunction::Poseidon2HashToAddress => dohash(helper, HashVariant::HashPSD2, LiteralType::Address)?,
        CoreFunction::Poseidon2HashToField => dohash(helper, HashVariant::HashPSD2, LiteralType::Field)?,
        CoreFunction::Poseidon2HashToGroup => dohash(helper, HashVariant::HashPSD2, LiteralType::Group)?,
        CoreFunction::Poseidon2HashToI8 => dohash(helper, HashVariant::HashPSD2, LiteralType::I8)?,
        CoreFunction::Poseidon2HashToI16 => dohash(helper, HashVariant::HashPSD2, LiteralType::I16)?,
        CoreFunction::Poseidon2HashToI32 => dohash(helper, HashVariant::HashPSD2, LiteralType::I32)?,
        CoreFunction::Poseidon2HashToI64 => dohash(helper, HashVariant::HashPSD2, LiteralType::I64)?,
        CoreFunction::Poseidon2HashToI128 => dohash(helper, HashVariant::HashPSD2, LiteralType::I128)?,
        CoreFunction::Poseidon2HashToU8 => dohash(helper, HashVariant::HashPSD2, LiteralType::U8)?,
        CoreFunction::Poseidon2HashToU16 => dohash(helper, HashVariant::HashPSD2, LiteralType::U16)?,
        CoreFunction::Poseidon2HashToU32 => dohash(helper, HashVariant::HashPSD2, LiteralType::U32)?,
        CoreFunction::Poseidon2HashToU64 => dohash(helper, HashVariant::HashPSD2, LiteralType::U64)?,
        CoreFunction::Poseidon2HashToU128 => dohash(helper, HashVariant::HashPSD2, LiteralType::U128)?,
        CoreFunction::Poseidon2HashToScalar => dohash(helper, HashVariant::HashPSD2, LiteralType::Scalar)?,
        CoreFunction::Poseidon4HashToAddress => dohash(helper, HashVariant::HashPSD4, LiteralType::Address)?,
        CoreFunction::Poseidon4HashToField => dohash(helper, HashVariant::HashPSD4, LiteralType::Field)?,
        CoreFunction::Poseidon4HashToGroup => dohash(helper, HashVariant::HashPSD4, LiteralType::Group)?,
        CoreFunction::Poseidon4HashToI8 => dohash(helper, HashVariant::HashPSD4, LiteralType::I8)?,
        CoreFunction::Poseidon4HashToI16 => dohash(helper, HashVariant::HashPSD4, LiteralType::I16)?,
        CoreFunction::Poseidon4HashToI32 => dohash(helper, HashVariant::HashPSD4, LiteralType::I32)?,
        CoreFunction::Poseidon4HashToI64 => dohash(helper, HashVariant::HashPSD4, LiteralType::I64)?,
        CoreFunction::Poseidon4HashToI128 => dohash(helper, HashVariant::HashPSD4, LiteralType::I128)?,
        CoreFunction::Poseidon4HashToU8 => dohash(helper, HashVariant::HashPSD4, LiteralType::U8)?,
        CoreFunction::Poseidon4HashToU16 => dohash(helper, HashVariant::HashPSD4, LiteralType::U16)?,
        CoreFunction::Poseidon4HashToU32 => dohash(helper, HashVariant::HashPSD4, LiteralType::U32)?,
        CoreFunction::Poseidon4HashToU64 => dohash(helper, HashVariant::HashPSD4, LiteralType::U64)?,
        CoreFunction::Poseidon4HashToU128 => dohash(helper, HashVariant::HashPSD4, LiteralType::U128)?,
        CoreFunction::Poseidon4HashToScalar => dohash(helper, HashVariant::HashPSD4, LiteralType::Scalar)?,
        CoreFunction::Poseidon8HashToAddress => dohash(helper, HashVariant::HashPSD8, LiteralType::Address)?,
        CoreFunction::Poseidon8HashToField => dohash(helper, HashVariant::HashPSD8, LiteralType::Field)?,
        CoreFunction::Poseidon8HashToGroup => dohash(helper, HashVariant::HashPSD8, LiteralType::Group)?,
        CoreFunction::Poseidon8HashToI8 => dohash(helper, HashVariant::HashPSD8, LiteralType::I8)?,
        CoreFunction::Poseidon8HashToI16 => dohash(helper, HashVariant::HashPSD8, LiteralType::I16)?,
        CoreFunction::Poseidon8HashToI32 => dohash(helper, HashVariant::HashPSD8, LiteralType::I32)?,
        CoreFunction::Poseidon8HashToI64 => dohash(helper, HashVariant::HashPSD8, LiteralType::I64)?,
        CoreFunction::Poseidon8HashToI128 => dohash(helper, HashVariant::HashPSD8, LiteralType::I128)?,
        CoreFunction::Poseidon8HashToU8 => dohash(helper, HashVariant::HashPSD8, LiteralType::U8)?,
        CoreFunction::Poseidon8HashToU16 => dohash(helper, HashVariant::HashPSD8, LiteralType::U16)?,
        CoreFunction::Poseidon8HashToU32 => dohash(helper, HashVariant::HashPSD8, LiteralType::U32)?,
        CoreFunction::Poseidon8HashToU64 => dohash(helper, HashVariant::HashPSD8, LiteralType::U64)?,
        CoreFunction::Poseidon8HashToU128 => dohash(helper, HashVariant::HashPSD8, LiteralType::U128)?,
        CoreFunction::Poseidon8HashToScalar => dohash(helper, HashVariant::HashPSD8, LiteralType::Scalar)?,
        CoreFunction::SHA3_256HashToAddress => dohash(helper, HashVariant::HashSha3_256, LiteralType::Address)?,
        CoreFunction::SHA3_256HashToField => dohash(helper, HashVariant::HashSha3_256, LiteralType::Field)?,
        CoreFunction::SHA3_256HashToGroup => dohash(helper, HashVariant::HashSha3_256, LiteralType::Group)?,
        CoreFunction::SHA3_256HashToI8 => dohash(helper, HashVariant::HashSha3_256, LiteralType::I8)?,
        CoreFunction::SHA3_256HashToI16 => dohash(helper, HashVariant::HashSha3_256, LiteralType::I16)?,
        CoreFunction::SHA3_256HashToI32 => dohash(helper, HashVariant::HashSha3_256, LiteralType::I32)?,
        CoreFunction::SHA3_256HashToI64 => dohash(helper, HashVariant::HashSha3_256, LiteralType::I64)?,
        CoreFunction::SHA3_256HashToI128 => dohash(helper, HashVariant::HashSha3_256, LiteralType::I128)?,
        CoreFunction::SHA3_256HashToU8 => dohash(helper, HashVariant::HashSha3_256, LiteralType::U8)?,
        CoreFunction::SHA3_256HashToU16 => dohash(helper, HashVariant::HashSha3_256, LiteralType::U16)?,
        CoreFunction::SHA3_256HashToU32 => dohash(helper, HashVariant::HashSha3_256, LiteralType::U32)?,
        CoreFunction::SHA3_256HashToU64 => dohash(helper, HashVariant::HashSha3_256, LiteralType::U64)?,
        CoreFunction::SHA3_256HashToU128 => dohash(helper, HashVariant::HashSha3_256, LiteralType::U128)?,
        CoreFunction::SHA3_256HashToScalar => dohash(helper, HashVariant::HashSha3_256, LiteralType::Scalar)?,
        CoreFunction::SHA3_384HashToAddress => dohash(helper, HashVariant::HashSha3_384, LiteralType::Address)?,
        CoreFunction::SHA3_384HashToField => dohash(helper, HashVariant::HashSha3_384, LiteralType::Field)?,
        CoreFunction::SHA3_384HashToGroup => dohash(helper, HashVariant::HashSha3_384, LiteralType::Group)?,
        CoreFunction::SHA3_384HashToI8 => dohash(helper, HashVariant::HashSha3_384, LiteralType::I8)?,
        CoreFunction::SHA3_384HashToI16 => dohash(helper, HashVariant::HashSha3_384, LiteralType::I16)?,
        CoreFunction::SHA3_384HashToI32 => dohash(helper, HashVariant::HashSha3_384, LiteralType::I32)?,
        CoreFunction::SHA3_384HashToI64 => dohash(helper, HashVariant::HashSha3_384, LiteralType::I64)?,
        CoreFunction::SHA3_384HashToI128 => dohash(helper, HashVariant::HashSha3_384, LiteralType::I128)?,
        CoreFunction::SHA3_384HashToU8 => dohash(helper, HashVariant::HashSha3_384, LiteralType::U8)?,
        CoreFunction::SHA3_384HashToU16 => dohash(helper, HashVariant::HashSha3_384, LiteralType::U16)?,
        CoreFunction::SHA3_384HashToU32 => dohash(helper, HashVariant::HashSha3_384, LiteralType::U32)?,
        CoreFunction::SHA3_384HashToU64 => dohash(helper, HashVariant::HashSha3_384, LiteralType::U64)?,
        CoreFunction::SHA3_384HashToU128 => dohash(helper, HashVariant::HashSha3_384, LiteralType::U128)?,
        CoreFunction::SHA3_384HashToScalar => dohash(helper, HashVariant::HashSha3_384, LiteralType::Scalar)?,
        CoreFunction::SHA3_512HashToAddress => dohash(helper, HashVariant::HashSha3_512, LiteralType::Address)?,
        CoreFunction::SHA3_512HashToField => dohash(helper, HashVariant::HashSha3_512, LiteralType::Field)?,
        CoreFunction::SHA3_512HashToGroup => dohash(helper, HashVariant::HashSha3_512, LiteralType::Group)?,
        CoreFunction::SHA3_512HashToI8 => dohash(helper, HashVariant::HashSha3_512, LiteralType::I8)?,
        CoreFunction::SHA3_512HashToI16 => dohash(helper, HashVariant::HashSha3_512, LiteralType::I16)?,
        CoreFunction::SHA3_512HashToI32 => dohash(helper, HashVariant::HashSha3_512, LiteralType::I32)?,
        CoreFunction::SHA3_512HashToI64 => dohash(helper, HashVariant::HashSha3_512, LiteralType::I64)?,
        CoreFunction::SHA3_512HashToI128 => dohash(helper, HashVariant::HashSha3_512, LiteralType::I128)?,
        CoreFunction::SHA3_512HashToU8 => dohash(helper, HashVariant::HashSha3_512, LiteralType::U8)?,
        CoreFunction::SHA3_512HashToU16 => dohash(helper, HashVariant::HashSha3_512, LiteralType::U16)?,
        CoreFunction::SHA3_512HashToU32 => dohash(helper, HashVariant::HashSha3_512, LiteralType::U32)?,
        CoreFunction::SHA3_512HashToU64 => dohash(helper, HashVariant::HashSha3_512, LiteralType::U64)?,
        CoreFunction::SHA3_512HashToU128 => dohash(helper, HashVariant::HashSha3_512, LiteralType::U128)?,
        CoreFunction::SHA3_512HashToScalar => dohash(helper, HashVariant::HashSha3_512, LiteralType::Scalar)?,
        CoreFunction::GroupToXCoordinate => {
            let g: Group = helper.pop_value()?.try_into().expect_tc(span)?;
            g.to_x_coordinate().into()
        }
        CoreFunction::GroupToYCoordinate => {
            let g: Group = helper.pop_value()?.try_into().expect_tc(span)?;
            g.to_y_coordinate().into()
        }
        CoreFunction::ChaChaRandAddress => random!(Address),
        CoreFunction::ChaChaRandBool => random!(Boolean),
        CoreFunction::ChaChaRandField => random!(Field),
        CoreFunction::ChaChaRandGroup => random!(Group),
        CoreFunction::ChaChaRandI8 => random!(i8),
        CoreFunction::ChaChaRandI16 => random!(i16),
        CoreFunction::ChaChaRandI32 => random!(i32),
        CoreFunction::ChaChaRandI64 => random!(i64),
        CoreFunction::ChaChaRandI128 => random!(i128),
        CoreFunction::ChaChaRandU8 => random!(u8),
        CoreFunction::ChaChaRandU16 => random!(u16),
        CoreFunction::ChaChaRandU32 => random!(u32),
        CoreFunction::ChaChaRandU64 => random!(u64),
        CoreFunction::ChaChaRandU128 => random!(u128),
        CoreFunction::ChaChaRandScalar => random!(Scalar),
        CoreFunction::CheatCodePrintMapping => {
            let (program, name) = match &arguments[0] {
                Expression::Path(id) => (None, id.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail2!(),
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
                tc_fail2!();
            }
            Value { id: None, contents: ValueVariants::Unit }
        }
        CoreFunction::CheatCodeSetBlockHeight => {
            let height: u32 = helper.pop_value()?.try_into().expect_tc(span)?;
            helper.set_block_height(height);
            Value { id: None, contents: ValueVariants::Unit }
        }
        CoreFunction::MappingGet => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail2!(),
            };
            helper.mapping_get(program, name, &key).expect_tc(span)?.clone()
        }
        CoreFunction::MappingGetOrUse => {
            let use_value = helper.pop_value().expect_tc(span)?;
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail2!(),
            };
            helper.mapping_get(program, name, &key).unwrap_or(use_value)
        }
        CoreFunction::MappingSet => {
            let value = helper.pop_value().expect_tc(span)?;
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail2!(),
            };
            helper.mapping_set(program, name, key, value).expect_tc(span)?;
            Value::make_unit()
        }
        CoreFunction::MappingRemove => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail2!(),
            };
            helper.mapping_remove(program, name, &key).expect_tc(span)?;
            Value::make_unit()
        }
        CoreFunction::MappingContains => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (None, path.identifier().name),
                Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                _ => tc_fail2!(),
            };
            helper.mapping_get(program, name, &key).is_some().into()
        }
        CoreFunction::OptionalUnwrap => {
            // TODO
            return Ok(None);
        }
        CoreFunction::OptionalUnwrapOr => {
            // TODO
            return Ok(None);
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
