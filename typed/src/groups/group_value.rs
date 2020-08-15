use crate::{common::span::Span, groups::GroupCoordinate};
use leo_ast::values::GroupValue as AstGroupValue;

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

impl fmt::Display for GroupValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
