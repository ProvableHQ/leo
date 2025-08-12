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

/// A transform pass that resolves and reconstructs AST paths by prefixing them with the current module path.
///
/// The main goal is to fully qualify all paths by prepending the current module's path segments
/// before semantic analysis is performed. Since semantic information (e.g., symbol resolution)
/// is not yet available at this stage, this pass conservatively prefixes all paths to ensure
/// that references to items (functions, types, constants, structs) are correctly scoped.
///
/// # Key behaviors:
/// - Composite types have their `resolved_path` set to the concatenation of the current module path and the type's path.
/// - Function call expressions have their function path fully qualified similarly.
/// - Struct initializers have their paths prefixed, and their member expressions recursively reconstructed.
/// - Standalone paths are prefixed as well, accounting for possible global constants.
///
/// # Note:
/// This pass does not perform full semantic resolution; it prepares the AST paths for later
/// stages by making all paths absolute or fully qualified relative to the current module context.
use crate::{Pass, PathResolution, SymbolTable, SymbolTableCreation, TypeChecking, TypeCheckingInput};

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct ProcessingAsync;

impl Pass for ProcessingAsync {
    type Input = TypeCheckingInput;
    type Output = ();

    const NAME: &str = "ProcessingAsync";

    fn do_pass(input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = ProcessingAsyncVisitor {
            state,
            max_inputs: input.max_inputs,
            current_program: Symbol::intern(""),
            current_function: Symbol::intern(""),
            new_async_functions: Vec::new(),
            modified: false,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;

        if visitor.modified {
            // If we actually changed anything in the program, then we need to recreate the symbol table and run type
            // checking again. That's because this pass introduces new `async function`s to the program.
            visitor.state.symbol_table = SymbolTable::default();
            SymbolTableCreation::do_pass((), state)?;
            PathResolution::do_pass((), state)?;
            TypeChecking::do_pass(input.clone(), state)?;
        }

        Ok(())
    }
}
