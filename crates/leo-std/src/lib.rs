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

//! Embedded source for the Leo standard library.
//!
//! Compiler and tooling crates depend on this to inject `std` into every
//! Leo build without requiring users to declare it in `program.json`.

/// The Leo identifier under which the standard library is exposed
/// (`std::foo(...)`).
pub const LIBRARY_NAME: &str = "std";

const LIB_LEO: &str = include_str!("leo/lib.leo");
const DUMMY_LEO: &str = include_str!("leo/dummy.leo");

const HASH_BHP256_LEO: &str = include_str!("leo/hash/bhp256.leo");
const HASH_BHP512_LEO: &str = include_str!("leo/hash/bhp512.leo");
const HASH_BHP768_LEO: &str = include_str!("leo/hash/bhp768.leo");
const HASH_BHP1024_LEO: &str = include_str!("leo/hash/bhp1024.leo");
const HASH_KECCAK256_LEO: &str = include_str!("leo/hash/keccak256.leo");
const HASH_KECCAK384_LEO: &str = include_str!("leo/hash/keccak384.leo");
const HASH_KECCAK512_LEO: &str = include_str!("leo/hash/keccak512.leo");
const HASH_PEDERSEN64_LEO: &str = include_str!("leo/hash/pedersen64.leo");
const HASH_PEDERSEN128_LEO: &str = include_str!("leo/hash/pedersen128.leo");
const HASH_POSEIDON2_LEO: &str = include_str!("leo/hash/poseidon2.leo");
const HASH_POSEIDON4_LEO: &str = include_str!("leo/hash/poseidon4.leo");
const HASH_POSEIDON8_LEO: &str = include_str!("leo/hash/poseidon8.leo");
const HASH_SHA3_256_LEO: &str = include_str!("leo/hash/sha3_256.leo");
const HASH_SHA3_384_LEO: &str = include_str!("leo/hash/sha3_384.leo");
const HASH_SHA3_512_LEO: &str = include_str!("leo/hash/sha3_512.leo");

const COMMIT_BHP256_LEO: &str = include_str!("leo/commit/bhp256.leo");
const COMMIT_BHP512_LEO: &str = include_str!("leo/commit/bhp512.leo");
const COMMIT_BHP768_LEO: &str = include_str!("leo/commit/bhp768.leo");
const COMMIT_BHP1024_LEO: &str = include_str!("leo/commit/bhp1024.leo");
const COMMIT_PEDERSEN64_LEO: &str = include_str!("leo/commit/pedersen64.leo");
const COMMIT_PEDERSEN128_LEO: &str = include_str!("leo/commit/pedersen128.leo");

const RAND_LEO: &str = include_str!("leo/rand.leo");
const SIG_LEO: &str = include_str!("leo/sig.leo");
const SERIALIZE_LEO: &str = include_str!("leo/serialize.leo");
const GRP_LEO: &str = include_str!("leo/grp.leo");
const CTX_LEO: &str = include_str!("leo/ctx.leo");
const PROG_LEO: &str = include_str!("leo/prog.leo");

/// Entry source of the standard library (contents of `lib.leo`).
pub fn entry_source() -> &'static str {
    LIB_LEO
}

/// Submodule sources, returned as `(virtual_path, source)` pairs.
///
/// The first element of each tuple is a label used in span/error reporting AND
/// drives the module key: `hash/bhp256.leo` → module `std::hash::bhp256`. Use
/// forward slashes for nested paths.
pub fn modules() -> &'static [(&'static str, &'static str)] {
    &[
        ("dummy.leo", DUMMY_LEO),
        ("hash/bhp256.leo", HASH_BHP256_LEO),
        ("hash/bhp512.leo", HASH_BHP512_LEO),
        ("hash/bhp768.leo", HASH_BHP768_LEO),
        ("hash/bhp1024.leo", HASH_BHP1024_LEO),
        ("hash/keccak256.leo", HASH_KECCAK256_LEO),
        ("hash/keccak384.leo", HASH_KECCAK384_LEO),
        ("hash/keccak512.leo", HASH_KECCAK512_LEO),
        ("hash/pedersen64.leo", HASH_PEDERSEN64_LEO),
        ("hash/pedersen128.leo", HASH_PEDERSEN128_LEO),
        ("hash/poseidon2.leo", HASH_POSEIDON2_LEO),
        ("hash/poseidon4.leo", HASH_POSEIDON4_LEO),
        ("hash/poseidon8.leo", HASH_POSEIDON8_LEO),
        ("hash/sha3_256.leo", HASH_SHA3_256_LEO),
        ("hash/sha3_384.leo", HASH_SHA3_384_LEO),
        ("hash/sha3_512.leo", HASH_SHA3_512_LEO),
        ("commit/bhp256.leo", COMMIT_BHP256_LEO),
        ("commit/bhp512.leo", COMMIT_BHP512_LEO),
        ("commit/bhp768.leo", COMMIT_BHP768_LEO),
        ("commit/bhp1024.leo", COMMIT_BHP1024_LEO),
        ("commit/pedersen64.leo", COMMIT_PEDERSEN64_LEO),
        ("commit/pedersen128.leo", COMMIT_PEDERSEN128_LEO),
        ("rand.leo", RAND_LEO),
        ("sig.leo", SIG_LEO),
        ("serialize.leo", SERIALIZE_LEO),
        ("grp.leo", GRP_LEO),
        ("ctx.leo", CTX_LEO),
        ("prog.leo", PROG_LEO),
    ]
}

/// The Leo identifier under which the standard library is exposed.
pub fn library_name() -> &'static str {
    LIBRARY_NAME
}
