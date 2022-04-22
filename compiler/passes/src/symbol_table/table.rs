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

use leo_ast::Function;
use leo_errors::{AstError, Result};
use leo_span::Symbol;

use indexmap::IndexMap;

use crate::VariableSymbol;

#[derive(Default)]
pub struct SymbolTable<'a> {
    functions: IndexMap<Symbol, &'a Function>,
    variables: VariableSymbol<'a>,
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

    pub fn insert_fn(&mut self, symbol: Symbol, function: &'a Function) -> Result<()> {
        self.check_shadowing(&symbol)?;
        self.functions.insert(symbol, function);
        Ok(())
    }
}
