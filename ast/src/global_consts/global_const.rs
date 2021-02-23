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

use crate::{
    statements::{Declare, VariableName},
    Expression,
    Node,
    Span,
    Type,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GlobalConst {
    pub declaration_type: Declare,
    pub variable_name: VariableName,
    pub type_: Option<Type>,
    pub value: Expression,
    pub span: Span,
}

impl fmt::Display for GlobalConst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.declaration_type, self.variable_name)?;

        if self.type_.is_some() {
            write!(f, ": {}", self.type_.as_ref().unwrap())?;
        }
        write!(f, " = {};", self.value)
    }
}

impl Node for GlobalConst {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
