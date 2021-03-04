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

use crate::ast::Rule;
use crate::common::MutSelfKeyword;
use crate::common::SelfKeyword;
use crate::functions::FunctionInput;
use crate::functions::InputKeyword;

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::input))]
pub enum Input<'ast> {
    InputKeyword(InputKeyword<'ast>),
    SelfKeyword(SelfKeyword<'ast>),
    MutSelfKeyword(MutSelfKeyword<'ast>),
    FunctionInput(FunctionInput<'ast>),
}
