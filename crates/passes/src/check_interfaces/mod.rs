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

mod visitor;
use visitor::*;

use crate::{CompilerState, Pass};

use leo_ast::ProgramVisitor;
use leo_errors::Result;

/// A pass to validate interface inheritance soundness.
///
/// Validates:
/// 1. Interface-to-interface inheritance has no conflicting members
/// 2. Programs implement all required interface members
/// 3. Signature matching is exact
pub struct CheckInterfaces;

impl Pass for CheckInterfaces {
    type Input = ();
    type Output = ();

    const NAME: &'static str = "CheckInterfaces";

    fn do_pass(_: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = CheckInterfacesVisitor::new(state);
        visitor.visit_program(ast.as_repr());
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}
