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

use leo_span::Symbol;

use serde::{Deserialize, Serialize};

/// A future stub definition.
#[derive(Clone, Serialize, Deserialize)]
pub struct FutureStub {
    program: Symbol,
    function: Symbol,
}

impl PartialEq for FutureStub {
    fn eq(&self, other: &Self) -> bool {
        self.program == other.program && self.function == other.function
    }
}

impl Eq for FutureStub {}

impl FutureStub {
    /// Initialize a new future stub.
    pub fn new(program: Symbol, function: Symbol) -> Self {
        FutureStub { program, function }
    }

    pub fn to_key(&self) -> (Symbol, Symbol) {
        (self.program, self.function)
    }

    /// Get the program.
    pub fn program(&self) -> Symbol {
        self.program
    }

    /// Get the function.
    pub fn function(&self) -> Symbol {
        self.function
    }
}
