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

use crate::{AsgConvertError, Circuit, Function, Input, Type, Variable};

use indexmap::IndexMap;
use std::{cell::RefCell, sync::Arc};
use uuid::Uuid;

/// An abstract data type that track the current bindings for variables, functions, and circuits.
#[derive(Debug)]
pub struct InnerScope {
    /// The unique id of the scope.
    pub id: Uuid,

    /// The parent scope that this scope inherits.
    pub parent_scope: Option<Scope>,

    /// The function definition that this scope occurs in.
    pub function: Option<Arc<Function>>,

    /// The circuit definition that this scope occurs in.
    pub circuit_self: Option<Arc<Circuit>>,

    /// Maps variable name => variable.
    pub variables: IndexMap<String, Variable>,

    /// Maps function name => function.
    pub functions: IndexMap<String, Arc<Function>>,

    /// Maps circuit name => circuit.
    pub circuits: IndexMap<String, Arc<Circuit>>,

    /// The main input to the program.
    pub input: Option<Input>,
}

pub type Scope = Arc<RefCell<InnerScope>>;

impl InnerScope {
    ///
    /// Returns a reference to the variable corresponding to the name.
    ///
    /// If the current scope did not have this name present, then the parent scope is checked.
    /// If there is no parent scope, then `None` is returned.
    ///
    pub fn resolve_variable(&self, name: &str) -> Option<Variable> {
        if let Some(resolved) = self.variables.get(name) {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_variable(name) {
                Some(resolved)
            } else {
                None
            }
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
    pub fn resolve_current_function(&self) -> Option<Arc<Function>> {
        if let Some(resolved) = self.function.as_ref() {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_current_function() {
                Some(resolved)
            } else {
                None
            }
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
    pub fn resolve_input(&self) -> Option<Input> {
        if let Some(input) = self.input.as_ref() {
            Some(input.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_input() {
                Some(resolved)
            } else {
                None
            }
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
    pub fn resolve_function(&self, name: &str) -> Option<Arc<Function>> {
        if let Some(resolved) = self.functions.get(name) {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_function(name) {
                Some(resolved)
            } else {
                None
            }
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
    pub fn resolve_circuit(&self, name: &str) -> Option<Arc<Circuit>> {
        if let Some(resolved) = self.circuits.get(name) {
            Some(resolved.clone())
        } else if name == "Self" && self.circuit_self.is_some() {
            self.circuit_self.clone()
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_circuit(name) {
                Some(resolved)
            } else {
                None
            }
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
    pub fn resolve_circuit_self(&self) -> Option<Arc<Circuit>> {
        if let Some(resolved) = self.circuit_self.as_ref() {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_circuit_self() {
                Some(resolved)
            } else {
                None
            }
        } else {
            None
        }
    }

    ///
    /// Returns a new scope given a parent scope.
    ///
    pub fn make_subscope(scope: &Scope) -> Scope {
        Arc::new(RefCell::new(InnerScope {
            id: Uuid::new_v4(),
            parent_scope: Some(scope.clone()),
            circuit_self: None,
            variables: IndexMap::new(),
            functions: IndexMap::new(),
            circuits: IndexMap::new(),
            function: None,
            input: None,
        }))
    }

    ///
    /// Returns the type returned by the current scope.
    ///
    pub fn resolve_ast_type(&self, type_: &leo_ast::Type) -> Result<Type, AsgConvertError> {
        use leo_ast::Type::*;
        Ok(match type_ {
            Address => Type::Address,
            Boolean => Type::Boolean,
            Field => Type::Field,
            Group => Type::Group,
            IntegerType(int_type) => Type::Integer(int_type.clone()),
            Array(sub_type, dimensions) => {
                let mut item = Box::new(self.resolve_ast_type(&*sub_type)?);
                for dimension in dimensions.0.iter().rev() {
                    let dimension = dimension
                        .value
                        .parse::<usize>()
                        .map_err(|_| AsgConvertError::parse_index_error())?;
                    item = Box::new(Type::Array(item, dimension));
                }
                *item
            }
            Tuple(sub_types) => Type::Tuple(
                sub_types
                    .iter()
                    .map(|x| self.resolve_ast_type(x))
                    .collect::<Result<Vec<_>, AsgConvertError>>()?,
            ),
            Circuit(name) if name.name == "Self" => Type::Circuit(
                self.resolve_circuit_self()
                    .ok_or_else(|| AsgConvertError::unresolved_circuit(&name.name, &name.span))?,
            ),
            SelfType => Type::Circuit(
                self.resolve_circuit_self()
                    .ok_or_else(AsgConvertError::reference_self_outside_circuit)?,
            ),
            Circuit(name) => Type::Circuit(
                self.resolve_circuit(&name.name)
                    .ok_or_else(|| AsgConvertError::unresolved_circuit(&name.name, &name.span))?,
            ),
        })
    }
}
