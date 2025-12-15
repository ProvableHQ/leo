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

use leo_ast::{Block, Constructor, Expression, Function, Statement};
use leo_parser_lossless::SyntaxNode;

pub(crate) mod early;
pub(crate) mod late;

use crate::context::{EarlyContext, LateContext};

/// A lint pass that is to be run late in the process after the type checking and
/// symbol table info is available to the compiler. Hence, the lints in this pass
/// have access to that information from the compiler state.
/// Note: This trait is currently minimal. We can easily add new checks as we add more lints.
pub trait LateLintPass<'ctx> {
    #[expect(clippy::new_ret_no_self)]
    fn new(context: LateContext<'ctx>) -> Box<dyn LateLintPass<'ctx> + 'ctx>
    where
        Self: Sized;

    fn get_name(&self) -> &str;

    fn check_expression(&mut self, _expr: &Expression) {}
    fn check_statement(&mut self, _statement: &Statement) {}
    fn check_block(&mut self, _block: &Block) {}
    fn check_block_post(&mut self, _block: &Block) {}
    fn check_function(&mut self, _function: &Function) {}
    fn check_constructor(&mut self, _constructor: &Constructor) {}
}

/// A lint pass that is to be run early in the process before the type checking and
/// symbol table creation. Hence, it doesn't have the type information available.
/// This pass expects and works on a raw concrete syntax tree from the parser.
/// It is intented to be used for lints that cannot be performed later in the pipeline
/// due to loss of revelant information when converting to an abstract syntax tree.
pub trait EarlyLintPass<'ctx> {
    #[expect(clippy::new_ret_no_self)]
    fn new(context: EarlyContext<'ctx>) -> Box<dyn EarlyLintPass<'ctx> + 'ctx>
    where
        Self: Sized;

    fn get_name(&self) -> &str;

    fn check_node(&mut self, node: &SyntaxNode);
}
