// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub mod definition;
pub use definition::*;

pub mod input_ast;
pub use input_ast::*;

pub mod input_value;
pub use input_value::*;

pub mod program_input;
pub use program_input::*;

pub mod section;
pub use section::*;

use indexmap::IndexMap;
use leo_errors::{InputError, LeoError, Result};
use leo_span::{sym, Span, Symbol};
use serde::{Deserialize, Serialize};

type Definitions = IndexMap<Symbol, InputValue>;
