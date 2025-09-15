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

use super::StorageLoweringVisitor;
use leo_ast::{
    AstReconstructor,
    Identifier,
    IntegerType,
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
        let mut mappings =
            input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect::<Vec<_>>();
        mappings.extend(self.new_mappings.clone().into_iter().collect::<Vec<_>>());

        ProgramScope {
            program_id: input.program_id,
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            structs: input.structs.into_iter().map(|(i, c)| (i, self.reconstruct_struct(c))).collect(),
            mappings,
            storage_variables,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    fn reconstruct_storage_variable(&mut self, input: StorageVariable) -> StorageVariable {
        let mapping_name = Symbol::intern(&(input.identifier.name.to_string() + "__"));
        match input.type_ {
            Type::Vector(VectorType { ref element_type }) => {
                self.new_mappings.insert(mapping_name, Mapping {
                    identifier: Identifier::new(mapping_name, self.state.node_builder.next_id()),
                    key_type: Type::Integer(IntegerType::U32),
                    value_type: *element_type.clone(),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });
                let vec_len_name = Symbol::intern(&(input.identifier.name.to_string() + "__len__"));
                self.new_mappings.insert(vec_len_name, Mapping {
                    identifier: Identifier::new(vec_len_name, self.state.node_builder.next_id()),
                    key_type: Type::Boolean,
                    value_type: Type::Integer(IntegerType::U32),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });
            }
            _ => {
                self.new_mappings.insert(mapping_name, Mapping {
                    identifier: Identifier::new(mapping_name, self.state.node_builder.next_id()),
                    key_type: Type::Boolean,
                    value_type: input.type_.clone(),
                    span: input.span,
                    id: self.state.node_builder.next_id(),
                });
            }
        }

        input
    }
}
