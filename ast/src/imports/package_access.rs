use crate::{
    ast::Rule,
    imports::{ImportSymbol, Package, Star},
};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::package_access))]
pub enum PackageAccess<'ast> {
    Star(Star),
    SubPackage(Box<Package<'ast>>),
    Multiple(Vec<Package<'ast>>),
    Symbol(ImportSymbol<'ast>),
}
