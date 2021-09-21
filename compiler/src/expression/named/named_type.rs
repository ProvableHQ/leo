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

//! Enforces a lengthof operator in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::NamedTypeExpression;
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Enforce array expressions
    pub fn enforce_named_type<CS: ConstraintSystem<F>>(
        &mut self,
        _cs: &mut CS,
        named_type: &'a NamedTypeExpression<'a>,
        _span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        Ok(ConstrainedValue::Named(named_type.named_type.name.clone()))
    }
}
