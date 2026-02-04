// Copyright (C) 2019-2025 Provable Inc.
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

//! Transformation phase of the ForceInlineConversion pass.
//! Converts Variant::Function to Variant::Inline based on the analysis results.

use indexmap::IndexSet;
use leo_ast::{AstReconstructor, ErrExpression, Expression, Function, ProgramReconstructor, Type, Variant};
use leo_span::{Symbol, sym};

#[derive(Debug)]
pub struct TransformVisitor {
    /// Functions marked for inlining by the analysis phase
    functions_to_inline: IndexSet<Vec<Symbol>>,
}

impl TransformVisitor {
    pub fn new(functions_to_inline: IndexSet<Vec<Symbol>>) -> Self {
        Self { functions_to_inline }
    }

    /// Check if a names an optional type. We don't need to check the type
    /// recursively with the symbol table, hiding optionals behind structs is
    /// allowed.
    fn names_optional_type(ty: &Type) -> bool {
        match ty {
            Type::Optional(_) => true,
            Type::Tuple(tuple) => tuple.elements().iter().any(Self::names_optional_type),
            Type::Array(array) => Self::names_optional_type(array.element_type()),
            _ => false,
        }
    }

    /// Check if a function should be converted to inline based on the conditions:
    fn should_convert_to_inline(&self, function: &Function) -> bool {
        function.variant == Variant::Function
            && !function.annotations.iter().any(|a| a.identifier.name == sym::no_inline)
            && (
                // Has const parameters
                !function.const_parameters.is_empty() ||
                // Has no arguments
                function.input.is_empty() ||
                // Has only empty arguments
                function.input.iter().all(|arg| arg.type_.is_empty()) ||
                // Has more than 16 arguments
                function.input.len() > 16 ||
                // Returns a type naming an optional
                Self::names_optional_type(&function.output_type) ||
                // Has an argument naming an optional
                function.input.iter().any(|arg| Self::names_optional_type(&arg.type_)) ||
                // Marked by the analysis phase
                self.functions_to_inline.contains(&vec![function.identifier.name])
            )
    }
}

impl AstReconstructor for TransformVisitor {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_err(
        &mut self,
        input: ErrExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        // Don't panic, we want to reach type checking
        (Expression::Err(input), ())
    }
}

impl ProgramReconstructor for TransformVisitor {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        // Determine the new variant
        let variant = if self.should_convert_to_inline(&input) { Variant::Inline } else { input.variant };

        // Reconstruct the function with potentially updated variant
        Function {
            annotations: input.annotations,
            variant,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| leo_ast::ConstParameter {
                    type_: self.reconstruct_type(param.type_.clone()).0,
                    ..param.clone()
                })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| leo_ast::Input { type_: self.reconstruct_type(input.type_.clone()).0, ..input.clone() })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| leo_ast::Output {
                    type_: self.reconstruct_type(output.type_.clone()).0,
                    ..output.clone()
                })
                .collect(),
            output_type: self.reconstruct_type(input.output_type).0,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        }
    }
}
