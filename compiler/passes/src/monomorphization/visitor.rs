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

use crate::{CompilerState, Replacer};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use leo_ast::{
    CallExpression,
    Composite,
    CompositeType,
    Expression,
    Function,
    Identifier,
    ProgramReconstructor,
    StructExpression,
};
use leo_span::Symbol;

pub struct MonomorphizationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The main program.
    pub program: Symbol,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_functions: IndexMap<Symbol, Function>,
    /// A set of all functions that have been monomorphized at least once. This keeps track of the _original_ names of
    /// the functions not the names of the monomorphized versions.
    pub monomorphized_functions: IndexSet<Symbol>,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_structs: IndexMap<Symbol, Composite>,
    /// A set of all functions that have been monomorphized at least once. This keeps track of the _original_ names of
    /// the functions not the names of the monomorphized versions.
    pub monomorphized_structs: IndexSet<Symbol>,
    /// A vector of all the calls to const generic functions that have not been resolved.
    pub unresolved_calls: Vec<CallExpression>,
    /// A vector of all the struct expressions of const generic structs that have not been resolved.
    pub unresolved_struct_exprs: Vec<StructExpression>,
    /// A vector of all the struct type instantiations of const generic structs that have not been resolved.
    pub unresolved_struct_types: Vec<CompositeType>,
    /// Have we actually modified the program at all?
    pub changed: bool,
}

impl leo_ast::StatementReconstructor for MonomorphizationVisitor<'_> {}

impl MonomorphizationVisitor<'_> {
    /// Monomorphizes a generic struct by substituting const parameters with concrete arguments and caching result.
    /// Generates a unique name like `Foo::[1u32, 2u32]` based on the original name and the provided const arguments.
    /// Replaces all const parameter references in the struct body with values, then resolves nested generics.
    /// Assigns a new identifier and struct ID, clears const params, and stores the result to avoid reprocessing.
    /// Panics if the original struct is not found in `reconstructed_structs` (should already be reconstructed).
    ///
    /// # Arguments
    /// * `name` - Symbol of the original generic struct.
    /// * `const_arguments` - Const values to substitute into the struct.
    /// * Returns a `Symbol` for the newly monomorphized struct.
    ///
    /// Note: this functions already assumes that all provided const arguments are literals.
    pub(crate) fn monomorphize_struct(&mut self, name: &Symbol, const_arguments: &Vec<Expression>) -> Symbol {
        // Generate a unique name for the monomorphized struct based on const arguments.
        //
        // For `struct Foo::[x: u32, y: u32](..)`, the generated name would be `Foo::[1u32, 2u32]` for a struct
        // expression that sets `x` to `1u32` and `y` to `2u32`. We know this name is safe to use because it's not a
        // valid identifier in the user code.
        //
        // Later, we have to legalize these names because they are not valid Aleo identifiers. We do this in codegen.
        let new_struct_name = leo_span::Symbol::intern(&format!("{}::[{}]", name, const_arguments.iter().format(", ")));

        // Check if the new struct name is not already present in `reconstructed_structs`. This ensures that we do not
        // add a duplicate definition for the same struct.
        if self.reconstructed_structs.get(&new_struct_name).is_none() {
            // Look up the already reconstructed struct by name.
            let struct_ = self
                .reconstructed_structs
                .get(name)
                .expect("Struct should already be reconstructed (post-order traversal).");

            // Build mapping from const parameters to const argument values.
            let const_param_map: IndexMap<_, _> =
                struct_.const_parameters.iter().map(|param| param.identifier().name).zip_eq(const_arguments).collect();

            // Function to replace identifiers with their corresponding const argument or keep them unchanged.
            let replace_identifier = |ident: &Identifier| {
                const_param_map.get(&ident.name).map_or(Expression::Identifier(*ident), |&expr| expr.clone())
            };

            let mut replacer = Replacer::new(replace_identifier, &self.state.node_builder);

            // Create a new version of `struct_` that has a new name, no const parameters, and a new struct ID.
            //
            // First, reconstruct the struct by changing all instances of const generic parameters to literals
            // according to `const_param_map`.
            let mut struct_ = replacer.reconstruct_struct(struct_.clone());

            // Now, reconstruct the struct to actually monomorphize its content such as generic struct type
            // instantiations.
            struct_ = self.reconstruct_struct(struct_);
            struct_.identifier = Identifier {
                name: new_struct_name,
                span: leo_span::Span::default(),
                id: self.state.node_builder.next_id(),
            };
            struct_.const_parameters = vec![];
            struct_.id = self.state.node_builder.next_id();

            // Keep track of the new struct in case other structs need it.
            self.reconstructed_structs.insert(new_struct_name, struct_);

            // Now keep track of the struct we just monomorphized
            self.monomorphized_structs.insert(*name);
        }

        new_struct_name
    }
}
