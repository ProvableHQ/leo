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

#[derive(Clone, Default)]
pub struct OutputOptions {
    /// Whether spans are enabled in the output ASTs.
    pub spans_enabled: bool,
    /// If enabled writes the AST after parsing.
    pub initial_ast: bool,
    /// If enabled writes the input AST after parsing.
    pub initial_input_ast: bool,
    /// If enabled writes the AST after function inlining.
    pub inlined_ast: bool,
    /// If enabled writes the AST after loop unrolling.
    pub unrolled_ast: bool,
    /// If enabled writes the AST after static single assignment.
    pub ssa_ast: bool,
}
