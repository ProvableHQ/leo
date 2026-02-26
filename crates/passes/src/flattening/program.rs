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

use super::{FlatteningVisitor, ReturnGuard};

use leo_ast::{
    AstReconstructor,
    Constructor,
    Expression,
    Function,
    ProgramReconstructor,
    ProgramScope,
    ReturnStatement,
    Statement,
};

impl ProgramReconstructor for FlatteningVisitor<'_> {
    /// Flattens a program scope.
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.name.name;
        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.into_iter().map(|(s, t)| (s, self.reconstruct_type(t).0)).collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            composites: input.composites.into_iter().map(|(i, c)| (i, self.reconstruct_composite(c))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            storage_variables: input
                .storage_variables
                .into_iter()
                .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
                .collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    /// Flattens a function's body
    fn reconstruct_function(&mut self, function: Function) -> Function {
        // Set when the function is an on-chain function (Finalize or FinalFn).
        self.is_onchain = function.variant.is_onchain();

        // Flatten the function body.
        let mut block = self.reconstruct_block(function.block).0;

        // Fold the return statements into the block.
        let returns = std::mem::take(&mut self.returns);
        let expression_returns: Vec<(Option<Expression>, ReturnStatement)> = returns
            .into_iter()
            .map(|(guard, statement)| match guard {
                ReturnGuard::None => (None, statement),
                ReturnGuard::Unconstructed(plain) | ReturnGuard::Constructed { plain, .. } => {
                    (Some(leo_ast::Path::from(plain).to_local().into()), statement)
                }
            })
            .collect();

        self.fold_returns(&mut block, expression_returns);

        Function {
            annotations: function.annotations,
            variant: function.variant,
            identifier: function.identifier,
            const_parameters: function.const_parameters,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block,
            span: function.span,
            id: function.id,
        }
    }

    /// Flattens a constructor's body.
    fn reconstruct_constructor(&mut self, constructor: Constructor) -> Constructor {
        // A constructor is always run onchain.
        self.is_onchain = true;

        // Flatten the function body.
        let mut block = self.reconstruct_block(constructor.block).0;

        // Fold the return statements into the block.
        let returns = std::mem::take(&mut self.returns);
        let expression_returns: Vec<(Option<Expression>, ReturnStatement)> = returns
            .into_iter()
            .map(|(guard, statement)| match guard {
                ReturnGuard::None => (None, statement),
                ReturnGuard::Unconstructed(plain) | ReturnGuard::Constructed { plain, .. } => {
                    (Some(plain.into()), statement)
                }
            })
            .collect();

        self.fold_returns(&mut block, expression_returns);

        Constructor { annotations: constructor.annotations, block, span: constructor.span, id: constructor.id }
    }
}
