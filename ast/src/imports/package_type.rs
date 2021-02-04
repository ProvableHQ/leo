use crate::{Package, Packages};
use leo_grammar::imports::PackageType as GrammarPackageType;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum PackageType {
    Package(Package),
    Packages(Packages),
}

impl<'ast> From<GrammarPackageType<'ast>> for PackageType {
    fn from(package_type: GrammarPackageType<'ast>) -> Self {
        match package_type {
            GrammarPackageType::Package(package) => PackageType::Package(Package::from(package)),
            GrammarPackageType::Packages(packages) => PackageType::Packages(Packages::from(packages)),
        }
    }
}

impl PackageType {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageType::Package(ref package) => write!(f, "{}", package),
            PackageType::Packages(ref packages) => {
                write!(f, "(")?;
                for (i, access) in packages.accesses.iter().enumerate() {
                    write!(f, "{}", access)?;
                    if i < packages.accesses.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Debug for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
