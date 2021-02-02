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

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitInitExpression {
    pub name: Identifier,
    pub members: Vec<CircuitVariableDefinition>,
    pub span: Span,
}

impl fmt::Display for CircuitInitExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {{", self.name)?;
        for (i, member) in self.members.iter().enumerate() {
            write!(f, "{}: {}", member.identifier, member.expression)?;
            if i < self.members.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl Node for CircuitInitExpression {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
