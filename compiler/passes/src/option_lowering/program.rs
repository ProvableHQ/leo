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

use super::OptionLoweringVisitor;
use leo_ast::{
    AstReconstructor,
    ConstParameter,
    Function,
    Input,
    Module,
    Output,
    Program,
    ProgramReconstructor,
    ProgramScope,
    Statement,
};
use leo_span::Symbol;

impl ProgramReconstructor for OptionLoweringVisitor<'_> {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        // Reconstruct all structs first and keep track of them in `self.reconstructed_structs`.
        for (_, scope) in &input.program_scopes {
            for (_, c) in &scope.structs {
                let new_struct = self.reconstruct_struct(c.clone());
                self.reconstructed_structs.insert(vec![new_struct.name()], new_struct);
            }
        }
        for (module_path, module) in &input.modules {
            for (_, c) in &module.structs {
                let full_name = module_path.iter().cloned().chain(std::iter::once(c.name())).collect::<Vec<Symbol>>();
                let new_struct = self.reconstruct_struct(c.clone());
                self.reconstructed_structs.insert(full_name, new_struct.clone());
            }
        }

        // Now we're ready to reconstruct everything else.
        Program {
            imports: input
                .imports
                .into_iter()
                .map(|(id, import)| (id, (self.reconstruct_import(import.0), import.1)))
                .collect(),
            stubs: input.stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect(),
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(id, scope)| (id, self.reconstruct_program_scope(scope)))
                .collect(),
            modules: input.modules.into_iter().map(|(id, module)| (id, self.reconstruct_module(module))).collect(),
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.name.name;

        let mut program = ProgramScope {
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(decl), _) => (i, decl),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            structs: self
                .reconstructed_structs
                .iter()
                .filter_map(|(path, s)| {
                    path.split_last().filter(|(_, rest)| rest.is_empty()).map(|(last, _)| (*last, s.clone()))
                })
                .collect(),
            mappings: input.mappings.into_iter().map(|(id, m)| (id, self.reconstruct_mapping(m))).collect(),
            storage_variables: input
                .storage_variables
                .into_iter()
                .map(|(id, v)| (id, self.reconstruct_storage_variable(v)))
                .collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            ..input
        };

        program.structs.extend(self.new_structs.drain(..));
        program
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        self.program = input.program_name;
        self.in_module_scope(&input.path.clone(), |slf| Module {
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match slf.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            structs: slf
                .reconstructed_structs
                .iter()
                .filter_map(|(path, c)| path.split_last().map(|(last, rest)| (last, rest, c)))
                .filter(|&(_, rest, _)| input.path == rest)
                .map(|(last, _, c)| (*last, c.clone()))
                .collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, slf.reconstruct_function(f))).collect(),
            ..input
        })
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        self.function = Some(input.identifier.name);
        Function {
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| ConstParameter { type_: self.reconstruct_type(param.type_.clone()).0, ..param.clone() })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| Input { type_: self.reconstruct_type(input.type_.clone()).0, ..input.clone() })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| Output { type_: self.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect(),
            output_type: self.reconstruct_type(input.output_type).0,
            block: self.reconstruct_block(input.block).0,
            ..input
        }
    }
}
