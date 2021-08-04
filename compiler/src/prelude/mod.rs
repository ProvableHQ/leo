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
pub use blake2s::*;

use crate::{ConstrainedValue, GroupType};
use leo_asg::Function;
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

pub trait CoreCircuit<'a, F: PrimeField, G: GroupType<F>>: Send + Sync {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>>;
}

pub fn resolve_core_circuit<'a, F: PrimeField, G: GroupType<F>>(name: &str) -> impl CoreCircuit<'a, F, G> {
    match name {
        "blake2s" => Blake2s,
        _ => unimplemented!("invalid core circuit: {}", name),
    }
}
