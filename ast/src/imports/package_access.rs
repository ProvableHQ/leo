use crate::{
    ast::Rule,
    imports::{ImportSymbol, Package, Star},
};

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::package_access))]
pub enum PackageAccess<'ast> {
    Star(Star<'ast>),
    SubPackage(Box<Package<'ast>>),
    Symbol(ImportSymbol<'ast>),
    Multiple(Vec<PackageAccess<'ast>>),
}
