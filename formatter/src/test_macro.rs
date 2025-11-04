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

#[macro_export]
macro_rules! impl_tests {
    ( $($test_name:ident, src = $source:expr, exp = $expected:expr, $kind:expr),* ) => {
        #[cfg(test)]
        mod test {
            use $crate::test_utils::{Kind, run_test};
            use leo_errors::Result;

            $(
                #[test]
                fn $test_name() -> Result<()> {
                    run_test($source, $expected, $kind)
                }
            )*
        }
    };
}
