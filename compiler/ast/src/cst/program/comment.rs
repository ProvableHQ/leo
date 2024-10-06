// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, )]
pub enum Comment {
    CommentLine(String),
    CommentBlock(String),
    _CommentLine(String),
    _CommentBlock(String),
    None
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Comment::CommentLine(content) => write!(f, "{}", content),
            Comment::CommentBlock(content) => writeln!(f, "{}", content),
            Comment::_CommentLine(content) => write!(f, "{}", content),
            Comment::_CommentBlock(content) => writeln!(f, "{}", content),
            Comment::None => write!(f, ""),
        }
    }
}