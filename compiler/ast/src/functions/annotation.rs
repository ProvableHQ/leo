// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{simple_node_impl, Node, NodeID};

use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// The supported annotations.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnnotationName {
    IntegrationTest,
    UnitTest,
}

/// An annotation, e.g. @program.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Annotation {
    // TODO: Consider using a symbol instead of an identifier.
    /// The name of the annotation.
    pub name: Symbol,
    /// A span locating where the annotation occurred in the source.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl Annotation {
    /// Extracts the annotation name.
    pub fn name(&self) -> Option<AnnotationName> {
        match self.name.to_string().as_str() {
            "integration_test" => Some(AnnotationName::IntegrationTest),
            "unit_test" => Some(AnnotationName::UnitTest),
            _ => None,
        }
    }
    
    /// Checks if the annotation is a test.
    pub fn is_test(&self) -> bool {
        self.name() == Some(AnnotationName::IntegrationTest) || self.name() == Some(AnnotationName::UnitTest)
    }
}

simple_node_impl!(Annotation);

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{}", self.name)
    }
}
