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

use crate::{CoreCircuitError, Value};
use leo_typed::{Circuit, Identifier, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

/// A core circuit type, accessible to all Leo programs by default
pub trait CoreCircuit {
    /// The name of the core circuit function
    fn name() -> String;

    /// Return the abstract syntax tree representation of the core circuit for compiler parsing.
    fn ast(circuit_name: Identifier, span: Span) -> Circuit;

    /// Call the gadget associated with this core circuit with arguments.
    /// Generate constraints on the given `ConstraintSystem`.
    fn call<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        cs: CS,
        arguments: Vec<Value>,
        span: Span,
    ) -> Result<Vec<Value>, CoreCircuitError>;
}
