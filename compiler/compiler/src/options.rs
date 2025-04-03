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

// NOTE: If compiler passes are made optional, pass preconditions and invariants may not necessarily hold true.

use std::collections::HashSet;

#[derive(Clone, Default)]
pub struct CompilerOptions {
    /// Build options.
    pub build: BuildOptions,
    /// Output options.
    pub output: OutputOptions,
}

#[derive(Clone)]
pub struct BuildOptions {
    /// Whether to enable dead code elimination.
    pub dce_enabled: bool,
    /// Max depth to type check nested conditionals.
    pub conditional_block_max_depth: usize,
    /// Whether to disable type checking for nested conditionals.
    pub disable_conditional_branch_type_checking: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self { dce_enabled: true, conditional_block_max_depth: 0, disable_conditional_branch_type_checking: false }
    }
}

#[derive(Clone, Debug)]
pub enum AstSnapshots {
    All,
    Some(HashSet<String>),
}

impl Default for AstSnapshots {
    fn default() -> Self {
        AstSnapshots::Some(Default::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct OutputOptions {
    /// Whether spans are enabled in the output ASTs.
    pub ast_spans_enabled: bool,

    pub ast_snapshots: AstSnapshots,

    pub initial_ast: bool,
}
