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

use crate::{CompilerState, Pass};

use leo_ast::*;
use leo_errors::Result;

/// Pass that turns ambiguous calls into their proper form after type checking
/// such as get and set for mappings/vectors
pub struct Disambiguate;

impl Pass for Disambiguate {
    type Input = ();
    type Output = ();

    const NAME: &str = "Disambiguate";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);

        let mut visitor = DisambiguateVisitor { state };

        let ast = match ast {
            Ast::Program(program) => Ast::Program(visitor.reconstruct_program(program)),
            Ast::Library(library) => Ast::Library(visitor.reconstruct_library(library)),
        };

        // Re-sync the symbol table's function bodies with the now-disambiguated AST. Bodies were
        // registered during global-item collection (before method calls were resolved), so passes
        // that read callee bodies from the table (e.g. CEI analysis) would otherwise see unresolved
        // `MethodCall` nodes. Refresh them from the reconstructed AST, including import stubs.
        match &ast {
            Ast::Program(program) => {
                let functions: Vec<_> =
                    crate::common::program_functions(program).map(|(loc, f)| (loc, f.clone())).collect();
                let stub_functions: Vec<_> = program
                    .stubs
                    .values()
                    .flat_map(crate::common::stub_functions)
                    .map(|(loc, f)| (loc, f.clone()))
                    .collect();
                for (loc, f) in functions.into_iter().chain(stub_functions) {
                    visitor.state.symbol_table.update_function_body(&loc, f);
                }
            }
            Ast::Library(library) => {
                let functions: Vec<_> =
                    crate::common::library_functions(library).map(|(loc, f)| (loc, f.clone())).collect();
                for (loc, f) in functions {
                    visitor.state.symbol_table.update_function_body(&loc, f);
                }
            }
        }

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        Ok(())
    }
}

pub struct DisambiguateVisitor<'state> {
    pub state: &'state mut CompilerState,
}

impl UnitReconstructor for DisambiguateVisitor<'_> {}

impl AstReconstructor for DisambiguateVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn interner(&self) -> &TypeInterner {
        &self.state.types
    }

    fn reconstruct_intrinsic(
        &mut self,
        mut input: IntrinsicExpression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        input.type_parameters =
            input.type_parameters.into_iter().map(|(ty, span)| (self.reconstruct_type(ty).0, span)).collect();
        input.input_types =
            input.input_types.into_iter().map(|(mode, ty, span)| (mode, self.reconstruct_type(ty).0, span)).collect();
        input.return_types =
            input.return_types.into_iter().map(|(mode, ty, span)| (mode, self.reconstruct_type(ty).0, span)).collect();
        input.arguments = input.arguments.into_iter().map(|arg| self.reconstruct_expression(arg, &()).0).collect();

        (input.into(), ())
    }

    /// Rewrite a `MethodCall` (resolved by type checking) into its concrete form. The receiver's
    /// type — recorded in the type table — selects a built-in intrinsic (`Vector`/`Mapping`/
    /// `Optional`/`signature`/`Future`) or a user method on a struct/record. `id` is preserved so the
    /// type recorded for the node stays valid for later passes.
    fn reconstruct_method_call(
        &mut self,
        input: MethodCall,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let receiver_ty =
            self.state.type_table.get(&input.receiver.id()).expect("type checking recorded the receiver's type");
        let span = input.span;

        // `[receiver, ..args]`, reconstructed.
        let arguments: Vec<Expression> = std::iter::once(input.receiver)
            .chain(input.arguments)
            .map(|arg| self.reconstruct_expression(arg, &()).0)
            .collect();

        match &receiver_ty {
            Type::Composite(ct) => {
                // User method: build a resolved call against `[Type, method]`.
                let base = ct.path.try_global_location().expect("type checking resolved the composite type");
                let mut path = base.path.clone();
                path.push(input.method.name);
                let location = Location::new(base.program, path);
                let ident = Identifier { name: input.method.name, span, id: self.state.node_builder.next_id() };
                let function =
                    Path::new(None, Vec::new(), ident, span, self.state.node_builder.next_id()).to_global(location);
                (CallExpression { function, const_arguments: Vec::new(), arguments, span, id: input.id }.into(), ())
            }
            _ => {
                // Built-in method: pick the concrete intrinsic symbol from the receiver type.
                let name = Intrinsic::builtin_method_symbol(&receiver_ty, input.method.name)
                    .expect("type checking guaranteed a built-in method here");
                let intrinsic = IntrinsicExpression {
                    name,
                    type_parameters: Vec::new(),
                    input_types: Vec::new(),
                    return_types: Vec::new(),
                    arguments,
                    span,
                    id: input.id,
                };
                (intrinsic.into(), ())
            }
        }
    }
}
