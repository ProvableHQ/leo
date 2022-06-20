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

use std::fmt::Display;

use leo_ast::{Circuit, Function};
use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

use crate::{VariableScope, VariableSymbol};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SymbolTable<'a> {
    /// Maps function names to function definitions.
    /// This field is populated at a first pass.
    functions: IndexMap<Symbol, &'a Function>,
    /// Maps circuit names to circuit definitions.
    /// This field is populated at a first pass.
    circuits: IndexMap<Symbol, &'a Circuit>,
    /// Variables represents functions variable definitions and input variables.
    /// This field is not populated till necessary.
    pub(crate) variables: VariableScope<'a>,
}

impl<'a> SymbolTable<'a> {
    pub fn check_shadowing(&self, symbol: &Symbol, span: Span) -> Result<()> {
        if self.functions.contains_key(symbol) {
            Err(AstError::shadowed_function(symbol, span).into())
        } else {
            self.variables.check_shadowing(symbol, span)?;
            Ok(())
        }
    }

    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    pub fn insert_fn(&mut self, symbol: Symbol, insert: &'a Function) -> Result<()> {
        self.check_shadowing(&symbol, insert.span)?;
        self.functions.insert(symbol, insert);
        Ok(())
    }

    pub fn insert_circuit(&mut self, symbol: Symbol, insert: &'a Circuit) -> Result<()> {
        if self.circuits.contains_key(&symbol) {
            // Return an error if the circuit name has already been inserted.
            return Err(AstError::shadowed_circuit(symbol, insert.span).into());
        }
        self.circuits.insert(symbol, insert);
        Ok(())
    }

    pub fn insert_variable(&mut self, symbol: Symbol, insert: VariableSymbol<'a>) -> Result<()> {
        self.check_shadowing(&symbol, insert.span)?;
        self.variables.variables.insert(symbol, insert);
        Ok(())
    }

    pub fn lookup_fn(&self, symbol: &Symbol) -> Option<&&'a Function> {
        self.functions.get(symbol)
    }

    pub fn lookup_circuit(&self, symbol: &Symbol) -> Option<&&'a Circuit> {
        self.circuits.get(symbol)
    }

    pub fn lookup_variable(&self, symbol: &Symbol) -> Option<&VariableSymbol<'a>> {
        self.variables.lookup_variable(symbol)
    }

    pub fn push_variable_scope(&mut self) {
        let current_scope = self.variables.clone();
        self.variables = VariableScope {
            parent: Some(Box::new(current_scope)),
            variables: Default::default(),
        };
    }

    pub fn pop_variable_scope(&mut self) {
        let parent = self.variables.parent.clone().unwrap();

        self.variables = *parent;
    }
}

impl<'a> Display for SymbolTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SymbolTable")?;

        for func in self.functions.values() {
            write!(f, "{func}")?;
        }

        for circ in self.circuits.values() {
            write!(f, "{circ}")?;
        }

        write!(f, "{}", self.variables)
    }
}
