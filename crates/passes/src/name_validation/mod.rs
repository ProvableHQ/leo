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

mod program;

mod ast;

mod visitor;
use visitor::*;

use crate::{CompilerState, Pass};

use leo_ast::ProgramVisitor;
use leo_errors::Result;

/// A pass to validate names.
///
/// Enforces various naming rules such as preventing SnarkVM keywords or the use of "aleo" where relevant.
pub struct NameValidation;

impl Pass for NameValidation {
    type Input = ();
    type Output = ();

    const NAME: &'static str = "NameValidation";

    fn do_pass(_: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let mut visitor = NameValidationVisitor { handler: &mut state.handler };
        visitor.visit_program(state.ast.as_repr());
        state.handler.last_err()?;

        Ok(())
    }
}
