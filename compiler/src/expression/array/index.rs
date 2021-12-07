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

//! Enforces an array index expression in a compiled Leo program.

use crate::program::Program;
use leo_asg::Expression;
use leo_errors::{CompilerError, Result};
use leo_span::Span;
use snarkvm_ir::Value;

impl<'a> Program<'a> {
    pub(crate) fn enforce_index(&mut self, index: &'a Expression<'a>, span: &Span) -> Result<Value> {
        match self.enforce_expression(index)? {
            value @ Value::Ref(_) => Ok(value),
            value @ Value::Integer(_) => Ok(value),
            value => Err(CompilerError::invalid_index_expression(value, span).into()),
        }
    }
}
