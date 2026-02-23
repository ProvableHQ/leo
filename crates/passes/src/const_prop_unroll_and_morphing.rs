// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{
    CompilerState,
    ConstPropagation,
    GlobalItemsCollection,
    GlobalVarsCollection,
    Monomorphization,
    Pass,
    PathResolution,
    RemoveUnreachable,
    TypeChecking,
    TypeCheckingInput,
    Unrolling,
};

use leo_ast::Node;
use leo_errors::{CompilerError, Result};

/// Pass that runs const propagation, loop unrolling, and monomorphization until a fixed point.
pub struct ConstPropUnrollAndMorphing;

impl Pass for ConstPropUnrollAndMorphing {
    type Input = TypeCheckingInput;
    type Output = ();

    const NAME: &str = "ConstantPropogation+LoopUnrolling+Monomorphization";

    fn do_pass(input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        const LARGE_LOOP_BOUND: usize = 1024usize;

        for _ in 0..LARGE_LOOP_BOUND {
            let loop_unroll_output = Unrolling::do_pass((), state)?;

            let const_prop_output = ConstPropagation::do_pass((), state)?;

            let remove_unreachable_output = RemoveUnreachable::do_pass((), state)?;

            let monomorphization_output = Monomorphization::do_pass((), state)?;

            // Clear the symbol table and create it again. This is important because after all the passes above run, the
            // program may have changed significantly (new functions may have been added, some functions may have been
            // deleted, etc.) We do want to retain evaluated consts, so that const propagation can tell when it has evaluated a new one.
            state.symbol_table.reset_but_consts();
            GlobalVarsCollection::do_pass((), state)?;
            PathResolution::do_pass((), state)?;
            GlobalItemsCollection::do_pass((), state)?;

            // Now run the type checker again to validate and infer types. Again, this is important because the program
            // may have changed significantly after the passes above.
            TypeChecking::do_pass(input.clone(), state)?;

            if !const_prop_output.changed
                && !loop_unroll_output.loop_unrolled
                && !monomorphization_output.changed
                && !remove_unreachable_output.changed
            {
                // We've got a fixed point, so see if we have any errors.
                if let Some(not_evaluated_span) = const_prop_output.const_not_evaluated {
                    return Err(CompilerError::const_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_evaluated_span) = const_prop_output.array_index_not_evaluated {
                    return Err(CompilerError::array_index_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_evaluated_span) = const_prop_output.repeat_count_not_evaluated {
                    return Err(CompilerError::repeat_count_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_evaluated_span) = const_prop_output.array_length_not_evaluated {
                    return Err(CompilerError::array_length_not_evaluated(not_evaluated_span).into());
                }

                // Emit errors for all problematic calls
                for call in &monomorphization_output.unresolved_calls {
                    if let Some(arg) =
                        call.const_arguments.iter().find(|arg| !matches!(arg, leo_ast::Expression::Literal(_)))
                    {
                        state.handler.emit_err(CompilerError::const_generic_not_resolved(
                            "call to generic function",
                            call.function.clone(),
                            arg.span(),
                        ));
                    }
                }

                // Emit errors for all problematic composite expressions
                for expr in &monomorphization_output.unresolved_composite_exprs {
                    if let Some(arg) =
                        expr.const_arguments.iter().find(|arg| !matches!(arg, leo_ast::Expression::Literal(_)))
                    {
                        state.handler.emit_err(CompilerError::const_generic_not_resolved(
                            "composite expression",
                            expr.path.clone(),
                            arg.span(),
                        ));
                    }
                }

                // Emit errors for all problematic composite type instantiations
                for ty in &monomorphization_output.unresolved_composite_types {
                    if let Some(arg) =
                        ty.const_arguments.iter().find(|arg| !matches!(arg, leo_ast::Expression::Literal(_)))
                    {
                        state.handler.emit_err(CompilerError::const_generic_not_resolved(
                            "composite type",
                            ty.path.clone(),
                            arg.span(),
                        ));
                    }
                }

                // Exit with the handler's last error.
                state.handler.last_err()?;

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
