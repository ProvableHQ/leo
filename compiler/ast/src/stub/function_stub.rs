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

use crate::{finalize_stub::*, Annotation, Identifier, Input, Node, NodeID, Output, TupleType, Type, Variant};
use leo_span::{sym, Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A function stub definition.
#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionStub {
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
    /// An optional finalize stub
    pub finalize_stub: Option<FinalizeStub>,
    /// The entire span of the function definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl PartialEq for FunctionStub {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for FunctionStub {}

impl FunctionStub {
    /// Initialize a new function.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        annotations: Vec<Annotation>,
        variant: Variant,
        identifier: Identifier,
        input: Vec<Input>,
        output: Vec<Output>,
        finalize_stub: Option<FinalizeStub>,
        span: Span,
        id: NodeID,
    ) -> Self {
        // Determine the output type of the function
        let get_output_type = |output: &Output| match &output {
            Output::Internal(output) => output.type_.clone(),
            Output::External(output) => output.type_(),
        };

        let output_type = match output.len() {
            0 => Type::Unit,
            1 => get_output_type(&output[0]),
            _ => Type::Tuple(TupleType::new(output.iter().map(get_output_type).collect())),
        };

        FunctionStub { annotations, variant, identifier, input, output, output_type, finalize_stub, span, id }
    }

    /// Returns function name.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    /// Returns `true` if the function name is `main`.
    pub fn is_main(&self) -> bool {
        self.name() == sym::main
    }

    ///
    /// Private formatting method used for optimizing [fmt::Debug] and [fmt::Display] implementations.
    ///
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            Variant::Inline => write!(f, "inline ")?,
            Variant::Standard => write!(f, "function ")?,
            Variant::Transition => write!(f, "transition ")?,
        }
        write!(f, "{}", self.identifier)?;

        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let returns = match self.output.len() {
            0 => "()".to_string(),
            1 => self.output[0].to_string(),
            _ => self.output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
        };
        write!(f, "({parameters}) -> {returns}")?;

        if let Some(finalize) = &self.finalize_stub {
            let parameters = finalize.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
            write!(f, " finalize ({parameters})")
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for FunctionStub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for FunctionStub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

crate::simple_node_impl!(FunctionStub);
