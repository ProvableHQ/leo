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

pub trait IntegerTester {
    /// Tests defining the smalled value that can be represented by the integer type
    fn test_min();

    /// Tests defining the largest value that can be represented by the integer type
    fn test_max();

    /// Tests a non-wrapping addition
    fn test_add();

    /// Tests a non-wrapping subtraction
    fn test_sub();

    /// Tests a non-wrapping multiplication
    fn test_mul();

    /// Tests a non-wrapping division
    fn test_div();

    /// Tests a non-wrapping exponentiation
    fn test_pow();

    /// Tests == evaluation
    fn test_eq();

    /// Tests != evaluation
    fn test_ne();

    /// Tests >= evaluation
    fn test_ge();

    /// Tests > evaluation
    fn test_gt();

    /// Tests <= evaluation
    fn test_le();

    /// Tests < evaluation
    fn test_lt();

    /// Test assert equals constraint keyword
    fn test_console_assert();

    /// Test ternary if bool ? num_1 : num_2;
    fn test_ternary();
}
