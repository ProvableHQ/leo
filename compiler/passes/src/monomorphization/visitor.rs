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
    CompositeExpression,
    CompositeType,
    Expression,
    Function,
    Identifier,
    Path,
    ProgramReconstructor,
};
use leo_span::Symbol;

pub struct MonomorphizationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The main program.
    pub program: Symbol,
    /// A map to provide faster lookup of functions.
    pub function_map: IndexMap<Vec<Symbol>, Function>,
    /// A map to provide faster lookup of composites.
    pub composite_map: IndexMap<Vec<Symbol>, Composite>,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_functions: IndexMap<Vec<Symbol>, Function>,
    /// A set of all functions that have been monomorphized at least once. This keeps track of the _original_ names of
    /// the functions not the names of the monomorphized versions.
    pub monomorphized_functions: IndexSet<Vec<Symbol>>,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_composites: IndexMap<Vec<Symbol>, Composite>,
    /// A set of all functions that have been monomorphized at least once. This keeps track of the _original_ names of
    /// the functions not the names of the monomorphized versions.
    pub monomorphized_composites: IndexSet<Vec<Symbol>>,
    /// A vector of all the calls to const generic functions that have not been resolved.
    pub unresolved_calls: Vec<CallExpression>,
    /// A vector of all the composite expressions of const generic composites that have not been resolved.
    pub unresolved_composite_exprs: Vec<CompositeExpression>,
    /// A vector of all the composite type instantiations of const generic composites that have not been resolved.
    pub unresolved_composite_types: Vec<CompositeType>,
    /// Have we actually modified the program at all?
    pub changed: bool,
}

impl MonomorphizationVisitor<'_> {
    /// Monomorphizes a generic composite by substituting const parameters with concrete arguments and caching result.
    /// Generates a unique name like `Foo::[1u32, 2u32]` based on the original name and the provided const arguments.
    /// Replaces all const parameter references in the composite body with values, then resolves nested generics.
    /// Assigns a new name and composite ID, clears const params, and stores the result to avoid reprocessing.
    /// Panics if the original composite is not found in `reconstructed_composites` (should already be reconstructed).
    ///
    /// # Arguments
    /// * `name` - Symbol of the original generic composite.
    /// * `const_arguments` - Const values to substitute into the composite.
    /// * Returns a `Symbol` for the newly monomorphized composite.
    ///
    /// Note: this functions already assumes that all provided const arguments are literals.
    pub(crate) fn monomorphize_composite(&mut self, path: &Path, const_arguments: &Vec<Expression>) -> Path {
        // Generate a unique name for the monomorphized composite based on const arguments.
        //
        // For `struct Foo::[x: u32, y: u32](..)`, the generated name would be `Foo::[1u32, 2u32]` for a composite
        // expression that sets `x` to `1u32` and `y` to `2u32`. We know this name is safe to use because it's not a
        // valid identifier in the user code.
        //
        // Later, we have to legalize these names because they are not valid Aleo identifiers. We do this in codegen.
        let new_composite_path = path.clone().with_updated_last_symbol(leo_span::Symbol::intern(&format!(
            "{}::[{}]",
            path.identifier().name,
            const_arguments.iter().format(", ")
        )));

        // Check if the new composite name is not already present in `reconstructed_composites`. This ensures that we do not
        // add a duplicate definition for the same composite.
        if self.reconstructed_composites.get(&new_composite_path.expect_global_location().path).is_none() {
            let full_name = &path.expect_global_location().path;
            // Look up the already reconstructed composite by name.
            let composite = self
                .reconstructed_composites
                .get(full_name)
                .expect("Composite should already be reconstructed (post-order traversal).");

            // Build mapping from const parameters to const argument values.
            let const_param_map: IndexMap<_, _> = composite
                .const_parameters
                .iter()
                .map(|param| param.identifier().name)
                .zip_eq(const_arguments)
                .collect();

            // Function to replace path expressions with their corresponding const argument or keep them unchanged.
            let replace_path = |expr: &Expression| match expr {
                Expression::Path(path) => const_param_map
                    .get(&path.identifier().name)
                    .map_or(Expression::Path(path.clone()), |&expr| expr.clone()),
                _ => expr.clone(),
            };

            let mut replacer = Replacer::new(replace_path, true /* refresh IDs */, self.state);

            // Create a new version of `composite` that has a new name, no const parameters, and a new composite ID.
            //
            // First, reconstruct the composite by changing all instances of const generic parameters to literals
            // according to `const_param_map`.
            let mut composite = replacer.reconstruct_composite(composite.clone());

            // Now, reconstruct the composite to actually monomorphize its content such as generic composite type
            // instantiations.
            composite = self.reconstruct_composite(composite);
            composite.identifier =
                Identifier::new(new_composite_path.identifier().name, self.state.node_builder.next_id());
            composite.const_parameters = vec![];
            composite.id = self.state.node_builder.next_id();

            // Keep track of the new composite in case other composites need it.
            self.reconstructed_composites.insert(new_composite_path.expect_global_location().path.clone(), composite);

            // Now keep track of the composite we just monomorphized
            self.monomorphized_composites.insert(full_name.clone());
        }

        new_composite_path
    }
}
