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

use std::io;

#[derive(Debug, Error)]
pub enum LockFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("`{}` creating: {}", _0, _1)]
    Creating(&'static str, io::Error),

    #[error("`{}` metadata: {}", _0, _1)]
    Metadata(&'static str, io::Error),

    #[error("`{}` opening: {}", _0, _1)]
    Opening(&'static str, io::Error),

    #[error("`{}` parsing: {}", _0, _1)]
    Parsing(&'static str, toml::de::Error),

    #[error("`{}` reading: {}", _0, _1)]
    Reading(&'static str, io::Error),

    #[error("`{}` writing: {}", _0, _1)]
    Writing(&'static str, io::Error),
}

impl From<toml::ser::Error> for LockFileError {
    fn from(error: toml::ser::Error) -> Self {
        LockFileError::Crate("toml", error.to_string())
    }
}
