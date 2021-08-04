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

//! Resolves assignees in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::Expression;
use leo_errors::{CompilerError, LeoError};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

use super::ResolverContext;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub(super) fn resolve_target_access_array_range<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        mut context: ResolverContext<'a, 'b, F, G>,
        start: Option<&'a Expression<'a>>,
        stop: Option<&'a Expression<'a>>,
    ) -> Result<(), LeoError> {
        let start_index = start
            .map(|start| self.enforce_index(cs, start, &context.span))
            .transpose()?
            .map(|x| {
                x.to_usize()
                    .ok_or_else(|| CompilerError::statement_array_assign_index_const(&context.span))
            })
            .transpose()?;
        let stop_index = stop
            .map(|stop| self.enforce_index(cs, stop, &context.span))
            .transpose()?
            .map(|x| {
                x.to_usize()
                    .ok_or_else(|| CompilerError::statement_array_assign_index_const(&context.span))
            })
            .transpose()?;
        let start_index = start_index.unwrap_or(0);

        if !context.from_range {
            // not a range of a range
            context.from_range = true;
            match context.input.remove(0) {
                ConstrainedValue::Array(old) => {
                    let stop_index = stop_index.unwrap_or(old.len());
                    Self::check_range_index(start_index, stop_index, old.len(), &context.span)?;

                    if context.remaining_accesses.is_empty() {
                        let target_values = match context.target_value {
                            ConstrainedValue::Array(x) => x,
                            _ => unimplemented!(),
                        };

                        for (target, target_value) in old[start_index..stop_index].iter_mut().zip(target_values) {
                            context.target_value = target_value;
                            self.enforce_assign_context(cs, &context, target)?;
                        }
                    } else {
                        context.input = old[start_index..stop_index].iter_mut().collect();
                        self.resolve_target_access(cs, context)?;
                    }
                    Ok(())
                }
                _ => Err(CompilerError::statement_array_assign_index(&context.span))?,
            }
        } else {
            // range of a range
            let stop_index = stop_index.unwrap_or(context.input.len());
            Self::check_range_index(start_index, stop_index, context.input.len(), &context.span)?;

            context.input = context
                .input
                .into_iter()
                .skip(start_index)
                .take(stop_index - start_index)
                .collect();
            if context.remaining_accesses.is_empty() {
                let target_values = match context.target_value {
                    ConstrainedValue::Array(x) => x,
                    _ => unimplemented!(),
                };

                let iter = context.input.into_iter().zip(target_values.into_iter());
                context.input = vec![];
                for (target, target_value) in iter {
                    context.target_value = target_value;
                    self.enforce_assign_context(cs, &context, target)?;
                }
                Ok(())
            } else {
                self.resolve_target_access(cs, context)
            }
        }
    }
}
