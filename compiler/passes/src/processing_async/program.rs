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

use super::ProcessingAsyncVisitor;
use leo_ast::{
    AstReconstructor,
    ConstParameter,
    Function,
    Input,
    Node,
    Output,
    ProgramReconstructor,
    ProgramScope,
    Statement,
};

impl ProgramReconstructor for ProcessingAsyncVisitor<'_> {
    /// Reconstructs a `ProgramScope` by rewriting all contained elements:
    /// - Updates the current program context.
    /// - Reconstructs all functions using `reconstruct_function`.
    /// - Reconstructs composites, mappings, and constants.
    /// - Inserts reconstructed functions, including any newly created async functions,
    ///   placing transitions before other functions.
    ///
    /// This prepares the scope for further analysis or compilation, ensuring all
    /// components have gone through transformation.
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program context
        self.current_program = input.program_id.name.name;

        // Reconstruct all functions and store them temporarily. This process also populates
        // `new_async_functions`.
        let mut reconstructed_functions: Vec<_> =
            input.functions.iter().map(|(name, func)| (*name, self.reconstruct_function(func.clone()))).collect();

        // Now append all newly created `async` functions. This ensures transition functions still show up before all other functions.
        reconstructed_functions.append(&mut self.new_async_functions);

        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.clone(),
            composites: input.composites.into_iter().map(|(id, def)| (id, self.reconstruct_composite(def))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            storage_variables: input
                .storage_variables
                .into_iter()
                .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
                .collect(),
            functions: reconstructed_functions,
            interfaces: input.interfaces.into_iter().map(|(id, int)| (id, self.reconstruct_interface(int))).collect(),
            constructor: input.constructor,
            consts: input
                .consts
                .into_iter()
                .map(|(id, stmt)| match self.reconstruct_const(stmt) {
                    (Statement::Const(decl), _) => (id, decl),
                    _ => panic!("`reconstruct_const` must return `Statement::Const`"),
                })
                .collect(),
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        self.current_function = input.name();

        // Enter the scope of the function for correct symbols lookup later
        self.in_scope(input.id(), |slf| Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| ConstParameter { type_: slf.reconstruct_type(param.type_.clone()).0, ..param.clone() })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| Input { type_: slf.reconstruct_type(input.type_.clone()).0, ..input.clone() })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| Output { type_: slf.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect(),
            output_type: slf.reconstruct_type(input.output_type).0,
            block: slf.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        })
    }
}
