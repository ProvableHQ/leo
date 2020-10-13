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

//! Enforces a branch of a conditional or iteration statement in a compiled Leo program.

use crate::{program::ConstrainedProgram, GroupType, IndicatorAndConstrainedValue, StatementResult};
use leo_typed::{Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_branch<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statements: Vec<Statement>,
        return_type: Option<Type>,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<F, G>>> {
        let mut results = vec![];
        // Evaluate statements. Only allow a single return argument to be returned.
        for statement in statements.iter() {
            let mut value = self.enforce_statement(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                indicator,
                statement.clone(),
                return_type.clone(),
                "".to_owned(),
            )?;

            results.append(&mut value);
        }

        Ok(results)
    }
}
