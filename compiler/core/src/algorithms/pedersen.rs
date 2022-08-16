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

use crate::algorithms::{CoreFunction, BOOL_INT64_STRING_TYPES, BOOL_INT_STRING_TYPES};
use leo_ast::Type;

pub struct Pedersen64Hash;

impl CoreFunction for Pedersen64Hash {
    const NUM_ARGS: usize = 1;

    fn first_arg_types() -> &'static [Type] {
        &BOOL_INT64_STRING_TYPES
    }

    fn return_type() -> Type {
        Type::Field
    }
}

pub struct Pedersen64Commit;

impl CoreFunction for Pedersen64Commit {
    const NUM_ARGS: usize = 2;

    fn first_arg_types() -> &'static [Type] {
        &BOOL_INT64_STRING_TYPES
    }

    fn second_arg_types() -> &'static [Type] {
        &[Type::Scalar]
    }

    fn return_type() -> Type {
        Type::Group
    }
}

pub struct Pedersen128Hash;

impl CoreFunction for Pedersen128Hash {
    const NUM_ARGS: usize = 1;

    fn first_arg_types() -> &'static [Type] {
        &BOOL_INT_STRING_TYPES
    }

    fn return_type() -> Type {
        Type::Field
    }
}

pub struct Pedersen128Commit;

impl CoreFunction for Pedersen128Commit {
    const NUM_ARGS: usize = 2;

    fn first_arg_types() -> &'static [Type] {
        &BOOL_INT_STRING_TYPES
    }

    fn second_arg_types() -> &'static [Type] {
        &[Type::Scalar]
    }

    fn return_type() -> Type {
        Type::Group
    }
}
