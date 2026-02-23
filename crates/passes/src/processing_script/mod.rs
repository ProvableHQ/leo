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

mod ast;

mod program;

mod visitor;
use visitor::*;

use crate::{CompilerState, Pass};

use leo_ast::{ProgramReconstructor as _, Variant};
use leo_errors::Result;
use leo_span::Symbol;

/// A pass to validate (and remove) uses of `interpret`.
pub struct ProcessingScript;

impl Pass for ProcessingScript {
    type Input = ();
    type Output = ();

    const NAME: &'static str = "ProcessingScript";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);

        // We set the `current_variant` before traversing each function. We use `Fn` here as a placeholder.
        let mut visitor =
            ProcessingScriptVisitor { state, current_variant: Variant::Fn, program_name: Symbol::default() };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.ast = ast;

        Ok(())
    }
}
