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

use indexmap::IndexMap;
use leo_errors::{AstError, Result};
use leo_span::{Span, Symbol};

use crate::VariableSymbol;

#[derive(Clone)]
pub struct VariableScope<'a> {
    /// The parent scope of variables if it exists.
    /// For example if we are in a if block inside a function.
    /// The parent would be the functions variables and inputs.
    /// This field is populated as necessary.
    pub(crate) parent: Option<Box<VariableScope<'a>>>,
    /// The variables defined in a scope.
    /// This field is populated as necessary.
    pub(crate) variables: IndexMap<Symbol, VariableSymbol<'a>>,
}

impl<'a> VariableScope<'a> {
    pub fn check_shadowing(&self, symbol: &Symbol, span: Span) -> Result<()> {
        if self.variables.contains_key(symbol) {
            Err(AstError::shadowed_variable(symbol, span).into())
        } else if let Some(parent) = &self.parent {
            parent.check_shadowing(symbol, span)
        } else {
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.parent = None;
        self.variables.clear();
    }

    pub fn lookup_variable(&self, symbol: &Symbol) -> Option<&VariableSymbol<'a>> {
        if let Some(var) = self.variables.get(symbol) {
            Some(var)
        } else if let Some(parent) = &self.parent {
            parent.lookup_variable(symbol)
        } else {
            None
        }
    }
}

impl<'a> Display for VariableScope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VariableScope")?;
        self.parent
            .as_ref()
            .map(|parent| write!(f, "parent {parent}"))
            .transpose()?;

        for (sym, var) in self.variables.iter() {
            write!(f, "{sym} {var}")?;
        }

        Ok(())
    }
}
