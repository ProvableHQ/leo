use crate::{ast::Rule, common::VariableName, types::Type, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::variables))]
pub struct Variables<'ast> {
    pub names: Vec<VariableName<'ast>>,
    pub type_: Option<Type<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Variables<'ast> {
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
