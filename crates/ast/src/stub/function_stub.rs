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
    Path,
    ProgramId,
    TupleType,
    TypeKind,
    TypeNode,
    Variant,
};
use leo_span::{Span, Symbol, sym};

/// No interner is in scope during disassembly; the frontend re-interns these when the stub is
/// merged into the AST.
fn stub_type(kind: TypeKind) -> TypeNode {
    TypeNode::unchecked(kind, Span::default())
}

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use snarkvm::{
    console::program::RegisterType,
    prelude::{FinalizeType, Network, ValueType},
    synthesizer::program::{ClosureCore, FunctionCore, ViewCore},
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
    pub output_type: TypeKind,
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
            0 => TypeKind::Unit,
            1 => output[0].type_.kind().clone(),
            _ => TypeKind::Tuple(TupleType::new(output.iter().map(|o| o.type_.kind().clone()).collect())),
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

    /// Returns `true` if any output of the function is a `Final`
    pub fn has_final_output(&self) -> bool {
        self.output.iter().any(|o| matches!(o.type_.kind(), TypeKind::Future(_)))
    }

    /// Private formatting method used for optimizing [fmt::Debug] and [fmt::Display] implementations.
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            Variant::FinalFn => write!(f, "final fn ")?,
            Variant::Finalize => write!(f, "finalize ")?,
            Variant::Fn => write!(f, "fn ")?,
            Variant::EntryPoint => write!(f, "entry ")?,
            Variant::View => write!(f, "view fn ")?,
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
    pub fn from_function_core<N: Network>(function: &FunctionCore<N>, program_id: ProgramId) -> Self {
        let outputs = function
            .outputs()
            .iter()
            .map(|output| match output.value_type() {
                ValueType::Constant(val) => vec![Output {
                    mode: Mode::Constant,
                    type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::Public(val) => vec![Output {
                    mode: Mode::Public,
                    type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::Private(val) => vec![Output {
                    mode: Mode::Private,
                    type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::Record(id) => vec![Output {
                    mode: Mode::None,
                    type_: stub_type(TypeKind::Composite(CompositeType {
                        path: {
                            let ident = Identifier::from(id);
                            Path::from(ident)
                                .to_global(Location::new(program_id.as_symbol(), vec![ident.name]))
                                .with_user_program(program_id)
                        },
                        const_arguments: Vec::new(),
                    })),
                    span: Default::default(),
                    id: Default::default(),
                }],
                ValueType::ExternalRecord(loc) => {
                    let external_program_id = ProgramId::from(loc.program_id());
                    vec![Output {
                        mode: Mode::None,
                        span: Default::default(),
                        id: Default::default(),
                        type_: stub_type(TypeKind::Composite(CompositeType {
                            path: {
                                let ident = Identifier::from(loc.resource());
                                Path::from(ident)
                                    .to_global(Location::new(external_program_id.as_symbol(), vec![ident.name]))
                                    .with_user_program(external_program_id)
                            },
                            const_arguments: Vec::new(),
                        })),
                    }]
                }
                ValueType::Future(_) => vec![Output {
                    mode: Mode::None,
                    span: Default::default(),
                    id: Default::default(),
                    type_: stub_type(TypeKind::Future(FutureType::new(
                        Vec::new(),
                        Some(Location::new(program_id.as_symbol(), vec![Symbol::intern(&function.name().to_string())])),
                        false,
                    ))),
                }],
                ValueType::DynamicRecord => vec![Output {
                    mode: Mode::None,
                    span: Default::default(),
                    id: Default::default(),
                    type_: stub_type(TypeKind::DynRecord),
                }],
                ValueType::DynamicFuture => vec![Output {
                    mode: Mode::None,
                    span: Default::default(),
                    id: Default::default(),
                    type_: stub_type(TypeKind::Future(FutureType::new(
                        Vec::new(),
                        Some(Location::new(program_id.as_symbol(), vec![Symbol::intern(&function.name().to_string())])),
                        false,
                    ))),
                }],
            })
            .collect_vec()
            .concat();
        let output_vec = outputs.iter().map(|output| output.type_.kind().clone()).collect_vec();
        let output_type = match output_vec.len() {
            0 => TypeKind::Unit,
            1 => output_vec[0].clone(),
            _ => TypeKind::Tuple(TupleType::new(output_vec)),
        };

        Self {
            annotations: Vec::new(),
            variant: Variant::EntryPoint,
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
                            type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::Public(val) => Input {
                            identifier: arg_name,
                            mode: Mode::Public,
                            type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::Private(val) => Input {
                            identifier: arg_name,
                            mode: Mode::Private,
                            type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::Record(id) => Input {
                            identifier: arg_name,
                            mode: Mode::None,
                            type_: stub_type(TypeKind::Composite(CompositeType {
                                path: {
                                    let ident = Identifier::from(id);
                                    Path::from(ident)
                                        .to_global(Location::new(program_id.as_symbol(), vec![ident.name]))
                                        .with_user_program(program_id)
                                },
                                const_arguments: Vec::new(),
                            })),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        ValueType::ExternalRecord(loc) => {
                            let external_program = ProgramId::from(loc.program_id());
                            Input {
                                identifier: arg_name,
                                mode: Mode::None,
                                span: Default::default(),
                                id: Default::default(),
                                type_: stub_type(TypeKind::Composite(CompositeType {
                                    path: {
                                        let ident = Identifier::from(loc.resource());
                                        Path::from(ident)
                                            .to_global(Location::new(external_program.as_symbol(), vec![ident.name]))
                                            .with_user_program(external_program)
                                    },
                                    const_arguments: Vec::new(),
                                })),
                            }
                        }
                        ValueType::Future(_) | ValueType::DynamicFuture => {
                            panic!("Functions do not contain futures as inputs")
                        }

                        ValueType::DynamicRecord => Input {
                            identifier: arg_name,
                            mode: Mode::None,
                            span: Default::default(),
                            id: Default::default(),
                            type_: stub_type(TypeKind::DynRecord),
                        },
                    }
                })
                .collect_vec(),
            output: outputs,
            output_type,
            span: Default::default(),
            id: Default::default(),
        }
    }

    pub fn from_finalize<N: Network>(function: &FunctionCore<N>, key_name: Symbol, program_id: ProgramId) -> Self {
        Self {
            annotations: Vec::new(),
            variant: Variant::Finalize,
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
                        FinalizeType::Plaintext(val) => stub_type(TypeKind::from_snarkvm(val, program_id)),
                        FinalizeType::Future(val) => stub_type(TypeKind::Future(FutureType::new(
                            Vec::new(),
                            Some(Location::new(ProgramId::from(val.program_id()).as_symbol(), vec![Symbol::intern(
                                &format!("finalize/{}", val.resource()),
                            )])),
                            false,
                        ))),
                        FinalizeType::DynamicFuture => {
                            stub_type(TypeKind::Future(FutureType::new(Vec::new(), None, false)))
                        }
                    },
                    span: Default::default(),
                    id: Default::default(),
                })
                .collect_vec(),
            output: Vec::new(),
            output_type: TypeKind::Unit,
            span: Default::default(),
            id: 0,
        }
    }

    /// Construct a Leo `FunctionStub` from a snarkVM `ViewCore` (V15 read-only entry).
    /// snarkVM's `ViewCore::add_input` and `ViewCore::add_output` enforce
    /// `matches!(finalize_type, FinalizeType::Plaintext(_))` — see
    /// `snarkvm/synthesizer/program/src/view/mod.rs` — so by the time a `ViewCore<N>` reaches
    /// this constructor, non-plaintext finalize types are unreachable. The panic below matches
    /// the defense-in-depth pattern used by `from_function_core` (`Functions do not contain
    /// futures as inputs`) — `disassemble_from_str` will surface a clean validation error long
    /// before this is hit, so reaching the panic indicates a bug in snarkVM or a caller that
    /// bypassed validation entirely.
    pub fn from_view<N: Network>(view: &ViewCore<N>, program_id: ProgramId) -> Self {
        let plaintext_or_panic = |finalize_type: &FinalizeType<N>| match finalize_type {
            FinalizeType::Plaintext(val) => TypeKind::from_snarkvm(val, program_id),
            FinalizeType::Future(_) | FinalizeType::DynamicFuture => {
                panic!("Views do not contain futures as inputs or outputs")
            }
        };

        let outputs = view
            .outputs()
            .iter()
            .map(|output| Output {
                mode: Mode::None,
                type_: stub_type(plaintext_or_panic(output.finalize_type())),
                span: Default::default(),
                id: Default::default(),
            })
            .collect_vec();
        let output_vec = outputs.iter().map(|o| o.type_.kind().clone()).collect_vec();
        let output_type = match output_vec.len() {
            0 => TypeKind::Unit,
            1 => output_vec[0].clone(),
            _ => TypeKind::Tuple(TupleType::new(output_vec)),
        };

        Self {
            annotations: Vec::new(),
            variant: Variant::View,
            identifier: Identifier::from(view.name()),
            input: view
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| Input {
                    identifier: Identifier::new(Symbol::intern(&format!("arg{}", index + 1)), Default::default()),
                    mode: Mode::None,
                    type_: stub_type(plaintext_or_panic(input.finalize_type())),
                    span: Default::default(),
                    id: Default::default(),
                })
                .collect_vec(),
            output: outputs,
            output_type,
            span: Default::default(),
            id: 0,
        }
    }

    pub fn from_closure<N: Network>(closure: &ClosureCore<N>, program_id: ProgramId) -> Self {
        let outputs = closure
            .outputs()
            .iter()
            .map(|output| match output.register_type() {
                RegisterType::Plaintext(val) => Output {
                    mode: Mode::None,
                    type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                    span: Default::default(),
                    id: Default::default(),
                },
                RegisterType::Record(_) | RegisterType::DynamicRecord => panic!("Closures do not return records"),
                RegisterType::ExternalRecord(_) => panic!("Closures do not return external records"),
                RegisterType::Future(_) | RegisterType::DynamicFuture => panic!("Closures do not return futures"),
            })
            .collect_vec();
        let output_vec = outputs.iter().map(|output| output.type_.kind().clone()).collect_vec();
        let output_type = match output_vec.len() {
            0 => TypeKind::Unit,
            1 => output_vec[0].clone(),
            _ => TypeKind::Tuple(TupleType::new(output_vec)),
        };
        Self {
            annotations: Vec::new(),
            variant: Variant::Fn,
            identifier: Identifier::from(closure.name()),
            input: closure
                .inputs()
                .iter()
                .enumerate()
                .map(|(index, input)| {
                    let arg_name = Identifier::new(Symbol::intern(&format!("arg{}", index + 1)), Default::default());
                    match input.register_type() {
                        RegisterType::Plaintext(val) => Input {
                            identifier: arg_name,
                            mode: Mode::None,
                            type_: stub_type(TypeKind::from_snarkvm(val, program_id)),
                            span: Default::default(),
                            id: Default::default(),
                        },
                        RegisterType::Record(_) | RegisterType::DynamicRecord => {
                            panic!("Closures do not contain records as inputs")
                        }
                        RegisterType::ExternalRecord(_) => panic!("Closures do not contain external records as inputs"),
                        RegisterType::Future(_) | RegisterType::DynamicFuture => {
                            panic!("Closures do not contain futures as inputs")
                        }
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
