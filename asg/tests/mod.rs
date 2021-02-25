// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use leo_asg::*;
use leo_ast::Ast;
use leo_grammar::Grammar;

use std::path::Path;

mod fail;
mod pass;

const TESTING_FILEPATH: &str = "input.leo";
const TESTING_PROGRAM_NAME: &str = "test_program";

fn load_asg(program_string: &str) -> Result<Program<'static>, AsgConvertError> {
    load_asg_imports(make_test_context(), program_string, &mut NullImportResolver)
}

fn load_asg_imports<'a, T: ImportResolver<'a>>(
    context: AsgContext<'a>,
    program_string: &str,
    imports: &mut T,
) -> Result<Program<'a>, AsgConvertError> {
    let grammar = Grammar::new(Path::new(&TESTING_FILEPATH), program_string)?;
    let ast = Ast::new(TESTING_PROGRAM_NAME, &grammar)?;
    InternalProgram::new(context, &ast.as_repr(), imports)
}

fn mocked_resolver<'a>(_context: AsgContext<'a>) -> MockedImportResolver<'a> {
    let packages = indexmap::IndexMap::new();
    MockedImportResolver { packages }
}

//convenience function for tests, leaks memory
pub(crate) fn make_test_context() -> AsgContext<'static> {
    let allocator = Box::leak(Box::new(new_alloc_context()));
    new_context(allocator)
}
