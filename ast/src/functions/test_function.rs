// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{Function, Identifier};
use leo_grammar::functions::TestFunction as GrammarTestFunction;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestFunction {
    pub function: Function,
    pub input_file: Option<Identifier>,
}

impl<'ast> From<GrammarTestFunction<'ast>> for TestFunction {
    fn from(test: GrammarTestFunction) -> Self {
        TestFunction {
            function: Function::from(test.function),
            input_file: None, // pass custom input file with `@context` annotation
        }
    }
}
