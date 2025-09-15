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

use super::SsaFormingVisitor;

use leo_ast::{
    Block,
    Composite,
    Constructor,
    ConstructorConsumer,
    Function,
    FunctionConsumer,
    Identifier,
    Member,
    Module,
    ModuleConsumer,
    Node as _,
    Program,
    ProgramConsumer,
    ProgramScope,
    ProgramScopeConsumer,
    StatementConsumer,
    StructConsumer,
};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;

impl StructConsumer for SsaFormingVisitor<'_> {
    type Output = Composite;

    /// Reconstructs records in the program, ordering its fields such that `owner` and is the first field.
    fn consume_struct(&mut self, struct_: Composite) -> Self::Output {
        match struct_.is_record {
            false => struct_,
            true => {
                let mut members = Vec::with_capacity(struct_.members.len());
                let mut member_map: IndexMap<Symbol, Member> =
                    struct_.members.into_iter().map(|member| (member.identifier.name, member)).collect();

                // Add the owner field to the beginning of the members list.
                // Note that type checking ensures that the owner field exists.
                members.push(member_map.shift_remove(&sym::owner).unwrap());

                // Add the remaining fields to the members list.
                members.extend(member_map.into_iter().map(|(_, member)| member));

                Composite { members, ..struct_ }
            }
        }
    }
}

impl FunctionConsumer for SsaFormingVisitor<'_> {
    type Output = Function;

    /// Reconstructs the `Function`s in the `Program`, while allocating the appropriate `RenameTable`s.
    fn consume_function(&mut self, mut function: Function) -> Self::Output {
        // Allocate a `RenameTable` for the function.
        self.push();

        if self.rename_defs {
            // For each input, change to a unique name.
            for input_variable in function.input.iter_mut() {
                let old_identifier = input_variable.identifier;
                let new_symbol = self.state.assigner.unique_symbol(old_identifier, "$$");
                let new_identifier = Identifier::new(new_symbol, self.state.node_builder.next_id());
                input_variable.identifier = new_identifier;

                // Add the new identifier to the type table.
                self.state.type_table.insert(new_identifier.id(), input_variable.type_.clone());

                // Associate the old name with its ID.
                self.rename_table.update(old_identifier.name, old_identifier.name, old_identifier.id);

                // And make the rename.
                self.rename_table.update(old_identifier.name, new_identifier.name, old_identifier.id);
            }
        }

        function.block =
            Block { span: function.block.span, id: function.block.id, statements: self.consume_block(function.block) };

        // Remove the `RenameTable` for the function.
        self.pop();

        function
    }
}

impl ConstructorConsumer for SsaFormingVisitor<'_> {
    type Output = Constructor;

    /// Reconstructs the `Constructor` in the `Program`, while allocating the appropriate `RenameTable`s.
    fn consume_constructor(&mut self, mut constructor: Constructor) -> Self::Output {
        // Allocate a `RenameTable` for the constructor.
        self.push();
        // Rename the constructor's block.
        constructor.block = Block {
            span: constructor.block.span,
            id: constructor.block.id,
            statements: self.consume_block(constructor.block),
        };
        // Remove the `RenameTable` for the constructor.
        self.pop();

        constructor
    }
}

impl ProgramScopeConsumer for SsaFormingVisitor<'_> {
    type Output = ProgramScope;

    fn consume_program_scope(&mut self, input: ProgramScope) -> Self::Output {
        self.program = input.program_id.name.name;
        ProgramScope {
            program_id: input.program_id,
            consts: input.consts,
            structs: input.structs.into_iter().map(|(i, s)| (i, self.consume_struct(s))).collect(),
            mappings: input.mappings,
            storage_variables: input.storage_variables,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.consume_function(f))).collect(),
            constructor: input.constructor.map(|c| self.consume_constructor(c)),
            span: input.span,
        }
    }
}

impl ProgramConsumer for SsaFormingVisitor<'_> {
    type Output = Program;

    fn consume_program(&mut self, input: Program) -> Self::Output {
        Program {
            modules: input.modules.into_iter().map(|(path, module)| (path, self.consume_module(module))).collect(),
            imports: input
                .imports
                .into_iter()
                .map(|(name, (import, span))| (name, (self.consume_program(import), span)))
                .collect(),
            stubs: input.stubs,
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(name, scope)| (name, self.consume_program_scope(scope)))
                .collect(),
        }
    }
}

impl ModuleConsumer for SsaFormingVisitor<'_> {
    type Output = Module;

    fn consume_module(&mut self, input: Module) -> Self::Output {
        Module {
            path: input.path,
            program_name: self.program,
            structs: input.structs.into_iter().map(|(i, s)| (i, self.consume_struct(s))).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.consume_function(f))).collect(),
            consts: input.consts,
        }
    }
}
