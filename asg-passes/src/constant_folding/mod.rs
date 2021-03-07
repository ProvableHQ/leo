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

use std::cell::Cell;

use leo_asg::*;

pub struct ConstantFolding<'a, 'b> {
    program: &'b Program<'a>,
}

impl<'a, 'b> ExpressionVisitor<'a> for ConstantFolding<'a, 'b> {
    fn visit_expression(&mut self, input: &Cell<&Expression<'a>>) -> VisitResult {
        let expr = input.get();
        if let Some(const_value) = expr.const_value() {
            let folded_expr = Expression::Constant(Constant {
                parent: Cell::new(expr.get_parent()),
                span: expr.span().cloned(),
                value: const_value,
            });
            let folded_expr = self.program.context.alloc_expression(folded_expr);
            input.set(folded_expr);
            VisitResult::SkipChildren
        } else {
            VisitResult::VisitChildren
        }
    }
}

impl<'a, 'b> StatementVisitor<'a> for ConstantFolding<'a, 'b> {}

impl<'a, 'b> ProgramVisitor<'a> for ConstantFolding<'a, 'b> {}

impl<'a, 'b> AsgPass<'a> for ConstantFolding<'a, 'b> {
    fn do_pass(asg: Program<'a>) -> Result<Program<'a>, FormattedError> {
        let pass = ConstantFolding { program: &asg };
        let mut director = VisitorDirector::new(pass);
        director.visit_program(&asg).ok();
        Ok(asg)
    }
}
