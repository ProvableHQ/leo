// Copyright (C) 2019-2026 Provable Inc.
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

//! Analysis phase of the ForceInlineConversion pass.

use indexmap::IndexSet;
use leo_ast::{AstVisitor, AsyncExpression, CallExpression, ErrExpression, Function, ProgramVisitor, Variant};
use leo_span::Symbol;

#[derive(Debug)]
pub struct AnalysisVisitor {
    /// The variant of the function we're currently visiting.
    current_variant: Option<Variant>,
    /// Whether we're currently inside a constructor
    in_constructor: bool,
    /// Functions called from functions, inline, constructors or async blocks.
    pub functions_to_inline: IndexSet<Vec<Symbol>>,
}

impl AnalysisVisitor {
    pub fn new() -> Self {
        Self { current_variant: None, in_constructor: false, functions_to_inline: IndexSet::new() }
    }
}

impl AstVisitor for AnalysisVisitor {
    type AdditionalInput = ();
    type Output = ();

    fn visit_call(&mut self, input: &CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Extract the function path segments
        let callee_path = input.function.segments();

        if matches!(self.current_variant, Some(Variant::Finalize | Variant::FinalFn | Variant::Fn))
            || self.in_constructor
        {
            self.functions_to_inline.insert(callee_path.clone());
        }

        input.const_arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
        input.arguments.iter().for_each(|expr| {
            self.visit_expression(expr, &Default::default());
        });
    }

    fn visit_async(&mut self, _input: &AsyncExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        panic!("Async blocks should no longer exist at this point in compilation");
    }

    fn visit_err(&mut self, _input: &ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Don't panic, we want to reach type checking
    }
}

impl ProgramVisitor for AnalysisVisitor {
    fn visit_function(&mut self, input: &Function) {
        // Save previous variant
        let prev_variant = self.current_variant;

        // Set current variant to this function's variant
        self.current_variant = Some(input.variant);

        // Visit function body (default implementation)
        input.const_parameters.iter().for_each(|input| self.visit_type(&input.type_));
        input.input.iter().for_each(|input| self.visit_type(&input.type_));
        input.output.iter().for_each(|output| self.visit_type(&output.type_));
        self.visit_type(&input.output_type);
        self.visit_block(&input.block);

        // Restore previous variant
        self.current_variant = prev_variant;
    }

    fn visit_constructor(&mut self, input: &leo_ast::Constructor) {
        let was_in_constructor = self.in_constructor;
        self.in_constructor = true;
        self.visit_block(&input.block);
        self.in_constructor = was_in_constructor;
    }
}
