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

mod ast;
mod program;
mod visitor;

use std::marker::PhantomData;

use leo_ast::ProgramVisitor;
use leo_errors::Result;
use leo_passes::{CompilerState, Pass};

use visitor::LateLintingVisitor;

use crate::{context::LateContext, diagnostics::DiagnosticReport, lints::get_late_lints};

/// A pass to perform late lints after type checking.
pub struct LateLinting<'ctx>(PhantomData<&'ctx ()>);

impl<'ctx> Pass for LateLinting<'ctx> {
    type Input = &'ctx DiagnosticReport;
    type Output = ();

    const NAME: &'static str = "late linting";

    fn do_pass(input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let context = LateContext::new(input, state);
        let lints = get_late_lints(context);
        let mut visitor = LateLintingVisitor { lints };
        visitor.visit_program(&ast.ast);
        drop(visitor);
        state.ast = ast;
        Ok(())
    }
}
