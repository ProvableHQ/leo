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
use leo_ast::{Block, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Evaluates a branch of one or more statements and returns a result in
    /// the given scope.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_block<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: &Boolean,
        block: Block,
        return_type: Option<Type>,
        declared_circuit_reference: &str,
        mut_self: bool,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<F, G>>> {
        let mut results = Vec::with_capacity(block.statements.len());
        // Evaluate statements. Only allow a single return argument to be returned.
        for statement in block.statements.into_iter() {
            let value = self.enforce_statement(
                cs,
                file_scope,
                function_scope,
                indicator,
                statement,
                return_type.clone(),
                declared_circuit_reference,
                mut_self,
            )?;

            results.extend(value);
        }

        Ok(results)
    }
}
