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

use super::*;

#[derive(Deserialize, PartialEq, Eq, Serialize)]
pub struct CompileOutput {
    pub initial_symbol_table: String,
    pub type_checked_symbol_table: String,
    pub unrolled_symbol_table: String,
    pub initial_ast: String,
    pub unrolled_ast: String,
    pub ssa_ast: String,
    pub flattened_ast: String,
    pub destructured_ast: String,
    pub inlined_ast: String,
    pub dce_ast: String,
    pub bytecode: String,
    pub errors: String,
    pub warnings: String,
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
pub struct ExecuteOutput {
    pub execution: Option<Execution<CurrentNetwork>>,
    pub verified: bool,
    pub status: String,
    pub errors: String,
    pub warnings: String,
}
