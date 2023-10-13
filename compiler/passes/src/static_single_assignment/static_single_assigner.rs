// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{Assigner, RenameTable, SymbolTable, TypeTable};

use leo_ast::{Expression, Identifier, Node, NodeBuilder, Statement};

pub struct StaticSingleAssigner<'a> {
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// The `SymbolTable` of the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// A mapping from node IDs to their types.
    pub(crate) type_table: &'a TypeTable,
    /// The `RenameTable` for the current basic block in the AST
    pub(crate) rename_table: RenameTable,
    /// A flag to determine whether or not the traversal is on the left-hand side of a definition or an assignment.
    pub(crate) is_lhs: bool,
    /// A struct used to construct (unique) assignment statements.
    pub(crate) assigner: &'a Assigner,
}

impl<'a> StaticSingleAssigner<'a> {
    /// Initializes a new `StaticSingleAssigner` with an empty `RenameTable`.
    pub(crate) fn new(
        node_builder: &'a NodeBuilder,
        symbol_table: &'a SymbolTable,
        type_table: &'a TypeTable,
        assigner: &'a Assigner,
    ) -> Self {
        Self { node_builder, symbol_table, type_table, rename_table: RenameTable::new(None), is_lhs: false, assigner }
    }

    /// Pushes a new scope, setting the current scope as the new scope's parent.
    pub(crate) fn push(&mut self) {
        let parent_table = core::mem::take(&mut self.rename_table);
        self.rename_table = RenameTable::new(Some(Box::from(parent_table)));
    }

    /// If the RenameTable has a parent, then `self.rename_table` is set to the parent, otherwise it is set to a default `RenameTable`.
    pub(crate) fn pop(&mut self) -> RenameTable {
        let parent = self.rename_table.parent.clone().unwrap_or_default();
        core::mem::replace(&mut self.rename_table, *parent)
    }

    pub(crate) fn simple_assign_statement(&mut self, identifier: Identifier, rhs: Expression) -> Statement {
        // Update the type table.
        let type_ = match self.type_table.get(&rhs.id()) {
            Some(type_) => type_,
            None => unreachable!("Type checking guarantees that all expressions have a type."),
        };
        self.type_table.insert(identifier.id(), type_);
        // Update the rename table.
        self.rename_table.update(identifier.name, identifier.name, identifier.id);
        // Construct the statement.
        self.assigner.simple_assign_statement(identifier, rhs, self.node_builder.next_id())
    }

    /// Constructs a simple assign statement for `expr` with a unique name.
    /// For example, `expr` is transformed into `$var$0 = expr;`.
    /// The lhs is guaranteed to be unique with respect to the `Assigner`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Identifier, Statement) {
        // Create a new variable for the expression.
        let name = self.assigner.unique_symbol("$var", "$");

        // Create a new identifier for the variable.
        let place = Identifier { name, span: Default::default(), id: self.node_builder.next_id() };

        // Construct the statement.
        let statement = self.simple_assign_statement(place, expr);

        (place, statement)
    }
}
