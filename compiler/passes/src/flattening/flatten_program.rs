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
    Finalize, FinalizeStatement, Function, Identifier, Member, ProgramReconstructor, Statement, StatementReconstructor,
    Struct, Type,
};
use leo_span::Symbol;

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
            // TODO: Flatten the function arguments.

            // Flatten the finalize block.
            let mut block = self.reconstruct_block(finalize.block).0;

            // Get all of the guards and return expression.
            let returns = self.clear_early_returns();

            // If the finalize block contains return statements, then we fold them into a single return statement.
            self.fold_returns(&mut block, returns);

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
        self.fold_returns(&mut block, returns);

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

            // TODO: Flatten any tuples in the produced finalize statement.

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

    /// Flattens the struct definitions in the program, flattening any tuples in the definitions.
    /// For example, the follow struct definitions:
    /// ```leo
    /// struct Bar {
    ///   a: u8,
    ///   b: (u16, u32),
    /// }
    ///
    /// struct Foo {
    ///   c: u8,
    ///   d: (Bar, (Bar, Bar)),
    /// }
    /// ```
    /// are flattened in the following way.
    /// ```leo
    /// struct Bar {
    ///   a_0: u8,
    ///   b_0: u16,
    ///   b_1: u32,
    /// }
    ///
    /// struct Foo {
    ///   c_0: u8,
    ///   d_0: Bar,
    ///   d_1_0: Bar,
    ///   d_1_1: Bar,
    /// }
    fn reconstruct_struct(&mut self, input: Struct) -> Struct {
        // Helper to rename and flatten a struct member.
        fn rename_member(identifier: Identifier, type_: Type, index: usize) -> Vec<Member> {
            // Construct the new name for the identifier.
            let identifier = Identifier::new(Symbol::intern(&format!("{}_{}", identifier.name, index)));
            match type_ {
                // If the member is a tuple, then it must be flattened.
                Type::Tuple(tuple) => {
                    let mut members = Vec::with_capacity(tuple.0.len());
                    tuple.0.into_iter().enumerate().for_each(|(i, element_type)| {
                        members.extend(rename_member(identifier, element_type, i));
                    });
                    members
                }
                // Otherwise, rename the member and return it as a singleton list.
                type_ => vec![Member { identifier, type_ }],
            }
        }

        // Flatten the circuit members.
        let mut members = Vec::with_capacity(input.members.len());
        input
            .members
            .into_iter()
            .enumerate()
            .for_each(|(i, Member { identifier, type_ })| {
                members.extend(rename_member(identifier, type_, i));
            });

        // Construct the flattened struct.
        let struct_ = Struct {
            identifier: input.identifier,
            members,
            is_record: input.is_record,
            span: input.span,
        };

        // Add the flattened struct to the struct map.
        self.flattened_structs.insert(struct_.identifier.name, struct_.clone());

        // Return the flattened struct.
        struct_
    }
}
