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

mod vec_append;
pub use vec_append::*;

mod set_append;
pub use set_append::*;

mod bool_and;
pub use bool_and::*;

pub trait Monoid: Default {
    fn append(self, other: Self) -> Self;

    fn append_all(self, others: impl Iterator<Item = Self>) -> Self {
        let mut current = self;
        for item in others {
            current = current.append(item);
        }
        current
    }

    fn append_option(self, other: Option<Self>) -> Self {
        match other {
            None => self,
            Some(other) => self.append(other),
        }
    }
}
