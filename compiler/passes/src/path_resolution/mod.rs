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

//! A transform pass that resolves and reconstructs AST paths by prefixing them with the current module path.
//!
//! The main goal is to fully qualify all paths by prepending the current module's path segments
//! before semantic analysis is performed. Since semantic information (e.g., symbol resolution)
//! is not yet available at this stage, this pass conservatively prefixes all paths to ensure
//! that references to items (functions, types, constants, structs) are correctly scoped.
//!
//! # Key behaviors:
//! - Composite types have their `resolved_path` set to the concatenation of the current module path and the type's path.
//! - Function call expressions have their function path fully qualified similarly.
//! - Struct initializers have their paths prefixed, and their member expressions recursively reconstructed.
//! - Standalone paths are prefixed as well, accounting for possible global constants.
//!
//! # Note:
//! This pass does not perform full semantic resolution; it prepares the AST paths for later
//! stages by making all paths absolute or fully qualified relative to the current module context.
//!
//! # Example
//!
//! Input (in module `foo`):
//! ```leo
//! struct Bar { x: u32 }
//! const Y: u32 = 1;
//! transition t() { let z = Bar { x: Y }; }
//! ```
//!
//! After `PathResolution`, all relevant paths are qualified with `foo::`:
//! ```leo
//! struct Bar { x: u32 }
//! const Y: u32 = 1;
//! transition t() { let z = foo::Bar { x: foo::Y }; }
//! ```

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct PathResolution;

impl Pass for PathResolution {
    type Input = ();
    type Output = ();

    const NAME: &str = "PathResolution";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = PathResolutionVisitor { state, module: Vec::new() };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;

        Ok(())
    }
}
