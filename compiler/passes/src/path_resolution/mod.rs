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

//! A transform pass that resolves AST paths into explicit local or global references
//! and constructs all local scopes in the symbol table.
//!
//! This pass walks the AST and transforms each `Path` from an unresolved, user-written
//! form into either a local or global target, based on syntactic context and the current
//! symbol table state. Global paths are resolved to fully qualified `Location`s, while
//! local paths are resolved to concrete local symbols. Unresolvable paths are reported
//! as errors.
//!
//! In addition to path resolution, this pass is responsible for creating all local
//! scopes and inserting local bindings (inputs, const parameters, local variables,
//! loop iterators, etc.) into the symbol table. After this pass completes, the symbol
//! table contains a complete and accurate representation of all lexical scopes.
//!
//! # Key behaviors:
//! - Paths with qualifiers are always resolved as global paths.
//! - Unqualified paths are resolved as global or local based on symbol table lookup.
//! - Global paths are resolved relative to the current module and program context.
//! - Local scopes are created for functions, blocks, composites, constructors, and loops.
//! - Local variables are inserted with their declaration kind, but without final types.
//!
//! # Pipeline position:
//! This pass runs after `GlobalVarsCollection` and before `GlobalItemsCollection`.
//! Subsequent passes (e.g. type checking) assume that all paths are resolved and
//! that all scopes already exist, and therefore do not create or mutate scopes.

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

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
        let mut visitor = PathResolutionVisitor { state, program: Symbol::intern(""), module: Vec::new() };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        Ok(())
    }
}
