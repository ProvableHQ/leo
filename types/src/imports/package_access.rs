use crate::{ImportSymbol, Package};
use leo_ast::imports::PackageAccess as AstPackageAccess;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub enum PackageAccess {
    Star,
    SubPackage(Box<Package>),
    Multiple(Vec<Package>),
    Symbol(ImportSymbol),
}

impl<'ast> From<AstPackageAccess<'ast>> for PackageAccess {
    fn from(access: AstPackageAccess<'ast>) -> Self {
        match access {
            AstPackageAccess::Star(_) => PackageAccess::Star,
            AstPackageAccess::SubPackage(package) => PackageAccess::SubPackage(Box::new(Package::from(*package))),
            AstPackageAccess::Multiple(packages) => {
                PackageAccess::Multiple(packages.into_iter().map(|package| Package::from(package)).collect())
            }
            AstPackageAccess::Symbol(symbol) => PackageAccess::Symbol(ImportSymbol::from(symbol)),
        }
    }
}

impl PackageAccess {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageAccess::Star => write!(f, ".*"),
            PackageAccess::SubPackage(ref package) => write!(f, ".{}", package),
            PackageAccess::Multiple(ref packages) => {
                write!(f, ".(")?;
                for (i, package) in packages.iter().enumerate() {
                    write!(f, "{}", package)?;
                    if i < packages.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            PackageAccess::Symbol(ref symbol) => write!(f, ".{}", symbol),
        }
    }
}

impl fmt::Debug for PackageAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for PackageAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
