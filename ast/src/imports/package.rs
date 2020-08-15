use crate::{
    ast::Rule,
    imports::{PackageAccess, PackageName},
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::package))]
pub struct Package<'ast> {
    pub name: PackageName<'ast>,
    pub access: PackageAccess<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
