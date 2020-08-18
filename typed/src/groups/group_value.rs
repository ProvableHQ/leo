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
use leo_ast::values::GroupValue as AstGroupValue;
use leo_input::values::GroupValue as InputGroupValue;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupValue {
    pub x: GroupCoordinate,
    pub y: GroupCoordinate,
    pub span: Span,
}

impl<'ast> From<AstGroupValue<'ast>> for GroupValue {
    fn from(ast_group: AstGroupValue<'ast>) -> Self {
        let ast_x = ast_group.value.x;
        let ast_y = ast_group.value.y;

        Self {
            x: GroupCoordinate::from(ast_x),
            y: GroupCoordinate::from(ast_y),
            span: Span::from(ast_group.span),
        }
    }
}

impl<'ast> From<InputGroupValue<'ast>> for GroupValue {
    fn from(ast_group: InputGroupValue<'ast>) -> Self {
        let ast_x = ast_group.value.x;
        let ast_y = ast_group.value.y;

        Self {
            x: GroupCoordinate::from(ast_x),
            y: GroupCoordinate::from(ast_y),
            span: Span::from(ast_group.span),
        }
    }
}

impl fmt::Display for GroupValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
