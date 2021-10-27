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

use crate::Identifier;
use leo_errors::Span;

use serde::{Deserialize, Serialize};
use std::fmt;
use tendril::StrTendril;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Annotation {
    pub span: Span,
    pub name: Identifier,
    #[serde(with = "crate::common::vec_tendril_json")]
    pub arguments: Vec<StrTendril>,
}

const ALLOWED_ANNOTATIONS: &[&str] = &["test"];

impl Annotation {
    pub fn is_valid_annotation(&self) -> bool {
        ALLOWED_ANNOTATIONS.iter().any(|name| self.name.name.as_ref() == *name)
    }
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{:}(", self.name)?;
        for tendril in &self.arguments {
            write!(f, "{:},", tendril)?;
        }
        write!(f, ")")
    }
}
