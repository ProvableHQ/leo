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
use std::fmt::Display;

use crate::SymbolTable;

// TODO: Is there a better name for this?
/// The call type of the function.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CallType {
    /// A function that is called internally.
    Helper,
    /// A function that is inlined.
    Inlined,
    /// A function that is called externally.
    Program,
}

impl Display for CallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallType::Helper => write!(f, "helper"),
            CallType::Inlined => write!(f, "inlined"),
            CallType::Program => write!(f, "program"),
        }
    }
}

/// An entry for a function in the symbol table.
#[derive(Clone, Debug)]
pub struct FunctionSymbol {
    /// The index associated with the scope in the parent symbol table.
    pub(crate) id: usize,
    /// The output type of the function.
    pub(crate) output: Type,
    /// The `Span` associated with the function.
    pub(crate) span: Span,
    /// The inputs to the function.
    pub(crate) input: Vec<FunctionInput>,
    /// The type of the function.
    pub(crate) call_type: CallType,
}

impl SymbolTable {
    pub(crate) fn new_function_symbol(id: usize, func: &Function, call_type: CallType) -> FunctionSymbol {
        FunctionSymbol {
            id,
            output: func.output.clone(),
            span: func.span,
            input: func.input.clone(),
            call_type,
        }
    }
}
