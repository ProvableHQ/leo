// Copyright (C) 2019-2026 Provable Inc.
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
use snarkvm::{
    prelude::{ToBits, ToBitsRaw},
    synthesizer::program::{DeserializeVariant, SerializeVariant},
};

use crate::{
    ArrayType,
    Expression,
    Intrinsic,
    Type,
    interpreter_value::{ExpectTc, Value},
    tc_fail2,
};
use leo_errors::{InterpreterHalt, Result};
use leo_span::{Span, Symbol};

use super::*;

/// A context in which we can evaluate intrinsics.
///
/// This is intended to be implemented by `Cursor`, which will be used during
/// execution of the interpreter, and by `Vec<Value>`, which will be used
/// during compile time evaluation for constant folding.
///
/// The default implementations for `rng`, `set_block_height`, and mapping lookup
/// do nothing, as those features will not be available during compile time
/// evaluation.
pub trait IntrinsicHelper {
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

    fn set_block_timestamp(&mut self, _timestamp: i64) {}

    fn set_signer(&mut self, _private_key: String) -> Result<()> {
        Ok(())
    }

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

impl IntrinsicHelper for Vec<Value> {
    fn pop_value_impl(&mut self) -> Option<Value> {
        self.pop()
    }
}

pub fn evaluate_intrinsic(
    helper: &mut dyn IntrinsicHelper,
    intrinsic: Intrinsic,
    arguments: &[Expression],
    span: Span,
) -> Result<Option<Value>> {
    use snarkvm::{
        prelude::LiteralType,
        synthesizer::program::{CommitVariant, ECDSAVerifyVariant, HashVariant},
    };

    let dohash = |helper: &mut dyn IntrinsicHelper, variant: HashVariant, typ: Type| -> Result<Value> {
        let input = helper.pop_value()?.try_into().expect_tc(span)?;
        let value = snarkvm::synthesizer::program::evaluate_hash(variant, &input, &typ.to_snarkvm()?)?;
        Ok(value.into())
    };

    let docommit = |helper: &mut dyn IntrinsicHelper, variant: CommitVariant, typ: LiteralType| -> Result<Value> {
        let randomizer: Scalar = helper.pop_value()?.try_into().expect_tc(span)?;
        let input: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let value = snarkvm::synthesizer::program::evaluate_commit(variant, &input, &randomizer, typ)?;
        Ok(value.into())
    };

    let doschnorr = |helper: &mut dyn IntrinsicHelper| -> Result<Value> {
        let signature: Signature = helper.pop_value()?.try_into().expect_tc(span)?;
        let address: Address = helper.pop_value()?.try_into().expect_tc(span)?;
        let message: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let is_valid = snarkvm::synthesizer::program::evaluate_schnorr_verification(&signature, &address, &message)?;
        Ok(Boolean::new(is_valid).into())
    };

    let doecdsa = |helper: &mut dyn IntrinsicHelper, variant: ECDSAVerifyVariant| -> Result<Value> {
        let signature: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let public_key: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let message: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let is_valid =
            snarkvm::synthesizer::program::evaluate_ecdsa_verification(variant, &signature, &public_key, &message)?;
        Ok(Boolean::new(is_valid).into())
    };

    let doserialize = |helper: &mut dyn IntrinsicHelper, variant: SerializeVariant| -> Result<Value> {
        let input: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let num_bits = match variant {
            SerializeVariant::ToBits => input.to_bits_le().len(),
            SerializeVariant::ToBitsRaw => input.to_bits_raw_le().len(),
        };
        let Ok(num_bits) = u32::try_from(num_bits) else {
            crate::halt_no_span2!("cannot serialize value with more than 2^32 bits");
        };
        let array_type = ArrayType::bit_array(num_bits).to_snarkvm()?;
        let value = snarkvm::synthesizer::program::evaluate_serialize(variant, &input, &array_type)?;
        Ok(value.into())
    };

    let dodeserialize = |helper: &mut dyn IntrinsicHelper, variant: DeserializeVariant, type_: Type| -> Result<Value> {
        let value: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let bits = match value {
            SvmValue::Plaintext(plaintext) => plaintext.as_bit_array()?,
            _ => crate::halt_no_span2!("expected array for deserialization"),
        };
        let get_struct_fail = |_: &SvmIdentifier| anyhow::bail!("structs are not supported");
        let get_external_struct_fail = |_: &SvmLocator| anyhow::bail!("structs are not supported");
        let value = snarkvm::synthesizer::program::evaluate_deserialize(
            variant,
            &bits,
            &type_.to_snarkvm()?,
            &get_struct_fail,
            &get_external_struct_fail,
        )?;
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

    let value = match intrinsic {
        Intrinsic::ChaChaRand(type_) => match type_ {
            LiteralType::Address => random!(Address),
            LiteralType::Boolean => random!(bool),
            LiteralType::Field => random!(Field),
            LiteralType::Group => random!(Group),
            LiteralType::I8 => random!(i8),
            LiteralType::I16 => random!(i16),
            LiteralType::I32 => random!(i32),
            LiteralType::I64 => random!(i64),
            LiteralType::I128 => random!(i128),
            LiteralType::U8 => random!(u8),
            LiteralType::U16 => random!(u16),
            LiteralType::U32 => random!(u32),
            LiteralType::U64 => random!(u64),
            LiteralType::U128 => random!(u128),
            LiteralType::Scalar => random!(Scalar),
            LiteralType::String | LiteralType::Signature => {
                crate::halt_no_span2!("cannot generate random value of type `{type_}`")
            }
        },
        Intrinsic::Commit(commit_variant, type_) => docommit(helper, commit_variant, type_)?,
        Intrinsic::Hash(hash_variant, type_) => dohash(helper, hash_variant, type_)?,
        Intrinsic::ECDSAVerify(ecdsa_variant) => doecdsa(helper, ecdsa_variant)?,
        Intrinsic::SignatureVerify => doschnorr(helper)?,
        Intrinsic::Serialize(variant) => doserialize(helper, variant)?,
        Intrinsic::Deserialize(variant, type_) => dodeserialize(helper, variant, type_)?,
        Intrinsic::GroupToXCoordinate => {
            let g: Group = helper.pop_value()?.try_into().expect_tc(span)?;
            g.to_x_coordinate().into()
        }
        Intrinsic::GroupToYCoordinate => {
            let g: Group = helper.pop_value()?.try_into().expect_tc(span)?;
            g.to_y_coordinate().into()
        }
        Intrinsic::CheatCodePrintMapping => {
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (path.program(), path.identifier().name),
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
            Value::make_unit()
        }
        Intrinsic::CheatCodeSetBlockHeight => {
            let height: u32 = helper.pop_value()?.try_into().expect_tc(span)?;
            helper.set_block_height(height);
            Value::make_unit()
        }
        Intrinsic::CheatCodeSetBlockTimestamp => {
            let timestamp: i64 = helper.pop_value()?.try_into().expect_tc(span)?;
            helper.set_block_timestamp(timestamp);
            Value::make_unit()
        }
        Intrinsic::CheatCodeSetSigner => {
            let private_key: String = helper.pop_value()?.try_into().expect_tc(span)?;
            helper.set_signer(private_key)?;
            Value::make_unit()
        }
        Intrinsic::MappingGet => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (path.program(), path.identifier().name),
                _ => tc_fail2!(),
            };
            helper.mapping_get(program, name, &key).expect_tc(span)?.clone()
        }
        Intrinsic::MappingSet => {
            let value = helper.pop_value().expect_tc(span)?;
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (path.program(), path.identifier().name),
                _ => tc_fail2!(),
            };
            helper.mapping_set(program, name, key, value).expect_tc(span)?;
            Value::make_unit()
        }
        Intrinsic::MappingGetOrUse => {
            let use_value = helper.pop_value().expect_tc(span)?;
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (path.program(), path.identifier().name),
                _ => tc_fail2!(),
            };
            helper.mapping_get(program, name, &key).unwrap_or(use_value)
        }
        Intrinsic::MappingRemove => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (path.program(), path.identifier().name),
                _ => tc_fail2!(),
            };
            helper.mapping_remove(program, name, &key).expect_tc(span)?;
            Value::make_unit()
        }
        Intrinsic::MappingContains => {
            let key = helper.pop_value().expect_tc(span)?;
            let (program, name) = match &arguments[0] {
                Expression::Path(path) => (path.program(), path.identifier().name),
                _ => tc_fail2!(),
            };
            helper.mapping_get(program, name, &key).is_some().into()
        }
        Intrinsic::GroupGen => Value::generator(),
        Intrinsic::OptionalUnwrap => {
            // TODO
            return Ok(None);
        }
        Intrinsic::OptionalUnwrapOr => {
            // TODO
            return Ok(None);
        }
        Intrinsic::VectorPush
        | Intrinsic::VectorLen
        | Intrinsic::VectorClear
        | Intrinsic::VectorPop
        | Intrinsic::VectorGet
        | Intrinsic::VectorSet
        | Intrinsic::VectorSwapRemove
        | Intrinsic::SelfAddress
        | Intrinsic::SelfCaller
        | Intrinsic::SelfChecksum
        | Intrinsic::SelfEdition
        | Intrinsic::SelfId
        | Intrinsic::SelfProgramOwner
        | Intrinsic::SelfSigner
        | Intrinsic::BlockHeight
        | Intrinsic::BlockTimestamp
        | Intrinsic::NetworkId => {
            // TODO
            return Ok(None);
        }
        Intrinsic::FinalRun => panic!("await must be handled elsewhere"),
        Intrinsic::ProgramChecksum => {
            // TODO: This is a placeholder. The actual implementation should look up the program in the global context and get its checksum.
            return Ok(None);
        }
        Intrinsic::ProgramEdition => {
            // TODO: This is a placeholder. The actual implementation should look up the program in the global context and get its edition.
            return Ok(None);
        }
        Intrinsic::ProgramOwner => {
            // TODO: This is a placeholder. The actual implementation should look up the program in the global context and get its owner.
            return Ok(None);
        }
    };

    Ok(Some(value))
}
