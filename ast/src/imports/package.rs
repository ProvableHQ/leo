use crate::{ast::Rule, common::Identifier, imports::PackageAccess, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::package))]
pub struct Package<'ast> {
    pub name: Identifier<'ast>,
    pub access: PackageAccess<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
