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

use crate::TestTypeInference;

#[test]
fn test_empty_array() {
    let bytes = include_bytes!("empty_array.leo");

    let check = TestTypeInference::new(bytes);

    check.expect_error();
}

#[test]
fn test_invalid_array_access() {
    let bytes = include_bytes!("invalid_array_access.leo");

    let check = TestTypeInference::new(bytes);

    check.expect_error();
}

#[test]
fn test_invalid_spread() {
    let bytes = include_bytes!("invalid_spread.leo");

    let check = TestTypeInference::new(bytes);

    check.expect_error();
}
