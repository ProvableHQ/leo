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

//! Enforces an assert equals statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::ConditionalEqGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_assert_eq_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        left: &ConstrainedValue<F, G>,
        right: &ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));
        let name_unique = format!("assert {} == {} {}:{}", left, right, span.line, span.start);
        let result = left.conditional_enforce_equal(cs.ns(|| name_unique), right, &condition);

        Ok(result.map_err(|_| StatementError::assertion_failed(left.to_string(), right.to_string(), span))?)
    }
}
