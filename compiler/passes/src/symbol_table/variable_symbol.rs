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

use std::fmt::Display;

use leo_ast::{ParamMode, Type};
use leo_span::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    Const,
    Input(ParamMode),
    Mut,
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Declaration::*;

        match self {
            Const => write!(f, "const var"),
            Input(m) => write!(f, "{m} input"),
            Mut => write!(f, "mut var"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableSymbol<'a> {
    pub type_: &'a Type,
    pub span: Span,
    pub declaration: Declaration,
}

impl<'a> Display for VariableSymbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.declaration, self.type_)?;
        Ok(())
    }
}
