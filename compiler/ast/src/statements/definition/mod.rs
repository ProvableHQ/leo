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

use crate::{Expression, Node, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

mod variable_name;
pub use variable_name::*;

mod declare;
pub use declare::*;

/// A `let` or `const` declaration statement.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct DefinitionStatement {
    /// What sort of declaration is this? `let` or `const`?.
    pub declaration_type: Declare,
    /// The bindings / variable names to declare.
    pub variable_names: Vec<VariableName>,
    /// Tracks whether the variable(s) are in parens.
    pub parened: bool,
    /// The types of the bindings, if specified, or inferred otherwise.
    pub type_: Type,
    /// An initializer value for the bindings.
    pub value: Expression,
    /// The span excluding the semicolon.
    pub span: Span,
}

impl fmt::Display for DefinitionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.declaration_type)?;
        if self.variable_names.len() == 1 {
            // mut a
            write!(f, "{}", self.variable_names[0])?;
        } else {
            // (a, mut b)
            let names = self
                .variable_names
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");

            write!(f, "({})", names)?;
        }

        write!(f, ": {}", self.type_)?;
        write!(f, " = {};", self.value)
    }
}

impl Node for DefinitionStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
