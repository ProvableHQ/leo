// Copyright (C) 2019-2024 Aleo Systems Inc.
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

mod generator;
pub use generator::TestManifestGenerator;

use crate::Pass;

use leo_ast::{Ast, ProgramVisitor, TestManifest};
use leo_errors::{Result, emitter::Handler};

use snarkvm::prelude::Network;

impl<'a, N: Network> Pass for TestManifestGenerator<'a, N> {
    type Input = (&'a Ast, &'a Handler);
    type Output = Result<TestManifest<N>>;

    fn do_pass((ast, handler): Self::Input) -> Self::Output {
        let mut visitor = TestManifestGenerator::<N>::new(handler);
        visitor.visit_program(ast.as_repr());

        handler.last_err().map_err(|e| *e)?;

        // Get the generated manifest.
        let Some(manifest) = visitor.manifest.take() else {
            unreachable!("Every test program should have an associated manifest")
        };
        Ok(manifest)
    }
}
