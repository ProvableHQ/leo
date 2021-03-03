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

//! Evaluates a formatted string in a compiled Leo program.

use crate::{errors::ConsoleError, program::ConstrainedProgram, GroupType};
use leo_asg::FormattedString;

use leo_ast::FormattedStringPart;
use snarkvm_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn format<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        formatted: &FormattedString<'a>,
    ) -> Result<String, ConsoleError> {
        // Check that containers and parameters match
        let container_count = formatted
            .parts
            .iter()
            .filter(|x| matches!(x, FormattedStringPart::Container))
            .count();
        if container_count != formatted.parameters.len() {
            return Err(ConsoleError::length(
                container_count,
                formatted.parameters.len(),
                &formatted.span,
            ));
        }

        let mut executed_containers = Vec::with_capacity(formatted.parameters.len());
        for parameter in formatted.parameters.iter() {
            executed_containers.push(self.enforce_expression(cs, parameter.get())?.to_string());
        }

        let mut out = vec![];
        let mut parameters = executed_containers.iter();
        for part in formatted.parts.iter() {
            match part {
                FormattedStringPart::Const(c) => out.push(&**c),
                FormattedStringPart::Container => out.push(&**parameters.next().unwrap()),
            }
        }

        Ok(out.join(""))
    }
}
