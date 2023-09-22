// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use leo_ast::*;

use crate::Unroller;

impl ProgramReconstructor for Unroller<'_> {
    fn reconstruct_function(&mut self, function: Function) -> Function {
        // Lookup function metadata in the symbol table.
        // Note that this unwrap is safe since function metadata is stored in a prior pass.
        let function_index = self.symbol_table.borrow().lookup_fn_symbol(function.identifier.name).unwrap().id;

        // Enter the function's scope.
        let previous_function_index = self.enter_scope(function_index);

        let previous_scope_index = self.enter_scope(self.scope_index);

        let block = self.reconstruct_block(function.block).0;

        self.exit_scope(previous_scope_index);

        let finalize = function.finalize.map(|finalize| {
            let previous_scope_index = self.enter_scope(self.scope_index);

            let block = self.reconstruct_block(finalize.block).0;

            self.exit_scope(previous_scope_index);

            Finalize {
                identifier: finalize.identifier,
                input: finalize.input,
                output: finalize.output,
                output_type: finalize.output_type,
                block,
                span: finalize.span,
                id: finalize.id,
            }
        });

        // Reconstruct the function block.
        let reconstructed_function = Function {
            annotations: function.annotations,
            variant: function.variant,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block,
            finalize,
            span: function.span,
            id: function.id,
        };

        // Exit the function's scope.
        self.exit_scope(previous_function_index);

        reconstructed_function
    }

    fn reconstruct_const(&mut self, input: DefinitionStatement) -> DefinitionStatement {
        // Reconstruct the RHS expression to allow for constant propagation
        let reconstructed_value_expression = self.reconstruct_expression(input.value.clone()).0;

        // Helper function to add global constants to constant variable table
        let insert_variable = |symbol: Symbol, value: &Expression| {
            if let Literal(literal) = value {
                if let Err(err) = self.constant_propagation_table.borrow_mut().insert_constant(symbol, literal.clone())
                {
                    self.handler.emit_err(err);
                }
            } else {
                unreachable!("Type checking guarantees that the value of a constant is a literal.");
            }
        };

        // No matter if doing multiple definitions in one line or not, insert all global constants into the constant propagation table
        match &input.place {
            Expression::Identifier(identifier) => {
                insert_variable(identifier.name, &reconstructed_value_expression);
            }
            Expression::Tuple(tuple_expression) => {
                let tuple_values: &Vec<Expression> = match &reconstructed_value_expression {
                    Expression::Tuple(tuple_value_expression) => &tuple_value_expression.elements,
                    _ => unreachable!(
                        "Definition statement that defines tuple of variables must be assigned to tuple of values"
                    ),
                };

                for (i, element) in tuple_expression.elements.iter().enumerate() {
                    let identifier = match element {
                        Expression::Identifier(identifier) => identifier,
                        _ => unreachable!("All elements of a definition tuple must be identifiers"),
                    };
                    insert_variable(identifier.name, &tuple_values[i].clone());
                }
            }
            _ => unreachable!(
                "Type checking guarantees that the lhs of a `DefinitionStatement` is either an identifier or tuple."
            ),
        }

        DefinitionStatement {
            declaration_type: input.declaration_type,
            place: input.place,
            type_: input.type_,
            value: reconstructed_value_expression,
            span: input.span,
            id: input.id,
        }
    }
}
