// Copyright (C) 2019-2025 Provable Inc.
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

mod composite;
mod consts;
mod expressions;
mod formatter;
mod functions;
mod global_const;
mod main_entrypoint;
mod mapping;
mod module;
mod non_semantics;
mod program;
mod statements;
mod storage;
mod test_macro;
#[cfg(test)]
mod test_utils;
mod types;
mod utils;

use biome_formatter::FormatResult;

pub use formatter::*;

type Output = FormatResult<()>;
