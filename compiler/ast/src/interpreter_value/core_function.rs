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
    CoreFunction, Expression,
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
        synthesizer::program::{CommitVariant, ECDSAVerifyVariant, HashVariant},
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

    let doecdsa = |helper: &mut dyn CoreFunctionHelper, variant: ECDSAVerifyVariant| -> Result<bool> {
        let public_key: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let signature: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let message: SvmValue = helper.pop_value()?.try_into().expect_tc(span)?;
        let is_valid =
            snarkvm::synthesizer::program::evaluate_ecdsa_verification(variant, &public_key, &message, &signature)?;
        Ok(is_valid)
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
        CoreFunction::ChaChaRand(type_) => todo!(),
        CoreFunction::Commit(commit_variant, type_) => docommit(helper, commit_variant, type_)?,
        CoreFunction::Hash(hash_variant, type_) => dohash(helper, hash_variant, type_)?,
        CoreFunction::ECDSAVerify(ecdsa_variant, type_) => todo!(),
        CoreFunction::GroupToXCoordinate => {
            let g: Group = helper.pop_value()?.try_into().expect_tc(span)?;
            g.to_x_coordinate().into()
        }
        CoreFunction::GroupToYCoordinate => {
            let g: Group = helper.pop_value()?.try_into().expect_tc(span)?;
            g.to_y_coordinate().into()
        }
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
        CoreFunction::SignatureVerify(_) => todo!(),
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
