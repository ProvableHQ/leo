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

//! Collects all *global items* into the symbol table after path resolution.
//!
//! This pass is responsible for registering globally visible program items,
//! including functions, structs, records, mappings, constants, and storage
//! variables, along with their associated types. It operates only on *resolved*
//! paths and assumes that global variables and local scopes have already been
//! established by earlier passes.
//!
//! Unlike earlier pipeline stages, this pass does **not** create scopes,
//! resolve names, or insert local symbols. Its sole responsibility is to
//! populate the global portion of the symbol table with fully qualified
//! `Location`s and to attach type information where applicable.
//!
//! This pass runs after `GlobalVarsCollection` and `PathResolution`, and
//! before type checking. After it completes, the symbol table is guaranteed
//! to contain all globally defined items, enabling subsequent passes to
//! perform type checking and validation without mutating symbol structure.

use crate::{CompilerState, Pass};

use leo_ast::{
    AleoProgram,
    Ast,
    AstVisitor,
    Composite,
    ConstDeclaration,
    Function,
    FunctionStub,
    Interface,
    Library,
    Location,
    Mapping,
    MappingType,
    Module,
    OptionalType,
    ProgramScope,
    StorageVariable,
    Type,
    UnitVisitor,
};
use leo_errors::Result;
use leo_span::Symbol;

/// A pass to fill the SymbolTable.
///
/// Only creates the global data - local data will be constructed during type checking.
pub struct GlobalItemsCollection;

impl Pass for GlobalItemsCollection {
    type Input = ();
    type Output = ();

    const NAME: &'static str = "GlobalItemsCollection";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = GlobalItemsCollectionVisitor { state, unit_name: Symbol::intern(""), module: vec![] };

        match &ast {
            Ast::Program(program) => visitor.visit_program(program),
            Ast::Library(library) => visitor.visit_library(library),
        }

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}

struct GlobalItemsCollectionVisitor<'a> {
    /// The state of the compiler.
    state: &'a mut CompilerState,
    /// The current program name.
    unit_name: Symbol,
    /// The current module name.
    module: Vec<Symbol>,
}

impl GlobalItemsCollectionVisitor<'_> {
    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }
}

impl AstVisitor for GlobalItemsCollectionVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_const(&mut self, input: &ConstDeclaration) {
        // Just set the type of the const in the symbol table.
        let const_path: Vec<Symbol> = self.module.iter().cloned().chain(std::iter::once(input.place.name)).collect();
        self.state.symbol_table.set_global_type(&Location::new(self.unit_name, const_path), input.type_.clone());
    }
}

impl UnitVisitor for GlobalItemsCollectionVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        // Set current program name
        self.unit_name = input.program_id.as_symbol();

        // Visit the program scope
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));
        input.composites.iter().for_each(|(_, c)| self.visit_composite(c));
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
        input.storage_variables.iter().for_each(|(_, c)| self.visit_storage_variable(c));
        input.functions.iter().for_each(|(_, c)| self.visit_function(c));
        input.interfaces.iter().for_each(|(_, c)| self.visit_interface(c));
        if let Some(c) = input.constructor.as_ref() {
            self.visit_constructor(c);
        }
    }

    fn visit_module(&mut self, input: &Module) {
        self.unit_name = input.unit_name;
        self.in_module_scope(&input.path.clone(), |slf| {
            input.composites.iter().for_each(|(_, c)| slf.visit_composite(c));
            input.functions.iter().for_each(|(_, c)| slf.visit_function(c));
            input.consts.iter().for_each(|(_, c)| slf.visit_const(c));
            input.interfaces.iter().for_each(|(_, c)| slf.visit_interface(c));
        })
    }

    fn visit_composite(&mut self, input: &Composite) {
        let full_name = self.module.iter().cloned().chain(std::iter::once(input.name())).collect::<Vec<Symbol>>();

        if input.is_record {
            // While records are not allowed in submodules, we stll use their full name in the records table.
            // We don't expect the full name to have more than a single Symbol though.
            if let Err(err) =
                self.state.symbol_table.insert_record(Location::new(self.unit_name, full_name), input.clone())
            {
                self.state.handler.emit_err(err);
            }
        } else if let Err(err) =
            self.state.symbol_table.insert_struct(Location::new(self.unit_name, full_name.clone()), input.clone())
        {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_mapping(&mut self, input: &Mapping) {
        // Set the type of the variable associated with the mapping in the symbol table.
        self.state.symbol_table.set_global_type(
            &Location::new(self.unit_name, vec![input.identifier.name]),
            Type::Mapping(MappingType {
                key: Box::new(input.key_type.clone()),
                value: Box::new(input.value_type.clone()),
            }),
        );
    }

    fn visit_storage_variable(&mut self, input: &StorageVariable) {
        // Set the type of the storage variable in the symbol table.

        // The type of non-vector storage variables is implicitly wrapped in an optional.
        let type_ = match input.type_ {
            Type::Vector(_) => input.type_.clone(),
            _ => Type::Optional(OptionalType { inner: Box::new(input.type_.clone()) }),
        };

        self.state.symbol_table.set_global_type(&Location::new(self.unit_name, vec![input.identifier.name]), type_);
    }

    fn visit_function(&mut self, input: &Function) {
        let full_name = self.module.iter().cloned().chain(std::iter::once(input.name())).collect::<Vec<Symbol>>();
        let loc = Location::new(self.unit_name, full_name);
        if let Err(err) = self.state.symbol_table.insert_function(loc, input.clone()) {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_interface(&mut self, input: &Interface) {
        let full_name = self.module.iter().cloned().chain(std::iter::once(input.name())).collect::<Vec<Symbol>>();
        if let Err(err) =
            self.state.symbol_table.insert_interface(Location::new(self.unit_name, full_name), input.clone())
        {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_library(&mut self, input: &Library) {
        self.unit_name = input.name;

        input.interfaces.iter().for_each(|(_, i)| self.visit_interface(i));
        input.structs.iter().for_each(|(_, s)| self.visit_composite(s));
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));
        input.functions.iter().for_each(|(_, f)| self.visit_function(f));
        input.modules.values().for_each(|m| {
            self.visit_module(m);
        });
        input.stubs.values().for_each(|stub| self.visit_stub(stub));
    }

    fn visit_aleo_program(&mut self, input: &AleoProgram) {
        self.unit_name = input.stub_id.as_symbol();

        input.functions.iter().for_each(|(_, c)| self.visit_function_stub(c));
        input.composites.iter().for_each(|(_, c)| self.visit_composite_stub(c));
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
    }

    fn visit_function_stub(&mut self, input: &FunctionStub) {
        // Construct the location for the function.
        let location = Location::new(self.unit_name, vec![input.name()]);
        // Initialize the function symbol.
        if let Err(err) = self.state.symbol_table.insert_function(location.clone(), Function::from(input.clone())) {
            self.state.handler.emit_err(err);
        }

        // If the `FunctionStub` is an async transition, attach the finalize logic to the function.
        // NOTE - for an external function like this, we really only need to attach the finalizer
        // for the use of `assert_simple_async_transition_call` in the static analyzer.
        // In principle that could be handled differently.
        if input.has_final_output() {
            // This matches the logic in the disassembler.
            let name = Symbol::intern(&format!("finalize/{}", input.name()));
            if let Err(err) = self.state.symbol_table.attach_finalizer(
                location,
                Location::new(self.unit_name, vec![name]),
                Vec::new(),
                Vec::new(),
            ) {
                self.state.handler.emit_err(err);
            }
        }
    }

    fn visit_composite_stub(&mut self, input: &Composite) {
        if input.is_record {
            if let Err(err) =
                self.state.symbol_table.insert_record(Location::new(self.unit_name, vec![input.name()]), input.clone())
            {
                self.state.handler.emit_err(err);
            }
        } else if let Err(err) =
            self.state.symbol_table.insert_struct(Location::new(self.unit_name, vec![input.name()]), input.clone())
        {
            self.state.handler.emit_err(err);
        }
    }
}
