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

use crate::{CallType, SymbolTable};

use leo_ast::*;
use leo_errors::emitter::Handler;
use leo_errors::{TypeCheckerError, TypeCheckerWarning};
use leo_span::sym;

/// A compiler pass during which the `SymbolTable` is created.
/// Note that this pass only creates the initial entries for functions and circuits.
/// The table is populated further during the type checking pass.
pub struct SymbolTableCreator<'a> {
    /// The `SymbolTable` constructed by this compiler pass.
    pub(crate) symbol_table: SymbolTable,
    /// The error handler.
    handler: &'a Handler,
}

impl<'a> SymbolTableCreator<'a> {
    pub fn new(handler: &'a Handler) -> Self {
        Self {
            symbol_table: Default::default(),
            handler,
        }
    }

    /// Emits a type checker error.
    pub(crate) fn emit_err(&self, err: TypeCheckerError) {
        self.handler.emit_err(err);
    }

    /// Emits a type checker warning.
    pub(crate) fn emit_warning(&self, warning: TypeCheckerWarning) {
        self.handler.emit_warning(warning);
    }
}

impl<'a> ExpressionVisitor<'a> for SymbolTableCreator<'a> {
    type AdditionalInput = ();
    type Output = ();
}

impl<'a> StatementVisitor<'a> for SymbolTableCreator<'a> {}

impl<'a> ProgramVisitor<'a> for SymbolTableCreator<'a> {
    fn visit_function(&mut self, func: &'a Function) {
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
                _ => self.emit_warning(TypeCheckerWarning::unknown_annotation(annotation, annotation.span)),
            }
        }

        // Determine the call type of the function.
        let call_type = match (is_program_function, is_inlined) {
            (false, false) => CallType::Helper,
            (false, true) => CallType::Inlined,
            (true, false) => CallType::Program,
            // If the function is annotated with both `@program` and `@inline`, emit an error.
            (true, true) => {
                let mut spans = func.annotations.iter().map(|annotation| annotation.span);
                // This is safe, since if either `is_program_function` or `is_inlined` is true, then the function must have at least one annotation.
                let first_span = spans.next().unwrap();

                // Sum up the spans of all the annotations.
                let span = spans.fold(first_span, |acc, span| acc + span);
                self.emit_err(TypeCheckerError::program_and_inline_annotation(span));

                // Return a dummy call type.
                CallType::Inlined
            }
        };

        if let Err(err) = self.symbol_table.insert_fn(func.name(), func, call_type) {
            self.handler.emit_err(err);
        }
    }

    fn visit_circuit(&mut self, input: &'a Circuit) {
        if let Err(err) = self.symbol_table.insert_circuit(input.name(), input) {
            self.handler.emit_err(err);
        }
    }

    fn visit_import(&mut self, input: &'a Program) {
        self.visit_program(input)
    }
}
