use crate::{ast::Rule, common::Identifier, imports::PackageAccess};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::package))]
pub struct Package<'ast> {
    pub name: Identifier<'ast>,
    pub access: PackageAccess<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
