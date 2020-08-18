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

use crate::{assert_satisfied, import::set_local_dir, parse_program};

#[test]
fn test_out_of_order() {
    let program_bytes = include_bytes!("out_of_order.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_out_of_order_with_import() {
    set_local_dir();

    let program_bytes = include_bytes!("out_of_order_with_import.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}
