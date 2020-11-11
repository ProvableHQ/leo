// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{FunctionInputVariableType, SymbolTable, Type, TypeError};
use leo_ast::{FunctionInput, Identifier};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionInputType {
    InputKeyword(Identifier),
    Variable(FunctionInputVariableType),
}

impl FunctionInputType {
    ///
    /// Return the `Identifier` containing name and span information about the current function input.
    ///
    pub fn identifier(&self) -> &Identifier {
        match self {
            FunctionInputType::InputKeyword(identifier) => identifier,
            FunctionInputType::Variable(variable) => &variable.identifier,
        }
    }

    ///
    /// Return the `Type` of the current function input.
    ///
    pub fn type_(&self) -> Type {
        match self {
            FunctionInputType::InputKeyword(identifier) => Type::Circuit(identifier.to_owned()),
            FunctionInputType::Variable(variable) => variable.type_.to_owned(),
        }
    }

    ///
    /// Return a new `FunctionInputType` from a given `FunctionInput`.
    ///
    /// Performs a lookup in the given symbol table if the function input contains
    /// user-defined types.
    ///
    pub fn new(table: &SymbolTable, unresolved: FunctionInput) -> Result<Self, TypeError> {
        Ok(match unresolved {
            FunctionInput::InputKeyword(identifier) => FunctionInputType::InputKeyword(identifier),
            FunctionInput::Variable(variable) => {
                let variable_resolved = FunctionInputVariableType::new(table, variable)?;

                FunctionInputType::Variable(variable_resolved)
            }
        })
    }

    ///
    /// Return a new `FunctionInputType` from a given `FunctionInput`.
    ///
    /// Performs a lookup in the given symbol table if the function input contains
    /// user-defined types.
    ///
    /// If the type of the function input is the `Self` keyword, then the given circuit identifier
    /// is used as the type.
    ///
    pub fn new_from_circuit(
        table: &SymbolTable,
        unresolved: FunctionInput,
        circuit_name: Identifier,
    ) -> Result<Self, TypeError> {
        Ok(match unresolved {
            FunctionInput::InputKeyword(identifier) => FunctionInputType::InputKeyword(identifier),
            FunctionInput::Variable(unresolved_function_input) => {
                let function_input =
                    FunctionInputVariableType::new_from_circuit(table, unresolved_function_input, circuit_name)?;

                FunctionInputType::Variable(function_input)
            }
        })
    }
}
