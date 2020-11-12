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

use crate::{Package, Span};
use leo_grammar::imports::Import as GrammarImport;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an import statement in a Leo program.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportStatement {
    pub package: Package,
    pub span: Span,
}

impl ImportStatement {
    ///
    /// Returns the the package file name of the self import statement.
    ///
    pub fn get_file_name(&self) -> &str {
        &self.package.name.name
    }
}

impl<'ast> From<GrammarImport<'ast>> for ImportStatement {
    fn from(import: GrammarImport<'ast>) -> Self {
        ImportStatement {
            package: Package::from(import.package),
            span: Span::from(import.span),
        }
    }
}

impl ImportStatement {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "import {};", self.package)
    }
}

impl fmt::Display for ImportStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for ImportStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
