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

//! Enforces an iteration statement in a compiled Leo program.

use crate::{
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    IndicatorAndConstrainedValue,
    Integer,
    IntegerTrait,
    StatementResult,
};
use leo_asg::IterationStatement;
use leo_errors::{new_backtrace, CompilerError};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{boolean::Boolean, integers::uint::UInt32};
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_iteration_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: &Boolean,
        statement: &IterationStatement<'a>,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<'a, F, G>>> {
        let mut results = vec![];

        let span = statement.span.clone().unwrap_or_default();

        let from = self
            .enforce_index(cs, statement.start.get(), &span)?
            .to_usize()
            .ok_or_else(|| CompilerError::statement_loop_index_const(&span, new_backtrace()))?;
        let to = self
            .enforce_index(cs, statement.stop.get(), &span)?
            .to_usize()
            .ok_or_else(|| CompilerError::statement_loop_index_const(&span, new_backtrace()))?;

        let iter: Box<dyn Iterator<Item = usize>> = match (from < to, statement.inclusive) {
            (true, true) => Box::new(from..=to),
            (true, false) => Box::new(from..to),
            (false, true) => Box::new((to..=from).rev()),
            // add the range to the values to get correct bound
            (false, false) => Box::new(((to + 1)..(from + 1)).rev()),
        };

        for i in iter {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let variable = statement.variable.borrow();

            // todo: replace definition with var typed
            self.store(
                variable.id,
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(i as u32))),
            );

            // Evaluate statements and possibly return early
            let result = self.enforce_statement(
                &mut cs.ns(|| format!("for loop iteration {} {}:{}", i, &span.line_start, &span.col_start)),
                indicator,
                statement.body.get(),
            )?;

            results.extend(result);
        }

        Ok(results)
    }
}
