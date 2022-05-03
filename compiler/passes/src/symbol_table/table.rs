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

use leo_ast::{DefinitionStatement, Function, FunctionInput};
use leo_errors::{AstError, Result};
use leo_span::Symbol;

use indexmap::IndexMap;

use crate::VariableSymbol;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SymbolTable<'a> {
    /// Functions represents the name of each function mapped to the Ast's function definition.
    /// This field is populated at a first pass.
    functions: IndexMap<Symbol, &'a Function>,
    /// Variables represents functions variable definitions and input variables.
    /// This field is not populated till necessary.
    pub(crate) variables: VariableSymbol<'a>,
}

impl<'a> SymbolTable<'a> {
    pub fn check_shadowing(&self, symbol: &Symbol) -> Result<()> {
        if let Some(function) = self.functions.get(symbol) {
            Err(AstError::shadowed_function(symbol, &function.span).into())
        } else {
            self.variables.check_shadowing(symbol)?;
            Ok(())
        }
    }

    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    pub fn insert_fn(&mut self, symbol: Symbol, insert: &'a Function) -> Result<()> {
        self.check_shadowing(&symbol)?;
        self.functions.insert(symbol, insert);
        Ok(())
    }

    pub fn insert_fn_input(&mut self, symbol: Symbol, insert: &'a FunctionInput) -> Result<()> {
        self.check_shadowing(&symbol)?;
        self.variables.inputs.insert(symbol, insert);
        Ok(())
    }

    pub fn insert_variable(&mut self, symbol: Symbol, insert: &'a DefinitionStatement) -> Result<()> {
        self.check_shadowing(&symbol)?;
        self.variables.variables.insert(symbol, insert);
        Ok(())
    }

    pub fn lookup_fn(&self, symbol: &Symbol) -> Option<&&'a Function> {
        self.functions.get(symbol)
    }

    pub fn lookup_fn_input(&self, symbol: &Symbol) -> Option<&&'a FunctionInput> {
        self.variables.inputs.get(symbol)
    }

    pub fn lookup_var(&self, symbol: &Symbol) -> Option<&&'a DefinitionStatement> {
        self.variables.variables.get(symbol)
    }
}

impl<'a> Display for SymbolTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SymbolTable")?;

        for func in self.functions.values() {
            write!(f, "{func}")?;
        }

        write!(f, "{}", self.variables)
    }
}
