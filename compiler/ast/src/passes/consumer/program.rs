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

use crate::*;

/// A Consumer trait for functions in the AST.
pub trait FunctionConsumer {
    type Output;

    fn consume_function(&mut self, input: Function) -> Self::Output;
}

/// A Consumer trait for structs in the AST.
pub trait StructConsumer {
    type Output;

    fn consume_struct(&mut self, input: Struct) -> Self::Output;
}

/// A Consumer trait for imported programs in the AST.
pub trait ImportConsumer {
    type Output;

    fn consume_import(&mut self, input: Program) -> Self::Output;
}

/// A Consumer trait for mappings in the AST.
pub trait MappingConsumer {
    type Output;

    fn consume_mapping(&mut self, input: Mapping) -> Self::Output;
}

/// A Consumer trait for program scopes in the AST.
pub trait ProgramScopeConsumer {
    type Output;

    fn consume_program_scope(&mut self, input: ProgramScope) -> Self::Output;
}

/// A Consumer trait for the program represented by the AST.
pub trait ProgramConsumer {
    type Output;
    fn consume_program(&mut self, input: Program) -> Self::Output;
}
