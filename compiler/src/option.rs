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

pub const DEFAULT_INLINE_LIMIT: u32 = 1000;

///
/// Toggles compiler optimizations on the program.
///
#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub constant_folding_enabled: bool,
    pub dead_code_elimination_enabled: bool,
    pub inline_limit: u32,
}

impl Default for CompilerOptions {
    ///
    /// All compiler optimizations are enabled by default.
    ///
    fn default() -> Self {
        CompilerOptions {
            constant_folding_enabled: true,
            dead_code_elimination_enabled: true,
            inline_limit: DEFAULT_INLINE_LIMIT,
        }
    }
}

#[derive(Clone, Default)]
pub struct OutputOptions {
    pub spans_enabled: bool,
    pub ast_initial: bool,
    pub ast_imports_resolved: bool,
    pub ast_canonicalized: bool,
    pub ast_type_inferenced: bool,
    pub asg_initial: bool,
    pub asg_constants_folded: bool,
    pub asg_dead_code_eliminated: bool,
    pub asg_exclude_edges: Vec<Box<str>>,
    pub asg_exclude_labels: Vec<Box<str>>,
    pub emit_ir: bool,
}
