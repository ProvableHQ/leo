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

use crate::{AsgContext, Circuit, DefinitionStatement, Function, Input, Type, Variable};
use leo_errors::{AsgError, Result, Span};

use indexmap::IndexMap;
use std::cell::{Cell, RefCell};

/// An abstract data type that track the current bindings for variables, functions, and circuits.
#[derive(Clone)]
pub struct Scope<'a> {
    pub context: AsgContext<'a>,

    /// The unique id of the scope.
    pub id: u32,

    /// The parent scope that this scope inherits.
    pub parent_scope: Cell<Option<&'a Scope<'a>>>,

    /// The function definition that this scope occurs in.
    pub function: Cell<Option<&'a Function<'a>>>,

    /// The circuit definition that this scope occurs in.
    pub circuit_self: Cell<Option<&'a Circuit<'a>>>,

    /// Maps variable name => variable.
    pub variables: RefCell<IndexMap<String, &'a Variable<'a>>>,

    /// Maps function name => function.
    pub functions: RefCell<IndexMap<String, &'a Function<'a>>>,

    /// Maps global constant name => global const code block.
    pub global_consts: RefCell<IndexMap<String, &'a DefinitionStatement<'a>>>,

    /// Maps circuit name => circuit.
    pub circuits: RefCell<IndexMap<String, &'a Circuit<'a>>>,

    /// The main input to the program.
    pub input: Cell<Option<Input<'a>>>,
}

#[allow(clippy::mut_from_ref)]
impl<'a> Scope<'a> {
    ///
    /// Returns a reference to the variable corresponding to the name.
    ///
    /// If the current scope did not have this name present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_variable(&self, name: &str) -> Option<&'a Variable<'a>> {
        if let Some(resolved) = self.variables.borrow().get(name) {
            Some(*resolved)
        } else if let Some(scope) = self.parent_scope.get() {
            scope.resolve_variable(name)
        } else {
            None
        }
    }

    ///
    /// Returns a reference to the current function.
    ///
    /// If the current scope did not have a function present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_current_function(&self) -> Option<&'a Function> {
        if let Some(resolved) = self.function.get() {
            Some(resolved)
        } else if let Some(scope) = self.parent_scope.get() {
            scope.resolve_current_function()
        } else {
            None
        }
    }

    ///
    /// Returns a reference to the current input.
    ///
    /// If the current scope did not have an input present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_input(&self) -> Option<Input<'a>> {
        if let Some(input) = self.input.get() {
            Some(input)
        } else if let Some(resolved) = self.parent_scope.get() {
            resolved.resolve_input()
        } else {
            None
        }
    }

    ///
    /// Returns a reference to the function corresponding to the name.
    ///
    /// If the current scope did not have this name present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_function(&self, name: &str) -> Option<&'a Function<'a>> {
        if let Some(resolved) = self.functions.borrow().get(name) {
            Some(*resolved)
        } else if let Some(resolved) = self.parent_scope.get() {
            resolved.resolve_function(name)
        } else {
            None
        }
    }

    ///
    /// Returns a reference to the circuit corresponding to the name.
    ///
    /// If the current scope did not have this name present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_circuit(&self, name: &str) -> Option<&'a Circuit<'a>> {
        if let Some(resolved) = self.circuits.borrow().get(name) {
            Some(*resolved)
        } else if name == "Self" && self.circuit_self.get().is_some() {
            self.circuit_self.get()
        } else if let Some(resolved) = self.parent_scope.get() {
            resolved.resolve_circuit(name)
        } else {
            None
        }
    }

    ///
    /// Returns a reference to the current circuit.
    ///
    /// If the current scope did not have a circuit self present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_circuit_self(&self) -> Option<&'a Circuit<'a>> {
        if let Some(resolved) = self.circuit_self.get() {
            Some(resolved)
        } else if let Some(resolved) = self.parent_scope.get() {
            resolved.resolve_circuit_self()
        } else {
            None
        }
    }

    ///
    /// Returns a new scope given a parent scope.
    ///
    pub fn make_subscope(self: &'a Scope<'a>) -> &'a Scope<'a> {
        self.context.alloc_scope(Scope::<'a> {
            context: self.context,
            id: self.context.get_id(),
            parent_scope: Cell::new(Some(self)),
            circuit_self: Cell::new(None),
            variables: RefCell::new(IndexMap::new()),
            functions: RefCell::new(IndexMap::new()),
            circuits: RefCell::new(IndexMap::new()),
            global_consts: RefCell::new(IndexMap::new()),
            function: Cell::new(None),
            input: Cell::new(None),
        })
    }

    ///
    /// Returns the type returned by the current scope.
    ///
    pub fn resolve_ast_type(&self, type_: &leo_ast::Type, span: &Span) -> Result<Type<'a>> {
        use leo_ast::Type::*;
        Ok(match type_ {
            Address => Type::Address,
            Boolean => Type::Boolean,
            Char => Type::Char,
            Field => Type::Field,
            Group => Type::Group,
            IntegerType(int_type) => Type::Integer(int_type.clone()),
            Array(sub_type, dimensions) => {
                let mut item = Box::new(self.resolve_ast_type(&*sub_type, span)?);
                for dimension in dimensions.0.iter().rev() {
                    let dimension = dimension
                        .value
                        .parse::<usize>()
                        .map_err(|_| AsgError::parse_index_error(span))?;
                    item = Box::new(Type::Array(item, dimension));
                }
                *item
            }
            Tuple(sub_types) => Type::Tuple(
                sub_types
                    .iter()
                    .map(|x| self.resolve_ast_type(x, span))
                    .collect::<Result<Vec<_>>>()?,
            ),
            Circuit(name) if name.name.as_ref() == "Self" => Type::Circuit(
                self.resolve_circuit_self()
                    .ok_or_else(|| AsgError::unresolved_circuit(&name.name, &name.span))?,
            ),
            SelfType => Type::Circuit(
                self.resolve_circuit_self()
                    .ok_or_else(|| AsgError::reference_self_outside_circuit(span))?,
            ),
            Circuit(name) => Type::Circuit(
                self.resolve_circuit(&name.name)
                    .ok_or_else(|| AsgError::unresolved_circuit(&name.name, &name.span))?,
            ),
        })
    }
}
