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

//! The Force Inline Conversion pass converts `Variant::Function` to `Variant::Inline`
//! when required by the underlying limitations such as:
//! 1. When the function has any const parameters
//! 2. When the function is called by another function, constructor or inline
//! 3. When the function is called from an async block or async fn
//! 4. When the function has no arguments or empty arguments
//! 5. When the function names optional types as arguments or return values
//! 6. When the function outputs a record TODO

use crate::Pass;

use leo_ast::{ProgramReconstructor as _, ProgramVisitor as _};
use leo_errors::Result;

mod analysis;
use analysis::*;

mod transform;
use transform::*;

pub struct ForceInlineConversion;

impl Pass for ForceInlineConversion {
    type Input = ();
    type Output = ();

    const NAME: &str = "ForceInlineConversion";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        // Phase 1: Analysis - collect functions called from Function variants and async blocks
        let mut analyzer = AnalysisVisitor::new();
        analyzer.visit_program(&state.ast.ast);

        // Phase 2: Transformation - convert Function to Inline where needed
        let mut ast = std::mem::take(&mut state.ast);
        let mut transformer = TransformVisitor::new(analyzer.functions_to_inline);
        ast.ast = transformer.reconstruct_program(ast.ast);
        state.ast = ast;

        Ok(())
    }
}
