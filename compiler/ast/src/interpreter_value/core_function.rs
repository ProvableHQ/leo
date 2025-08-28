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

use snarkvm::prelude::{CastLossy as _, Network as _, TestnetV0, ToBits, ToBitsRaw};

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
        CoreFunction::ChaChaRand(_) => todo!(),
        CoreFunction::Commit(_, _) => todo!(),
        CoreFunction::Hash(_, _, _) => todo!(),
        CoreFunction::ECDSAVerify(_, _, _) => todo!(),
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
