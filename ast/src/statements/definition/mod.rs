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

use crate::{Expression, Node, Span, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

mod variable_name;
pub use variable_name::*;

mod declare;
pub use declare::*;
use leo_grammar::statements::DefinitionStatement as GrammarDefinitionStatement;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct DefinitionStatement {
    pub declaration_type: Declare,
    pub variable_names: Vec<VariableName>,
    pub type_: Option<Type>,
    pub value: Expression,
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

        if self.type_.is_some() {
            write!(f, ": {}", self.type_.as_ref().unwrap())?;
        }
        write!(f, " = {};", self.value)
    }
}

impl<'ast> From<GrammarDefinitionStatement<'ast>> for DefinitionStatement {
    fn from(statement: GrammarDefinitionStatement<'ast>) -> Self {
        let variable_names = statement
            .variables
            .names
            .into_iter()
            .map(VariableName::from)
            .collect::<Vec<_>>();

        let type_ = statement.variables.type_.map(Type::from);

        DefinitionStatement {
            declaration_type: Declare::from(statement.declare),
            variable_names,
            type_,
            value: Expression::from(statement.expression),
            span: Span::from(statement.span),
        }
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
