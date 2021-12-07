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

use crate::dropless::DroplessArena;

use core::num::NonZeroU32;
use core::str;
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use std::intrinsics::transmute;

macro_rules! consts {
    ([$sym:ident, $($rest:ident),*], $val: expr) => {
        consts!([$sym], $val);
        consts!([$($rest),*], $val + 1);
    };
    ([$sym:ident], $val: expr) => {
        #[allow(non_upper_case_globals)]
        pub const $sym: $crate::symbol::Symbol = $crate::symbol::Symbol::new($val);
    };
}

macro_rules! symbols {
    ($($sym:ident,)*) => {
        const PRE_DEFINED: &[&str] = &[$(stringify!($sym)),*];

        pub mod sym {
            consts!([$($sym),*], 0);
        }
    };
}

symbols! {
    foo,
    bar,
}

/// An interned string.
///
/// Represented as an index internally, with all operations based on that.
/// A `Symbol` reserves the value `0`, so that `Option<Symbol>` only takes up 4 bytes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(NonZeroU32);

impl Symbol {
    /// Returns the corresponding `Symbol` for the given `index`.
    pub const fn new(index: u32) -> Self {
        let index = index.saturating_add(1);
        // SAFETY: per above addition, we know `index > 0` always applies.
        Self(unsafe { NonZeroU32::new_unchecked(index) })
    }

    /// Converts this symbol to the raw index.
    pub const fn as_u32(self) -> u32 {
        self.0.get() - 1
    }
}

/// An interner for strings
pub struct Interner {
    /// Arena used to allocate the strings, giving us `&'static str`s from it.
    arena: DroplessArena,
    /// Registration of strings and symbol index allocation is done in this set.
    set: IndexSet<&'static str, FxBuildHasher>,
}

impl Interner {
    /// Returns an interner prefilled with commonly used strings in Leo.
    pub fn prefilled() -> Self {
        Self::prefill(PRE_DEFINED)
    }

    /// Returns an interner prefilled with `init`.
    fn prefill(init: &[&'static str]) -> Self {
        Interner {
            arena: <_>::default(),
            set: init.iter().copied().collect(),
        }
    }

    /// Interns `string`, returning a `Symbol` corresponding to it.
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(sym) = self.set.get_index_of(string) {
            // Already internet, return that symbol.
            return Symbol::new(sym as u32);
        }

        // SAFETY: `from_utf8_unchecked` is safe since we just allocated a `&str`,
        // which is known to be UTF-8.
        let bytes = self.arena.alloc_slice(string.as_bytes());
        let string: &str = unsafe { str::from_utf8_unchecked(bytes) };

        unsafe fn transmute_lt<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
            transmute(x)
        }

        // SAFETY: Extending to `'static` is fine. Accesses only happen while the arena is alive.
        let string: &'static _ = unsafe { transmute_lt(string) };

        Symbol::new(self.set.insert_full(string).1 as u32)
    }

    /// Returns the corresponding string for the given symbol.
    pub fn get(&self, symbol: Symbol) -> &str {
        self.set.get_index(symbol.as_u32() as usize).unwrap()
    }
}
