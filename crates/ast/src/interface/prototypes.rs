// Copyright (C) 2019-2026 Provable Inc.
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

use std::fmt;

use crate::{Annotation, ConstParameter, Identifier, Input, Node, NodeID, Output, TupleType, Type};
use itertools::Itertools;
use leo_span::Span;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FunctionPrototype {
    /// Annotations on the function.
    pub annotations: Vec<Annotation>,
    /// The function identifier, e.g., `foo` in `function foo(...) { ... }`.
    pub identifier: Identifier,
    /// The function's const parameters.
    pub const_parameters: Vec<ConstParameter>,
    /// The function's input parameters.
    pub input: Vec<Input>,
    /// The function's output declarations.
    pub output: Vec<Output>,
    /// The function's output type.
    pub output_type: Type,
    /// The entire span of the function definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl FunctionPrototype {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        annotations: Vec<Annotation>,
        identifier: Identifier,
        const_parameters: Vec<ConstParameter>,
        input: Vec<Input>,
        output: Vec<Output>,
        span: Span,
        id: NodeID,
    ) -> Self {
        let output_type = match output.len() {
            0 => Type::Unit,
            1 => output[0].type_.clone(),
            _ => Type::Tuple(TupleType::new(output.iter().map(|o| o.type_.clone()).collect())),
        };

        Self { annotations, identifier, const_parameters, input, output, output_type, span, id }
    }
}

impl PartialEq for FunctionPrototype {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for FunctionPrototype {}

impl fmt::Debug for FunctionPrototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for FunctionPrototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for annotation in &self.annotations {
            writeln!(f, "{annotation}")?;
        }
        write!(f, "fn {}", self.identifier)?;
        if !self.const_parameters.is_empty() {
            write!(f, "::[{}]", self.const_parameters.iter().format(", "))?;
        }
        write!(f, "({})", self.input.iter().format(", "))?;
        match self.output.len() {
            0 => {}
            1 => {
                if !matches!(self.output[0].type_, Type::Unit) {
                    write!(f, " -> {}", self.output[0])?;
                }
            }
            _ => {
                write!(f, " -> ({})", self.output.iter().format(", "))?;
            }
        }
        write!(f, ";")
    }
}

crate::simple_node_impl!(FunctionPrototype);

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RecordPrototype {
    /// The record identifier
    pub identifier: Identifier,
    /// The entire span of the composite definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl PartialEq for RecordPrototype {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for RecordPrototype {}

impl fmt::Debug for RecordPrototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for RecordPrototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "record {};", self.identifier)
    }
}

crate::simple_node_impl!(RecordPrototype);
