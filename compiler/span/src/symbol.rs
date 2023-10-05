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
    // Base case 4): A single identifier left.
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
    // unary operators
    abs,
    abs_wrapped,
    double,
    inv,
    neg,
    not,
    square,
    square_root,

    // binary operators
    add,
    add_wrapped,
    and,
    div,
    div_wrapped,
    eq,
    gte,
    gt,
    lte,
    lt,
    Mod: "mod",
    mul,
    mul_wrapped,
    nand,
    neq,
    nor,
    or,
    pow,
    pow_wrapped,
    rem,
    rem_wrapped,
    shl,
    shl_wrapped,
    shr,
    shr_wrapped,
    sub,
    sub_wrapped,
    xor,

    // core constants
    GEN,

    // core functions
    BHP256,
    BHP512,
    BHP768,
    BHP1024,
    ChaCha,
    commit_to_address,
    commit_to_field,
    commit_to_group,
    contains,
    get,
    get_or_use,
    hash_to_address,
    hash_to_field,
    hash_to_group,
    hash_to_i8,
    hash_to_i16,
    hash_to_i32,
    hash_to_i64,
    hash_to_i128,
    hash_to_u8,
    hash_to_u16,
    hash_to_u32,
    hash_to_u64,
    hash_to_u128,
    hash_to_scalar,
    Keccak256,
    Keccak384,
    Keccak512,
    Mapping,
    Pedersen64,
    Pedersen128,
    Poseidon2,
    Poseidon4,
    Poseidon8,
    rand_address,
    rand_bool,
    rand_field,
    rand_group,
    rand_i8,
    rand_i16,
    rand_i32,
    rand_i64,
    rand_i128,
    rand_scalar,
    rand_u8,
    rand_u16,
    rand_u32,
    rand_u64,
    rand_u128,
    remove,
    set,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    to_x_coordinate,
    to_y_coordinate,
    verify,

    // types
    address,
    bool,
    field,
    group,
    i8,
    i16,
    i32,
    i64,
    i128,
    record,
    scalar,
    signature,
    string,
    Struct: "struct",
    u8,
    u16,
    u32,
    u64,
    u128,

    // values
    False: "false",
    True: "true",

    // general keywords
    As: "as",
    assert,
    assert_eq,
    assert_neq,
    caller,
    console,
    Const: "const",
    constant,
    decrement,
    Else: "else",
    finalize,
    For: "for",
    function,
    If: "if",
    In: "in",
    import,
    increment,
    inline,
    input,
    Let: "let",
    leo,
    main,
    mapping,
    Mut: "mut",
    Return: "return",
    SelfLower: "self",
    SelfUpper: "Self",
    signer,
    Star: "*",
    then,
    transition,
    Type: "type",

    aleo,
    public,
    private,
    owner,
    _nonce,
    program,
    block,
    height,
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
