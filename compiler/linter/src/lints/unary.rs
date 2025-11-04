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

use leo_ast::{Expression, LiteralVariant, Node, UnaryExpression, UnaryOperation};
use leo_errors::Lint;

use crate::{context::LateContext, passes::LateLintPass};

/// This linter performs linting on all lints that come under the
/// unary category.
pub(super) struct UnaryLints<'ctx> {
    context: LateContext<'ctx>,
}

impl<'ctx> LateLintPass<'ctx> for UnaryLints<'ctx> {
    fn new(context: LateContext<'ctx>) -> Box<dyn LateLintPass<'ctx> + 'ctx> {
        Box::new(Self { context })
    }

    fn get_name(&self) -> &str {
        "unary"
    }

    fn check_expression(&mut self, expr: &Expression) {
        if let Expression::Unary(unary_expr) = expr {
            self.double_negation_and_nonminimal_boolean(unary_expr);
        }
    }
}

impl<'ctx> UnaryLints<'ctx> {
    fn double_negation_and_nonminimal_boolean(&self, input: &UnaryExpression) {
        if matches!(input.op, UnaryOperation::Not) {
            if let Expression::Unary(unary) = &input.receiver
                && matches!(unary.op, UnaryOperation::Not)
            {
                self.context.emit_lint(Lint::nonminimal_expression("double_negation", input.span()));
            }

            if matches!(&input.receiver, Expression::Literal(lit) if matches!(lit.variant, LiteralVariant::Boolean(_)))
            {
                self.context.emit_lint(Lint::nonminimal_expression("nonminimal_boolean", input.span()));
            }
        }
    }
}
