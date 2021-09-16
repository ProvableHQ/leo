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

//! Enforces a value access expression in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{NamedTypeAccess, Node};
use leo_errors::{CompilerError, Result};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_named_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        access: &NamedTypeAccess<'a>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        //todo: we can prob pass values by ref here to avoid copying the entire circuit on access
        let named_type = access.named_type.get();
        let named_type_value = self.enforce_expression(cs, named_type)?;

        let access = access.access.get();
        let _access_value = self.enforce_expression(cs, access)?;

        // TODO
        match named_type_value {
            _ => {
                return Err(CompilerError::statement_array_assign_index_const(
                    &named_type.span().cloned().unwrap_or_default(),
                )
                .into())
            }
        }
    }
}
