// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! This module defines the program node for an asg.
//!
//!

mod circuit;
pub use circuit::*;

mod function;
pub use function::*;

use crate::{node::FromAst, ArenaNode, AsgContext, DefinitionStatement, Input, Scope, Statement};
// use leo_ast::{PackageAccess, PackageOrPackages};
use leo_errors::{AsgError, Result};

use indexmap::IndexMap;
use std::cell::{Cell, RefCell};

/// Stores the Leo program abstract semantic graph (ASG).
#[derive(Clone)]
pub struct Program<'a> {
    pub context: AsgContext<'a>,

    /// The unique id of the program.
    pub id: u32,

    /// The program file name.
    pub name: String,

    /// The packages imported by this program.
    /// these should generally not be accessed directly, but through scoped imports
    pub imported_modules: IndexMap<String, Program<'a>>,

    /// Maps function name => function code block.
    pub functions: IndexMap<String, &'a Function<'a>>,

    /// Maps global constant name => global const code block.
    pub global_consts: IndexMap<String, &'a DefinitionStatement<'a>>,

    /// Maps circuit name => circuit code block.
    pub circuits: IndexMap<String, &'a Circuit<'a>>,

    pub scope: &'a Scope<'a>,
}

impl<'a> Program<'a> {
    /// Returns a new Leo program ASG from the given Leo program AST and its imports.
    ///
    /// Stages:
    /// 1. resolve imports into super scope
    /// 2. finalize declared types
    /// 3. finalize declared functions
    /// 4. resolve all asg nodes
    ///
    pub fn new(context: AsgContext<'a>, program: &leo_ast::Program) -> Result<Program<'a>> {
        let mut imported_modules: IndexMap<String, Program> = IndexMap::new();

        for (name, program) in program.imports.iter() {
            imported_modules.insert(name.clone(), Program::new(context, program)?);
        }

        let import_scope = match context.arena.alloc(ArenaNode::Scope(Box::new(Scope {
            context,
            id: context.get_id(),
            parent_scope: Cell::new(None),
            variables: RefCell::new(IndexMap::new()),
            functions: RefCell::new(IndexMap::new()),
            global_consts: RefCell::new(IndexMap::new()),
            circuits: RefCell::new(IndexMap::new()),
            function: Cell::new(None),
            input: Cell::new(None),
        }))) {
            ArenaNode::Scope(c) => c,
            _ => unimplemented!(),
        };

        let scope = import_scope.context.alloc_scope(Scope {
            context,
            input: Cell::new(Some(Input::new(import_scope))), // we use import_scope to avoid recursive scope ref here
            id: context.get_id(),
            parent_scope: Cell::new(Some(import_scope)),
            variables: RefCell::new(IndexMap::new()),
            functions: RefCell::new(IndexMap::new()),
            global_consts: RefCell::new(IndexMap::new()),
            circuits: RefCell::new(IndexMap::new()),
            function: Cell::new(None),
        });

        // Prepare header-like scope entries.
        for (name, circuit) in program.circuits.iter() {
            let asg_circuit = Circuit::init(scope, circuit)?;

            scope.circuits.borrow_mut().insert(name.clone(), asg_circuit);
        }

        // Second pass for circuit members.
        for (name, circuit) in program.circuits.iter() {
            let asg_circuit = Circuit::init_member(scope, circuit)?;

            scope.circuits.borrow_mut().insert(name.clone(), asg_circuit);
        }

        for (name, function) in program.functions.iter() {
            let function = Function::init(scope, function)?;

            scope.functions.borrow_mut().insert(name.clone(), function);
        }

        for (name, global_const) in program.global_consts.iter() {
            global_const
                .variable_names
                .iter()
                .for_each(|variable_name| assert!(name.contains(&variable_name.identifier.name.to_string())));
            let gc = <&Statement<'a>>::from_ast(scope, global_const, None)?;
            if let Statement::Definition(gc) = gc {
                scope.global_consts.borrow_mut().insert(name.clone(), gc);
            }
        }

        // Load concrete definitions.
        let mut global_consts = IndexMap::new();
        for (name, global_const) in program.global_consts.iter() {
            global_const
                .variable_names
                .iter()
                .for_each(|variable_name| assert!(name.contains(&variable_name.identifier.name.to_string())));
            let asg_global_const = *scope.global_consts.borrow().get(name).unwrap();

            global_consts.insert(name.clone(), asg_global_const);
        }

        let mut functions = IndexMap::new();
        for (name, function) in program.functions.iter() {
            let asg_function = *scope.functions.borrow().get(name).unwrap();

            asg_function.fill_from_ast(function)?;

            let name = name.clone();

            if functions.contains_key(&name) {
                return Err(AsgError::duplicate_function_definition(name, &function.span).into());
            }

            functions.insert(name, asg_function);
        }

        let mut circuits = IndexMap::new();
        for (name, circuit) in program.circuits.iter() {
            let asg_circuit = *scope.circuits.borrow().get(name).unwrap();

            asg_circuit.fill_from_ast(circuit)?;

            circuits.insert(name.clone(), asg_circuit);
        }

        Ok(Program {
            context,
            id: context.get_id(),
            name: program.name.clone(),
            functions,
            global_consts,
            circuits,
            imported_modules,
            scope,
        })
    }

    /* pub(crate) fn set_core_mapping(&self, mapping: &str) {
        for (_, circuit) in self.circuits.iter() {
            circuit.core_mapping.replace(Some(mapping.to_string()));
        }
    } */
}
