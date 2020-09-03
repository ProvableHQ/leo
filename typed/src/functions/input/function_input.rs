// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{Identifier, Span, Type};
use leo_ast::functions::FunctionInput as AstFunctionInput;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionInput {
    pub identifier: Identifier,
    pub mutable: bool,
    pub type_: Type,
    pub span: Span,
}

impl<'ast> From<AstFunctionInput<'ast>> for FunctionInput {
    fn from(parameter: AstFunctionInput<'ast>) -> Self {
        FunctionInput {
            identifier: Identifier::from(parameter.identifier),
            mutable: parameter.mutable.is_some(),
            type_: Type::from(parameter.type_),
            span: Span::from(parameter.span),
        }
    }
}

impl FunctionInput {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // mut var: bool
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: ", self.identifier)?;
        write!(f, "{}", self.type_)
    }
}

impl fmt::Display for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
