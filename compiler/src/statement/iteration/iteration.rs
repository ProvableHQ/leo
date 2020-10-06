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

//! Enforces an iteration statement in a compiled Leo program.

use crate::{
    new_scope,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    IndicatorAndConstrainedValue,
    Integer,
    StatementResult,
};
use leo_typed::{Expression, Identifier, Span, Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, uint::UInt32},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_iteration_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        index: Identifier,
        start: Expression,
        stop: Expression,
        statements: Vec<Statement>,
        return_type: Option<Type>,
        span: Span,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<F, G>>> {
        let mut results = vec![];

        let from = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), start, span.clone())?;
        let to = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), stop, span.clone())?;

        for i in from..to {
            // Store index in current function scope.
            // For loop scope is not implemented.

            let index_name = new_scope(function_scope.clone(), index.to_string());

            self.store(
                index_name,
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(i as u32))),
            );

            // Evaluate statements and possibly return early
            let name_unique = format!("for loop iteration {} {}:{}", i, span.line, span.start);
            let mut result = self.evaluate_branch(
                &mut cs.ns(|| name_unique),
                file_scope.clone(),
                function_scope.clone(),
                indicator,
                statements.clone(),
                return_type.clone(),
            )?;

            results.append(&mut result);
        }

        Ok(results)
    }
}
