// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use std::cell::RefCell;

use leo_ast::{Circuit, Function};
use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

use crate::{FunctionSymbol, VariableSymbol};

#[derive(Clone, Debug, Default)]
pub struct SymbolTable {
    /// The parent scope if it exists.
    /// For example, the parent scope of a then-block is the scope containing the associated ConditionalStatement.
    pub(crate) parent: Option<Box<SymbolTable>>,
    /// Functions represents the name of each function mapped to the AST's function definition.
    /// This field is populated at a first pass.
    pub functions: IndexMap<Symbol, FunctionSymbol>,
    /// Maps circuit names to circuit definitions.
    /// This field is populated at a first pass.
    pub circuits: IndexMap<Symbol, Circuit>,
    /// The variables defined in a scope.
    /// This field is populated as necessary.
    pub(crate) variables: IndexMap<Symbol, VariableSymbol>,
    /// The index of the current scope.
    pub(crate) scope_index: usize,
    /// The sub-scopes of this scope.
    pub(crate) scopes: Vec<RefCell<SymbolTable>>,
}

impl SymbolTable {
    /// Recursively checks if the symbol table contains an entry for the given symbol.
    /// Leo does not allow any variable shadowing or overlap between different symbols.
    pub fn check_shadowing(&self, symbol: Symbol, span: Span) -> Result<()> {
        if self.variables.contains_key(&symbol) {
            Err(AstError::shadowed_variable(symbol, span).into())
        } else if self.functions.contains_key(&symbol) {
            Err(AstError::shadowed_function(symbol, span).into())
        } else if let Some(existing) = self.circuits.get(&symbol) {
            match existing.is_record {
                true => Err(AstError::shadowed_record(symbol, span).into()),
                false => Err(AstError::shadowed_circuit(symbol, span).into()),
            }
        } else if let Some(parent) = self.parent.as_ref() {
            parent.check_shadowing(symbol, span)
        } else {
            Ok(())
        }
    }

    /// Returns the current scope index.
    /// Increments the scope index.
    pub fn scope_index(&mut self) -> usize {
        let index = self.scope_index;
        self.scope_index += 1;
        index
    }

    /// Inserts a function into the symbol table.
    pub fn insert_fn(&mut self, symbol: Symbol, insert: &Function) -> Result<()> {
        self.check_shadowing(symbol, insert.span)?;
        let id = self.scope_index();
        self.functions.insert(symbol, Self::new_function_symbol(id, insert)?);
        self.scopes.push(Default::default());
        Ok(())
    }

    /// Inserts a circuit into the symbol table.
    pub fn insert_circuit(&mut self, symbol: Symbol, insert: &Circuit) -> Result<()> {
        self.check_shadowing(symbol, insert.span)?;
        self.circuits.insert(symbol, insert.clone());
        Ok(())
    }

    /// Inserts a variable into the symbol table.
    pub fn insert_variable(&mut self, symbol: Symbol, insert: VariableSymbol) -> Result<()> {
        self.check_shadowing(symbol, insert.span)?;
        self.variables.insert(symbol, insert);
        Ok(())
    }

    /// Creates a new scope for the block and stores it in the symbol table.
    pub fn insert_block(&mut self) -> usize {
        self.scopes.push(RefCell::new(Default::default()));
        self.scope_index()
    }

    /// Attempts to lookup a function in the symbol table.
    pub fn lookup_fn_symbol(&self, symbol: Symbol) -> Option<&FunctionSymbol> {
        if let Some(func) = self.functions.get(&symbol) {
            Some(func)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_fn_symbol(symbol)
        } else {
            None
        }
    }

    /// Attempts to lookup a circuit in the symbol table.
    pub fn lookup_circuit(&self, symbol: Symbol) -> Option<&Circuit> {
        if let Some(circ) = self.circuits.get(&symbol) {
            Some(circ)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_circuit(symbol)
        } else {
            None
        }
    }

    /// Attempts to lookup a variable in the symbol table.
    pub fn lookup_variable(&self, symbol: Symbol) -> Option<&VariableSymbol> {
        if let Some(var) = self.variables.get(&symbol) {
            Some(var)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_variable(symbol)
        } else {
            None
        }
    }

    /// Returns true if the variable exists in the local scope
    pub fn variable_in_local_scope(&self, symbol: Symbol) -> bool {
        self.variables.contains_key(&symbol)
    }

    /// Returns true if the variable exists in any parent scope
    pub fn variable_in_parent_scope(&self, symbol: Symbol) -> bool {
        if let Some(parent) = self.parent.as_ref() {
            if parent.variables.contains_key(&symbol) {
                true
            } else {
                parent.variable_in_parent_scope(symbol)
            }
        } else {
            false
        }
    }

    /// Returns a mutable reference to the `VariableSymbol` if it exists in the symbol table.
    pub fn lookup_variable_mut(&mut self, symbol: Symbol) -> Option<&mut VariableSymbol> {
        if let Some(var) = self.variables.get_mut(&symbol) {
            Some(var)
        } else if let Some(parent) = self.parent.as_mut() {
            parent.lookup_variable_mut(symbol)
        } else {
            None
        }
    }

    /// Returns the scope associated with the function symbol, if it exists in the symbol table.
    pub fn lookup_fn_scope(&self, symbol: Symbol) -> Option<&RefCell<Self>> {
        self.lookup_fn_symbol(symbol).and_then(|func| self.scopes.get(func.id))
    }

    /// Returns the scope associated with `index`, if it exists in the symbol table.
    pub fn lookup_scope_by_index(&self, index: usize) -> Option<&RefCell<Self>> {
        self.scopes.get(index)
    }
}
