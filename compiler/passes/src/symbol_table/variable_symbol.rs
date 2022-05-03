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
use leo_ast::{DefinitionStatement, FunctionInput, Node};
use leo_errors::{AstError, Result};
use leo_span::Symbol;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VariableSymbol<'a> {
    /// The parent scope of variables if it exists.
    /// For example if we are in a if block inside a function.
    /// The parent would be the functions variables and inputs.
    /// This field is populated as necessary.
    pub(crate) parent: Option<Box<VariableSymbol<'a>>>,
    /// The input variables defined in a scope.
    /// This field is populated as necessary.
    pub(crate) inputs: IndexMap<Symbol, &'a FunctionInput>,
    /// The variables defined in a scope.
    /// This field is populated as necessary.
    pub(crate) variables: IndexMap<Symbol, &'a DefinitionStatement>,
}

impl<'a> VariableSymbol<'a> {
    pub fn check_shadowing(&self, symbol: &Symbol) -> Result<()> {
        if let Some(input) = self.inputs.get(symbol) {
            let span = input.span();
            Err(AstError::shadowed_function_input(symbol, span).into())
        } else if let Some(var) = self.variables.get(symbol) {
            Err(AstError::shadowed_variable(symbol, &var.span).into())
        } else if let Some(parent) = &self.parent {
            parent.check_shadowing(symbol)
        } else {
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.parent = None;
        self.inputs.clear();
        self.variables.clear();
    }
}

impl<'a> Display for VariableSymbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VariableSymbol")?;
        self.parent
            .as_ref()
            .map(|parent| write!(f, "parent {parent}"))
            .transpose()?;

        for input in self.inputs.values() {
            write!(f, "{input}")?;
        }

        for var in self.variables.values() {
            write!(f, "{var}")?;
        }

        Ok(())
    }
}
