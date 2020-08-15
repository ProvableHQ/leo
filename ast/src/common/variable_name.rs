use crate::{
    ast::Rule,
    common::{Identifier, Mutable},
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::variable_name))]
pub struct VariableName<'ast> {
    pub mutable: Option<Mutable>,
    pub identifier: Identifier<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for VariableName<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref _mutable) = self.mutable {
            write!(f, "mut ")?;
        }

        write!(f, "{}", self.identifier)
    }
}
