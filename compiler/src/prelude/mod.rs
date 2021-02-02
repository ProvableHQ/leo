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

pub mod blake2s;
use std::sync::Arc;

pub use blake2s::*;

use crate::{errors::ExpressionError, ConstrainedValue, GroupType};
use leo_asg::{FunctionBody, Span};
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub trait CoreCircuit<F: Field + PrimeField, G: GroupType<F>>: Send + Sync {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: Arc<FunctionBody>,
        span: &Span,
        target: Option<ConstrainedValue<F, G>>,
        arguments: Vec<ConstrainedValue<F, G>>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError>;
}

pub fn resolve_core_circuit<F: Field + PrimeField, G: GroupType<F>>(name: &str) -> impl CoreCircuit<F, G> {
    match name {
        "blake2s" => Blake2s,
        _ => unimplemented!("invalid core circuit: {}", name),
    }
}
