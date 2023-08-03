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

use super::*;

/// The example programs that can be generated.
pub enum Example {
    Lottery,
    TicTacToe,
    Token,
}

impl Example {
    pub fn main_file_string(&self, package_name: &str) -> String {
        match self {
            Self::Lottery => include_str!("../../examples/lottery/src/main.leo").to_string(),
            Self::TicTacToe => include_str!("../../examples/tic_tac_toe/src/main.leo").to_string(),
            Self::Token => include_str!("../../examples/token/src/main.leo").to_string(),
        }
    }

    pub fn input_file_string(&self, package_name: &str) -> String {
        match self {
            Self::Lottery => include_str!("../../examples/lottery/inputs/input.in").to_string(),
            Self::TicTacToe => include_str!("../../examples/tic_tac_toe/inputs/input.in").to_string(),
            Self::Token => include_str!("../../examples/token/inputs/input.in").to_string(),
        }
    }
}
