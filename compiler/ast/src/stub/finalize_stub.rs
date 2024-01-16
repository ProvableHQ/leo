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

use crate::{Finalize, FunctionInput, Identifier, Input, Mode, Node, NodeID, Output, TupleType, Type};

use leo_span::{Span, Symbol};

use core::fmt;
use serde::{Deserialize, Serialize};
use snarkvm::{
    prelude::{
        FinalizeType::{Future, Plaintext},
        Network,
    },
    synthesizer::program::{CommandTrait, FinalizeCore},
};

/// A finalize stub.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct FinalizeStub {
    /// The finalize identifier.
    pub identifier: Identifier,
    /// The finalize block's input parameters.
    pub input: Vec<Input>,
    /// The finalize blocks's output declaration.
    pub output: Vec<Output>,
    /// The finalize block's output type.
    pub output_type: Type,
    /// The entire span of the finalize stub.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl FinalizeStub {
    /// Create a new finalize stub.
    pub fn new(identifier: Identifier, input: Vec<Input>, output: Vec<Output>, span: Span, id: NodeID) -> Self {
        let output_type = match output.len() {
            0 => Type::Unit,
            1 => output[0].type_(),
            _ => Type::Tuple(TupleType::new(output.iter().map(|output| output.type_()).collect())),
        };

        Self { identifier, input, output, output_type, span, id }
    }
}

impl<N: Network, Command: CommandTrait<N>> From<&FinalizeCore<N, Command>> for FinalizeStub {
    fn from(finalize: &FinalizeCore<N, Command>) -> Self {
        let mut inputs = Vec::new();

        finalize.inputs().iter().enumerate().for_each(|(index, input)| {
            let arg_name = Identifier::new(Symbol::intern(&format!("a{}", index + 1)), Default::default());
            match input.finalize_type() {
                Plaintext(val) => inputs.push(Input::Internal(FunctionInput {
                    identifier: arg_name,
                    mode: Mode::None,
                    type_: Type::from(val),
                    span: Default::default(),
                    id: Default::default(),
                })),
                Future(_) => {} // Don't need to worry about nested futures
            }
        });

        Self::new(Identifier::from(finalize.name()), inputs, Vec::new(), Default::default(), Default::default())
    }
}

impl From<Finalize> for FinalizeStub {
    fn from(finalize: Finalize) -> Self {
        Self::new(finalize.identifier, finalize.input, finalize.output, Default::default(), Default::default())
    }
}

impl fmt::Display for FinalizeStub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let returns = match self.output.len() {
            0 => "()".to_string(),
            1 => self.output[0].to_string(),
            _ => format!("({})", self.output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),
        };
        write!(f, " finalize {}({parameters}) -> {returns}", self.identifier)
    }
}

crate::simple_node_impl!(FinalizeStub);
