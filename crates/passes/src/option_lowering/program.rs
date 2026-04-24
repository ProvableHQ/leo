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

use super::OptionLoweringVisitor;
use crate::common::{items_at_path, library_composites, program_composites};
use leo_ast::{
    AleoProgram,
    AstReconstructor,
    Composite,
    ConstParameter,
    Function,
    Input,
    Library,
    Location,
    Module,
    Output,
    Program,
    ProgramScope,
    Statement,
    Stub,
    UnitReconstructor,
};

impl OptionLoweringVisitor<'_> {
    /// Phase 1 (collect): walk the entire AST rooted at `input` and pre-populate
    /// `self.composites` with every composite encountered, keyed by its owning unit
    /// (program scope, library, or Aleo program) and its full path within that unit.
    ///
    /// Registering every composite up front ensures that any later call to `wrap_none`
    /// can synthesize a zero value for any struct referenced by the program, even when
    /// that struct lives in a different unit (a library or an imported Aleo program).
    ///
    /// This call also inserts wrapper structs for any `Optional<T>` types appearing
    /// inside struct member types; those wrappers are keyed to the unit containing
    /// the struct.
    pub fn collect_composites_from_program(&mut self, input: &Program) {
        for (loc, c) in program_composites(input) {
            self.collect_composite(loc, c);
        }
        for (_, stub) in &input.stubs {
            self.collect_composites_from_stub(stub);
        }
    }

    pub fn collect_composites_from_library(&mut self, input: &Library) {
        for (loc, c) in library_composites(input) {
            self.collect_composite(loc, c);
        }
        for (_, stub) in &input.stubs {
            self.collect_composites_from_stub(stub);
        }
    }

    pub fn collect_composites_from_aleo_program(&mut self, input: &AleoProgram) {
        let program = input.stub_id.as_symbol();
        for (name, c) in &input.composites {
            self.collect_composite(Location::new(program, vec![*name]), c);
        }
    }

    pub fn collect_composites_from_stub(&mut self, stub: &Stub) {
        match stub {
            Stub::FromLeo { program, .. } => self.collect_composites_from_program(program),
            Stub::FromAleo { program, .. } => self.collect_composites_from_aleo_program(program),
            Stub::FromLibrary { library, .. } => self.collect_composites_from_library(library),
        }
    }

    fn collect_composite(&mut self, loc: Location, c: &Composite) {
        // A composite may be reachable through multiple stub paths (e.g. a library imported
        // by both the main program and a consumed library); skip reconstruction on revisit to
        // avoid redundant cloning and wrapper-insertion work.
        if self.composites.contains_key(&loc) {
            return;
        }
        self.program = loc.program;
        let reconstructed = self.reconstruct_composite(c.clone());
        self.composites.insert(loc, reconstructed);
    }
}

impl UnitReconstructor for OptionLoweringVisitor<'_> {
    fn reconstruct_library(&mut self, input: Library) -> Library {
        self.program = input.name;

        // Reconstruct everything that may produce Optional wrapper structs before pulling
        // the final `structs:` list, so wrappers generated in submodules or function bodies
        // are included.
        let modules = input.modules.into_iter().map(|(id, m)| (id, self.reconstruct_module(m))).collect();
        let consts = input
            .consts
            .into_iter()
            .map(|(i, c)| match self.reconstruct_const(c) {
                (Statement::Const(declaration), _) => (i, declaration),
                _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
            })
            .collect();
        let functions = input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect();
        let stubs = input.stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect();
        let structs = items_at_path(&self.composites, input.name, &[]).collect();

        Library { name: input.name, modules, consts, structs, functions, interfaces: input.interfaces, stubs }
    }

    fn reconstruct_program(&mut self, input: Program) -> Program {
        Program {
            modules: input.modules.into_iter().map(|(id, module)| (id, self.reconstruct_module(module))).collect(),
            imports: input.imports,
            stubs: input.stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect(),
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(id, scope)| (id, self.reconstruct_program_scope(scope)))
                .collect(),
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.as_symbol();
        let program_name = self.program;

        // Reconstruct everything that may produce Optional wrapper structs before pulling
        // the final `composites:` list.
        let consts = input
            .consts
            .into_iter()
            .map(|(i, c)| match self.reconstruct_const(c) {
                (Statement::Const(decl), _) => (i, decl),
                _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
            })
            .collect();
        let mappings = input.mappings.into_iter().map(|(id, m)| (id, self.reconstruct_mapping(m))).collect();
        let storage_variables =
            input.storage_variables.into_iter().map(|(id, v)| (id, self.reconstruct_storage_variable(v))).collect();
        let functions = input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect();
        let interfaces = input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect();
        let constructor = input.constructor.map(|c| self.reconstruct_constructor(c));
        let composites = items_at_path(&self.composites, program_name, &[]).collect();

        ProgramScope { consts, composites, mappings, storage_variables, functions, interfaces, constructor, ..input }
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        self.program = input.unit_name;
        self.in_module_scope(&input.path.clone(), |slf| Module {
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match slf.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            composites: items_at_path(&slf.composites, slf.program, &input.path).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, slf.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, slf.reconstruct_interface(int))).collect(),
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
