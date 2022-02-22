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

use core::cmp::PartialEq;
use core::convert::AsRef;
use core::num::NonZeroU32;
use core::ops::Deref;
use core::{fmt, str};
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::intrinsics::transmute;
use std::marker::PhantomData;

/// A helper for `symbols` defined below.
/// The macro's job is to bind conveniently  usable `const` items to the symbol names provided.
/// For example, with `symbol { a, b }` you'd have `sym::a` and `sym::b`.
macro_rules! consts {
    ($val: expr, $sym:ident $(,)?) => {
        #[allow(non_upper_case_globals)]
        pub const $sym: $crate::symbol::Symbol = $crate::symbol::Symbol::new($val);
    };
    ($val: expr, $sym:ident: $_s:literal $(,)?) => {
        consts!($val, $sym);
    };
    ($val: expr, $sym:ident: $_s:literal, $($rest:tt)*) => {
        consts!($val, $sym);
        consts!($val + 1, $($rest)*);
    };
    ($val: expr, $sym:ident, $($rest:tt)*) => {
        consts!($val, $sym);
        consts!($val + 1, $($rest)*);
    };
}

/// A helper for `symbols` defined below.
/// The macro's job is to merge all the hard coded strings into a single array of strings.
/// The strategy applied is [push-down accumulation](https://danielkeep.github.io/tlborm/book/pat-push-down-accumulation.html).
macro_rules! strings {
    // Final step 0) in the push-down accumulation.
    // Here, the actual strings gathered in `$acc` are made into an array.
    // We have to use this approach because e.g., `$e1 , $e2` is not recognized by any syntactic
    // category in Rust, so a macro cannot produce it.
    ([$($acc:expr),*] []) => {
        [$($acc),*]
    };
    // Recursive case 1): Handles e.g., `x: "frodo"` and `y: "sam"`
    // in `symbols! { x: "frodo", y: "sam", z }`.
    ([$($acc:expr),*] [$_sym:ident: $string:literal, $($rest:tt)*]) => {
        strings!([$($acc,)* $string] [$($rest)*])
    };
    // Recursive case 2): Handles e.g., `x` and `y` in `symbols! { x, y, z }`.
    ([$($acc:expr),*] [$sym:ident, $($rest:tt)*]) => {
        strings!([$($acc,)* stringify!($sym)] [$($rest)*])
    };
    // Base case 3): As below, but with a specified string, e.g., `symbols! { x, y: "gandalf" }`.
    ([$($acc:expr),*] [$_sym:ident: $string:literal $(,)?]) => {
        strings!([$($acc,)* $string] [])
    };
    // Base case 4): A single identitifier left.
    // So in e.g., `symbols! { x, y }`, this handles `y` with `x` already in `$acc`.
    ([$($acc:expr),*] [$sym:ident $(,)?]) => {
        strings!([$($acc,)* stringify!($sym)] [])
    };
}

/// Creates predefined symbols used throughout the Leo compiler and language.
/// Broadly speaking, any hard coded string in the compiler should be defined here.
///
/// The macro accepts symbols separated by commas,
/// and a symbol is either specified as a Rust identifier, in which case it is `stringify!`ed,
/// or as `ident: "string"` where `"string"` is the actual hard coded string.
/// The latter case can be used when the hard coded string is not a valid identifier.
/// In either case, a `const $ident: Symbol` will be created that you can access as `sym::$ident`.
macro_rules! symbols {
    ($($symbols:tt)*) => {
        const PRE_DEFINED: &[&str] = &strings!([] [$($symbols)*]);

        pub mod sym {
            consts!(0, $($symbols)*);
        }
    };
}

symbols! {
    address,
    AlwaysConst,
    Ampersand: "@",
    array,
    As: "as",
    assert,
    bool,
    char,
    circuit,
    Class: "class",
    context,
    CoreFunction,
    console,
    Const: "const",
    Else: "else",
    error,
    False: "false",
    field,
    For: "for",
    function,
    group,
    i8,
    i16,
    i32,
    i64,
    i128,
    If: "if",
    import,
    In: "in",
    input,
    Let: "let",
    log,
    main,
    Mut: "mut",
    prelude,
    Return: "return",
    SelfLower: "self",
    SelfUpper: "Self",
    Star: "*",
    std,
    Static: "static",
    Struct: "struct",
    test,
    True: "true",
    Type: "type",
    u8,
    u16,
    u32,
    u64,
    u128,

    CONTAINER_PSEUDO_CIRCUIT: "$InputContainer",
    REGISTERS_PSEUDO_CIRCUIT: "$InputRegister",
    RECORD_PSEUDO_CIRCUIT: "$InputRecord",
    STATE_PSEUDO_CIRCUIT: "$InputState",
    STATE_LEAF_PSEUDO_CIRCUIT: "$InputStateLeaf",

    registers,
    record,
    state,
    state_leaf,
}

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

impl Symbol {
    /// Returns the corresponding `Symbol` for the given `index`.
    pub const fn new(index: u32) -> Self {
        let index = index.saturating_add(1);
        // SAFETY: per above addition, we know `index > 0` always applies.
        Self(unsafe { NonZeroU32::new_unchecked(index) })
    }

    /// Maps a string to its interned representation.
    pub fn intern(string: &str) -> Self {
        with_session_globals(|session_globals| session_globals.symbol_interner.intern(string))
    }

    /// Convert to effectively a `&'static str`.
    /// This is a slowish operation because it requires locking the symbol interner.
    pub fn as_str(self) -> SymbolStr {
        with_session_globals(|session_globals| {
            let symbol_str = session_globals.symbol_interner.get(self);
            SymbolStr::new(unsafe { std::mem::transmute::<&str, &str>(symbol_str) })
        })
    }

    /// Converts this symbol to the raw index.
    pub const fn as_u32(self) -> u32 {
        self.0.get() - 1
    }

    fn serde_to_symbol<'de, D: Deserializer<'de>>(de: D) -> Result<NonZeroU32, D::Error> {
        Ok(Symbol::intern(<&str>::deserialize(de)?).0)
    }

    fn serde_from_symbol<S: Serializer>(index: &NonZeroU32, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&Self(*index).as_str())
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), f)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), f)
    }
}

/// An alternative to [`Symbol`], useful when the chars within the symbol need to
/// be accessed. It deliberately has limited functionality and should only be
/// used for temporary values.
///
/// Because the interner outlives any thread which uses this type, we can
/// safely treat `string` which points to interner data, as an immortal string,
/// as long as this type never crosses between threads.
#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct SymbolStr {
    string: &'static str,
    /// Ensures the type is neither `Sync` nor `Send`,
    /// so that we satisfy "never crosses between threads" per above.
    not_sync_send: PhantomData<*mut ()>,
}

impl SymbolStr {
    /// Create a `SymbolStr` from a `&'static str`.
    pub fn new(string: &'static str) -> Self {
        Self {
            string,
            not_sync_send: PhantomData,
        }
    }
}

// This impl allows a `SymbolStr` to be directly equated with a `String` or `&str`.
impl<T: Deref<Target = str>> PartialEq<T> for SymbolStr {
    fn eq(&self, other: &T) -> bool {
        self.string == other.deref()
    }
}

/// This impl means that if `ss` is a `SymbolStr`:
/// - `*ss` is a `str`;
/// - `&*ss` is a `&str` (and `match &*ss { ... }` is a common pattern).
/// - `&ss as &str` is a `&str`, which means that `&ss` can be passed to a
///   function expecting a `&str`.
impl Deref for SymbolStr {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        self.string
    }
}

impl AsRef<str> for SymbolStr {
    fn as_ref(&self) -> &str {
        self.string
    }
}

impl fmt::Debug for SymbolStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.string, f)
    }
}

impl fmt::Display for SymbolStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.string, f)
    }
}

/// All the globals for a compiler sessions.
pub struct SessionGlobals {
    /// The interner for `Symbol`s used in the compiler.
    symbol_interner: Interner,
}

impl SessionGlobals {
    fn new() -> Self {
        Self {
            symbol_interner: Interner::prefilled(),
        }
    }
}

scoped_tls::scoped_thread_local!(static SESSION_GLOBALS: SessionGlobals);

/// Creates the session globals and then runs the closure `f`.
#[inline]
pub fn create_session_if_not_set_then<R>(f: impl FnOnce(&SessionGlobals) -> R) -> R {
    if !SESSION_GLOBALS.is_set() {
        let sg = SessionGlobals::new();
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

/// The inner interner.
/// This construction is used to get interior mutability in `Interner`.
struct InnerInterner {
    /// Arena used to allocate the strings, giving us `&'static str`s from it.
    arena: DroplessArena,
    /// Registration of strings and symbol index allocation is done in this set.
    set: IndexSet<&'static str, FxBuildHasher>,
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
            arena: <_>::default(),
            set: init.iter().copied().collect(),
        };
        Self {
            inner: RefCell::new(inner),
        }
    }

    /// Interns `string`, returning a `Symbol` corresponding to it.
    fn intern(&self, string: &str) -> Symbol {
        let InnerInterner { arena, set } = &mut *self.inner.borrow_mut();

        if let Some(sym) = set.get_index_of(string) {
            // Already internet, return that symbol.
            return Symbol::new(sym as u32);
        }

        // SAFETY: `from_utf8_unchecked` is safe since we just allocated a `&str`,
        // which is known to be UTF-8.
        let bytes = arena.alloc_slice(string.as_bytes());
        let string: &str = unsafe { str::from_utf8_unchecked(bytes) };

        unsafe fn transmute_lt<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
            transmute(x)
        }

        // SAFETY: Extending to `'static` is fine. Accesses only happen while the arena is alive.
        let string: &'static _ = unsafe { transmute_lt(string) };

        Symbol::new(set.insert_full(string).0 as u32)
    }

    /// Returns the corresponding string for the given symbol.
    fn get(&self, symbol: Symbol) -> &str {
        self.inner.borrow().set.get_index(symbol.as_u32() as usize).unwrap()
    }
}
