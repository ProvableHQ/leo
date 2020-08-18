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

use crate::{FormattedString, MacroName, Span};
use leo_ast::macros::FormattedMacro as AstFormattedMacro;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedMacro {
    pub name: MacroName,
    pub string: Option<FormattedString>,
    pub span: Span,
}

impl<'ast> From<AstFormattedMacro<'ast>> for FormattedMacro {
    fn from(formatted: AstFormattedMacro<'ast>) -> Self {
        Self {
            name: MacroName::from(formatted.name),
            string: formatted.string.map(|string| FormattedString::from(string)),
            span: Span::from(formatted.span),
        }
    }
}

impl fmt::Display for FormattedMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}!({});",
            self.name,
            self.string.as_ref().map(|s| s.to_string()).unwrap_or("".to_string()),
        )
    }
}
