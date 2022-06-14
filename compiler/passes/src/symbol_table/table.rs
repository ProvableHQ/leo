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

use std::cell::{Cell, RefCell};

use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

use crate::{FunctionSymbol, VariableSymbol};

#[derive(Clone, Default)]
pub struct SymbolTable<'a> {
    /// The parent scope if it exists.
    /// For example if we are in a if block inside a function.
    pub(crate) parent: Cell<Option<&'a SymbolTable<'a>>>,
    /// Functions represents the name of each function mapped to the Ast's function definition.
    /// This field is populated at a first pass.
    functions: RefCell<IndexMap<Symbol, &'a FunctionSymbol<'a>>>,
    /// The variables defined in a scope.
    /// This field is populated as necessary.
    pub(crate) variables: RefCell<IndexMap<Symbol, &'a VariableSymbol<'a>>>,
}

impl<'a> SymbolTable<'a> {
    pub fn check_shadowing(&self, symbol: Symbol, span: Span) -> Result<()> {
        if self.variables.borrow().contains_key(&symbol) {
            Err(AstError::shadowed_variable(symbol, span).into())
        } else if self.functions.borrow().contains_key(&symbol) {
            Err(AstError::shadowed_function(symbol, span).into())
        } else if let Some(parent) = self.parent.get() {
            parent.check_shadowing(symbol, span)
        } else {
            Ok(())
        }
    }

    pub fn insert_fn(&self, symbol: Symbol, insert: &'a FunctionSymbol<'a>) -> Result<()> {
        self.check_shadowing(symbol, insert.span)?;
        self.functions.borrow_mut().insert(symbol, insert);
        Ok(())
    }

    pub fn insert_variable(&self, symbol: Symbol, insert: &'a VariableSymbol<'a>) -> Result<()> {
        self.check_shadowing(symbol, insert.span)?;
        self.variables.borrow_mut().insert(symbol, insert);
        Ok(())
    }

    pub fn lookup_fn(&self, symbol: Symbol) -> Option<&'a FunctionSymbol<'a>> {
        if let Some(func) = self.functions.borrow().get(&symbol) {
            Some(func)
        } else if let Some(parent) = self.parent.get() {
            parent.lookup_fn(symbol)
        } else {
            None
        }
    }

    pub fn lookup_variable(&self, symbol: Symbol) -> Option<&'a VariableSymbol<'a>> {
        if let Some(var) = self.variables.borrow().get(&symbol) {
            Some(var)
        } else if let Some(parent) = self.parent.get() {
            parent.lookup_variable(symbol)
        } else {
            None
        }
    }

    pub fn subscope(self: &'a SymbolTable<'a>) -> Self {
        Self {
            parent: Cell::new(Some(self)),
            ..Default::default()
        }
    }
}
