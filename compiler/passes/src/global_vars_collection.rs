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

//! A collection pass that registers all *global variables* in the symbol table.
//!
//! This pass walks the program, modules, and stubs to collect globally declared
//! variables that exist independently of functions or local scopes. Specifically,
//! it inserts constants, mappings, and storage variables into the symbol table
//! under their fully qualified program/module paths.
//!
//! No semantic validation is performed at this stage. Types are left unset and
//! name conflicts are merely reported, deferring full validation to later passes
//! such as path resolution and type checking.
//!
//! # Responsibilities:
//! - Collect program-level and module-level `const` declarations.
//! - Collect global `mapping` and `storage` variables.
//! - Record each global variable with its declaration kind and source span.
//!
//! # Non-responsibilities:
//! - Does not resolve paths or distinguish local vs global usage.
//! - Does not create local scopes or insert local bindings.
//! - Does not infer or validate types.
//!
//! # Pipeline position:
//! This pass runs early in the pipeline, before `PathResolution`. It establishes
//! the set of globally declared variables so that later passes can correctly
//! resolve references and perform semantic analysis.

use crate::{CompilerState, Pass, VariableSymbol, VariableType};

use leo_ast::{
    AleoProgram,
    AstVisitor,
    ConstDeclaration,
    Location,
    Mapping,
    Module,
    ProgramScope,
    ProgramVisitor,
    StorageVariable,
};
use leo_errors::Result;
use leo_span::Symbol;

pub struct GlobalVarsCollection;

impl Pass for GlobalVarsCollection {
    type Input = ();
    type Output = ();

    const NAME: &'static str = "GlobalVarsCollection";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = GlobalVarsCollectionVisitor { state, program_name: Symbol::intern(""), module: vec![] };
        visitor.visit_program(ast.as_repr());
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}

struct GlobalVarsCollectionVisitor<'a> {
    /// The state of the compiler.
    state: &'a mut CompilerState,
    /// The current program name.
    program_name: Symbol,
    /// The current module name.
    module: Vec<Symbol>,
}

impl GlobalVarsCollectionVisitor<'_> {
    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }
}

impl AstVisitor for GlobalVarsCollectionVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_const(&mut self, input: &ConstDeclaration) {
        // Just add the const to the symbol table without validating it; that will happen later
        // in type checking.
        let const_path: Vec<Symbol> = self.module.iter().cloned().chain(std::iter::once(input.place.name)).collect();
        if let Err(err) = self.state.symbol_table.insert_variable(self.program_name, &const_path, VariableSymbol {
            type_: None,
            span: input.place.span,
            declaration: VariableType::Const,
        }) {
            self.state.handler.emit_err(err);
        }
    }
}

impl ProgramVisitor for GlobalVarsCollectionVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        // Set current program name
        self.program_name = input.program_id.name.name;

        // Visit the program scope
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
        input.storage_variables.iter().for_each(|(_, c)| self.visit_storage_variable(c));
    }

    fn visit_module(&mut self, input: &Module) {
        self.program_name = input.program_name;
        self.in_module_scope(&input.path.clone(), |slf| {
            input.consts.iter().for_each(|(_, c)| slf.visit_const(c));
        })
    }

    fn visit_mapping(&mut self, input: &Mapping) {
        if let Err(err) = self.state.symbol_table.insert_global(
            Location::new(self.program_name, vec![input.identifier.name]),
            VariableSymbol { type_: None, span: input.span, declaration: VariableType::Storage },
        ) {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_storage_variable(&mut self, input: &StorageVariable) {
        if let Err(err) = self.state.symbol_table.insert_global(
            Location::new(self.program_name, vec![input.identifier.name]),
            VariableSymbol { type_: None, span: input.span, declaration: VariableType::Storage },
        ) {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_aleo_program(&mut self, input: &AleoProgram) {
        self.program_name = input.stub_id.name.name;
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
    }
}
