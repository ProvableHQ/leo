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

use crate::Pass;

use leo_errors::Result;

mod expression;

mod program;

mod statement;

mod type_;

mod visitor;
use visitor::*;

pub struct CodeGenerating;

impl Pass for CodeGenerating {
    type Input = ();
    type Output = String;

    const NAME: &str = "CodeGenerating";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut visitor = CodeGeneratingVisitor {
            state,
            next_register: 0,
            current_function: None,
            variable_mapping: Default::default(),
            composite_mapping: Default::default(),
            global_mapping: Default::default(),
            variant: None,
            program: &state.ast.ast,
            program_id: None,
            finalize_caller: None,
            next_label: 0,
            conditional_depth: 0,
        };
        let code = visitor.visit_program(visitor.state.ast.as_repr());
        Ok(code)
    }
}
