use crate::{Type, VariableName};
use leo_ast::common::Variables as AstVariables;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variables {
    pub names: Vec<VariableName>,
    pub types: Vec<Type>,
}

impl<'ast> From<AstVariables<'ast>> for Variables {
    fn from(variables: AstVariables<'ast>) -> Self {
        let names = variables
            .names
            .into_iter()
            .map(|x| VariableName::from(x))
            .collect::<Vec<_>>();

        let types = variables.types.into_iter().map(|x| Type::from(x)).collect::<Vec<_>>();

        Self { names, types }
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

        if !self.types.is_empty() {
            write!(f, ": ")?;

            if self.types.len() == 1 {
                // : u32
                write!(f, "{}", self.types[0])?;
            } else {
                // : (bool, u32)
                let types = self
                    .types
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(",");

                write!(f, "({})", types)?;
            }
        }

        write!(f, "")
    }
}
