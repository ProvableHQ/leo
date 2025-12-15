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

use leo_ast::{BinaryExpression, BinaryOperation, Expression, Node};
use leo_errors::Lint;

use crate::{context::LateContext, passes::LateLintPass};

/// This linter performs linting checks on various common
/// code patterns for binary operators.
pub(super) struct BinaryLints<'ctx> {
    context: LateContext<'ctx>,
}

impl<'ctx> LateLintPass<'ctx> for BinaryLints<'ctx> {
    fn new(context: LateContext<'ctx>) -> Box<dyn LateLintPass<'ctx> + 'ctx> {
        Box::new(Self { context })
    }

    fn get_name(&self) -> &str {
        "binary"
    }

    fn check_expression(&mut self, expr: &Expression) {
        if let Expression::Binary(binary_expr) = expr {
            BinaryLinter { context: self.context, input: binary_expr.as_ref() }.lint();
        }
    }
}

struct BinaryLinter<'ctx> {
    context: LateContext<'ctx>,
    input: &'ctx BinaryExpression,
}

impl BinaryLinter<'_> {
    fn lint(&self) {
        self.addition_with_zero();
        self.divison_by_one();
        self.multiplication_by_one();
        self.divison_by_zero();
        self.irrefutable_pattern();
    }

    fn addition_with_zero(&self) {
        if [BinaryOperation::Add, BinaryOperation::AddWrapped].contains(&self.input.op) {
            let name = if matches!(self.input.left.as_u32(), Some(0)) {
                &self.input.right
            } else if matches!(self.input.right.as_u32(), Some(0)) {
                &self.input.left
            } else {
                return;
            };

            self.context.emit_lint(Lint::identity_op(name, self.input.span()));
        }
    }

    fn multiplication_by_one(&self) {
        if [BinaryOperation::Mul, BinaryOperation::MulWrapped].contains(&self.input.op) {
            let name = if matches!(self.input.left.as_u32(), Some(1)) {
                &self.input.right
            } else if matches!(self.input.right.as_u32(), Some(1)) {
                &self.input.left
            } else {
                return;
            };

            self.context.emit_lint(Lint::identity_op(name, self.input.span()));
        }
    }

    fn divison_by_one(&self) {
        if [BinaryOperation::Div, BinaryOperation::DivWrapped].contains(&self.input.op)
            && matches!(self.input.right.as_u32(), Some(1))
        {
            self.context.emit_lint(Lint::identity_op(&self.input.left, self.input.span()));
        }
    }

    fn divison_by_zero(&self) {
        if [BinaryOperation::Div, BinaryOperation::DivWrapped].contains(&self.input.op)
            && matches!(self.input.right.as_u32(), Some(0))
        {
            self.context.emit_lint(Lint::divison_by_zero(self.input.span()));
        }
    }

    fn irrefutable_pattern(&self) {
        if let (Expression::Literal(left), Expression::Literal(right)) = (&self.input.left, &self.input.right) {
            match self.input.op {
                BinaryOperation::Eq if left.variant == right.variant => {}
                BinaryOperation::Neq if left.variant != right.variant => {}
                _ => return,
            }

            self.context.emit_lint(Lint::irrefutable_pattern(self.input.span()));
        }
    }
}
