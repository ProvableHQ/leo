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

use crate::{Type, VariableName};
use leo_ast::common::Variables as AstVariables;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variables {
    pub names: Vec<VariableName>,
    pub type_: Option<Type>,
}

impl<'ast> From<AstVariables<'ast>> for Variables {
    fn from(variables: AstVariables<'ast>) -> Self {
        let names = variables.names.into_iter().map(VariableName::from).collect::<Vec<_>>();

        let type_ = variables.type_.map(Type::from);

        Self { names, type_ }
    }
}

impl fmt::Display for Variables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.names.len() == 1 {
            // mut a
            write!(f, "{}", self.names[0])?;
        } else {
            // (a, mut b)
            let names = self
                .names
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(",");

            write!(f, "({})", names)?;
        }

        if self.type_.is_some() {
            write!(f, ": {}", self.type_.as_ref().unwrap())?;
        }

        write!(f, "")
    }
}
