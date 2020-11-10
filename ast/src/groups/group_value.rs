// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{common::span::Span, groups::GroupCoordinate};
use leo_grammar::values::{
    GroupRepresentation as GrammarGroupRepresentation,
    GroupTuple as GrammarGroupTuple,
    GroupValue as GrammarGroupValue,
};
use leo_input::values::{
    GroupRepresentation as InputGroupRepresentation,
    GroupTuple as InputGroupTuple,
    GroupValue as InputGroupValue,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupValue {
    Single(String, Span),
    Tuple(GroupTuple),
}

impl GroupValue {
    pub fn set_span(&mut self, new_span: Span) {
        match self {
            GroupValue::Single(_, old_span) => *old_span = new_span,
            GroupValue::Tuple(tuple) => tuple.span = new_span,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            GroupValue::Single(_, span) => span,
            GroupValue::Tuple(tuple) => &tuple.span,
        }
    }
}

impl<'ast> From<GrammarGroupValue<'ast>> for GroupValue {
    fn from(ast_group: GrammarGroupValue) -> Self {
        let span = Span::from(ast_group.span);

        match ast_group.value {
            GrammarGroupRepresentation::Single(number) => GroupValue::Single(number.to_string(), span),
            GrammarGroupRepresentation::Tuple(tuple) => GroupValue::Tuple(GroupTuple::from(tuple)),
        }
    }
}

impl<'ast> From<InputGroupValue<'ast>> for GroupValue {
    fn from(ast_group: InputGroupValue) -> Self {
        let span = Span::from(ast_group.span);

        match ast_group.value {
            InputGroupRepresentation::Single(number) => GroupValue::Single(number.to_string(), span),
            InputGroupRepresentation::Tuple(tuple) => GroupValue::Tuple(GroupTuple::from(tuple)),
        }
    }
}

impl fmt::Display for GroupValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupValue::Single(string, _) => write!(f, "{}", string),
            GroupValue::Tuple(tuple) => write!(f, "{}", tuple),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupTuple {
    pub x: GroupCoordinate,
    pub y: GroupCoordinate,
    pub span: Span,
}

impl<'ast> From<GrammarGroupTuple<'ast>> for GroupTuple {
    fn from(ast_group: GrammarGroupTuple<'ast>) -> Self {
        let ast_x = ast_group.x;
        let ast_y = ast_group.y;

        Self {
            x: GroupCoordinate::from(ast_x),
            y: GroupCoordinate::from(ast_y),
            span: Span::from(ast_group.span),
        }
    }
}

impl<'ast> From<InputGroupTuple<'ast>> for GroupTuple {
    fn from(ast_group: InputGroupTuple<'ast>) -> Self {
        let ast_x = ast_group.x;
        let ast_y = ast_group.y;

        Self {
            x: GroupCoordinate::from(ast_x),
            y: GroupCoordinate::from(ast_y),
            span: Span::from(ast_group.span),
        }
    }
}

impl fmt::Display for GroupTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
