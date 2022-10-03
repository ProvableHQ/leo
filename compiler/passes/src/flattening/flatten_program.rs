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

use crate::Flattener;

use leo_ast::{
    Finalize, FinalizeStatement, Function, ProgramReconstructor, ReturnStatement, Statement, StatementReconstructor,
    Type,
};

impl ProgramReconstructor for Flattener<'_> {
    /// Flattens a function's body and finalize block, if it exists.
    fn reconstruct_function(&mut self, function: Function) -> Function {
        // First, flatten the finalize block. This allows us to initialize self.finalizes correctly.
        // Note that this is safe since the finalize block is independent of the function body.
        let finalize = function.finalize.map(|finalize| {
            // Initialize `self.structs` with the finalize's input as necessary.
            self.structs = Default::default();
            for input in &finalize.input {
                if let Type::Identifier(struct_name) = input.type_() {
                    self.structs.insert(input.identifier().name, struct_name.name);
                }
            }

            // Flatten the finalize block.
            let mut block = self.reconstruct_block(finalize.block).0;

            // Get all of the guards and return expression.
            let returns = self.clear_early_returns();

            // If the finalize block contains return statements, then we fold them into a single return statement.
            if !returns.is_empty() {
                let (expression, stmts) = self.fold_guards("ret$", returns);

                // Add all of the accumulated statements to the end of the block.
                block.statements.extend(stmts);

                // Add the `ReturnStatement` to the end of the block.
                block.statements.push(Statement::Return(ReturnStatement {
                    expression,
                    span: Default::default(),
                }));
            }

            // Initialize `self.finalizes` with the appropriate number of vectors.
            self.finalizes = vec![vec![]; finalize.input.len()];

            Finalize {
                identifier: finalize.identifier,
                input: finalize.input,
                output: finalize.output,
                output_type: finalize.output_type,
                block,
                span: finalize.span,
            }
        });

        // Initialize `self.structs` with the function's input as necessary.
        self.structs = Default::default();
        for input in &function.input {
            if let Type::Identifier(struct_name) = input.type_() {
                self.structs.insert(input.identifier().name, struct_name.name);
            }
        }

        // Flatten the function body.
        let mut block = self.reconstruct_block(function.block).0;

        // Get all of the guards and return expression.
        let returns = self.clear_early_returns();

        // If the function contains return statements, then we fold them into a single return statement.
        if !returns.is_empty() {
            let (expression, stmts) = self.fold_guards("ret$", returns);

            // Add all of the accumulated statements to the end of the block.
            block.statements.extend(stmts);

            // Add the `ReturnStatement` to the end of the block.
            block.statements.push(Statement::Return(ReturnStatement {
                expression,
                span: Default::default(),
            }));
        }

        // If the function has a finalize block, then type checking guarantees that it has at least one finalize statement.
        if finalize.is_some() {
            // Get all of the guards and finalize expression.
            let finalize_arguments = self.clear_early_finalizes();
            let arguments = match finalize_arguments.iter().all(|component| component.is_empty()) {
                // If the finalize statement takes no arguments, then output an empty vector.
                true => vec![],
                // If the function contains finalize statements with at least one argument, then we fold them into a vector of arguments.
                // Note that `finalizes` is always initialized to the appropriate number of vectors.
                false => {
                    // Construct an expression for each argument to the finalize statement.
                    finalize_arguments
                        .into_iter()
                        .enumerate()
                        .map(|(i, component)| {
                            let (expression, stmts) = self.fold_guards(format!("fin${i}$").as_str(), component);

                            // Add all of the accumulated statements to the end of the block.
                            block.statements.extend(stmts);

                            expression
                        })
                        .collect()
                }
            };

            // Add the `FinalizeStatement` to the end of the block.
            block.statements.push(Statement::Finalize(FinalizeStatement {
                arguments,
                span: Default::default(),
            }));
        }

        Function {
            annotations: function.annotations,
            call_type: function.call_type,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block,
            finalize,
            span: function.span,
        }
    }
}
