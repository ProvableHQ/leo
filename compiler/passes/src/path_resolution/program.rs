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

use super::PathResolutionVisitor;
use crate::{VariableSymbol, VariableType};
use leo_ast::{
    AleoProgram,
    AstReconstructor,
    Composite,
    ConstParameter,
    Constructor,
    Function,
    FunctionStub,
    Input,
    Member,
    Module,
    Output,
    ProgramReconstructor,
    ProgramScope,
    Statement,
};

impl ProgramReconstructor for PathResolutionVisitor<'_> {
    fn reconstruct_aleo_program(&mut self, input: AleoProgram) -> AleoProgram {
        AleoProgram {
            imports: input.imports,
            stub_id: input.stub_id,
            consts: input.consts,
            composites: input.composites,
            mappings: input.mappings,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function_stub(f))).collect(),
            span: input.span,
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.name.name;

        // This is the default implementation.
        ProgramScope {
            program_id: input.program_id,
            parent: input.parent,
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            composites: input.composites.into_iter().map(|(i, c)| (i, self.reconstruct_composite(c))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            storage_variables: input
                .storage_variables
                .into_iter()
                .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
                .collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        self.program = input.program_name;
        self.in_module_scope(&input.path.clone(), |slf| Module {
            program_name: input.program_name,
            path: input.path,
            composites: input.composites.into_iter().map(|(i, c)| (i, slf.reconstruct_composite(c))).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, slf.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, slf.reconstruct_interface(int))).collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match slf.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
        })
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        self.in_scope(input.id, |slf| Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|const_param| {
                    let (ty, _) = slf.reconstruct_type(const_param.type_.clone());

                    if let Err(err) = slf.state.symbol_table.insert_variable(
                        slf.program,
                        &[const_param.identifier.name],
                        VariableSymbol {
                            type_: Some(ty.clone()),
                            span: const_param.identifier.span,
                            declaration: VariableType::ConstParameter,
                        },
                    ) {
                        slf.state.handler.emit_err(err);
                    }

                    ConstParameter { type_: ty, ..const_param.clone() }
                })
                .collect(),
            input: input
                .input
                .iter()
                .map(|inp| {
                    let (ty, _) = slf.reconstruct_type(inp.type_.clone());
                    let name = inp.identifier().name;

                    if let Err(err) = slf.state.symbol_table.insert_variable(slf.program, &[name], VariableSymbol {
                        type_: Some(ty.clone()),
                        span: inp.identifier.span,
                        declaration: VariableType::Input(inp.mode()),
                    }) {
                        slf.state.handler.emit_err(err);
                    }

                    Input { type_: ty, ..inp.clone() }
                })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| Output { type_: slf.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect(),
            output_type: slf.reconstruct_type(input.output_type).0,
            block: slf.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        })
    }

    fn reconstruct_function_stub(&mut self, input: FunctionStub) -> FunctionStub {
        self.in_scope(input.id, |slf| FunctionStub {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            input: input
                .input
                .iter()
                .map(|inp| Input { type_: slf.reconstruct_type(inp.type_.clone()).0, ..inp.clone() })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| Output { type_: slf.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect(),
            output_type: slf.reconstruct_type(input.output_type).0,
            span: input.span,
            id: input.id,
        })
    }

    fn reconstruct_composite(&mut self, input: Composite) -> Composite {
        self.in_scope(input.id, |slf| {
            Composite {
                const_parameters: input
                    .const_parameters
                    .iter()
                    .map(|const_param| {
                        // Reconstruct type early
                        let (ty, _) = slf.reconstruct_type(const_param.type_.clone());

                        // Insert with reconstructed type
                        if let Err(err) = slf.state.symbol_table.insert_variable(
                            slf.program,
                            &[const_param.identifier.name],
                            VariableSymbol {
                                type_: Some(ty.clone()),
                                span: const_param.identifier.span,
                                declaration: VariableType::ConstParameter,
                            },
                        ) {
                            slf.state.handler.emit_err(err);
                        }

                        // Yield reconstructed ConstParameter
                        ConstParameter { type_: ty, ..const_param.clone() }
                    })
                    .collect(),

                members: input
                    .members
                    .iter()
                    .map(|member| {
                        let (ty, _) = slf.reconstruct_type(member.type_.clone());
                        Member { type_: ty, ..member.clone() }
                    })
                    .collect(),

                ..input
            }
        })
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        self.in_scope(input.id, |slf| Constructor {
            annotations: input.annotations,
            block: slf.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        })
    }
}
