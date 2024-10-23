// Copyright (C) 2019-2024 Aleo Systems Inc.
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

pub struct UserData<'a> {
    pub code: &'a str,
    pub highlight: Option<(usize, usize)>,
    pub message: &'a str,
    pub futures: &'a [String],
    pub watchpoints: &'a [String],
    pub result: Option<&'a str>,
}

pub trait Ui {
    fn display_user_data(&mut self, data: &UserData<'_>);

    fn receive_user_input(&mut self) -> String;
}
