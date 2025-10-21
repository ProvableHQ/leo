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

use crate::{CompilerState, Pass, VariableSymbol, VariableType};

use leo_ast::{
    AstVisitor,
    Composite,
    ConstDeclaration,
    Function,
    FunctionStub,
    Location,
    Mapping,
    MappingType,
    Module,
    OptionalType,
    Program,
    ProgramScope,
    ProgramVisitor,
    StorageVariable,
    Stub,
    Type,
    Variant,
};
use leo_errors::{AstError, LeoError, Result};
use leo_span::Symbol;

use indexmap::IndexSet;

/// A pass to fill the SymbolTable.
///
/// Only creates the global data - local data will be constructed during type checking.
pub struct SymbolTableCreation;

impl Pass for SymbolTableCreation {
    type Input = ();
    type Output = ();

    const NAME: &'static str = "SymbolTableCreation";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = SymbolTableCreationVisitor {
            state,
            structs: IndexSet::new(),
            program_name: Symbol::intern(""),
            module: vec![],
            is_stub: false,
        };
        visitor.visit_program(ast.as_repr());
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}

struct SymbolTableCreationVisitor<'a> {
    /// The state of the compiler.
    state: &'a mut CompilerState,
    /// The current program name.
    program_name: Symbol,
    /// The current module name.
    module: Vec<Symbol>,
    /// Whether or not traversing stub.
    is_stub: bool,
    /// The set of local structs that have been successfully visited.
    structs: IndexSet<Vec<Symbol>>,
}

impl SymbolTableCreationVisitor<'_> {
    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }
}

impl AstVisitor for SymbolTableCreationVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_const(&mut self, input: &ConstDeclaration) {
        // Just add the const to the symbol table without validating it; that will happen later
        // in type checking.
        let const_path: Vec<Symbol> = self.module.iter().cloned().chain(std::iter::once(input.place.name)).collect();
        if let Err(err) = self.state.symbol_table.insert_variable(self.program_name, &const_path, VariableSymbol {
            type_: input.type_.clone(),
            span: input.place.span,
            declaration: VariableType::Const,
        }) {
            self.state.handler.emit_err(err);
        }
    }
}

impl ProgramVisitor for SymbolTableCreationVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        // Set current program name
        self.program_name = input.program_id.name.name;
        self.is_stub = false;

        // Visit the program scope
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));
        input.structs.iter().for_each(|(_, c)| self.visit_struct(c));
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
        input.storage_variables.iter().for_each(|(_, c)| self.visit_storage_variable(c));
        input.functions.iter().for_each(|(_, c)| self.visit_function(c));
        if let Some(c) = input.constructor.as_ref() {
            self.visit_constructor(c);
        }
    }

    fn visit_module(&mut self, input: &Module) {
        self.program_name = input.program_name;
        self.in_module_scope(&input.path.clone(), |slf| {
            input.structs.iter().for_each(|(_, c)| slf.visit_struct(c));
            input.functions.iter().for_each(|(_, c)| slf.visit_function(c));
            input.consts.iter().for_each(|(_, c)| slf.visit_const(c));
        })
    }

    fn visit_import(&mut self, input: &Program) {
        self.visit_program(input)
    }

    fn visit_struct(&mut self, input: &Composite) {
        // Allow up to one local redefinition for each external struct.
        let full_name = self.module.iter().cloned().chain(std::iter::once(input.name())).collect::<Vec<Symbol>>();

        if !input.is_record && !self.structs.insert(full_name.clone()) {
            return self.state.handler.emit_err::<LeoError>(AstError::shadowed_struct(input.name(), input.span).into());
        }
        if input.is_record {
            // While records are not allowed in submodules, we stll use their full name in the records table.
            // We don't expect the full name to have more than a single Symbol though.
            let program_name = input.external.unwrap_or(self.program_name);
            if let Err(err) =
                self.state.symbol_table.insert_record(Location::new(program_name, full_name), input.clone())
            {
                self.state.handler.emit_err(err);
            }
        } else if let Err(err) = self.state.symbol_table.insert_struct(self.program_name, &full_name, input.clone()) {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_mapping(&mut self, input: &Mapping) {
        // Add the variable associated with the mapping to the symbol table.
        if let Err(err) = self.state.symbol_table.insert_global(
            Location::new(self.program_name, vec![input.identifier.name]),
            VariableSymbol {
                type_: Type::Mapping(MappingType {
                    key: Box::new(input.key_type.clone()),
                    value: Box::new(input.value_type.clone()),
                    program: self.program_name,
                }),
                span: input.span,
                declaration: VariableType::Storage,
            },
        ) {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_storage_variable(&mut self, input: &StorageVariable) {
        // Add the variable associated with the mapping to the symbol table.

        // The type of non-vector storage variables is implicitly wrapped in an optional.
        let type_ = match input.type_ {
            Type::Vector(_) => input.type_.clone(),
            _ => Type::Optional(OptionalType { inner: Box::new(input.type_.clone()) }),
        };

        if let Err(err) = self.state.symbol_table.insert_global(
            Location::new(self.program_name, vec![input.identifier.name]),
            VariableSymbol { type_, span: input.span, declaration: VariableType::Storage },
        ) {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_function(&mut self, input: &Function) {
        let full_name = self.module.iter().cloned().chain(std::iter::once(input.name())).collect::<Vec<Symbol>>();
        if let Err(err) =
            self.state.symbol_table.insert_function(Location::new(self.program_name, full_name), input.clone())
        {
            self.state.handler.emit_err(err);
        }
    }

    fn visit_stub(&mut self, input: &Stub) {
        self.is_stub = true;
        self.program_name = input.stub_id.name.name;
        input.functions.iter().for_each(|(_, c)| self.visit_function_stub(c));
        input.structs.iter().for_each(|(_, c)| self.visit_struct_stub(c));
        input.mappings.iter().for_each(|(_, c)| self.visit_mapping(c));
    }

    fn visit_function_stub(&mut self, input: &FunctionStub) {
        // Construct the location for the function.
        let location = Location::new(self.program_name, vec![input.name()]);
        // Initialize the function symbol.
        if let Err(err) = self.state.symbol_table.insert_function(location.clone(), Function::from(input.clone())) {
            self.state.handler.emit_err(err);
        }

        // If the `FunctionStub` is an async transition, attach the finalize logic to the function.
        // NOTE - for an external function like this, we really only need to attach the finalizer
        // for the use of `assert_simple_async_transition_call` in the static analyzer.
        // In principle that could be handled differently.
        if matches!(input.variant, Variant::AsyncTransition) {
            // This matches the logic in the disassembler.
            let name = Symbol::intern(&format!("finalize/{}", input.name()));
            if let Err(err) = self.state.symbol_table.attach_finalizer(
                location,
                Location::new(self.program_name, vec![name]),
                Vec::new(),
                Vec::new(),
            ) {
                self.state.handler.emit_err(err);
            }
        }
    }

    fn visit_struct_stub(&mut self, input: &Composite) {
        if let Some(program) = input.external {
            assert_eq!(program, self.program_name);
        }

        if input.is_record {
            let program_name = input.external.unwrap_or(self.program_name);
            if let Err(err) =
                self.state.symbol_table.insert_record(Location::new(program_name, vec![input.name()]), input.clone())
            {
                self.state.handler.emit_err(err);
            }
        } else if let Err(err) =
            self.state.symbol_table.insert_struct(self.program_name, &[input.name()], input.clone())
        {
            self.state.handler.emit_err(err);
        }
    }
}
