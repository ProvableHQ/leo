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

use num_traits::One;
use std::{fmt::Display, ops::Add};

use leo_ast::Value;
use leo_errors::LeoError;

// TODO: Consider the sealing this trait.
// TODO: Better name.
/// A trait for whose implementors are concrete values for loop bounds.
pub(crate) trait LoopBound:
    Add<Output = Self> + Copy + Display + One + PartialOrd + TryFrom<Value, Error = LeoError>
{
}

impl LoopBound for i128 {}
impl LoopBound for u128 {}

/// Whether or not a bound is inclusive or exclusive.
pub(crate) enum Clusivity {
    Inclusive,
    Exclusive,
}

/// An iterator over a range of values.
pub(crate) struct RangeIterator<I: LoopBound> {
    end: I,
    current: Option<I>,
    clusivity: Clusivity,
}

impl<I: LoopBound> RangeIterator<I> {
    pub(crate) fn new(start: I, end: I, clusivity: Clusivity) -> Self {
        Self { end, current: Some(start), clusivity }
    }
}

impl<I: LoopBound> Iterator for RangeIterator<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(value) if value < self.end => {
                self.current = Some(value.add(I::one()));
                Some(value)
            }
            Some(value) => {
                self.current = None;
                match self.clusivity {
                    Clusivity::Exclusive => None,
                    Clusivity::Inclusive => Some(value),
                }
            }
        }
    }
}
