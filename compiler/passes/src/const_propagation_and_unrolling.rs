// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{CompilerState, ConstPropagation, Pass, Unrolling};

use leo_errors::{CompilerError, Result};

/// Pass that runs const propagation and loop unrolling until a fixed point.
pub struct ConstPropagationAndUnrolling;

impl Pass for ConstPropagationAndUnrolling {
    type Input = ();
    type Output = ();

    const NAME: &str = "ConstPropagationAndUnrolling";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        const LARGE_LOOP_BOUND: usize = 1024usize;

        for _ in 0..LARGE_LOOP_BOUND {
            let loop_unroll_output = Unrolling::do_pass((), state)?;

            let const_prop_output = ConstPropagation::do_pass((), state)?;

            if !const_prop_output.changed && !loop_unroll_output.loop_unrolled {
                // We've got a fixed point, so see if we have any errors.
                if let Some(not_evaluated_span) = const_prop_output.const_not_evaluated {
                    return Err(CompilerError::const_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_evaluated_span) = const_prop_output.array_index_not_evaluated {
                    return Err(CompilerError::array_index_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_unrolled_span) = loop_unroll_output.loop_not_unrolled {
                    return Err(CompilerError::loop_bounds_not_evaluated(not_unrolled_span).into());
                }

                return Ok(());
            }
        }

        // Note that it's challenging to write code in practice that demonstrates this error, because Leo code
        // with many nested loops or operations will blow the stack in the compiler before this bound is hit.
        Err(CompilerError::const_prop_unroll_many_loops(LARGE_LOOP_BOUND, Default::default()).into())
    }
}
