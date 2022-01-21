// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{Node, Package, Packages};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum PackageOrPackages {
    Package(Package),
    Packages(Packages),
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

impl Node for PackageOrPackages {
    fn span(&self) -> &Span {
        match self {
            PackageOrPackages::Package(package) => &package.span,
            PackageOrPackages::Packages(packages) => &packages.span,
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            PackageOrPackages::Package(package) => package.span = span,
            PackageOrPackages::Packages(packages) => packages.span = span,
        }
    }
}
