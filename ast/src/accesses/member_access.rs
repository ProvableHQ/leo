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

use crate::{Expression, Identifier, Node};
use leo_span::Span;

use std::fmt;

use serde::{Deserialize, Serialize};

/// A field access expression `inner.name` to some structure with *named fields*.
///
/// For accesses to a positional fields in e.g., a tuple, see `TupleAccess`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberAccess {
    /// The structure that the field `name` is being extracted from.
    pub inner: Box<Expression>,
    /// The name of the field to extract in `inner`.
    pub name: Identifier,
    /// The span covering all of `inner.name`.
    pub span: Span,
    // FIXME(Centril): Type information shouldn't be injected into an AST,
    // so this field should eventually be removed.
    pub type_: Option<crate::Type>,
}

impl fmt::Display for MemberAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.inner, self.name)
    }
}

impl Node for MemberAccess {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
