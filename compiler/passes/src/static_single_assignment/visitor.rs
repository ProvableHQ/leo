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

use crate::{CompilerState, RenameTable};

use leo_ast::{Expression, Identifier, Node, Statement};
use leo_span::Symbol;

pub struct SsaFormingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The `RenameTable` for the current basic block in the AST
    pub rename_table: RenameTable,
    /// The main program name.
    pub program: Symbol,
    /// Whether to rename places in definitions.
    pub rename_defs: bool,
}

impl SsaFormingVisitor<'_> {
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

    pub(crate) fn rename_identifier(&mut self, mut identifier: Identifier) -> Identifier {
        // Associate this name with its id.
        self.rename_table.update(identifier.name, identifier.name, identifier.id);

        let new_name = self.state.assigner.unique_symbol(identifier.name, "$#");
        self.rename_table.update(identifier.name, new_name, identifier.id);
        identifier.name = new_name;
        identifier
    }

    pub(crate) fn simple_definition(&mut self, identifier: Identifier, rhs: Expression) -> Statement {
        // Update the type table.
        let type_ = match self.state.type_table.get(&rhs.id()) {
            Some(type_) => type_,
            None => unreachable!("Type checking guarantees that all expressions have a type."),
        };
        self.state.type_table.insert(identifier.id(), type_);
        // Update the rename table.
        self.rename_table.update(identifier.name, identifier.name, identifier.id);
        // Construct the statement.
        self.state.assigner.simple_definition(identifier, rhs, self.state.node_builder.next_id())
    }

    /// Constructs a simple assign statement for `expr` with a unique name.
    /// For example, `expr` is transformed into `$var$0 = expr;`.
    /// The lhs is guaranteed to be unique with respect to the `Assigner`.
    pub(crate) fn unique_simple_definition(&mut self, expr: Expression) -> (Identifier, Statement) {
        // Create a new variable for the expression.
        let name = self.state.assigner.unique_symbol("$var", "$");

        // Create a new identifier for the variable.
        let place = Identifier { name, span: Default::default(), id: self.state.node_builder.next_id() };

        // Construct the statement.
        let statement = self.simple_definition(place, expr);

        (place, statement)
    }
}
