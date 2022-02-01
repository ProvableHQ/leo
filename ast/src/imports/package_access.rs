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

use crate::{ImportSymbol, Node, Package, Packages};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum PackageAccess {
    /// A glob import `*`.
    Star {
        /// The span for the `*`.
        span: Span,
    },
    /// A subpackage to import.
    SubPackage(Box<Package>),
    /// A leaf package to import.
    Symbol(ImportSymbol),
    /// Several subpackages to import.
    // FIXME(Centril): This structure seems convoluted and unclear.
    // Refactor and simplify the types to:
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_ast/ast/struct.UseTree.html.
    Multiple(Packages),
}

impl Node for PackageAccess {
    fn span(&self) -> &Span {
        match self {
            PackageAccess::Star { span } => span,
            PackageAccess::SubPackage(package) => &package.span,
            PackageAccess::Symbol(package) => &package.span,
            PackageAccess::Multiple(package) => &package.span,
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            PackageAccess::Star { span } => *span = span.clone(),
            PackageAccess::SubPackage(package) => package.span = span,
            PackageAccess::Symbol(package) => package.span = span,
            PackageAccess::Multiple(package) => package.span = span,
        }
    }
}

impl PackageAccess {
    /// Formats `self` to `f`.
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageAccess::Star { .. } => write!(f, "*"),
            PackageAccess::SubPackage(ref package) => write!(f, "{}", package),
            PackageAccess::Symbol(ref symbol) => write!(f, "{}", symbol),
            PackageAccess::Multiple(ref packages) => {
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
