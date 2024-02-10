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

use crate::{
    finalize_stub::*,
    Annotation,
    External,
    Function,
    FunctionInput,
    FunctionOutput,
    Identifier,
    Input,
    Mode,
    Node,
    NodeID,
    Output,
    ProgramId,
    TupleType,
    Type,
    Variant,
};
use leo_span::{sym, Span, Symbol};

use crate::Type::Identifier as IdentifierType;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use snarkvm::{
    console::program::RegisterType::{ExternalRecord, Future, Plaintext, Record},
    prelude::{Network, ValueType},
    synthesizer::program::{ClosureCore, CommandTrait, FunctionCore, InstructionTrait},
};
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

impl From<Function> for FunctionStub {
    fn from(function: Function) -> Self {
        Self {
            annotations: function.annotations,
            variant: function.variant,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            finalize_stub: function.finalize.map(FinalizeStub::from),
            span: function.span,
            id: function.id,
        }
    }
}

impl<N: Network, Instruction: InstructionTrait<N>> From<&ClosureCore<N, Instruction>> for FunctionStub {
    fn from(closure: &ClosureCore<N, Instruction>) -> Self {
        let outputs = closure
            .outputs()
            .iter()
            .map(|output| match output.register_type() {
                Plaintext(val) => Output::Internal(FunctionOutput {
                    mode: Mode::None,
                    type_: Type::from(val),
                    span: Default::default(),
                    id: Default::default(),
                }),
                Record(_) => panic!("Closures do not return records"),
                ExternalRecord(_) => panic!("Closures do not return external records"),
                Future(_) => panic!("Closures do not return futures"),
            })
            .collect_vec();
        let output_vec = outputs
            .iter()
            .map(|output| match output {
                Output::Internal(output) => output.type_.clone(),
                Output::External(_) => panic!("Closures do not return external records"),
            })
            .collect_vec();
        let output_type = match output_vec.len() {
            0 => Type::Unit,
            1 => output_vec[0].clone(),
            _ => Type::Tuple(TupleType::new(output_vec)),
        };
        Self {
            annotations: Vec::new(),
            variant: Variant::Standard,
            identifier: Identifier::from(closure.name()),
            input: closure
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| {
                    let arg_name = Identifier::new(Symbol::intern(&format!("a{}", index + 1)), Default::default());
                    match input.register_type() {
                        Plaintext(val) => Input::Internal(FunctionInput {
                            identifier: arg_name,
                            mode: Mode::None,
                            type_: Type::from(val),
                            span: Default::default(),
                            id: Default::default(),
                        }),
                        Record(_) => panic!("Closures do not contain records as inputs"),
                        ExternalRecord(_) => panic!("Closures do not contain external records as inputs"),
                        Future(_) => panic!("Closures do not contain futures as inputs"),
                    }
                })
                .collect_vec(),
            output: outputs,
            output_type,
            span: Default::default(),
            id: Default::default(),
            finalize_stub: None,
        }
    }
}

impl<N: Network, Instruction: InstructionTrait<N>, Command: CommandTrait<N>>
    From<&FunctionCore<N, Instruction, Command>> for FunctionStub
{
    fn from(function: &FunctionCore<N, Instruction, Command>) -> Self {
        let outputs = function
            .outputs()
            .iter()
            .map(|output| match output.value_type() {
                ValueType::Constant(val) => vec![Output::Internal(FunctionOutput {
                    mode: Mode::Constant,
                    type_: Type::from(val),
                    span: Default::default(),
                    id: Default::default(),
                })],
                ValueType::Public(val) => vec![Output::Internal(FunctionOutput {
                    mode: Mode::Public,
                    type_: Type::from(val),
                    span: Default::default(),
                    id: Default::default(),
                })],
                ValueType::Private(val) => vec![Output::Internal(FunctionOutput {
                    mode: Mode::Private,
                    type_: Type::from(val),
                    span: Default::default(),
                    id: Default::default(),
                })],
                ValueType::Record(id) => vec![Output::Internal(FunctionOutput {
                    mode: Mode::None,
                    type_: IdentifierType(Identifier::from(id)),
                    span: Default::default(),
                    id: Default::default(),
                })],
                ValueType::ExternalRecord(loc) => vec![Output::External(External {
                    identifier: Identifier::new(Symbol::intern("dummy"), Default::default()),
                    program_name: ProgramId::from(loc.program_id()).name,
                    record: Identifier::from(loc.resource()),
                    span: Default::default(),
                    id: Default::default(),
                })],
                ValueType::Future(_) => Vec::new(), // Don't include futures in the output signature
            })
            .collect_vec()
            .concat();
        let output_vec = outputs
            .iter()
            .map(|output| match output {
                Output::Internal(output) => output.type_.clone(),
                Output::External(output) => Type::Identifier(output.record),
            })
            .collect_vec();
        let output_type = match output_vec.len() {
            0 => Type::Unit,
            1 => output_vec[0].clone(),
            _ => Type::Tuple(TupleType::new(output_vec)),
        };

        Self {
            annotations: Vec::new(),
            variant: Variant::Transition,
            identifier: Identifier::from(function.name()),
            input: function
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| {
                    let arg_name = Identifier::new(Symbol::intern(&format!("a{}", index + 1)), Default::default());
                    match input.value_type() {
                        ValueType::Constant(val) => Input::Internal(FunctionInput {
                            identifier: arg_name,
                            mode: Mode::Constant,
                            type_: Type::from(val),
                            span: Default::default(),
                            id: Default::default(),
                        }),
                        ValueType::Public(val) => Input::Internal(FunctionInput {
                            identifier: arg_name,
                            mode: Mode::Public,
                            type_: Type::from(val),
                            span: Default::default(),
                            id: Default::default(),
                        }),
                        ValueType::Private(val) => Input::Internal(FunctionInput {
                            identifier: arg_name,
                            mode: Mode::Private,
                            type_: Type::from(val),
                            span: Default::default(),
                            id: Default::default(),
                        }),
                        ValueType::Record(id) => Input::Internal(FunctionInput {
                            identifier: arg_name,
                            mode: Mode::None,
                            type_: IdentifierType(Identifier::from(id)),
                            span: Default::default(),
                            id: Default::default(),
                        }),
                        ValueType::ExternalRecord(loc) => Input::External(External {
                            identifier: arg_name,
                            program_name: ProgramId::from(loc.program_id()).name,
                            record: Identifier::from(loc.resource()),
                            span: Default::default(),
                            id: Default::default(),
                        }),
                        ValueType::Future(_) => panic!("Functions do not contain futures as inputs"),
                    }
                })
                .collect_vec(),
            output: outputs,
            output_type,
            finalize_stub: function.finalize_logic().map(FinalizeStub::from),
            span: Default::default(),
            id: Default::default(),
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
