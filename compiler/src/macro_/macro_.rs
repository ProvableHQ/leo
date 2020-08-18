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

//! Evaluates a macro in a compiled Leo program.

use crate::{errors::MacroError, program::ConstrainedProgram, GroupType};
use leo_typed::{FormattedMacro, MacroName};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_macro<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        macro_: FormattedMacro,
    ) -> Result<(), MacroError> {
        let string = macro_
            .string
            .map(|string| self.format(cs, file_scope, function_scope, string))
            .unwrap_or(Ok("".to_string()))?;

        match macro_.name {
            MacroName::Debug(_) => log::debug!("{}", string),
            MacroName::Error(_) => log::error!("{}", string),
            MacroName::Print(_) => println!("{}", string),
        }

        Ok(())
    }
}
