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

use super::MonomorphizationVisitor;
use leo_ast::{
    AstReconstructor,
    Library,
    Location,
    Module,
    Program,
    ProgramReconstructor,
    ProgramScope,
    Statement,
    Stub,
    Variant,
};
use leo_span::{Symbol, sym};

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_library(&mut self, input: Library) -> Library {
        // Collect the (re-)constructed versions of library composites that were processed
        // during reconstruct_program_scope. We filter by the library's program name so that
        // only composites belonging to this library are included.
        //
        // This ensures the library stub is updated with correct field types after monomorphization
        // (e.g., `Bar.f` pointing to the consuming program's monomorphized `Foo::[42u32]`).

        // Seed `reconstructed_functions` with non-generic library module functions.
        // These functions are not processed by the main reconstruction loop (which only handles
        // generic library functions and current-program functions), so we process them here via
        // `reconstruct_function` to update any composite type references (e.g., a non-generic
        // function that constructs or returns a generic composite with concrete const arguments).
        // Without this, the composite expressions in the function body and its signature would
        // still reference the generic composite location rather than the monomorphized one.
        for (module_path, module) in &input.modules {
            for (name, f) in &module.functions {
                if f.const_parameters.is_empty() {
                    let path: Vec<Symbol> = module_path.iter().cloned().chain(std::iter::once(*name)).collect();
                    let loc = Location::new(input.name, path);
                    if !self.reconstructed_functions.contains_key(&loc) {
                        let processed = self.reconstruct_function(f.clone());
                        self.reconstructed_functions.insert(loc, processed);
                    }
                }
            }
        }

        Library {
            name: input.name,
            // Reconstruct each module, which collects its monomorphized composites and functions
            // from reconstructed_composites/reconstructed_functions via reconstruct_module.
            modules: input.modules.into_iter().map(|(id, m)| (id, self.reconstruct_module(m))).collect(),
            structs: self
                .reconstructed_composites
                .iter()
                .filter_map(|(loc, c)| {
                    loc.path
                        .split_last()
                        .filter(|(_, rest)| rest.is_empty() && loc.program == input.name)
                        .map(|(last, _)| (*last, c.clone()))
                })
                .collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            // Collect top-level library function instances. Non-generic functions are kept from
            // the original input; generic functions are superseded by their reconstructed versions
            // in `reconstructed_functions`. Excluding generic originals prevents duplicates that
            // would cause TypeChecking to report "defined multiple times" errors on the next
            // iteration.
            functions: input
                .functions
                .into_iter()
                .filter(|(_, f)| f.const_parameters.is_empty())
                .chain(self.reconstructed_functions.iter().filter_map(|(loc, f)| {
                    loc.path
                        .split_last()
                        .filter(|(_, rest)| rest.is_empty() && loc.program == input.name)
                        .map(|(last, _)| (*last, f.clone()))
                }))
                .collect(),
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.as_symbol();

        // We first reconstruct all composites. Composite fields can instantiate other generic composites that we need to handle
        // first. We'll then address composite expressions and other compoiste type instantiations.
        let composite_order = self.state.composite_graph.post_order().unwrap();

        // Reconstruct composites in post-order.
        for loc in &composite_order {
            if let Some(composite) = self.composite_map.swap_remove(loc) {
                // Perform monomorphization or other reconstruction logic.
                let reconstructed_composite = self.reconstruct_composite(composite);
                // Store the reconstructed composite under its original location.
                // For library composites loc.program != self.program; preserving the library
                // program name here is what allows monomorphize_composite to find them later.
                // Both the program-scope and module output-collection loops filter by
                // `loc.program == self.program`, so library entries stay invisible to the output.
                self.reconstructed_composites.insert(loc.clone(), reconstructed_composite);
            }
        }

        // If there are some local composites left in `composite_map`, that means these are dead
        // composites since they do not show up in `composite_graph`. Therefore, they won't be
        // reconstructed, implying a change in the reconstructed program.
        if !self.composite_map.is_empty() {
            self.changed = true;
        }

        // Next, handle generic functions
        //
        // Compute a post-order traversal of the call graph. This ensures that functions are processed after all their callees.
        // Make sure to only compute the post order by considering the entry points of the program, which are `async transition`, `transition` and `function`.
        // We must consider entry points to ignore const generic inlines that have already been monomorphized but never called.
        let order = self
            .state
            .call_graph
            .post_order_with_filter(|location| {
                // Filter out locations that are not from this program.
                if location.program != self.program {
                    return false;
                }
                // Allow constructors.
                if location.program == self.program && location.path == vec![sym::constructor] {
                    return true;
                }
                self.function_map
                    .get(location)
                    .map(|f| {
                        matches!(f.variant, Variant::EntryPoint)
                            | (f.variant == Variant::Fn && f.const_parameters.is_empty())
                    })
                    .unwrap_or(false)
            })
            .unwrap() // This unwrap is safe because the type checker guarantees an acyclic graph.
            .into_iter()
            .filter(|location| {
                // Always include current-program functions.
                if location.program == self.program {
                    return true;
                }
                // Also include generic library functions reachable from the current program.
                // Non-generic library functions need no monomorphization and are handled by
                // the early-return in `reconstruct_call` (no const arguments).
                self.state.symbol_table.is_library(location.program)
                    && self.function_map.get(location).map(|f| !f.const_parameters.is_empty()).unwrap_or(false)
            })
            .collect::<Vec<_>>();

        for function_name in &order {
            // Reconstruct functions in post-order.
            if let Some(function) = self.function_map.swap_remove(function_name) {
                // Reconstruct the function.
                let reconstructed_function = self.reconstruct_function(function);
                // Add the reconstructed function to the mapping.
                self.reconstructed_functions.insert(function_name.clone(), reconstructed_function);
            }
        }

        // Now reconstruct mappings and storage variables
        let mappings =
            input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect();
        let storage_variables = input
            .storage_variables
            .into_iter()
            .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
            .collect();

        // Then consts
        let consts = input
            .consts
            .into_iter()
            .map(|(i, c)| match self.reconstruct_const(c) {
                (Statement::Const(declaration), _) => (i, declaration),
                _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
            })
            .collect();

        // Reconstruct the constructor last, as it cannot be called by any other function.
        let constructor = input.constructor.map(|c| self.reconstruct_constructor(c));

        // Now retain only functions that are either not yet monomorphized or are still referenced by calls.
        self.reconstructed_functions.retain(|l, _| {
            let is_monomorphized = self.monomorphized_functions.contains(l);
            let is_still_called = self.unresolved_calls.iter().any(|c| c.function.expect_global_location() == l);
            !is_monomorphized || is_still_called
        });

        // Move reconstructed functions into the final `ProgramScope`.
        // Make sure to place transitions before all the other functions.
        let (transitions, mut non_transitions): (Vec<_>, Vec<_>) =
            self.reconstructed_functions.clone().into_iter().partition(|(_, f)| f.variant.is_entry());

        let mut all_functions = transitions;
        all_functions.append(&mut non_transitions);

        // Return the fully reconstructed scope with updated functions.
        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.into_iter().map(|(s, t)| (s, self.reconstruct_type(t).0)).collect(),
            composites: self
                .reconstructed_composites
                .iter()
                .filter_map(|(loc, c)| {
                    // Only consider composites defined at program scope within the same program.
                    loc.path
                        .split_last()
                        .filter(|(_, rest)| rest.is_empty() && loc.program == self.program)
                        .map(|(last, _)| (*last, c.clone()))
                })
                .collect(),
            mappings,
            storage_variables,
            functions: all_functions
                .iter()
                .filter_map(|(loc, f)| {
                    // Only consider functions defined at program scope within the same program.
                    loc.path
                        .split_last()
                        .filter(|(_, rest)| rest.is_empty() && loc.program == self.program)
                        .map(|(last, _)| (*last, f.clone()))
                })
                .collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor,
            consts,
            span: input.span,
        }
    }

    fn reconstruct_program(&mut self, input: Program) -> Program {
        // STUB ORDERING INVARIANT
        //
        // The ordering of operations in this function is critical for correctness:
        //
        // 1. FromLeo stubs are processed FIRST (before the current program's scope).
        //    Each FromLeo stub recursively calls reconstruct_program for the dependency,
        //    which populates `reconstructed_composites` with the dependency's composites.
        //    This means that when the current program's scope is processed, any composites
        //    from FromLeo dependencies (e.g. `child.aleo::Bar`) are already available for
        //    monomorphization.
        //
        // 2. The current program's composite_map and function_map are populated AFTER
        //    FromLeo stubs are processed (since each FromLeo recursive call clears and
        //    repopulates these maps for its own scope).
        //
        // 3. Library composites (from FromLibrary stubs) are added to composite_map so
        //    they are processed in reconstruct_program_scope (their field types updated).
        //
        // 4. program_scopes are processed AFTER FromLeo stubs and after composite_map is
        //    populated for the current program.
        //
        // 5. FromLibrary/FromAleo stubs are processed AFTER program_scopes, so that
        //    reconstruct_library can collect monomorphized composites from reconstructed_composites.

        // Capture the original stub insertion order before partitioning, so we can
        // reassemble stubs in their original order after processing.  The type-checking
        // pass that runs after monomorphization depends on this order.
        let stub_key_order: Vec<_> = input.stubs.keys().cloned().collect();

        // Partition stubs early (non-consuming borrow not possible, so partition now and
        // process in two phases).
        let (from_leo_stubs, other_stubs): (Vec<_>, Vec<_>) =
            input.stubs.into_iter().partition(|(_, stub)| matches!(stub, Stub::FromLeo { .. }));

        // Pre-Phase 1: Seed composite_map with library composites BEFORE recursing into
        // FromLeo stubs. This is critical for correctness: when a FromLeo dependency
        // (e.g. `helper.aleo`) uses library structs, its inner `Program` may not carry the
        // library stubs (they are only present on the top-level compilation unit). By inserting
        // the library composites here — before the recursive call — they are already in
        // composite_map when the inner call's Phase 2 retains non-previous-program entries.
        // Entries that are already in reconstructed_composites (processed by a prior inner call)
        // will be silently skipped by the composite_order loop via `swap_remove` returning None.
        other_stubs.iter().for_each(|(_, stub)| {
            if let Stub::FromLibrary { library, .. } = stub {
                library.structs.iter().for_each(|(name, c)| {
                    self.composite_map.entry(Location::new(library.name, vec![*name])).or_insert_with(|| c.clone());
                });
                // Seed function_map with library function definitions so that reconstruct_call can
                // inline them into the consuming program under mangled names.
                library.functions.iter().for_each(|(name, f)| {
                    self.function_map.entry(Location::new(library.name, vec![*name])).or_insert_with(|| f.clone());
                });
                // Seed composite_map and function_map with items from library submodules.
                library.modules.iter().for_each(|(module_path, m)| {
                    m.composites.iter().for_each(|(name, c)| {
                        let path: Vec<Symbol> = module_path.iter().cloned().chain(std::iter::once(*name)).collect();
                        self.composite_map.entry(Location::new(library.name, path)).or_insert_with(|| c.clone());
                    });
                    m.functions.iter().for_each(|(name, f)| {
                        let path: Vec<Symbol> = module_path.iter().cloned().chain(std::iter::once(*name)).collect();
                        self.function_map.entry(Location::new(library.name, path)).or_insert_with(|| f.clone());
                    });
                });
            }
        });

        // Phase 1: Process FromLeo stubs.
        // This recursively calls reconstruct_program for each dependency, populating
        // reconstructed_composites with their composites before the current scope runs.
        let from_leo_stubs: indexmap::IndexMap<_, _> =
            from_leo_stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect();

        // Phase 2: Set up for the current program.
        // After processing FromLeo stubs, reset and populate composite_map / function_map
        // for the current program's scope.
        //
        // IMPORTANT: Instead of clearing composite_map entirely, we only remove composites
        // belonging to the previous program (self.program). Library composites (loc.program !=
        // self.program) are retained so that deeply nested FromLeo programs — whose inner
        // Program objects may not carry library stubs — can still access them.
        let prev_program = self.program;
        self.composite_map.retain(|loc, _| loc.program != prev_program);
        // Retain library functions (analogous to composite_map.retain above) so that deeply-nested
        // FromLeo programs — whose inner Program objects may not carry library stubs — can still
        // inline library functions when their scopes are processed.
        self.function_map.retain(|loc, _| self.state.symbol_table.is_library(loc.program));

        self.program =
            *input.program_scopes.first().expect("a program must have a single program scope at this stage").0;

        // Populate `self.function_map` using the functions in the program scopes and the modules.
        input
            .modules
            .iter()
            .flat_map(|(module_path, m)| {
                m.functions.iter().map(move |(name, f)| {
                    (module_path.iter().cloned().chain(std::iter::once(*name)).collect(), f.clone())
                })
            })
            .chain(
                input
                    .program_scopes
                    .iter()
                    .flat_map(|(_, scope)| scope.functions.iter().map(|(name, f)| (vec![*name], f.clone()))),
            )
            .for_each(|(full_name, f)| {
                self.function_map.insert(Location::new(self.program, full_name), f);
            });

        // Populate `self.composite_map` using the composites in the program scopes and the modules.
        input
            .modules
            .iter()
            .flat_map(|(module_path, m)| {
                m.composites.iter().map(move |(name, f)| {
                    (module_path.iter().cloned().chain(std::iter::once(*name)).collect(), f.clone())
                })
            })
            .chain(
                input
                    .program_scopes
                    .iter()
                    .flat_map(|(_, scope)| scope.composites.iter().map(|(name, f)| (vec![*name], f.clone()))),
            )
            .for_each(|(full_name, f)| {
                self.composite_map.insert(Location::new(self.program, full_name), f);
            });

        // Phase 3: Process the current program's scope.
        // composite_map now has:
        //   - the current program's own composites,
        //   - library composites from FromLibrary stubs.
        // reconstructed_composites now has composites from all FromLeo dependencies.
        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.reconstruct_program_scope(scope))).collect();

        // Phase 4: Process FromLibrary/FromAleo stubs AFTER program_scopes.
        // reconstruct_library collects monomorphized composites from reconstructed_composites,
        // which is now populated with the current program's library composite instances.
        let other_stubs: indexmap::IndexMap<_, _> =
            other_stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect();

        let mut from_leo_map = from_leo_stubs;
        let mut other_map = other_stubs;
        let stubs = stub_key_order
            .into_iter()
            .map(|id| {
                let stub = from_leo_map
                    .swap_remove(&id)
                    .or_else(|| other_map.swap_remove(&id))
                    .expect("every stub key must appear in exactly one partition");
                (id, stub)
            })
            .collect();

        // Reconstruct modules after `self.reconstructed_composites` and `self.reconstructed_functions`
        // have been populated.
        Program {
            stubs,
            program_scopes,
            modules: input.modules.into_iter().map(|(id, module)| (id, self.reconstruct_module(module))).collect(),
            ..input
        }
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        // Here we're reconstructing composites and functions from `reconstructed_functions` and
        // `reconstructed_composites` based on their paths and whether they match the module path.
        // Use `input.program_name` rather than `self.program` since `self.program` may have been
        // updated by a nested reconstruct_program call (e.g., for a Stub::FromLeo dependency).
        Module {
            composites: self
                .reconstructed_composites
                .iter()
                .filter_map(|(loc, c)| {
                    loc.path
                        .split_last()
                        .filter(|(_, rest)| *rest == input.path && loc.program == input.program_name)
                        .map(|(last, _)| (*last, c.clone()))
                })
                .collect(),

            // Collect functions for this module from `reconstructed_functions`. Non-generic
            // functions are included because the main reconstruction loop processes all
            // reachable functions (both generic and non-generic). Generic originals are
            // excluded because only their monomorphized specializations are relevant.
            functions: self
                .reconstructed_functions
                .iter()
                .filter_map(|(loc, f)| {
                    loc.path
                        .split_last()
                        .filter(|(_, rest)| *rest == input.path && loc.program == input.program_name)
                        .map(|(last, _)| (*last, f.clone()))
                })
                .collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),

            ..input
        }
    }
}
