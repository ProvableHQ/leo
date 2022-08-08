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
use leo_errors::{Result, TypeCheckerError};
use leo_span::{sym, Span};
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
    pub(crate) fn new_function_symbol(id: usize, func: &Function) -> Result<FunctionSymbol> {
        // Is the function a program function?
        let mut is_program_function = false;
        // Is the function inlined?
        let mut is_inlined = false;

        // Check that the function's annotations are valid.
        for annotation in func.annotations.iter() {
            match annotation.identifier.name {
                // Set `is_program_function` to true if the corresponding annotation is found.
                sym::program => is_program_function = true,
                sym::inline => is_inlined = true,
                _ => return Err(TypeCheckerError::unknown_annotation(annotation, annotation.span).into()),
            }
        }

        // Determine the call type of the function.
        let call_type = match (is_program_function, is_inlined) {
            (false, false) => CallType::Helper,
            (false, true) => CallType::Inlined,
            (true, false) => CallType::Program,
            (true, true) => {
                let mut spans = func.annotations.iter().map(|annotation| annotation.span);
                // This is safe, since if either `is_program_function` or `is_inlined` is true, then the function must have at least one annotation.
                let first_span = spans.next().unwrap();

                // Sum up the spans of all the annotations.
                let span = spans.fold(first_span, |acc, span| acc + span);
                return Err(TypeCheckerError::program_and_inline_annotation(span).into());
            }
        };

        Ok(FunctionSymbol {
            id,
            output: func.output.clone(),
            span: func.span,
            input: func.input.clone(),
            call_type,
        })
    }
}
