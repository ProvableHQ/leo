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

use indexmap::IndexMap;
use leo_ast::{DefinitionStatement, FunctionInput, Type};
use leo_errors::Result;
use leo_span::Symbol;

#[derive(Debug, Default)]
pub struct VariableSymbol<'a> {
    parent: Option<Box<VariableSymbol<'a>>>,
    inputs: IndexMap<Symbol, Type>,
    variables: IndexMap<Symbol, &'a DefinitionStatement>,
}

impl<'a> VariableSymbol<'a> {
    pub fn new(parent: Option<Box<VariableSymbol<'a>>>, inputs: Vec<&'a FunctionInput>) -> Self {
        Self {
            parent,
            inputs: inputs
                .iter()
                .map(|input| {
                    let inner = input.get_variable();
                    (inner.identifier.name, inner.type_())
                })
                .collect(),
            variables: IndexMap::new(),
        }
    }

    pub fn check_shadowing(&self, symbol: &Symbol) -> Result<()> {
        if let Some(input) = self.inputs.get(symbol) {
            todo!("error");
        } else if let Some(var) = self.variables.get(symbol) {
            todo!("error");
        } else if let Some(parent) = &self.parent {
            parent.check_shadowing(symbol)
        } else {
            Ok(())
        }
    }
}
