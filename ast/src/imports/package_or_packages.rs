// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::Package;
use crate::Packages;
use leo_grammar::imports::PackageOrPackages as GrammarPackageOrPackages;

use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum PackageOrPackages {
    Package(Package),
    Packages(Packages),
}

impl<'ast> From<GrammarPackageOrPackages<'ast>> for PackageOrPackages {
    fn from(package_or_packages: GrammarPackageOrPackages<'ast>) -> Self {
        match package_or_packages {
            GrammarPackageOrPackages::Package(package) => PackageOrPackages::Package(Package::from(package)),
            GrammarPackageOrPackages::Packages(packages) => PackageOrPackages::Packages(Packages::from(packages)),
        }
    }
}

impl PackageOrPackages {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageOrPackages::Package(ref package) => write!(f, "{}", package),
            PackageOrPackages::Packages(ref packages) => {
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

impl fmt::Debug for PackageOrPackages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for PackageOrPackages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
