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

use crate::{CompilerState, Replacer, common::items_at_path};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use leo_ast::{
    AstReconstructor,
    CallExpression,
    Composite,
    CompositeExpression,
    CompositeType,
    Expression,
    Function,
    Identifier,
    Location,
    Module,
    Path,
    Program,
    ProgramReconstructor,
    ProgramScope,
    Statement,
};
use leo_span::Symbol;

pub struct MonomorphizationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The main program.
    pub program: Symbol,
    /// A map to provide faster lookup of functions.
    pub function_map: IndexMap<Location, Function>,
    /// A map to provide faster lookup of composites.
    pub composite_map: IndexMap<Location, Composite>,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_functions: IndexMap<Location, Function>,
    /// A set of all functions that have been monomorphized at least once. This keeps track of the _original_ names of
    /// the functions not the names of the monomorphized versions.
    pub monomorphized_functions: IndexSet<Location>,
    /// A map of reconstructed composites in the current program scope.
    pub reconstructed_composites: IndexMap<Location, Composite>,
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
        let original_loc = path.expect_global_location();

        // Generate a unique monomorphized name for the composite.
        //
        // For both local and library composites, the name uses the base struct name plus its
        // const arguments: e.g. `Foo::[1u32]`. Uniqueness within the consuming program is
        // guaranteed by the storage location (see below).
        //
        // These names are not valid Leo identifiers and are legalized in codegen.
        let monomorphized_name =
            leo_span::Symbol::intern(&format!("{}::[{}]", path.identifier().name, const_arguments.iter().format(", ")));

        // Compute the storage location for the monomorphized composite.
        //
        // All composites — local, library, and external FromLeo — preserve their module path:
        // the prefix is kept and only the last segment (the struct name) is replaced with the
        // monomorphized name. For example:
        //   - local `types::Vec`       → `types::Vec::[3u32]`
        //   - library `lib::dep::Foo`  → `lib::dep::Foo::[3u32]`
        //   - external `child::sub::T` → `child::sub::T::[3u32]`
        //
        // Top-level composites (path length 1) are unaffected: `path[..0]` is empty,
        // so the result is just `[Name::[args]]`.
        //
        // Preserving the full path is necessary for correctness when two submodules in the same
        // library define structs with the same name: flattening would cause the second
        // monomorphized entry to silently overwrite the first in the symbol table.
        let mut path_segs = Vec::with_capacity(original_loc.path.len());
        path_segs.extend_from_slice(&original_loc.path[..original_loc.path.len() - 1]);
        path_segs.push(monomorphized_name);
        let storage_loc = Location::new(original_loc.program, path_segs);

        // Build the new path pointing directly to the storage location.
        let new_composite_path =
            path.clone().with_updated_last_symbol(monomorphized_name).to_global(storage_loc.clone());

        // Check if the new composite name is not already present in `reconstructed_composites`. This ensures that we do not
        // add a duplicate definition for the same composite.
        if self.reconstructed_composites.get(&storage_loc).is_none() {
            // Look up the already reconstructed composite by name.
            let composite = self
                .reconstructed_composites
                .get(original_loc)
                .unwrap_or_else(|| panic!("Composite should already be reconstructed (post-order traversal)."));

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
            composite.identifier = Identifier::new(monomorphized_name, self.state.node_builder.next_id());
            composite.const_parameters = vec![];
            composite.id = self.state.node_builder.next_id();

            // Keep track of the new composite in case other composites need it. The generic
            // original is dropped later by filtering on `const_parameters.is_empty()`.
            self.reconstructed_composites.insert(storage_loc.clone(), composite);
        }

        new_composite_path
    }

    /// Assembles a `Module` from the already-populated `reconstructed_*` maps. Interfaces are
    /// reconstructed because they may reference composites that have been monomorphized.
    /// `input.program_name` is used (not `self.program`) because a nested `reconstruct_program`
    /// call on a `Stub::FromLeo` dependency may have moved `self.program` off the module's own
    /// program.
    pub(super) fn assemble_module(&mut self, input: Module) -> Module {
        Module {
            composites: items_at_path(&self.reconstructed_composites, input.program_name, &input.path).collect(),
            functions: items_at_path(&self.reconstructed_functions, input.program_name, &input.path).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            ..input
        }
    }

    /// Assembles a `FromLeo` stub's program from the already-populated `reconstructed_*` maps.
    /// `input.stubs` is always empty on a stub's program (only the top-level `Program` carries
    /// stubs), so it passes through unchanged.
    pub(super) fn assemble_from_leo_program(&mut self, input: Program) -> Program {
        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.assemble_from_leo_scope(id, scope))).collect();
        let modules = input.modules.into_iter().map(|(mid, m)| (mid, self.assemble_module(m))).collect();
        Program { program_scopes, modules, stubs: input.stubs, imports: input.imports }
    }

    /// Assembles a single ProgramScope for a FromLeo stub from `reconstructed_*`.
    fn assemble_from_leo_scope(&mut self, program_name: Symbol, input: ProgramScope) -> ProgramScope {
        // Exclude generic composites that have been monomorphized — only their concrete
        // specializations should appear in the output.
        let composites = items_at_path(&self.reconstructed_composites, program_name, &[])
            .filter(|(_, c)| c.const_parameters.is_empty())
            .collect();

        // Entry-point functions must appear before finalize functions so the type checker
        // can populate `async_function_callers` before visiting finalizers.
        let (entry_points, non_entry_points): (Vec<_>, Vec<_>) =
            items_at_path(&self.reconstructed_functions, program_name, &[]).partition(|(_, f)| f.variant.is_entry());
        let functions: Vec<_> = entry_points.into_iter().chain(non_entry_points).collect();

        // Reconstruct other items that might reference monomorphized composites.
        let mappings = input.mappings.into_iter().map(|(id, m)| (id, self.reconstruct_mapping(m))).collect();
        let storage_variables =
            input.storage_variables.into_iter().map(|(id, sv)| (id, self.reconstruct_storage_variable(sv))).collect();
        let consts = input
            .consts
            .into_iter()
            .map(|(i, c)| match self.reconstruct_const(c) {
                (Statement::Const(declaration), _) => (i, declaration),
                _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
            })
            .collect();
        let constructor = input.constructor.map(|c| self.reconstruct_constructor(c));

        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.into_iter().map(|(s, t)| (s, self.reconstruct_type(t).0)).collect(),
            composites,
            mappings,
            storage_variables,
            functions,
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor,
            consts,
            span: input.span,
        }
    }
}
