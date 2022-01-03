// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use std::{cell::Cell, unimplemented};

use typed_arena::Arena;

use crate::{Alias, ArenaNode, AsgId, Circuit, Expression, Function, Scope, Statement, Variable};

pub struct AsgContextInner<'a> {
    pub arena: &'a Arena<ArenaNode<'a>>,
    pub next_id: Cell<AsgId>,
}

impl<'a> AsgContextInner<'a> {
    pub fn new(arena: &'a Arena<ArenaNode<'a>>) -> &'a Self {
        match arena.alloc(ArenaNode::Inner(AsgContextInner {
            arena,
            next_id: Cell::new(1.into()), // Reserve the value zero
        })) {
            ArenaNode::Inner(x) => x,
            _ => unimplemented!(),
        }
    }

    pub fn get_id(&self) -> AsgId {
        let next_id = self.next_id.get();
        self.next_id.replace(next_id + 1.into());
        next_id
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_expression(&'a self, expr: Expression<'a>) -> &'a Expression<'a> {
        match self.arena.alloc(ArenaNode::Expression(expr)) {
            ArenaNode::Expression(e) => e,
            _ => unimplemented!(),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_statement(&'a self, statement: Statement<'a>) -> &'a Statement<'a> {
        match self.arena.alloc(ArenaNode::Statement(statement)) {
            ArenaNode::Statement(e) => e,
            _ => unimplemented!(),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_variable(&'a self, variable: Variable<'a>) -> &'a Variable<'a> {
        match self.arena.alloc(ArenaNode::Variable(variable)) {
            ArenaNode::Variable(e) => e,
            _ => unimplemented!(),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_scope(&'a self, scope: Scope<'a>) -> &'a Scope<'a> {
        match self.arena.alloc(ArenaNode::Scope(Box::new(scope))) {
            ArenaNode::Scope(e) => e,
            _ => unimplemented!(),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_alias(&'a self, expr: Alias<'a>) -> &'a Alias<'a> {
        match self.arena.alloc(ArenaNode::Alias(expr)) {
            ArenaNode::Alias(e) => e,
            _ => unimplemented!(),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_circuit(&'a self, circuit: Circuit<'a>) -> &'a Circuit<'a> {
        match self.arena.alloc(ArenaNode::Circuit(circuit)) {
            ArenaNode::Circuit(e) => e,
            _ => unimplemented!(),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc_function(&'a self, function: Function<'a>) -> &'a Function<'a> {
        match self.arena.alloc(ArenaNode::Function(function)) {
            ArenaNode::Function(e) => e,
            _ => unimplemented!(),
        }
    }
}

pub type AsgContext<'a> = &'a AsgContextInner<'a>;
