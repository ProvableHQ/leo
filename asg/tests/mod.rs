// Copyright (C) 2019-2022 Aleo Systems Inc.
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
use leo_ast::AstPass;
use leo_errors::emitter::{ErrBuffer, Handler};
use leo_errors::LeoError;
use leo_parser::parse_ast;
use leo_span::symbol::create_session_if_not_set_then;

mod fail;
mod pass;

const TESTING_FILEPATH: &str = "input.leo";

fn load_asg(program_string: &str) -> Result<Program<'static>, ErrBuffer> {
    create_session_if_not_set_then(|_| Handler::with(|h| load_asg_imports(h, make_test_context(), program_string)))
}

fn load_asg_imports<'a>(
    handler: &Handler,
    context: AsgContext<'a>,
    program_string: &str,
) -> Result<Program<'a>, LeoError> {
    let ast = parse_ast(handler, &TESTING_FILEPATH, program_string)?;
    let ast = leo_ast_passes::Canonicalizer::do_pass(Default::default(), ast.into_repr())?;
    Program::new(context, &ast.as_repr())
}

//convenience function for tests, leaks memory
pub(crate) fn make_test_context() -> AsgContext<'static> {
    let allocator = Box::leak(Box::new(new_alloc_context()));
    new_context(allocator)
}
