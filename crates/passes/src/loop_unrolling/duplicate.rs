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

use leo_ast::{AstReconstructor, Block, Expression, Node as _, *};

use crate::{SymbolTable, TypeTable};

/// Duplicate this block, recursively giving new `NodeID`s to scopes and expressions,
/// and duplicating the new scopes in the `SymbolTable`.
pub fn duplicate(
    block: Block,
    symbol_table: &mut SymbolTable,
    node_builder: &NodeBuilder,
    type_table: &TypeTable,
) -> Block {
    Duplicator { symbol_table, node_builder, type_table }.reconstruct_block(block).0
}

struct Duplicator<'a> {
    symbol_table: &'a mut SymbolTable,
    node_builder: &'a NodeBuilder,
    type_table: &'a TypeTable,
}

impl Duplicator<'_> {
    fn in_scope_duped<T>(&mut self, old_id: NodeID, func: impl FnOnce(&mut Self, NodeID) -> T) -> T {
        let new_id = self.symbol_table.enter_scope_duped(old_id, self.node_builder);
        let result = func(self, new_id);
        self.symbol_table.enter_parent();
        result
    }
}

impl AstReconstructor for Duplicator<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_expression(
        &mut self,
        input: Expression,
        _additional: &Self::AdditionalInput,
    ) -> (Expression, Self::AdditionalOutput) {
        let old_id = input.id();

        // Dispatch to the appropriate specific method (mirrors the default trait implementation).
        let (mut expr, output) = match input {
            Expression::Array(e) => self.reconstruct_array(e, &()),
            Expression::ArrayAccess(e) => self.reconstruct_array_access(*e, &()),
            Expression::Async(e) => self.reconstruct_async(e, &()),
            Expression::Binary(e) => self.reconstruct_binary(*e, &()),
            Expression::Call(e) => self.reconstruct_call(*e, &()),
            Expression::Cast(e) => self.reconstruct_cast(*e, &()),
            Expression::Composite(e) => self.reconstruct_composite_init(e, &()),
            Expression::DynamicCall(e) => self.reconstruct_dynamic_call(*e, &()),
            Expression::Err(e) => self.reconstruct_err(e, &()),
            Expression::Intrinsic(e) => self.reconstruct_intrinsic(*e, &()),
            Expression::Literal(e) => self.reconstruct_literal(e, &()),
            Expression::MemberAccess(e) => self.reconstruct_member_access(*e, &()),
            Expression::Path(e) => self.reconstruct_path(e, &()),
            Expression::Repeat(e) => self.reconstruct_repeat(*e, &()),
            Expression::Ternary(e) => self.reconstruct_ternary(*e, &()),
            Expression::Tuple(e) => self.reconstruct_tuple(e, &()),
            Expression::TupleAccess(e) => self.reconstruct_tuple_access(*e, &()),
            Expression::Unary(e) => self.reconstruct_unary(*e, &()),
            Expression::Unit(e) => self.reconstruct_unit(e, &()),
        };

        // Assign a fresh node ID.
        let new_id = self.node_builder.next_id();
        expr.set_id(new_id);

        // Copy the type table entry from the old ID to the new ID.
        if let Some(ty) = self.type_table.get(&old_id) {
            self.type_table.insert(new_id, ty);
        }

        (expr, output)
    }

    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope_duped(input.id(), |slf, new_id| {
            input.id = new_id;
            input.statements = input.statements.into_iter().map(|stmt| slf.reconstruct_statement(stmt).0).collect();
            (input, Default::default())
        })
    }

    fn reconstruct_conditional(&mut self, mut input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        input.condition = self.reconstruct_expression(input.condition, &()).0;
        input.then = self.reconstruct_block(input.then).0;
        if let Some(mut otherwise) = input.otherwise {
            *otherwise = self.reconstruct_statement(*otherwise).0;
            input.otherwise = Some(otherwise);
        }

        (input.into(), Default::default())
    }

    fn reconstruct_iteration(&mut self, mut input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        self.in_scope_duped(input.id(), |slf, new_id| {
            input.id = new_id;
            input.start = slf.reconstruct_expression(input.start, &()).0;
            input.stop = slf.reconstruct_expression(input.stop, &()).0;
            input.block = slf.reconstruct_block(input.block).0;
            (input.into(), Default::default())
        })
    }
}
