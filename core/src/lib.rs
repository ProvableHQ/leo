// Copyright (C) 2019-2020 Aleo Systems Inc.
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

#[macro_use]
extern crate thiserror;

pub mod packages;
pub use self::packages::*;

pub mod errors;
pub use self::errors::*;

pub mod types;
pub use self::types::*;

use crate::CoreCircuit;
use leo_core_ast::Span;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

/// Calls a core circuit by it's given name.
/// This function should be called by the compiler when enforcing a core circuit function expression.
pub fn call_core_circuit<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: CS,
    circuit_name: String,
    arguments: Vec<Value>,
    span: Span,
) -> Result<Vec<Value>, LeoCoreError> {
    // Match core circuit name
    Ok(match circuit_name.as_str() {
        CORE_UNSTABLE_BLAKE2S_NAME => Blake2sCircuit::call(cs, arguments, span)?,
        _ => return Err(LeoCoreError::undefined_core_circuit(circuit_name, span)),
    })
}
