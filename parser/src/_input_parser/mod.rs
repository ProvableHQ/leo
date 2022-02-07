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

use crate::common::ParserContext;

use leo_ast::*;
use leo_errors::emitter::Handler;
use leo_errors::Result;

/// Creates a new program from a given file path and source code text.
pub fn parse(handler: &Handler, path: &str, source: &str) -> Result<Program> {
    let mut tokens = ParserContext::new(handler, crate::tokenize(path, source.into())?);

    tokens.parse_program()
}
