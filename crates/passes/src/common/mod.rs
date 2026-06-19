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

use crate::CompilerState;
use leo_ast::{Expression, Intrinsic};

mod assigner;
pub use assigner::*;

mod block_to_function_rewriter;
pub use block_to_function_rewriter::*;

mod rename_table;
pub use rename_table::*;

mod replacer;
pub use replacer::*;

mod item_walkers;
pub use item_walkers::*;

mod symbol_access_collector;
pub use symbol_access_collector::*;

mod symbol_table;
pub use symbol_table::*;

mod tree_node;
pub use tree_node::ConditionalTreeNode;

mod type_table;
pub use type_table::*;

/// Returns true if `expr` can be removed without changing eager evaluation.
///
/// This is stricter than [`Expression::is_pure`]: source-level operations that
/// can lower to assertions or storage reads must still be evaluated even when
/// their result is unused.
pub(crate) fn expression_can_be_discarded(expr: &Expression, state: &CompilerState) -> bool {
    expr.is_pure(&|id| state.type_table.get(&id).expect("Types should be assigned."))
        && !contains_non_discardable_operation(expr, state)
}

fn contains_non_discardable_operation(expr: &Expression, state: &CompilerState) -> bool {
    match expr {
        Expression::Intrinsic(intr) => {
            let intrinsic = Intrinsic::from_symbol(intr.name, &intr.type_parameters);
            intrinsic.is_some_and(|intrinsic| !intrinsic_can_be_discarded(&intrinsic))
                || intr.arguments.iter().any(|arg| contains_non_discardable_operation(arg, state))
        }
        Expression::Array(expr) => expr.elements.iter().any(|elem| contains_non_discardable_operation(elem, state)),
        Expression::ArrayAccess(expr) => {
            contains_non_discardable_operation(&expr.array, state)
                || contains_non_discardable_operation(&expr.index, state)
        }
        Expression::Binary(expr) => {
            contains_non_discardable_operation(&expr.left, state)
                || contains_non_discardable_operation(&expr.right, state)
        }
        Expression::Composite(expr) => {
            expr.const_arguments.iter().any(|arg| contains_non_discardable_operation(arg, state))
                || expr
                    .members
                    .iter()
                    // Const propagation and SSA make shorthand members explicit before
                    // this check is used.
                    .filter_map(|init| init.expression.as_ref())
                    .any(|member| contains_non_discardable_operation(member, state))
        }
        Expression::MemberAccess(expr) => contains_non_discardable_operation(&expr.inner, state),
        Expression::Repeat(expr) => {
            contains_non_discardable_operation(&expr.expr, state)
                || contains_non_discardable_operation(&expr.count, state)
        }
        Expression::Ternary(expr) => {
            contains_non_discardable_operation(&expr.condition, state)
                || contains_non_discardable_operation(&expr.if_true, state)
                || contains_non_discardable_operation(&expr.if_false, state)
        }
        Expression::Tuple(expr) => expr.elements.iter().any(|elem| contains_non_discardable_operation(elem, state)),
        Expression::TupleAccess(expr) => contains_non_discardable_operation(&expr.tuple, state),
        Expression::Unary(expr) => contains_non_discardable_operation(&expr.receiver, state),
        Expression::Path(path) => path_is_storage_read(path, state),
        Expression::Async(_)
        | Expression::Call(_)
        | Expression::Cast(_)
        | Expression::DynamicOp(_)
        | Expression::Err(_)
        | Expression::Literal(_)
        | Expression::Unit(_) => false,
    }
}

fn path_is_storage_read(path: &leo_ast::Path, state: &CompilerState) -> bool {
    path.try_global_location()
        .and_then(|location| state.symbol_table.lookup_global(location.program, location))
        .is_some_and(|var| {
            var.declaration == symbol_table::VariableType::Storage
                && var.type_.as_ref().is_none_or(|type_| !type_.is_mapping())
        })
}

fn intrinsic_can_be_discarded(intrinsic: &Intrinsic) -> bool {
    !matches!(
        intrinsic,
        // Lowers to an assertion on the option tag.
        Intrinsic::OptionalUnwrap
            // Lower to storage reads before code generation.
            | Intrinsic::VectorGet
            | Intrinsic::VectorLen
    ) && intrinsic.is_pure()
}
