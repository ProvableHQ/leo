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

use crate::algorithms::CoreFunction;
use leo_ast::Type;

pub struct Poseidon2Hash;

impl CoreFunction for Poseidon2Hash {
    const NUM_ARGS: usize = 1;

    fn first_arg_is_allowed_type(type_: &Type) -> bool {
        !matches!(type_, Type::Mapping(_) | Type::Tuple(_) | Type::Err | Type::Unit)
    }

    fn return_type() -> Type {
        Type::Field
    }
}

pub struct Poseidon4Hash;

impl CoreFunction for Poseidon4Hash {
    const NUM_ARGS: usize = 1;

    fn first_arg_is_allowed_type(type_: &Type) -> bool {
        !matches!(type_, Type::Mapping(_) | Type::Tuple(_) | Type::Err | Type::Unit)
    }

    fn return_type() -> Type {
        Type::Field
    }
}

pub struct Poseidon8Hash;

impl CoreFunction for Poseidon8Hash {
    const NUM_ARGS: usize = 1;

    fn first_arg_is_allowed_type(type_: &Type) -> bool {
        !matches!(type_, Type::Mapping(_) | Type::Tuple(_) | Type::Err | Type::Unit)
    }

    fn return_type() -> Type {
        Type::Field
    }
}
