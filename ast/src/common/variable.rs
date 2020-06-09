use crate::{
    ast::Rule,
    common::{Identifier, Mutable},
    types::Type,
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::variable))]
pub struct Variable<'ast> {
    pub mutable: Option<Mutable>,
    pub identifier: Identifier<'ast>,
    pub _type: Option<Type<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Variable<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref _mutable) = self.mutable {
            write!(f, "mut ")?;
        }

        write!(f, "{}", self.identifier)?;

        if let Some(ref _type) = self._type {
            write!(f, ": {}", _type)?;
        }

        write!(f, "")
    }
}
