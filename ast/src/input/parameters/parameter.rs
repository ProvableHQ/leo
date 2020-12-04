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

use crate::{Identifier, Span, Type};
use leo_input::parameters::Parameter as GrammarParameter;

use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub variable: Identifier,
    pub type_: Type,
    pub span: Span,
}

impl<'ast> From<GrammarParameter<'ast>> for Parameter {
    fn from(parameter: GrammarParameter<'ast>) -> Self {
        Self {
            variable: Identifier::from(parameter.variable),
            type_: Type::from(parameter.type_),
            span: Span::from(parameter.span),
        }
    }
}

impl Ord for Parameter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.variable.cmp(&other.variable)
    }
}

impl PartialOrd for Parameter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
