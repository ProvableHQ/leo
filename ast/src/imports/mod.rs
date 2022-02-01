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

use crate::Identifier;
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an import statement in a Leo program.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportStatement {
    /// The tree specifying what items or packages to import.
    pub tree: ImportTree,
    /// The span, excluding the `;`.
    pub span: Span,
}

impl ImportStatement {
    /// Returns the the package file name of the self import statement.
    pub fn get_file_name(&self) -> Symbol {
        self.tree.base.first().unwrap().name
    }
}

impl fmt::Display for ImportStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "import {};", self.tree)
    }
}

impl fmt::Debug for ImportStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// An import tree specifies item(s) to import.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportTree {
    /// A path to the base item or package to import or import from.
    /// The list is always non-empty.
    pub base: Vec<Identifier>,
    /// Specifies the kind of import and the meaning of `base`.
    /// This includes plain imports, renames, globs (`*`), and nested imports.
    pub kind: ImportTreeKind,
    /// The span for the import excluding `import` and `;`.
    pub span: Span,
}

impl fmt::Display for ImportTree {
    /// Formats `self` to `f`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the path.
        for (i, part) in self.base.iter().enumerate() {
            write!(f, "{}", part)?;
            if i < self.base.len() - 1 {
                write!(f, ".")?;
            }
        }

        // Format the kind.
        match self.kind {
            ImportTreeKind::Glob { .. } => write!(f, ".*"),
            ImportTreeKind::Leaf { alias: None } => Ok(()),
            ImportTreeKind::Leaf { alias: Some(alias) } => write!(f, "as {}", alias),
            ImportTreeKind::Nested { tree } => {
                write!(f, ".(")?;
                for (i, node) in tree.iter().enumerate() {
                    write!(f, "{}", node)?;
                    if i < tree.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Debug for ImportTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Specifies the import kind and the meaning of `base`.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ImportTreeKind {
    /// A glob import `*`.
    Glob {
        /// The span for the `*`.
        span: Span,
    },
    /// A leaf package to import.
    Leaf {
        /// When specified, the package is imported under a different name.
        /// Otherwise, the `base` name is used as in the `ImportTree`.
        alias: Option<Identifier>,
    },
    /// A nested import of items or sub-packages.
    Nested {
        /// The sub-tree specifying what to import from the `base`.
        tree: Vec<ImportTree>,
    },
}
