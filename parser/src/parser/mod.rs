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

use std::unimplemented;

use crate::{tokenizer::*, DeprecatedError, ParserContext, SyntaxError, Token};
use indexmap::IndexMap;
use leo_ast::*;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

mod expression;
mod file;
mod statement;
mod type_;

pub fn parse(path: &str, script: &str) -> SyntaxResult<Program> {
    let mut tokens = ParserContext::new(crate::tokenize(script, path)?);

    match tokens.parse_program() {
        Ok(x) => Ok(x),
        Err(mut e) => {
            e.set_path(
                path,
                &script.lines().map(|x| x.to_string()).collect::<Vec<String>>()[..],
            );
            Err(e)
        }
    }
}
