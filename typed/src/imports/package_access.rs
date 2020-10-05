// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{ImportSymbol, Package, Span};
use leo_ast::imports::PackageAccess as AstPackageAccess;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PackageAccess {
    Star(Span),
    SubPackage(Box<Package>),
    Symbol(ImportSymbol),
    Multiple(Vec<PackageAccess>),
}

impl<'ast> From<AstPackageAccess<'ast>> for PackageAccess {
    fn from(access: AstPackageAccess<'ast>) -> Self {
        match access {
            AstPackageAccess::Star(star) => PackageAccess::Star(Span::from(star.span)),
            AstPackageAccess::SubPackage(package) => PackageAccess::SubPackage(Box::new(Package::from(*package))),
            AstPackageAccess::Symbol(symbol) => PackageAccess::Symbol(ImportSymbol::from(symbol)),
            AstPackageAccess::Multiple(accesses) => {
                PackageAccess::Multiple(accesses.into_iter().map(PackageAccess::from).collect())
            }
        }
    }
}

impl PackageAccess {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageAccess::Star(ref _span) => write!(f, "*"),
            PackageAccess::SubPackage(ref package) => write!(f, "{}", package),
            PackageAccess::Symbol(ref symbol) => write!(f, "{}", symbol),
            PackageAccess::Multiple(ref accesses) => {
                write!(f, "(")?;
                for (i, access) in accesses.iter().enumerate() {
                    write!(f, "{}", access)?;
                    if i < accesses.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
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
