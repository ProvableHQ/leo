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

use crate::{SymbolTable, Type, TypeError};

use leo_typed::{Identifier, Span, Type as UnresolvedType};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionOutputType {
    /// Type of function output.
    pub type_: Type,
}

impl FunctionOutputType {
    ///
    /// Return a new `FunctionOutputType` from a given optional function return type and span.
    ///
    /// Performs a lookup in the given symbol table if the return type is user-defined.
    ///
    pub(crate) fn new(
        table: &SymbolTable,
        function_output: Option<UnresolvedType>,
        span: Span,
    ) -> Result<Self, TypeError> {
        let type_ = match function_output {
            None => Type::Tuple(vec![]), // functions with no return value return an empty tuple
            Some(type_) => Type::new(table, type_, span)?,
        };

        Ok(FunctionOutputType { type_ })
    }

    ///
    /// Return a new `FunctionOutputType` from a given optional function return type and span.
    ///
    /// Performs a lookup in the given symbol table if the return type is user-defined.
    ///
    /// If the type of the function return type is the `Self` keyword, then the given circuit
    /// identifier is used as the type.
    ///
    pub fn new_from_circuit(
        table: &SymbolTable,
        circuit_name: Identifier,
        unresolved: Option<UnresolvedType>,
        span: Span,
    ) -> Result<Self, TypeError> {
        let output_type = match unresolved {
            None => Type::Tuple(vec![]),
            Some(type_) => Type::new_from_circuit(table, type_, circuit_name, span)?,
        };

        Ok(FunctionOutputType { type_: output_type })
    }
}
