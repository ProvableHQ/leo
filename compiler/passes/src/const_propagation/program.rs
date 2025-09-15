// Copyright (C) 2019-2025 Provable Inc.
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

use super::ConstPropagationVisitor;

use leo_ast::{
    AstReconstructor as _,
    ConstParameter,
    Constructor,
    Function,
    Input,
    Module,
    Node,
    Output,
    ProgramReconstructor,
    ProgramScope,
    Statement,
};

impl ProgramReconstructor for ConstPropagationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, mut input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.name.name;

        for (_sym, c) in input.consts.iter_mut() {
            let Statement::Const(declaration) = self.reconstruct_const(std::mem::take(c)).0 else {
                panic!("`reconstruct_const` always returns `Statement::Const`");
            };
            *c = declaration;
        }

        for (_sym, f) in input.functions.iter_mut() {
            *f = self.reconstruct_function(std::mem::take(f));
        }

        if let Some(c) = input.constructor.as_mut() {
            *c = self.reconstruct_constructor(std::mem::take(c));
        }

        input.structs = input.structs.into_iter().map(|(i, c)| (i, self.reconstruct_struct(c))).collect();
        input.mappings =
            input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect();
        input.storage_variables =
            input.storage_variables.into_iter().map(|(id, var)| (id, self.reconstruct_storage_variable(var))).collect();

        input
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        self.program = input.program_name;
        self.in_module_scope(&input.path.clone(), |slf| {
            Module {
                // Reconstruct consts firsts
                consts: input
                    .consts
                    .into_iter()
                    .map(|(i, c)| match slf.reconstruct_const(c) {
                        (Statement::Const(declaration), _) => (i, declaration),
                        _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                    })
                    .collect(),
                program_name: input.program_name,
                path: input.path,
                structs: input.structs.into_iter().map(|(i, c)| (i, slf.reconstruct_struct(c))).collect(),
                functions: input.functions.into_iter().map(|(i, f)| (i, slf.reconstruct_function(f))).collect(),
            }
        })
    }

    fn reconstruct_function(&mut self, mut function: Function) -> Function {
        self.in_scope(function.id(), |slf| {
            function.const_parameters = function
                .const_parameters
                .iter()
                .map(|param| ConstParameter { type_: slf.reconstruct_type(param.type_.clone()).0, ..param.clone() })
                .collect();
            function.input = function
                .input
                .iter()
                .map(|input| Input { type_: slf.reconstruct_type(input.type_.clone()).0, ..input.clone() })
                .collect();
            function.output = function
                .output
                .iter()
                .map(|output| Output { type_: slf.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect();
            function.output_type = slf.reconstruct_type(function.output_type).0;
            function.block = slf.reconstruct_block(function.block).0;
            function
        })
    }

    fn reconstruct_constructor(&mut self, mut constructor: Constructor) -> Constructor {
        self.in_scope(constructor.id(), |slf| {
            constructor.block = slf.reconstruct_block(constructor.block).0;
            constructor
        })
    }
}
