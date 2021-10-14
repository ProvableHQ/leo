// Copyright (C) 2019-2021 Aleo Systems Inc.
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

/// This monoid ignores all append operations
pub struct Fixed<T: Default>(pub T);

impl<T: Default> Default for Fixed<T> {
    fn default() -> Self {
        Fixed(T::default())
    }
}

impl<T: Default> Monoid for Fixed<T> {
    fn append(self, _other: Self) -> Self {
        self
    }

    fn append_all(self, _others: impl Iterator<Item = Self>) -> Self {
        self
    }
}
