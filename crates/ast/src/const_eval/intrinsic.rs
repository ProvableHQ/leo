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

use snarkvm::{
    prelude::{ToBits, ToBitsRaw},
    synthesizer::program::{DeserializeVariant, SerializeVariant},
};

use crate::{
    ArrayType,
    Expression,
    Intrinsic,
    Type,
    const_eval::{ExpectTc, Value},
};
use leo_errors::{ConstEvalError, Result};
use leo_span::Span;

use super::*;

fn pop_value(values: &mut Vec<Value>) -> Result<Value> {
    match values.pop() {
        Some(v) => Ok(v),
        None => Err(ConstEvalError::new("value expected during constant evaluation".to_string()).into()),
    }
}

pub fn evaluate_intrinsic(
    values: &mut Vec<Value>,
    intrinsic: Intrinsic,
    _arguments: &[Expression],
    span: Span,
) -> Result<Option<Value>> {
    use snarkvm::{
        prelude::LiteralType,
        synthesizer::program::{CommitVariant, ECDSAVerifyVariant, HashVariant},
    };

    let dohash = |values: &mut Vec<Value>, variant: HashVariant, typ: Type| -> Result<Value> {
        let input = pop_value(values)?.try_into().expect_tc(span)?;
        let value = snarkvm::synthesizer::program::evaluate_hash(variant, &input, &typ.to_snarkvm()?)?;
        Ok(value.into())
    };

    let docommit = |values: &mut Vec<Value>, variant: CommitVariant, typ: LiteralType| -> Result<Value> {
        let randomizer: Scalar = pop_value(values)?.try_into().expect_tc(span)?;
        let input: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
        let value = snarkvm::synthesizer::program::evaluate_commit(variant, &input, &randomizer, typ)?;
        Ok(value.into())
    };

    let doschnorr = |values: &mut Vec<Value>| -> Result<Value> {
        let signature: Signature = pop_value(values)?.try_into().expect_tc(span)?;
        let address: Address = pop_value(values)?.try_into().expect_tc(span)?;
        let message: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
        let is_valid = snarkvm::synthesizer::program::evaluate_schnorr_verification(&signature, &address, &message)?;
        Ok(Boolean::new(is_valid).into())
    };

    let doecdsa = |values: &mut Vec<Value>, variant: ECDSAVerifyVariant| -> Result<Value> {
        let signature: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
        let public_key: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
        let message: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
        let is_valid =
            snarkvm::synthesizer::program::evaluate_ecdsa_verification(variant, &signature, &public_key, &message)?;
        Ok(Boolean::new(is_valid).into())
    };

    let doserialize = |values: &mut Vec<Value>, variant: SerializeVariant| -> Result<Value> {
        let input: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
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

    let dodeserialize = |values: &mut Vec<Value>, variant: DeserializeVariant, type_: Type| -> Result<Value> {
        let value: SvmValue = pop_value(values)?.try_into().expect_tc(span)?;
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

    let value = match intrinsic {
        Intrinsic::ChaChaRand(_) => {
            // Random values cannot be evaluated at compile time.
            return Ok(None);
        }
        Intrinsic::Commit(commit_variant, type_) => docommit(values, commit_variant, type_)?,
        Intrinsic::Hash(hash_variant, type_) => dohash(values, hash_variant, type_)?,
        Intrinsic::ECDSAVerify(ecdsa_variant) => doecdsa(values, ecdsa_variant)?,
        Intrinsic::SignatureVerify => doschnorr(values)?,
        Intrinsic::Serialize(variant) => doserialize(values, variant)?,
        Intrinsic::Deserialize(variant, type_) => dodeserialize(values, variant, type_)?,
        Intrinsic::GroupToXCoordinate => {
            let g: Group = pop_value(values)?.try_into().expect_tc(span)?;
            g.to_x_coordinate().into()
        }
        Intrinsic::GroupToYCoordinate => {
            let g: Group = pop_value(values)?.try_into().expect_tc(span)?;
            g.to_y_coordinate().into()
        }
        Intrinsic::MappingGet
        | Intrinsic::MappingSet
        | Intrinsic::MappingGetOrUse
        | Intrinsic::MappingRemove
        | Intrinsic::MappingContains => {
            // Mapping operations cannot be evaluated at compile time.
            return Ok(None);
        }
        Intrinsic::GroupGen => Value::generator(),
        Intrinsic::OptionalUnwrap | Intrinsic::OptionalUnwrapOr => {
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
            return Ok(None);
        }
        Intrinsic::FinalRun => panic!("await must be handled elsewhere"),
        Intrinsic::ProgramChecksum | Intrinsic::ProgramEdition | Intrinsic::ProgramOwner => {
            return Ok(None);
        }
    };

    Ok(Some(value))
}
