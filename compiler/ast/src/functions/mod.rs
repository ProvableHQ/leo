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

pub mod annotation;
pub use annotation::*;

pub mod core_function;
pub use core_function::*;

pub mod variant;
pub use variant::*;

pub mod input;
pub use input::*;

pub mod output;
pub use output::*;

pub mod mode;
pub use mode::*;

use crate::{Block, FunctionStub, Identifier, Node, NodeID, TupleType, Type};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A function definition.
#[derive(Clone, Serialize, Deserialize)]
pub struct Function {
    /// Annotations on the function.
    pub annotations: Vec<Annotation>,
    /// Is this function a transition, inlined, or a regular function?.
    pub variant: Variant,
    /// The function identifier, e.g., `foo` in `function foo(...) { ... }`.
    pub identifier: Identifier,
    /// The function's input parameters.
    pub input: Vec<Input>,
    /// The function's output declarations.
    pub output: Vec<Output>,
    /// The function's output type.
    pub output_type: Type,
    /// The body of the function.
    pub block: Block,
    /// The entire span of the function definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Function {}

impl Function {
    /// Initialize a new function.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        annotations: Vec<Annotation>,
        variant: Variant,
        identifier: Identifier,
        input: Vec<Input>,
        output: Vec<Output>,
        block: Block,
        span: Span,
        id: NodeID,
    ) -> Self {
        let output_type = match output.len() {
            0 => Type::Unit,
            1 => output[0].type_.clone(),
            _ => Type::Tuple(TupleType::new(output.iter().map(|o| o.type_.clone()).collect())),
        };

        Function { annotations, variant, identifier, input, output, output_type, block, span, id }
    }

    /// Returns function name.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    ///
    /// Private formatting method used for optimizing [fmt::Debug] and [fmt::Display] implementations.
    ///
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            Variant::Inline => write!(f, "inline ")?,
            Variant::Function | Variant::AsyncFunction => write!(f, "function ")?,
            Variant::Transition | Variant::AsyncTransition => write!(f, "transition ")?,
        }
        write!(f, "{}", self.identifier)?;

        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let returns = match self.output.len() {
            0 => "()".to_string(),
            1 => self.output[0].to_string(),
            _ => self.output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
        };
        write!(f, "({parameters}) -> {returns} {}", self.block)?;

        Ok(())
    }
}

impl From<FunctionStub> for Function {
    fn from(function: FunctionStub) -> Self {
        Self {
            annotations: function.annotations,
            variant: function.variant,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block: Block::default(),
            span: function.span,
            id: function.id,
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

crate::simple_node_impl!(Function);
