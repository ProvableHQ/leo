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

use leo_ast::{Function, FunctionInput, Type};
use leo_span::Span;

use crate::SymbolTable;

/// An entry for a function in the symbol table.
#[derive(Clone, Debug)]
pub struct FunctionSymbol {
    /// The index associated with the scope in the parent symbol table.
    pub(crate) index: usize,
    /// The output type of the function.
    pub(crate) output: Type,
    /// The `Span` associated with the function.
    pub(crate) span: Span,
    /// The inputs to the function.
    pub(crate) input: Vec<FunctionInput>,
}

impl SymbolTable {
    pub(crate) fn new_function_symbol(id: usize, func: &Function) -> FunctionSymbol {
        FunctionSymbol {
            index: id,
            output: func.output.clone(),
            span: func.span,
            input: func.input.clone(),
        }
    }
}
