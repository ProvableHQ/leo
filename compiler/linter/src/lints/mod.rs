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

mod binary;
mod duplicate_imports;
mod semantics;

mod unary;
mod unnecessary_braces;
mod unnecessary_paranthesis;
mod zero_prefixed_literal;

use binary::*;
use duplicate_imports::*;
use semantics::*;

use unary::*;
use unnecessary_braces::*;
use unnecessary_paranthesis::*;
use zero_prefixed_literal::*;

use crate::{
    context::{EarlyContext, LateContext},
    passes::*,
};

pub(crate) fn get_early_lints<'ctx>(context: EarlyContext<'ctx>) -> Vec<Box<dyn EarlyLintPass<'ctx> + 'ctx>> {
    vec![
        DuplicateImportsLint::new(context),
        UnnecessaryBraces::new(context),
        UnnecessaryParens::new(context),
        ZeroPrefixedLiteral::new(context),
    ]
}

pub(crate) fn get_late_lints<'ctx>(context: LateContext<'ctx>) -> Vec<Box<dyn LateLintPass<'ctx> + 'ctx>> {
    vec![BinaryLints::new(context), SemanticLinter::new(context), UnaryLints::new(context)]
}
