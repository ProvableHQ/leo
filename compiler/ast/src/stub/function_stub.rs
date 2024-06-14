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
    Annotation,
    CompositeType,
    Function,
    FutureType,
    Identifier,
    Input,
    Location,
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

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use snarkvm::{
    console::program::{
        FinalizeType::{Future as FutureFinalizeType, Plaintext as PlaintextFinalizeType},
        RegisterType::{ExternalRecord, Future, Plaintext, Record},
    },
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
        _is_async: bool,
        variant: Variant,
        identifier: Identifier,
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

        FunctionStub { annotations, variant, identifier, input, output, output_type, span, id }
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
        write!(f, "({parameters}) -> {returns}")?;

        Ok(())
    }

    /// Converts from snarkvm function type to leo FunctionStub, while also carrying the parent program name.
    pub fn from_function_core<N: Network, Instruction: InstructionTrait<N>, Command: CommandTrait<N>>(
        function: &FunctionCore<N, Instruction, Command>,
        program: Symbol,
    ) -> Self {
        let outputs = function
            .outputs()
            .iter()
            .map(|output| match output.value_type() {
                ValueType::Constant(val) => vec![Output {
                    mode: Mode::Constant,
                    type_: Type::from_snarkvm(val, None),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::Public(val) => vec![Output {
                    mode: Mode::Public,
                    type_: Type::from_snarkvm(val, None),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::Private(val) => vec![Output {
                    mode: Mode::Private,
                    type_: Type::from_snarkvm(val, None),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::Record(id) => vec![Output {
                    mode: Mode::None,
                    type_: Type::Composite(CompositeType { id: Identifier::from(id), program: Some(program) }),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::ExternalRecord(loc) => {
                    vec![Output {
                        mode: Mode::None,
                        span: Default::default(),
                        id: Default::default(),
                        type_: Type::Composite(CompositeType {
                            id: Identifier::from(loc.resource()),
                            program: Some(ProgramId::from(loc.program_id()).name.name),
                        }),
                    }]
                }
                ValueType::Future(_) => vec![Output {
                    mode: Mode::None,
                    span: Default::default(),
                    id: Default::default(),
                    type_: Type::Future(FutureType::new(
                        Vec::new(),
                        Some(Location::new(Some(program), Identifier::from(function.name()).name)),
                        false,
                    )),
                }],
            })
            .collect_vec()
            .concat();
        let output_vec = outputs.iter().map(|output| output.type_.clone()).collect_vec();
        let output_type = match output_vec.len() {
            0 => Type::Unit,
            1 => output_vec[0].clone(),
            _ => Type::Tuple(TupleType::new(output_vec)),
        };

        Self {
            annotations: Vec::new(),
            variant: match function.finalize_logic().is_some() {
                true => Variant::AsyncTransition,
                false => Variant::Transition,
            },
            identifier: Identifier::from(function.name()),
            input: function
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| {
                    let arg_name = Identifier::new(Symbol::intern(&format!("arg{}", index + 1)), Default::default());
                    match input.value_type() {
                        ValueType::Constant(val) => Input {
                            identifier: arg_name,
                            mode: Mode::Constant,
                            type_: Type::from_snarkvm(val, None),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::Public(val) => Input {
                            identifier: arg_name,
                            mode: Mode::Public,
                            type_: Type::from_snarkvm(val, None),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::Private(val) => Input {
                            identifier: arg_name,
                            mode: Mode::Private,
                            type_: Type::from_snarkvm(val, None),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::Record(id) => Input {
                            identifier: arg_name,
                            mode: Mode::None,
                            type_: Type::Composite(CompositeType { id: Identifier::from(id), program: Some(program) }),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::ExternalRecord(loc) => Input {
                            identifier: arg_name,
                            mode: Mode::None,
                            span: Default::default(),
                            id: Default::default(),
                            type_: Type::Composite(CompositeType {
                                id: Identifier::from(loc.resource()),
                                program: Some(ProgramId::from(loc.program_id()).name.name),
                            }),
                        },
                        ValueType::Future(_) => panic!("Functions do not contain futures as inputs"),
                    }
                })
                .collect_vec(),
            output: outputs,
            output_type,
            span: Default::default(),
            id: Default::default(),
        }
    }

    pub fn from_finalize<N: Network, Instruction: InstructionTrait<N>, Command: CommandTrait<N>>(
        function: &FunctionCore<N, Instruction, Command>,
        key_name: Symbol,
        program: Symbol,
    ) -> Self {
        Self {
            annotations: Vec::new(),
            variant: Variant::AsyncFunction,
            identifier: Identifier::new(key_name, Default::default()),
            input: function
                .finalize_logic()
                .unwrap()
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| Input {
                    identifier: Identifier::new(Symbol::intern(&format!("arg{}", index + 1)), Default::default()),
                    mode: Mode::None,
                    type_: match input.finalize_type() {
                        PlaintextFinalizeType(val) => Type::from_snarkvm(val, Some(program)),
                        FutureFinalizeType(val) => Type::Future(FutureType::new(
                            Vec::new(),
                            Some(Location::new(
                                Some(Identifier::from(val.program_id().name()).name),
                                Symbol::intern(&format!("finalize/{}", val.resource())),
                            )),
                            false,
                        )),
                    },
                    span: Default::default(),
                    id: Default::default(),
                })
                .collect_vec(),
            output: Vec::new(),
            output_type: Type::Unit,
            span: Default::default(),
            id: 0,
        }
    }

    pub fn from_closure<N: Network, Instruction: InstructionTrait<N>>(
        closure: &ClosureCore<N, Instruction>,
        program: Symbol,
    ) -> Self {
        let outputs = closure
            .outputs()
            .iter()
            .map(|output| match output.register_type() {
                Plaintext(val) => Output {
                    mode: Mode::None,
                    type_: Type::from_snarkvm(val, Some(program)),
                    span: Default::default(),
                    id: Default::default(),
                },
                Record(_) => panic!("Closures do not return records"),
                ExternalRecord(_) => panic!("Closures do not return external records"),
                Future(_) => panic!("Closures do not return futures"),
            })
            .collect_vec();
        let output_vec = outputs.iter().map(|output| output.type_.clone()).collect_vec();
        let output_type = match output_vec.len() {
            0 => Type::Unit,
            1 => output_vec[0].clone(),
            _ => Type::Tuple(TupleType::new(output_vec)),
        };
        Self {
            annotations: Vec::new(),
            variant: Variant::Function,
            identifier: Identifier::from(closure.name()),
            input: closure
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| {
                    let arg_name = Identifier::new(Symbol::intern(&format!("arg{}", index + 1)), Default::default());
                    match input.register_type() {
                        Plaintext(val) => Input {
                            identifier: arg_name,
                            mode: Mode::None,
                            type_: Type::from_snarkvm(val, None),
                            span: Default::default(),
                            id: Default::default(),
                        },
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
            span: function.span,
            id: function.id,
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
