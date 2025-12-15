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
mod r#macro;
mod program;
mod visitor;

use leo_parser_lossless::SyntaxNode;
use leo_passes::{CompilerState, Pass};

use crate::{context::EarlyContext, diagnostics::DiagnosticReport, lints::get_early_lints, passes::early::visitor::*};

use std::marker::PhantomData;

/// A pass to perform early lints before type checking on the raw CST.
pub struct EarlyLinting<'a>(PhantomData<&'a ()>);

impl<'a> Pass for EarlyLinting<'a> {
    type Input = EarlyLintingInput<'a>;
    type Output = DiagnosticReport;

    const NAME: &'static str = "early linting";

    fn do_pass(input: Self::Input, _state: &mut CompilerState) -> leo_errors::Result<Self::Output> {
        let report = DiagnosticReport::default();
        let context = EarlyContext::new(&report);
        let lints = get_early_lints(context);
        let mut visitor = EarlyLintingVisitor { lints };

        if let Some(tree) = input.program_tree {
            visitor.visit_main(tree);
        }

        for tree in input.module_trees {
            visitor.visit_module(tree);
        }

        drop(visitor);

        Ok(report)
    }
}

pub struct EarlyLintingInput<'a> {
    pub(crate) module_trees: &'a [&'a SyntaxNode<'a>],
    pub(crate) program_tree: Option<&'a SyntaxNode<'a>>,
}
