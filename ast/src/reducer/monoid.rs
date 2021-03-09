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

//! This module contains the reducer which iterates through ast nodes - converting them into
//! asg nodes and saving relevant information.

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

pub struct VecAppend<T>(Vec<T>);

impl<T> Default for VecAppend<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T> Monoid for VecAppend<T> {
    fn append(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        VecAppend(self.0)
    }

    fn append_all(mut self, others: impl Iterator<Item = Self>) -> Self {
        let all: Vec<Vec<T>> = others.map(|x| x.0).collect();
        let total_size = all.iter().fold(0, |acc, v| acc + v.len());
        self.0.reserve(total_size);
        for item in all.into_iter() {
            self.0.extend(item);
        }
        self
    }
}

impl<T> Into<Vec<T>> for VecAppend<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
