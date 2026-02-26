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

use super::StorageLoweringVisitor;
use leo_ast::{
    AstReconstructor,
    Identifier,
    IntegerType,
    Location,
    Mapping,
    ProgramReconstructor,
    ProgramScope,
    Statement,
    StorageVariable,
    Type,
    VectorType,
};
use leo_span::Symbol;

impl ProgramReconstructor for StorageLoweringVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.name.name;

        let storage_variables = input
            .storage_variables
            .into_iter()
            .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
            .collect::<Vec<_>>();

        // After reconstructing the storage variables, `self.new_mappings` might be non-empty.
        // Reconstruct old mappings and then append the new mappings to the final list.
        let mut mappings =
            input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect::<Vec<_>>();
        mappings.extend(
            self.new_mappings
                .iter()
                .filter(|(Location { program, .. }, _)| *program == self.program)
                .map(|(Location { program, .. }, mapping)| (*program, mapping.clone())),
        );

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
            mappings,
            storage_variables,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    fn reconstruct_storage_variable(&mut self, input: StorageVariable) -> StorageVariable {
        // For each storage variable, create one or more backing mappings that represent
        // how the value is stored in lower-level form.
        //
        // Example:
        //   storage x: field;
        // becomes:
        //   mapping x__: bool => field;
        //
        // Example (vector):
        //   storage vec: [u32];
        // becomes:
        //   mapping vec__: u32 => u32;       // element mapping
        //   mapping vec__len__: bool => u32; // length tracking

        let id = || self.state.node_builder.next_id();
        let name = input.identifier.name.to_string();

        // Common base mapping name: e.g. "x__" or "vec__"
        let mapping_name = Symbol::intern(&(name.clone() + "__"));

        match &input.type_ {
            Type::Vector(VectorType { element_type }) => {
                // === Vector storage ===
                //
                // Create two mappings:
                //   1. `<name>__`:  index → element
                //   2. `<name>__len__`:  bool → length

                // Mapping for the vector’s contents
                self.new_mappings.insert(Location::new(self.program, vec![mapping_name]), Mapping {
                    identifier: Identifier::new(mapping_name, id()),
                    key_type: Type::Integer(IntegerType::U32),
                    value_type: *element_type.clone(),
                    span: input.span,
                    id: id(),
                });

                // Mapping for the vector’s length
                let len_name = Symbol::intern(&(name + "__len__"));
                self.new_mappings.insert(Location::new(self.program, vec![len_name]), Mapping {
                    identifier: Identifier::new(len_name, id()),
                    key_type: Type::Boolean,
                    value_type: Type::Integer(IntegerType::U32),
                    span: input.span,
                    id: id(),
                });
            }

            _ => {
                // === Singleton storage ===
                //
                // Every non-vector storage variable is represented as a single mapping:
                //   `<name>__`: bool → <type>
                //
                // The `bool` key acts as a presence indicator (typically `false`).

                self.new_mappings.insert(Location::new(self.program, vec![mapping_name]), Mapping {
                    identifier: Identifier::new(mapping_name, id()),
                    key_type: Type::Boolean,
                    value_type: input.type_.clone(),
                    span: input.span,
                    id: id(),
                });
            }
        }

        // Return the original (unmodified) storage variable node.
        input
    }
}
