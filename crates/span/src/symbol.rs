// Copyright (C) 2019-2026 Provable Inc.
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

use crate::source_map::SourceMap;

use core::{
    borrow::Borrow,
    cmp::PartialEq,
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU32,
    ops::Deref,
    str,
};
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;

// Predefined symbols generated at build time.
include!(concat!(env!("OUT_DIR"), "/symbols_generated.rs"));

/// An interned string.
///
/// Represented as an index internally, with all operations based on that.
/// A `Symbol` reserves the value `0`, so that `Option<Symbol>` only takes up 4 bytes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Symbol(
    #[serde(deserialize_with = "Symbol::serde_to_symbol")]
    #[serde(serialize_with = "Symbol::serde_from_symbol")]
    NonZeroU32,
);

impl Default for Symbol {
    fn default() -> Self {
        Symbol(NonZeroU32::MIN)
    }
}

impl Symbol {
    /// Returns the corresponding `Symbol` for the given `index`.
    pub const fn new(index: u32) -> Self {
        let index = index.saturating_add(1);
        Self(match NonZeroU32::new(index) {
            None => unreachable!(),
            Some(x) => x,
        })
    }

    /// Maps a string to its interned representation.
    pub fn intern(string: &str) -> Self {
        with_session_globals(|session_globals| session_globals.symbol_interner.intern(string))
    }

    /// Convert to effectively a `&'static str` given the `SessionGlobals`.
    pub fn as_str<R>(self, s: &SessionGlobals, with: impl FnOnce(&str) -> R) -> R {
        s.symbol_interner.get(self, with)
    }

    /// Converts this symbol to the raw index.
    pub const fn as_u32(self) -> u32 {
        self.0.get() - 1
    }

    fn serde_to_symbol<'de, D: Deserializer<'de>>(de: D) -> Result<NonZeroU32, D::Error> {
        Ok(Symbol::intern(<&str>::deserialize(de)?).0)
    }

    fn serde_from_symbol<S: Serializer>(index: &NonZeroU32, ser: S) -> Result<S::Ok, S::Error> {
        with_session_globals(|sg| Self(*index).as_str(sg, |s| ser.serialize_str(s)))
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_session_globals(|s| self.as_str(s, |s| fmt::Debug::fmt(s, f)))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_session_globals(|s| self.as_str(s, |s| fmt::Display::fmt(s, f)))
    }
}

/// All the globals for a compiler sessions.
pub struct SessionGlobals {
    /// The interner for `Symbol`s used in the compiler.
    symbol_interner: Interner,
    /// The source map used in the compiler.
    pub source_map: SourceMap,
}

impl Default for SessionGlobals {
    fn default() -> Self {
        Self { symbol_interner: Interner::prefilled(), source_map: SourceMap::default() }
    }
}

scoped_tls::scoped_thread_local!(pub static SESSION_GLOBALS: SessionGlobals);

/// Creates the session globals and then runs the closure `f`.
#[inline]
pub fn create_session_if_not_set_then<R>(f: impl FnOnce(&SessionGlobals) -> R) -> R {
    if !SESSION_GLOBALS.is_set() {
        let sg = SessionGlobals::default();
        SESSION_GLOBALS.set(&sg, || SESSION_GLOBALS.with(f))
    } else {
        SESSION_GLOBALS.with(f)
    }
}

/// Gives access to read or modify the session globals in `f`.
#[inline]
pub fn with_session_globals<R>(f: impl FnOnce(&SessionGlobals) -> R) -> R {
    SESSION_GLOBALS.with(f)
}

/// An interned string,
/// either prefilled "at compile time" (`Static`),
/// or created at runtime (`Owned`).
#[derive(Eq)]
enum InternedStr {
    /// String is stored "at compile time", i.e. prefilled.
    Static(&'static str),
    /// String is constructed and stored during runtime.
    Owned(Box<str>),
}

impl Borrow<str> for InternedStr {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl Deref for InternedStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s,
        }
    }
}

impl PartialEq for InternedStr {
    fn eq(&self, other: &InternedStr) -> bool {
        self.deref() == other.deref()
    }
}

impl Hash for InternedStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

/// The inner interner.
/// This construction is used to get interior mutability in `Interner`.
struct InnerInterner {
    // /// Arena used to allocate the strings, giving us `&'static str`s from it.
    // arena: DroplessArena,
    /// Registration of strings and symbol index allocation is done in this set.
    set: IndexSet<InternedStr, FxBuildHasher>,
}

/// A symbol-to-string interner.
struct Interner {
    inner: RefCell<InnerInterner>,
}

impl Interner {
    /// Returns an interner prefilled with commonly used strings in Leo.
    fn prefilled() -> Self {
        Self::prefill(PRE_DEFINED)
    }

    /// Returns an interner prefilled with `init`.
    fn prefill(init: &[&'static str]) -> Self {
        let inner = InnerInterner {
            // arena: <_>::default(),
            set: init.iter().copied().map(InternedStr::Static).collect(),
        };
        Self { inner: RefCell::new(inner) }
    }

    /// Interns `string`, returning a `Symbol` corresponding to it.
    fn intern(&self, string: &str) -> Symbol {
        let InnerInterner { set } = &mut *self.inner.borrow_mut();

        if let Some(sym) = set.get_index_of(string) {
            // Already interned, return that symbol.
            return Symbol::new(sym as u32);
        }

        Symbol::new(set.insert_full(InternedStr::Owned(string.into())).0 as u32)
    }

    /// Returns the corresponding string for the given symbol.
    fn get<R>(&self, symbol: Symbol, with: impl FnOnce(&str) -> R) -> R {
        let set = &self.inner.borrow().set;
        with(set.get_index(symbol.as_u32() as usize).unwrap())
    }
}
