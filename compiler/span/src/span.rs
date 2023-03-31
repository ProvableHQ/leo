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

//! Defines the `Span` type used to track where code comes from.

use core::ops::{Add, Sub};
use serde::{Deserialize, Serialize};
use std::{fmt, usize};

use crate::symbol::with_session_globals;

/// The span type which tracks where formatted errors originate from in a Leo file.
/// This is used in many spots throughout the rest of the Leo crates.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Span {
    /// The start (low) position of the span, inclusive.
    pub lo: BytePos,
    /// The end (high) position of the span, exclusive.
    /// The length of the span is `hi - lo`.
    pub hi: BytePos,
}

impl Span {
    /// Generate a new span from the `start`ing and `end`ing positions.
    pub fn new(start: BytePos, end: BytePos) -> Self {
        Self { lo: start, hi: end }
    }

    /// Generates a dummy span with all defaults.
    /// Should only be used in temporary situations.
    pub const fn dummy() -> Self {
        Self { lo: BytePos(0), hi: BytePos(0) }
    }

    /// Is the span a dummy?
    pub fn is_dummy(&self) -> bool {
        self == &Self::dummy()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_session_globals(|s| write!(f, "{}", s.source_map.span_to_string(*self)))
    }
}

impl std::ops::Add for &Span {
    type Output = Span;

    /// Add two spans (by reference) together.
    fn add(self, other: &Span) -> Span {
        *self + *other
    }
}

impl std::ops::Add for Span {
    type Output = Self;

    /// Add two spans together.
    /// The resulting span is the smallest span that includes both.
    fn add(self, other: Self) -> Self {
        let lo = self.lo.min(other.lo);
        let hi = self.hi.max(other.hi);
        Self::new(lo, hi)
    }
}

// _____________________________________________________________________________
// Pos, BytePos, CharPos
//

/// Offsets (i.e. positions), in some units (e.g. bytes or characters),
/// with conversions between unsigned integers.
pub trait Pos {
    fn from_usize(n: usize) -> Self;
    fn to_usize(&self) -> usize;
    fn from_u32(n: u32) -> Self;
    fn to_u32(&self) -> u32;
}

/// Generate one-component tuple structs that implement the [`Pos`] trait.
macro_rules! impl_pos {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
        )*
    ) => {
        $(
            $(#[$attr])*
            $vis struct $ident($inner_vis $inner_ty);

            impl Pos for $ident {
                #[inline(always)]
                fn from_usize(n: usize) -> $ident {
                    $ident(n as $inner_ty)
                }

                #[inline(always)]
                fn to_usize(&self) -> usize {
                    self.0 as usize
                }

                #[inline(always)]
                fn from_u32(n: u32) -> $ident {
                    $ident(n as $inner_ty)
                }

                #[inline(always)]
                fn to_u32(&self) -> u32 {
                    self.0 as u32
                }
            }

            impl Add for $ident {
                type Output = $ident;

                #[inline(always)]
                fn add(self, rhs: $ident) -> $ident {
                    $ident(self.0 + rhs.0)
                }
            }

            impl Sub for $ident {
                type Output = $ident;

                #[inline(always)]
                fn sub(self, rhs: $ident) -> $ident {
                    $ident(self.0 - rhs.0)
                }
            }
        )*
    };
}

impl_pos! {
    /// A byte offset.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize, Default)]
    pub struct BytePos(pub u32);

    /// A character offset.
    ///
    /// Because of multibyte UTF-8 characters,
    /// a byte offset is not equivalent to a character offset.
    /// The [`SourceMap`] will convert [`BytePos`] values to `CharPos` values as necessary.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct CharPos(pub usize);
}
