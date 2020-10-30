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

//! Stores all defined names in a compiled Leo program.

use crate::{
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_core_ast::Identifier;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn store_definition(
        &mut self,
        function_scope: &str,
        mutable: bool,
        identifier: Identifier,
        mut value: ConstrainedValue<F, G>,
    ) {
        // Store with given mutability
        if mutable {
            value = ConstrainedValue::Mutable(Box::new(value));
        }

        let variable_program_identifier = new_scope(function_scope, &identifier.name);

        self.store(variable_program_identifier, value);
    }
}
