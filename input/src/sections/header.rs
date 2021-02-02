// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{
    ast::Rule,
    common::Identifier,
    sections::{Main, Record, Registers, State, StateLeaf},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::header))]
pub enum Header<'ast> {
    Main(Main<'ast>),
    Record(Record<'ast>),
    Registers(Registers<'ast>),
    State(State<'ast>),
    StateLeaf(StateLeaf<'ast>),
    Identifier(Identifier<'ast>),
}

impl<'ast> Header<'ast> {
    pub fn span(self) -> Span<'ast> {
        match self {
            Header::Main(main) => main.span,
            Header::Record(record) => record.span,
            Header::Registers(registers) => registers.span,
            Header::State(state) => state.span,
            Header::StateLeaf(state_leaf) => state_leaf.span,
            Header::Identifier(identifier) => identifier.span,
        }
    }
}

impl<'ast> fmt::Display for Header<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::Main(_main) => write!(f, "main"),
            Header::Record(_record) => write!(f, "record"),
            Header::Registers(_registers) => write!(f, "registers"),
            Header::State(_state) => write!(f, "state"),
            Header::StateLeaf(_state_leaf) => write!(f, "state_leaf"),
            Header::Identifier(identifier) => write!(f, "{}", identifier.value),
        }
    }
}
