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

#[macro_use]
extern crate thiserror;

pub mod tokenizer;
use leo_ast::Ast;
pub use tokenizer::*;

pub mod token;
pub use token::*;

pub mod errors;
pub use errors::*;

pub mod parser;
pub use parser::*;

pub mod context;
pub use context::*;

#[cfg(test)]
mod test;

pub fn parse_ast<T: AsRef<str>, Y: AsRef<str>>(path: T, source: Y) -> SyntaxResult<Ast> {
    Ok(Ast::new(parser::parse(path.as_ref(), source.as_ref())?))
}
