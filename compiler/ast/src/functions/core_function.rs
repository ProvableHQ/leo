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

use leo_span::{sym, Symbol};

/// A core instruction that maps directly to an AVM bytecode instruction.
#[derive(Clone, PartialEq, Eq)]
pub enum CoreFunction {
    BHP256CommitToAddress,
    BHP256CommitToField,
    BHP256CommitToGroup,
    BHP256HashToAddress,
    BHP256HashToField,
    BHP256HashToGroup,
    BHP256HashToI8,
    BHP256HashToI16,
    BHP256HashToI32,
    BHP256HashToI64,
    BHP256HashToI128,
    BHP256HashToU8,
    BHP256HashToU16,
    BHP256HashToU32,
    BHP256HashToU64,
    BHP256HashToU128,
    BHP256HashToScalar,

    BHP512CommitToAddress,
    BHP512CommitToField,
    BHP512CommitToGroup,
    BHP512HashToAddress,
    BHP512HashToField,
    BHP512HashToGroup,
    BHP512HashToI8,
    BHP512HashToI16,
    BHP512HashToI32,
    BHP512HashToI64,
    BHP512HashToI128,
    BHP512HashToU8,
    BHP512HashToU16,
    BHP512HashToU32,
    BHP512HashToU64,
    BHP512HashToU128,
    BHP512HashToScalar,

    BHP768CommitToAddress,
    BHP768CommitToField,
    BHP768CommitToGroup,
    BHP768HashToAddress,
    BHP768HashToField,
    BHP768HashToGroup,
    BHP768HashToI8,
    BHP768HashToI16,
    BHP768HashToI32,
    BHP768HashToI64,
    BHP768HashToI128,
    BHP768HashToU8,
    BHP768HashToU16,
    BHP768HashToU32,
    BHP768HashToU64,
    BHP768HashToU128,
    BHP768HashToScalar,

    BHP1024CommitToAddress,
    BHP1024CommitToField,
    BHP1024CommitToGroup,
    BHP1024HashToAddress,
    BHP1024HashToField,
    BHP1024HashToGroup,
    BHP1024HashToI8,
    BHP1024HashToI16,
    BHP1024HashToI32,
    BHP1024HashToI64,
    BHP1024HashToI128,
    BHP1024HashToU8,
    BHP1024HashToU16,
    BHP1024HashToU32,
    BHP1024HashToU64,
    BHP1024HashToU128,
    BHP1024HashToScalar,

    ChaChaRandAddress,
    ChaChaRandBool,
    ChaChaRandField,
    ChaChaRandGroup,
    ChaChaRandI8,
    ChaChaRandI16,
    ChaChaRandI32,
    ChaChaRandI64,
    ChaChaRandI128,
    ChaChaRandU8,
    ChaChaRandU16,
    ChaChaRandU32,
    ChaChaRandU64,
    ChaChaRandU128,
    ChaChaRandScalar,

    Keccak256HashToAddress,
    Keccak256HashToField,
    Keccak256HashToGroup,
    Keccak256HashToI8,
    Keccak256HashToI16,
    Keccak256HashToI32,
    Keccak256HashToI64,
    Keccak256HashToI128,
    Keccak256HashToU8,
    Keccak256HashToU16,
    Keccak256HashToU32,
    Keccak256HashToU64,
    Keccak256HashToU128,
    Keccak256HashToScalar,

    Keccak384HashToAddress,
    Keccak384HashToField,
    Keccak384HashToGroup,
    Keccak384HashToI8,
    Keccak384HashToI16,
    Keccak384HashToI32,
    Keccak384HashToI64,
    Keccak384HashToI128,
    Keccak384HashToU8,
    Keccak384HashToU16,
    Keccak384HashToU32,
    Keccak384HashToU64,
    Keccak384HashToU128,
    Keccak384HashToScalar,

    Keccak512HashToAddress,
    Keccak512HashToField,
    Keccak512HashToGroup,
    Keccak512HashToI8,
    Keccak512HashToI16,
    Keccak512HashToI32,
    Keccak512HashToI64,
    Keccak512HashToI128,
    Keccak512HashToU8,
    Keccak512HashToU16,
    Keccak512HashToU32,
    Keccak512HashToU64,
    Keccak512HashToU128,
    Keccak512HashToScalar,

    Pedersen64CommitToAddress,
    Pedersen64CommitToField,
    Pedersen64CommitToGroup,
    Pedersen64HashToAddress,
    Pedersen64HashToField,
    Pedersen64HashToGroup,
    Pedersen64HashToI8,
    Pedersen64HashToI16,
    Pedersen64HashToI32,
    Pedersen64HashToI64,
    Pedersen64HashToI128,
    Pedersen64HashToU8,
    Pedersen64HashToU16,
    Pedersen64HashToU32,
    Pedersen64HashToU64,
    Pedersen64HashToU128,
    Pedersen64HashToScalar,

    Pedersen128CommitToAddress,
    Pedersen128CommitToField,
    Pedersen128CommitToGroup,
    Pedersen128HashToAddress,
    Pedersen128HashToField,
    Pedersen128HashToGroup,
    Pedersen128HashToI8,
    Pedersen128HashToI16,
    Pedersen128HashToI32,
    Pedersen128HashToI64,
    Pedersen128HashToI128,
    Pedersen128HashToU8,
    Pedersen128HashToU16,
    Pedersen128HashToU32,
    Pedersen128HashToU64,
    Pedersen128HashToU128,
    Pedersen128HashToScalar,

    Poseidon2HashToAddress,
    Poseidon2HashToField,
    Poseidon2HashToGroup,
    Poseidon2HashToI8,
    Poseidon2HashToI16,
    Poseidon2HashToI32,
    Poseidon2HashToI64,
    Poseidon2HashToI128,
    Poseidon2HashToU8,
    Poseidon2HashToU16,
    Poseidon2HashToU32,
    Poseidon2HashToU64,
    Poseidon2HashToU128,
    Poseidon2HashToScalar,

    Poseidon4HashToAddress,
    Poseidon4HashToField,
    Poseidon4HashToGroup,
    Poseidon4HashToI8,
    Poseidon4HashToI16,
    Poseidon4HashToI32,
    Poseidon4HashToI64,
    Poseidon4HashToI128,
    Poseidon4HashToU8,
    Poseidon4HashToU16,
    Poseidon4HashToU32,
    Poseidon4HashToU64,
    Poseidon4HashToU128,
    Poseidon4HashToScalar,

    Poseidon8HashToAddress,
    Poseidon8HashToField,
    Poseidon8HashToGroup,
    Poseidon8HashToI8,
    Poseidon8HashToI16,
    Poseidon8HashToI32,
    Poseidon8HashToI64,
    Poseidon8HashToI128,
    Poseidon8HashToU8,
    Poseidon8HashToU16,
    Poseidon8HashToU32,
    Poseidon8HashToU64,
    Poseidon8HashToU128,
    Poseidon8HashToScalar,

    SHA3_256HashToAddress,
    SHA3_256HashToField,
    SHA3_256HashToGroup,
    SHA3_256HashToI8,
    SHA3_256HashToI16,
    SHA3_256HashToI32,
    SHA3_256HashToI64,
    SHA3_256HashToI128,
    SHA3_256HashToU8,
    SHA3_256HashToU16,
    SHA3_256HashToU32,
    SHA3_256HashToU64,
    SHA3_256HashToU128,
    SHA3_256HashToScalar,

    SHA3_384HashToAddress,
    SHA3_384HashToField,
    SHA3_384HashToGroup,
    SHA3_384HashToI8,
    SHA3_384HashToI16,
    SHA3_384HashToI32,
    SHA3_384HashToI64,
    SHA3_384HashToI128,
    SHA3_384HashToU8,
    SHA3_384HashToU16,
    SHA3_384HashToU32,
    SHA3_384HashToU64,
    SHA3_384HashToU128,
    SHA3_384HashToScalar,

    SHA3_512HashToAddress,
    SHA3_512HashToField,
    SHA3_512HashToGroup,
    SHA3_512HashToI8,
    SHA3_512HashToI16,
    SHA3_512HashToI32,
    SHA3_512HashToI64,
    SHA3_512HashToI128,
    SHA3_512HashToU8,
    SHA3_512HashToU16,
    SHA3_512HashToU32,
    SHA3_512HashToU64,
    SHA3_512HashToU128,
    SHA3_512HashToScalar,

    MappingGet,
    MappingGetOrUse,
    MappingSet,
    MappingRemove,
    MappingContains,

    GroupToXCoordinate,
    GroupToYCoordinate,

    SignatureVerify,
}

impl CoreFunction {
    /// Returns a `CoreFunction` from the given module and method symbols.
    pub fn from_symbols(module: Symbol, function: Symbol) -> Option<Self> {
        Some(match (module, function) {
            (sym::BHP256, sym::commit_to_address) => Self::BHP256CommitToAddress,
            (sym::BHP256, sym::commit_to_field) => Self::BHP256CommitToField,
            (sym::BHP256, sym::commit_to_group) => Self::BHP256CommitToGroup,

            (sym::BHP256, sym::hash_to_address) => Self::BHP256HashToAddress,
            (sym::BHP256, sym::hash_to_field) => Self::BHP256HashToField,
            (sym::BHP256, sym::hash_to_group) => Self::BHP256HashToGroup,
            (sym::BHP256, sym::hash_to_i8) => Self::BHP256HashToI8,
            (sym::BHP256, sym::hash_to_i16) => Self::BHP256HashToI16,
            (sym::BHP256, sym::hash_to_i32) => Self::BHP256HashToI32,
            (sym::BHP256, sym::hash_to_i64) => Self::BHP256HashToI64,
            (sym::BHP256, sym::hash_to_i128) => Self::BHP256HashToI128,
            (sym::BHP256, sym::hash_to_u8) => Self::BHP256HashToU8,
            (sym::BHP256, sym::hash_to_u16) => Self::BHP256HashToU16,
            (sym::BHP256, sym::hash_to_u32) => Self::BHP256HashToU32,
            (sym::BHP256, sym::hash_to_u64) => Self::BHP256HashToU64,
            (sym::BHP256, sym::hash_to_u128) => Self::BHP256HashToU128,
            (sym::BHP256, sym::hash_to_scalar) => Self::BHP256HashToScalar,

            (sym::BHP512, sym::commit_to_address) => Self::BHP512CommitToAddress,
            (sym::BHP512, sym::commit_to_field) => Self::BHP512CommitToField,
            (sym::BHP512, sym::commit_to_group) => Self::BHP512CommitToGroup,
            (sym::BHP512, sym::hash_to_address) => Self::BHP512HashToAddress,
            (sym::BHP512, sym::hash_to_field) => Self::BHP512HashToField,
            (sym::BHP512, sym::hash_to_group) => Self::BHP512HashToGroup,
            (sym::BHP512, sym::hash_to_i8) => Self::BHP512HashToI8,
            (sym::BHP512, sym::hash_to_i16) => Self::BHP512HashToI16,
            (sym::BHP512, sym::hash_to_i32) => Self::BHP512HashToI32,
            (sym::BHP512, sym::hash_to_i64) => Self::BHP512HashToI64,
            (sym::BHP512, sym::hash_to_i128) => Self::BHP512HashToI128,
            (sym::BHP512, sym::hash_to_u8) => Self::BHP512HashToU8,
            (sym::BHP512, sym::hash_to_u16) => Self::BHP512HashToU16,
            (sym::BHP512, sym::hash_to_u32) => Self::BHP512HashToU32,
            (sym::BHP512, sym::hash_to_u64) => Self::BHP512HashToU64,
            (sym::BHP512, sym::hash_to_u128) => Self::BHP512HashToU128,
            (sym::BHP512, sym::hash_to_scalar) => Self::BHP512HashToScalar,

            (sym::BHP768, sym::commit_to_address) => Self::BHP768CommitToAddress,
            (sym::BHP768, sym::commit_to_field) => Self::BHP768CommitToField,
            (sym::BHP768, sym::commit_to_group) => Self::BHP768CommitToGroup,
            (sym::BHP768, sym::hash_to_address) => Self::BHP768HashToAddress,
            (sym::BHP768, sym::hash_to_field) => Self::BHP768HashToField,
            (sym::BHP768, sym::hash_to_group) => Self::BHP768HashToGroup,
            (sym::BHP768, sym::hash_to_i8) => Self::BHP768HashToI8,
            (sym::BHP768, sym::hash_to_i16) => Self::BHP768HashToI16,
            (sym::BHP768, sym::hash_to_i32) => Self::BHP768HashToI32,
            (sym::BHP768, sym::hash_to_i64) => Self::BHP768HashToI64,
            (sym::BHP768, sym::hash_to_i128) => Self::BHP768HashToI128,
            (sym::BHP768, sym::hash_to_u8) => Self::BHP768HashToU8,
            (sym::BHP768, sym::hash_to_u16) => Self::BHP768HashToU16,
            (sym::BHP768, sym::hash_to_u32) => Self::BHP768HashToU32,
            (sym::BHP768, sym::hash_to_u64) => Self::BHP768HashToU64,
            (sym::BHP768, sym::hash_to_u128) => Self::BHP768HashToU128,
            (sym::BHP768, sym::hash_to_scalar) => Self::BHP768HashToScalar,

            (sym::BHP1024, sym::commit_to_address) => Self::BHP1024CommitToAddress,
            (sym::BHP1024, sym::commit_to_field) => Self::BHP1024CommitToField,
            (sym::BHP1024, sym::commit_to_group) => Self::BHP1024CommitToGroup,
            (sym::BHP1024, sym::hash_to_address) => Self::BHP1024HashToAddress,
            (sym::BHP1024, sym::hash_to_field) => Self::BHP1024HashToField,
            (sym::BHP1024, sym::hash_to_group) => Self::BHP1024HashToGroup,
            (sym::BHP1024, sym::hash_to_i8) => Self::BHP1024HashToI8,
            (sym::BHP1024, sym::hash_to_i16) => Self::BHP1024HashToI16,
            (sym::BHP1024, sym::hash_to_i32) => Self::BHP1024HashToI32,
            (sym::BHP1024, sym::hash_to_i64) => Self::BHP1024HashToI64,
            (sym::BHP1024, sym::hash_to_i128) => Self::BHP1024HashToI128,
            (sym::BHP1024, sym::hash_to_u8) => Self::BHP1024HashToU8,
            (sym::BHP1024, sym::hash_to_u16) => Self::BHP1024HashToU16,
            (sym::BHP1024, sym::hash_to_u32) => Self::BHP1024HashToU32,
            (sym::BHP1024, sym::hash_to_u64) => Self::BHP1024HashToU64,
            (sym::BHP1024, sym::hash_to_u128) => Self::BHP1024HashToU128,
            (sym::BHP1024, sym::hash_to_scalar) => Self::BHP1024HashToScalar,

            (sym::ChaCha, sym::rand_address) => Self::ChaChaRandAddress,
            (sym::ChaCha, sym::rand_bool) => Self::ChaChaRandBool,
            (sym::ChaCha, sym::rand_field) => Self::ChaChaRandField,
            (sym::ChaCha, sym::rand_group) => Self::ChaChaRandGroup,
            (sym::ChaCha, sym::rand_i8) => Self::ChaChaRandI8,
            (sym::ChaCha, sym::rand_i16) => Self::ChaChaRandI16,
            (sym::ChaCha, sym::rand_i32) => Self::ChaChaRandI32,
            (sym::ChaCha, sym::rand_i64) => Self::ChaChaRandI64,
            (sym::ChaCha, sym::rand_i128) => Self::ChaChaRandI128,
            (sym::ChaCha, sym::rand_scalar) => Self::ChaChaRandScalar,
            (sym::ChaCha, sym::rand_u8) => Self::ChaChaRandU8,
            (sym::ChaCha, sym::rand_u16) => Self::ChaChaRandU16,
            (sym::ChaCha, sym::rand_u32) => Self::ChaChaRandU32,
            (sym::ChaCha, sym::rand_u64) => Self::ChaChaRandU64,
            (sym::ChaCha, sym::rand_u128) => Self::ChaChaRandU128,

            (sym::Keccak256, sym::hash_to_address) => Self::Keccak256HashToAddress,
            (sym::Keccak256, sym::hash_to_field) => Self::Keccak256HashToField,
            (sym::Keccak256, sym::hash_to_group) => Self::Keccak256HashToGroup,
            (sym::Keccak256, sym::hash_to_i8) => Self::Keccak256HashToI8,
            (sym::Keccak256, sym::hash_to_i16) => Self::Keccak256HashToI16,
            (sym::Keccak256, sym::hash_to_i32) => Self::Keccak256HashToI32,
            (sym::Keccak256, sym::hash_to_i64) => Self::Keccak256HashToI64,
            (sym::Keccak256, sym::hash_to_i128) => Self::Keccak256HashToI128,
            (sym::Keccak256, sym::hash_to_u8) => Self::Keccak256HashToU8,
            (sym::Keccak256, sym::hash_to_u16) => Self::Keccak256HashToU16,
            (sym::Keccak256, sym::hash_to_u32) => Self::Keccak256HashToU32,
            (sym::Keccak256, sym::hash_to_u64) => Self::Keccak256HashToU64,
            (sym::Keccak256, sym::hash_to_u128) => Self::Keccak256HashToU128,
            (sym::Keccak256, sym::hash_to_scalar) => Self::Keccak256HashToScalar,

            (sym::Keccak384, sym::hash_to_address) => Self::Keccak384HashToAddress,
            (sym::Keccak384, sym::hash_to_field) => Self::Keccak384HashToField,
            (sym::Keccak384, sym::hash_to_group) => Self::Keccak384HashToGroup,
            (sym::Keccak384, sym::hash_to_i8) => Self::Keccak384HashToI8,
            (sym::Keccak384, sym::hash_to_i16) => Self::Keccak384HashToI16,
            (sym::Keccak384, sym::hash_to_i32) => Self::Keccak384HashToI32,
            (sym::Keccak384, sym::hash_to_i64) => Self::Keccak384HashToI64,
            (sym::Keccak384, sym::hash_to_i128) => Self::Keccak384HashToI128,
            (sym::Keccak384, sym::hash_to_u8) => Self::Keccak384HashToU8,
            (sym::Keccak384, sym::hash_to_u16) => Self::Keccak384HashToU16,
            (sym::Keccak384, sym::hash_to_u32) => Self::Keccak384HashToU32,
            (sym::Keccak384, sym::hash_to_u64) => Self::Keccak384HashToU64,
            (sym::Keccak384, sym::hash_to_u128) => Self::Keccak384HashToU128,
            (sym::Keccak384, sym::hash_to_scalar) => Self::Keccak384HashToScalar,

            (sym::Keccak512, sym::hash_to_address) => Self::Keccak512HashToAddress,
            (sym::Keccak512, sym::hash_to_field) => Self::Keccak512HashToField,
            (sym::Keccak512, sym::hash_to_group) => Self::Keccak512HashToGroup,
            (sym::Keccak512, sym::hash_to_i8) => Self::Keccak512HashToI8,
            (sym::Keccak512, sym::hash_to_i16) => Self::Keccak512HashToI16,
            (sym::Keccak512, sym::hash_to_i32) => Self::Keccak512HashToI32,
            (sym::Keccak512, sym::hash_to_i64) => Self::Keccak512HashToI64,
            (sym::Keccak512, sym::hash_to_i128) => Self::Keccak512HashToI128,
            (sym::Keccak512, sym::hash_to_u8) => Self::Keccak512HashToU8,
            (sym::Keccak512, sym::hash_to_u16) => Self::Keccak512HashToU16,
            (sym::Keccak512, sym::hash_to_u32) => Self::Keccak512HashToU32,
            (sym::Keccak512, sym::hash_to_u64) => Self::Keccak512HashToU64,
            (sym::Keccak512, sym::hash_to_u128) => Self::Keccak512HashToU128,
            (sym::Keccak512, sym::hash_to_scalar) => Self::Keccak512HashToScalar,

            (sym::Pedersen64, sym::commit_to_address) => Self::Pedersen64CommitToAddress,
            (sym::Pedersen64, sym::commit_to_field) => Self::Pedersen64CommitToField,
            (sym::Pedersen64, sym::commit_to_group) => Self::Pedersen64CommitToGroup,
            (sym::Pedersen64, sym::hash_to_address) => Self::Pedersen64HashToAddress,
            (sym::Pedersen64, sym::hash_to_field) => Self::Pedersen64HashToField,
            (sym::Pedersen64, sym::hash_to_group) => Self::Pedersen64HashToGroup,
            (sym::Pedersen64, sym::hash_to_i8) => Self::Pedersen64HashToI8,
            (sym::Pedersen64, sym::hash_to_i16) => Self::Pedersen64HashToI16,
            (sym::Pedersen64, sym::hash_to_i32) => Self::Pedersen64HashToI32,
            (sym::Pedersen64, sym::hash_to_i64) => Self::Pedersen64HashToI64,
            (sym::Pedersen64, sym::hash_to_i128) => Self::Pedersen64HashToI128,
            (sym::Pedersen64, sym::hash_to_u8) => Self::Pedersen64HashToU8,
            (sym::Pedersen64, sym::hash_to_u16) => Self::Pedersen64HashToU16,
            (sym::Pedersen64, sym::hash_to_u32) => Self::Pedersen64HashToU32,
            (sym::Pedersen64, sym::hash_to_u64) => Self::Pedersen64HashToU64,
            (sym::Pedersen64, sym::hash_to_u128) => Self::Pedersen64HashToU128,
            (sym::Pedersen64, sym::hash_to_scalar) => Self::Pedersen64HashToScalar,

            (sym::Pedersen128, sym::commit_to_address) => Self::Pedersen128CommitToAddress,
            (sym::Pedersen128, sym::commit_to_field) => Self::Pedersen128CommitToField,
            (sym::Pedersen128, sym::commit_to_group) => Self::Pedersen128CommitToGroup,
            (sym::Pedersen128, sym::hash_to_address) => Self::Pedersen128HashToAddress,
            (sym::Pedersen128, sym::hash_to_field) => Self::Pedersen128HashToField,
            (sym::Pedersen128, sym::hash_to_group) => Self::Pedersen128HashToGroup,
            (sym::Pedersen128, sym::hash_to_i8) => Self::Pedersen128HashToI8,
            (sym::Pedersen128, sym::hash_to_i16) => Self::Pedersen128HashToI16,
            (sym::Pedersen128, sym::hash_to_i32) => Self::Pedersen128HashToI32,
            (sym::Pedersen128, sym::hash_to_i64) => Self::Pedersen128HashToI64,
            (sym::Pedersen128, sym::hash_to_i128) => Self::Pedersen128HashToI128,
            (sym::Pedersen128, sym::hash_to_u8) => Self::Pedersen128HashToU8,
            (sym::Pedersen128, sym::hash_to_u16) => Self::Pedersen128HashToU16,
            (sym::Pedersen128, sym::hash_to_u32) => Self::Pedersen128HashToU32,
            (sym::Pedersen128, sym::hash_to_u64) => Self::Pedersen128HashToU64,
            (sym::Pedersen128, sym::hash_to_u128) => Self::Pedersen128HashToU128,
            (sym::Pedersen128, sym::hash_to_scalar) => Self::Pedersen128HashToScalar,

            (sym::Poseidon2, sym::hash_to_address) => Self::Poseidon2HashToAddress,
            (sym::Poseidon2, sym::hash_to_field) => Self::Poseidon2HashToField,
            (sym::Poseidon2, sym::hash_to_group) => Self::Poseidon2HashToGroup,
            (sym::Poseidon2, sym::hash_to_i8) => Self::Poseidon2HashToI8,
            (sym::Poseidon2, sym::hash_to_i16) => Self::Poseidon2HashToI16,
            (sym::Poseidon2, sym::hash_to_i32) => Self::Poseidon2HashToI32,
            (sym::Poseidon2, sym::hash_to_i64) => Self::Poseidon2HashToI64,
            (sym::Poseidon2, sym::hash_to_i128) => Self::Poseidon2HashToI128,
            (sym::Poseidon2, sym::hash_to_u8) => Self::Poseidon2HashToU8,
            (sym::Poseidon2, sym::hash_to_u16) => Self::Poseidon2HashToU16,
            (sym::Poseidon2, sym::hash_to_u32) => Self::Poseidon2HashToU32,
            (sym::Poseidon2, sym::hash_to_u64) => Self::Poseidon2HashToU64,
            (sym::Poseidon2, sym::hash_to_u128) => Self::Poseidon2HashToU128,
            (sym::Poseidon2, sym::hash_to_scalar) => Self::Poseidon2HashToScalar,

            (sym::Poseidon4, sym::hash_to_address) => Self::Poseidon4HashToAddress,
            (sym::Poseidon4, sym::hash_to_field) => Self::Poseidon4HashToField,
            (sym::Poseidon4, sym::hash_to_group) => Self::Poseidon4HashToGroup,
            (sym::Poseidon4, sym::hash_to_i8) => Self::Poseidon4HashToI8,
            (sym::Poseidon4, sym::hash_to_i16) => Self::Poseidon4HashToI16,
            (sym::Poseidon4, sym::hash_to_i32) => Self::Poseidon4HashToI32,
            (sym::Poseidon4, sym::hash_to_i64) => Self::Poseidon4HashToI64,
            (sym::Poseidon4, sym::hash_to_i128) => Self::Poseidon4HashToI128,
            (sym::Poseidon4, sym::hash_to_u8) => Self::Poseidon4HashToU8,
            (sym::Poseidon4, sym::hash_to_u16) => Self::Poseidon4HashToU16,
            (sym::Poseidon4, sym::hash_to_u32) => Self::Poseidon4HashToU32,
            (sym::Poseidon4, sym::hash_to_u64) => Self::Poseidon4HashToU64,
            (sym::Poseidon4, sym::hash_to_u128) => Self::Poseidon4HashToU128,
            (sym::Poseidon4, sym::hash_to_scalar) => Self::Poseidon4HashToScalar,

            (sym::Poseidon8, sym::hash_to_address) => Self::Poseidon8HashToAddress,
            (sym::Poseidon8, sym::hash_to_field) => Self::Poseidon8HashToField,
            (sym::Poseidon8, sym::hash_to_group) => Self::Poseidon8HashToGroup,
            (sym::Poseidon8, sym::hash_to_i8) => Self::Poseidon8HashToI8,
            (sym::Poseidon8, sym::hash_to_i16) => Self::Poseidon8HashToI16,
            (sym::Poseidon8, sym::hash_to_i32) => Self::Poseidon8HashToI32,
            (sym::Poseidon8, sym::hash_to_i64) => Self::Poseidon8HashToI64,
            (sym::Poseidon8, sym::hash_to_i128) => Self::Poseidon8HashToI128,
            (sym::Poseidon8, sym::hash_to_u8) => Self::Poseidon8HashToU8,
            (sym::Poseidon8, sym::hash_to_u16) => Self::Poseidon8HashToU16,
            (sym::Poseidon8, sym::hash_to_u32) => Self::Poseidon8HashToU32,
            (sym::Poseidon8, sym::hash_to_u64) => Self::Poseidon8HashToU64,
            (sym::Poseidon8, sym::hash_to_u128) => Self::Poseidon8HashToU128,
            (sym::Poseidon8, sym::hash_to_scalar) => Self::Poseidon8HashToScalar,

            (sym::SHA3_256, sym::hash_to_address) => Self::SHA3_256HashToAddress,
            (sym::SHA3_256, sym::hash_to_field) => Self::SHA3_256HashToField,
            (sym::SHA3_256, sym::hash_to_group) => Self::SHA3_256HashToGroup,
            (sym::SHA3_256, sym::hash_to_i8) => Self::SHA3_256HashToI8,
            (sym::SHA3_256, sym::hash_to_i16) => Self::SHA3_256HashToI16,
            (sym::SHA3_256, sym::hash_to_i32) => Self::SHA3_256HashToI32,
            (sym::SHA3_256, sym::hash_to_i64) => Self::SHA3_256HashToI64,
            (sym::SHA3_256, sym::hash_to_i128) => Self::SHA3_256HashToI128,
            (sym::SHA3_256, sym::hash_to_u8) => Self::SHA3_256HashToU8,
            (sym::SHA3_256, sym::hash_to_u16) => Self::SHA3_256HashToU16,
            (sym::SHA3_256, sym::hash_to_u32) => Self::SHA3_256HashToU32,
            (sym::SHA3_256, sym::hash_to_u64) => Self::SHA3_256HashToU64,
            (sym::SHA3_256, sym::hash_to_u128) => Self::SHA3_256HashToU128,
            (sym::SHA3_256, sym::hash_to_scalar) => Self::SHA3_256HashToScalar,

            (sym::SHA3_384, sym::hash_to_address) => Self::SHA3_384HashToAddress,
            (sym::SHA3_384, sym::hash_to_field) => Self::SHA3_384HashToField,
            (sym::SHA3_384, sym::hash_to_group) => Self::SHA3_384HashToGroup,
            (sym::SHA3_384, sym::hash_to_i8) => Self::SHA3_384HashToI8,
            (sym::SHA3_384, sym::hash_to_i16) => Self::SHA3_384HashToI16,
            (sym::SHA3_384, sym::hash_to_i32) => Self::SHA3_384HashToI32,
            (sym::SHA3_384, sym::hash_to_i64) => Self::SHA3_384HashToI64,
            (sym::SHA3_384, sym::hash_to_i128) => Self::SHA3_384HashToI128,
            (sym::SHA3_384, sym::hash_to_u8) => Self::SHA3_384HashToU8,
            (sym::SHA3_384, sym::hash_to_u16) => Self::SHA3_384HashToU16,
            (sym::SHA3_384, sym::hash_to_u32) => Self::SHA3_384HashToU32,
            (sym::SHA3_384, sym::hash_to_u64) => Self::SHA3_384HashToU64,
            (sym::SHA3_384, sym::hash_to_u128) => Self::SHA3_384HashToU128,
            (sym::SHA3_384, sym::hash_to_scalar) => Self::SHA3_384HashToScalar,

            (sym::SHA3_512, sym::hash_to_address) => Self::SHA3_512HashToAddress,
            (sym::SHA3_512, sym::hash_to_field) => Self::SHA3_512HashToField,
            (sym::SHA3_512, sym::hash_to_group) => Self::SHA3_512HashToGroup,
            (sym::SHA3_512, sym::hash_to_i8) => Self::SHA3_512HashToI8,
            (sym::SHA3_512, sym::hash_to_i16) => Self::SHA3_512HashToI16,
            (sym::SHA3_512, sym::hash_to_i32) => Self::SHA3_512HashToI32,
            (sym::SHA3_512, sym::hash_to_i64) => Self::SHA3_512HashToI64,
            (sym::SHA3_512, sym::hash_to_i128) => Self::SHA3_512HashToI128,
            (sym::SHA3_512, sym::hash_to_u8) => Self::SHA3_512HashToU8,
            (sym::SHA3_512, sym::hash_to_u16) => Self::SHA3_512HashToU16,
            (sym::SHA3_512, sym::hash_to_u32) => Self::SHA3_512HashToU32,
            (sym::SHA3_512, sym::hash_to_u64) => Self::SHA3_512HashToU64,
            (sym::SHA3_512, sym::hash_to_u128) => Self::SHA3_512HashToU128,
            (sym::SHA3_512, sym::hash_to_scalar) => Self::SHA3_512HashToScalar,

            (sym::Mapping, sym::get) => Self::MappingGet,
            (sym::Mapping, sym::get_or_use) => Self::MappingGetOrUse,
            (sym::Mapping, sym::set) => Self::MappingSet,
            (sym::Mapping, sym::remove) => Self::MappingRemove,
            (sym::Mapping, sym::contains) => Self::MappingContains,

            (sym::group, sym::to_x_coordinate) => Self::GroupToXCoordinate,
            (sym::group, sym::to_y_coordinate) => Self::GroupToYCoordinate,

            (sym::signature, sym::verify) => Self::SignatureVerify,
            _ => return None,
        })
    }

    /// Returns the number of arguments required by the instruction.
    pub fn num_args(&self) -> usize {
        match self {
            Self::BHP256CommitToAddress => 2,
            Self::BHP256CommitToField => 2,
            Self::BHP256CommitToGroup => 2,

            Self::BHP256HashToAddress => 1,
            Self::BHP256HashToField => 1,
            Self::BHP256HashToGroup => 1,
            Self::BHP256HashToI8 => 1,
            Self::BHP256HashToI16 => 1,
            Self::BHP256HashToI32 => 1,
            Self::BHP256HashToI64 => 1,
            Self::BHP256HashToI128 => 1,
            Self::BHP256HashToU8 => 1,
            Self::BHP256HashToU16 => 1,
            Self::BHP256HashToU32 => 1,
            Self::BHP256HashToU64 => 1,
            Self::BHP256HashToU128 => 1,
            Self::BHP256HashToScalar => 1,

            Self::BHP512CommitToAddress => 2,
            Self::BHP512CommitToField => 2,
            Self::BHP512CommitToGroup => 2,
            Self::BHP512HashToAddress => 1,
            Self::BHP512HashToField => 1,
            Self::BHP512HashToGroup => 1,
            Self::BHP512HashToI8 => 1,
            Self::BHP512HashToI16 => 1,
            Self::BHP512HashToI32 => 1,
            Self::BHP512HashToI64 => 1,
            Self::BHP512HashToI128 => 1,
            Self::BHP512HashToU8 => 1,
            Self::BHP512HashToU16 => 1,
            Self::BHP512HashToU32 => 1,
            Self::BHP512HashToU64 => 1,
            Self::BHP512HashToU128 => 1,
            Self::BHP512HashToScalar => 1,

            Self::BHP768CommitToAddress => 2,
            Self::BHP768CommitToField => 2,
            Self::BHP768CommitToGroup => 2,
            Self::BHP768HashToAddress => 1,
            Self::BHP768HashToField => 1,
            Self::BHP768HashToGroup => 1,
            Self::BHP768HashToI8 => 1,
            Self::BHP768HashToI16 => 1,
            Self::BHP768HashToI32 => 1,
            Self::BHP768HashToI64 => 1,
            Self::BHP768HashToI128 => 1,
            Self::BHP768HashToU8 => 1,
            Self::BHP768HashToU16 => 1,
            Self::BHP768HashToU32 => 1,
            Self::BHP768HashToU64 => 1,
            Self::BHP768HashToU128 => 1,
            Self::BHP768HashToScalar => 1,

            Self::BHP1024CommitToAddress => 2,
            Self::BHP1024CommitToField => 2,
            Self::BHP1024CommitToGroup => 2,
            Self::BHP1024HashToAddress => 1,
            Self::BHP1024HashToField => 1,
            Self::BHP1024HashToGroup => 1,
            Self::BHP1024HashToI8 => 1,
            Self::BHP1024HashToI16 => 1,
            Self::BHP1024HashToI32 => 1,
            Self::BHP1024HashToI64 => 1,
            Self::BHP1024HashToI128 => 1,
            Self::BHP1024HashToU8 => 1,
            Self::BHP1024HashToU16 => 1,
            Self::BHP1024HashToU32 => 1,
            Self::BHP1024HashToU64 => 1,
            Self::BHP1024HashToU128 => 1,
            Self::BHP1024HashToScalar => 1,

            Self::ChaChaRandAddress => 0,
            Self::ChaChaRandBool => 0,
            Self::ChaChaRandField => 0,
            Self::ChaChaRandGroup => 0,
            Self::ChaChaRandI8 => 0,
            Self::ChaChaRandI16 => 0,
            Self::ChaChaRandI32 => 0,
            Self::ChaChaRandI64 => 0,
            Self::ChaChaRandI128 => 0,
            Self::ChaChaRandU8 => 0,
            Self::ChaChaRandU16 => 0,
            Self::ChaChaRandU32 => 0,
            Self::ChaChaRandU64 => 0,
            Self::ChaChaRandU128 => 0,
            Self::ChaChaRandScalar => 0,

            Self::Keccak256HashToAddress => 1,
            Self::Keccak256HashToField => 1,
            Self::Keccak256HashToGroup => 1,
            Self::Keccak256HashToI8 => 1,
            Self::Keccak256HashToI16 => 1,
            Self::Keccak256HashToI32 => 1,
            Self::Keccak256HashToI64 => 1,
            Self::Keccak256HashToI128 => 1,
            Self::Keccak256HashToU8 => 1,
            Self::Keccak256HashToU16 => 1,
            Self::Keccak256HashToU32 => 1,
            Self::Keccak256HashToU64 => 1,
            Self::Keccak256HashToU128 => 1,
            Self::Keccak256HashToScalar => 1,

            Self::Keccak384HashToAddress => 1,
            Self::Keccak384HashToField => 1,
            Self::Keccak384HashToGroup => 1,
            Self::Keccak384HashToI8 => 1,
            Self::Keccak384HashToI16 => 1,
            Self::Keccak384HashToI32 => 1,
            Self::Keccak384HashToI64 => 1,
            Self::Keccak384HashToI128 => 1,
            Self::Keccak384HashToU8 => 1,
            Self::Keccak384HashToU16 => 1,
            Self::Keccak384HashToU32 => 1,
            Self::Keccak384HashToU64 => 1,
            Self::Keccak384HashToU128 => 1,
            Self::Keccak384HashToScalar => 1,

            Self::Keccak512HashToAddress => 1,
            Self::Keccak512HashToField => 1,
            Self::Keccak512HashToGroup => 1,
            Self::Keccak512HashToI8 => 1,
            Self::Keccak512HashToI16 => 1,
            Self::Keccak512HashToI32 => 1,
            Self::Keccak512HashToI64 => 1,
            Self::Keccak512HashToI128 => 1,
            Self::Keccak512HashToU8 => 1,
            Self::Keccak512HashToU16 => 1,
            Self::Keccak512HashToU32 => 1,
            Self::Keccak512HashToU64 => 1,
            Self::Keccak512HashToU128 => 1,
            Self::Keccak512HashToScalar => 1,

            Self::Pedersen64CommitToAddress => 2,
            Self::Pedersen64CommitToField => 2,
            Self::Pedersen64CommitToGroup => 2,
            Self::Pedersen64HashToAddress => 1,
            Self::Pedersen64HashToField => 1,
            Self::Pedersen64HashToGroup => 1,
            Self::Pedersen64HashToI8 => 1,
            Self::Pedersen64HashToI16 => 1,
            Self::Pedersen64HashToI32 => 1,
            Self::Pedersen64HashToI64 => 1,
            Self::Pedersen64HashToI128 => 1,
            Self::Pedersen64HashToU8 => 1,
            Self::Pedersen64HashToU16 => 1,
            Self::Pedersen64HashToU32 => 1,
            Self::Pedersen64HashToU64 => 1,
            Self::Pedersen64HashToU128 => 1,
            Self::Pedersen64HashToScalar => 1,

            Self::Pedersen128CommitToAddress => 2,
            Self::Pedersen128CommitToField => 2,
            Self::Pedersen128CommitToGroup => 2,
            Self::Pedersen128HashToAddress => 1,
            Self::Pedersen128HashToField => 1,
            Self::Pedersen128HashToGroup => 1,
            Self::Pedersen128HashToI8 => 1,
            Self::Pedersen128HashToI16 => 1,
            Self::Pedersen128HashToI32 => 1,
            Self::Pedersen128HashToI64 => 1,
            Self::Pedersen128HashToI128 => 1,
            Self::Pedersen128HashToU8 => 1,
            Self::Pedersen128HashToU16 => 1,
            Self::Pedersen128HashToU32 => 1,
            Self::Pedersen128HashToU64 => 1,
            Self::Pedersen128HashToU128 => 1,
            Self::Pedersen128HashToScalar => 1,

            Self::Poseidon2HashToAddress => 1,
            Self::Poseidon2HashToField => 1,
            Self::Poseidon2HashToGroup => 1,
            Self::Poseidon2HashToI8 => 1,
            Self::Poseidon2HashToI16 => 1,
            Self::Poseidon2HashToI32 => 1,
            Self::Poseidon2HashToI64 => 1,
            Self::Poseidon2HashToI128 => 1,
            Self::Poseidon2HashToU8 => 1,
            Self::Poseidon2HashToU16 => 1,
            Self::Poseidon2HashToU32 => 1,
            Self::Poseidon2HashToU64 => 1,
            Self::Poseidon2HashToU128 => 1,
            Self::Poseidon2HashToScalar => 1,

            Self::Poseidon4HashToAddress => 1,
            Self::Poseidon4HashToField => 1,
            Self::Poseidon4HashToGroup => 1,
            Self::Poseidon4HashToI8 => 1,
            Self::Poseidon4HashToI16 => 1,
            Self::Poseidon4HashToI32 => 1,
            Self::Poseidon4HashToI64 => 1,
            Self::Poseidon4HashToI128 => 1,
            Self::Poseidon4HashToU8 => 1,
            Self::Poseidon4HashToU16 => 1,
            Self::Poseidon4HashToU32 => 1,
            Self::Poseidon4HashToU64 => 1,
            Self::Poseidon4HashToU128 => 1,
            Self::Poseidon4HashToScalar => 1,

            Self::Poseidon8HashToAddress => 1,
            Self::Poseidon8HashToField => 1,
            Self::Poseidon8HashToGroup => 1,
            Self::Poseidon8HashToI8 => 1,
            Self::Poseidon8HashToI16 => 1,
            Self::Poseidon8HashToI32 => 1,
            Self::Poseidon8HashToI64 => 1,
            Self::Poseidon8HashToI128 => 1,
            Self::Poseidon8HashToU8 => 1,
            Self::Poseidon8HashToU16 => 1,
            Self::Poseidon8HashToU32 => 1,
            Self::Poseidon8HashToU64 => 1,
            Self::Poseidon8HashToU128 => 1,
            Self::Poseidon8HashToScalar => 1,

            Self::SHA3_256HashToAddress => 1,
            Self::SHA3_256HashToField => 1,
            Self::SHA3_256HashToGroup => 1,
            Self::SHA3_256HashToI8 => 1,
            Self::SHA3_256HashToI16 => 1,
            Self::SHA3_256HashToI32 => 1,
            Self::SHA3_256HashToI64 => 1,
            Self::SHA3_256HashToI128 => 1,
            Self::SHA3_256HashToU8 => 1,
            Self::SHA3_256HashToU16 => 1,
            Self::SHA3_256HashToU32 => 1,
            Self::SHA3_256HashToU64 => 1,
            Self::SHA3_256HashToU128 => 1,
            Self::SHA3_256HashToScalar => 1,

            Self::SHA3_384HashToAddress => 1,
            Self::SHA3_384HashToField => 1,
            Self::SHA3_384HashToGroup => 1,
            Self::SHA3_384HashToI8 => 1,
            Self::SHA3_384HashToI16 => 1,
            Self::SHA3_384HashToI32 => 1,
            Self::SHA3_384HashToI64 => 1,
            Self::SHA3_384HashToI128 => 1,
            Self::SHA3_384HashToU8 => 1,
            Self::SHA3_384HashToU16 => 1,
            Self::SHA3_384HashToU32 => 1,
            Self::SHA3_384HashToU64 => 1,
            Self::SHA3_384HashToU128 => 1,
            Self::SHA3_384HashToScalar => 1,

            Self::SHA3_512HashToAddress => 1,
            Self::SHA3_512HashToField => 1,
            Self::SHA3_512HashToGroup => 1,
            Self::SHA3_512HashToI8 => 1,
            Self::SHA3_512HashToI16 => 1,
            Self::SHA3_512HashToI32 => 1,
            Self::SHA3_512HashToI64 => 1,
            Self::SHA3_512HashToI128 => 1,
            Self::SHA3_512HashToU8 => 1,
            Self::SHA3_512HashToU16 => 1,
            Self::SHA3_512HashToU32 => 1,
            Self::SHA3_512HashToU64 => 1,
            Self::SHA3_512HashToU128 => 1,
            Self::SHA3_512HashToScalar => 1,

            Self::MappingGet => 2,
            Self::MappingGetOrUse => 3,
            Self::MappingSet => 3,
            Self::MappingRemove => 2,
            Self::MappingContains => 2,

            Self::GroupToXCoordinate => 1,
            Self::GroupToYCoordinate => 1,

            Self::SignatureVerify => 3,
        }
    }

    /// Returns whether or not this function is finalize command.
    pub fn is_finalize_command(&self) -> bool {
        match self {
            CoreFunction::ChaChaRandAddress
            | CoreFunction::ChaChaRandBool
            | CoreFunction::ChaChaRandField
            | CoreFunction::ChaChaRandGroup
            | CoreFunction::ChaChaRandI8
            | CoreFunction::ChaChaRandI16
            | CoreFunction::ChaChaRandI32
            | CoreFunction::ChaChaRandI64
            | CoreFunction::ChaChaRandI128
            | CoreFunction::ChaChaRandU8
            | CoreFunction::ChaChaRandU16
            | CoreFunction::ChaChaRandU32
            | CoreFunction::ChaChaRandU64
            | CoreFunction::ChaChaRandU128
            | CoreFunction::MappingGet
            | CoreFunction::MappingGetOrUse
            | CoreFunction::ChaChaRandScalar
            | CoreFunction::MappingSet
            | CoreFunction::MappingRemove
            | CoreFunction::MappingContains => true,
            CoreFunction::BHP256CommitToAddress
            | CoreFunction::BHP256CommitToField
            | CoreFunction::BHP256CommitToGroup
            | CoreFunction::BHP256HashToAddress
            | CoreFunction::BHP256HashToField
            | CoreFunction::BHP256HashToGroup
            | CoreFunction::BHP256HashToI8
            | CoreFunction::BHP256HashToI16
            | CoreFunction::BHP256HashToI32
            | CoreFunction::BHP256HashToI64
            | CoreFunction::BHP256HashToI128
            | CoreFunction::BHP256HashToU8
            | CoreFunction::BHP256HashToU16
            | CoreFunction::BHP256HashToU32
            | CoreFunction::BHP256HashToU64
            | CoreFunction::BHP256HashToU128
            | CoreFunction::BHP256HashToScalar
            | CoreFunction::BHP512CommitToAddress
            | CoreFunction::BHP512CommitToField
            | CoreFunction::BHP512CommitToGroup
            | CoreFunction::BHP512HashToAddress
            | CoreFunction::BHP512HashToField
            | CoreFunction::BHP512HashToGroup
            | CoreFunction::BHP512HashToI8
            | CoreFunction::BHP512HashToI16
            | CoreFunction::BHP512HashToI32
            | CoreFunction::BHP512HashToI64
            | CoreFunction::BHP512HashToI128
            | CoreFunction::BHP512HashToU8
            | CoreFunction::BHP512HashToU16
            | CoreFunction::BHP512HashToU32
            | CoreFunction::BHP512HashToU64
            | CoreFunction::BHP512HashToU128
            | CoreFunction::BHP512HashToScalar
            | CoreFunction::BHP768CommitToAddress
            | CoreFunction::BHP768CommitToField
            | CoreFunction::BHP768CommitToGroup
            | CoreFunction::BHP768HashToAddress
            | CoreFunction::BHP768HashToField
            | CoreFunction::BHP768HashToGroup
            | CoreFunction::BHP768HashToI8
            | CoreFunction::BHP768HashToI16
            | CoreFunction::BHP768HashToI32
            | CoreFunction::BHP768HashToI64
            | CoreFunction::BHP768HashToI128
            | CoreFunction::BHP768HashToU8
            | CoreFunction::BHP768HashToU16
            | CoreFunction::BHP768HashToU32
            | CoreFunction::BHP768HashToU64
            | CoreFunction::BHP768HashToU128
            | CoreFunction::BHP768HashToScalar
            | CoreFunction::BHP1024CommitToAddress
            | CoreFunction::BHP1024CommitToField
            | CoreFunction::BHP1024CommitToGroup
            | CoreFunction::BHP1024HashToAddress
            | CoreFunction::BHP1024HashToField
            | CoreFunction::BHP1024HashToGroup
            | CoreFunction::BHP1024HashToI8
            | CoreFunction::BHP1024HashToI16
            | CoreFunction::BHP1024HashToI32
            | CoreFunction::BHP1024HashToI64
            | CoreFunction::BHP1024HashToI128
            | CoreFunction::BHP1024HashToU8
            | CoreFunction::BHP1024HashToU16
            | CoreFunction::BHP1024HashToU32
            | CoreFunction::BHP1024HashToU64
            | CoreFunction::BHP1024HashToU128
            | CoreFunction::BHP1024HashToScalar
            | CoreFunction::Keccak256HashToAddress
            | CoreFunction::Keccak256HashToField
            | CoreFunction::Keccak256HashToGroup
            | CoreFunction::Keccak256HashToI8
            | CoreFunction::Keccak256HashToI16
            | CoreFunction::Keccak256HashToI32
            | CoreFunction::Keccak256HashToI64
            | CoreFunction::Keccak256HashToI128
            | CoreFunction::Keccak256HashToU8
            | CoreFunction::Keccak256HashToU16
            | CoreFunction::Keccak256HashToU32
            | CoreFunction::Keccak256HashToU64
            | CoreFunction::Keccak256HashToU128
            | CoreFunction::Keccak256HashToScalar
            | CoreFunction::Keccak384HashToAddress
            | CoreFunction::Keccak384HashToField
            | CoreFunction::Keccak384HashToGroup
            | CoreFunction::Keccak384HashToI8
            | CoreFunction::Keccak384HashToI16
            | CoreFunction::Keccak384HashToI32
            | CoreFunction::Keccak384HashToI64
            | CoreFunction::Keccak384HashToI128
            | CoreFunction::Keccak384HashToU8
            | CoreFunction::Keccak384HashToU16
            | CoreFunction::Keccak384HashToU32
            | CoreFunction::Keccak384HashToU64
            | CoreFunction::Keccak384HashToU128
            | CoreFunction::Keccak384HashToScalar
            | CoreFunction::Keccak512HashToAddress
            | CoreFunction::Keccak512HashToField
            | CoreFunction::Keccak512HashToGroup
            | CoreFunction::Keccak512HashToI8
            | CoreFunction::Keccak512HashToI16
            | CoreFunction::Keccak512HashToI32
            | CoreFunction::Keccak512HashToI64
            | CoreFunction::Keccak512HashToI128
            | CoreFunction::Keccak512HashToU8
            | CoreFunction::Keccak512HashToU16
            | CoreFunction::Keccak512HashToU32
            | CoreFunction::Keccak512HashToU64
            | CoreFunction::Keccak512HashToU128
            | CoreFunction::Keccak512HashToScalar
            | CoreFunction::Pedersen64CommitToAddress
            | CoreFunction::Pedersen64CommitToField
            | CoreFunction::Pedersen64CommitToGroup
            | CoreFunction::Pedersen64HashToAddress
            | CoreFunction::Pedersen64HashToField
            | CoreFunction::Pedersen64HashToGroup
            | CoreFunction::Pedersen64HashToI8
            | CoreFunction::Pedersen64HashToI16
            | CoreFunction::Pedersen64HashToI32
            | CoreFunction::Pedersen64HashToI64
            | CoreFunction::Pedersen64HashToI128
            | CoreFunction::Pedersen64HashToU8
            | CoreFunction::Pedersen64HashToU16
            | CoreFunction::Pedersen64HashToU32
            | CoreFunction::Pedersen64HashToU64
            | CoreFunction::Pedersen64HashToU128
            | CoreFunction::Pedersen64HashToScalar
            | CoreFunction::Pedersen128CommitToAddress
            | CoreFunction::Pedersen128CommitToField
            | CoreFunction::Pedersen128CommitToGroup
            | CoreFunction::Pedersen128HashToAddress
            | CoreFunction::Pedersen128HashToField
            | CoreFunction::Pedersen128HashToGroup
            | CoreFunction::Pedersen128HashToI8
            | CoreFunction::Pedersen128HashToI16
            | CoreFunction::Pedersen128HashToI32
            | CoreFunction::Pedersen128HashToI64
            | CoreFunction::Pedersen128HashToI128
            | CoreFunction::Pedersen128HashToU8
            | CoreFunction::Pedersen128HashToU16
            | CoreFunction::Pedersen128HashToU32
            | CoreFunction::Pedersen128HashToU64
            | CoreFunction::Pedersen128HashToU128
            | CoreFunction::Pedersen128HashToScalar
            | CoreFunction::Poseidon2HashToAddress
            | CoreFunction::Poseidon2HashToField
            | CoreFunction::Poseidon2HashToGroup
            | CoreFunction::Poseidon2HashToI8
            | CoreFunction::Poseidon2HashToI16
            | CoreFunction::Poseidon2HashToI32
            | CoreFunction::Poseidon2HashToI64
            | CoreFunction::Poseidon2HashToI128
            | CoreFunction::Poseidon2HashToU8
            | CoreFunction::Poseidon2HashToU16
            | CoreFunction::Poseidon2HashToU32
            | CoreFunction::Poseidon2HashToU64
            | CoreFunction::Poseidon2HashToU128
            | CoreFunction::Poseidon2HashToScalar
            | CoreFunction::Poseidon4HashToAddress
            | CoreFunction::Poseidon4HashToField
            | CoreFunction::Poseidon4HashToGroup
            | CoreFunction::Poseidon4HashToI8
            | CoreFunction::Poseidon4HashToI16
            | CoreFunction::Poseidon4HashToI32
            | CoreFunction::Poseidon4HashToI64
            | CoreFunction::Poseidon4HashToI128
            | CoreFunction::Poseidon4HashToU8
            | CoreFunction::Poseidon4HashToU16
            | CoreFunction::Poseidon4HashToU32
            | CoreFunction::Poseidon4HashToU64
            | CoreFunction::Poseidon4HashToU128
            | CoreFunction::Poseidon4HashToScalar
            | CoreFunction::Poseidon8HashToAddress
            | CoreFunction::Poseidon8HashToField
            | CoreFunction::Poseidon8HashToGroup
            | CoreFunction::Poseidon8HashToI8
            | CoreFunction::Poseidon8HashToI16
            | CoreFunction::Poseidon8HashToI32
            | CoreFunction::Poseidon8HashToI64
            | CoreFunction::Poseidon8HashToI128
            | CoreFunction::Poseidon8HashToU8
            | CoreFunction::Poseidon8HashToU16
            | CoreFunction::Poseidon8HashToU32
            | CoreFunction::Poseidon8HashToU64
            | CoreFunction::Poseidon8HashToU128
            | CoreFunction::Poseidon8HashToScalar
            | CoreFunction::SHA3_256HashToAddress
            | CoreFunction::SHA3_256HashToField
            | CoreFunction::SHA3_256HashToGroup
            | CoreFunction::SHA3_256HashToI8
            | CoreFunction::SHA3_256HashToI16
            | CoreFunction::SHA3_256HashToI32
            | CoreFunction::SHA3_256HashToI64
            | CoreFunction::SHA3_256HashToI128
            | CoreFunction::SHA3_256HashToU8
            | CoreFunction::SHA3_256HashToU16
            | CoreFunction::SHA3_256HashToU32
            | CoreFunction::SHA3_256HashToU64
            | CoreFunction::SHA3_256HashToU128
            | CoreFunction::SHA3_256HashToScalar
            | CoreFunction::SHA3_384HashToAddress
            | CoreFunction::SHA3_384HashToField
            | CoreFunction::SHA3_384HashToGroup
            | CoreFunction::SHA3_384HashToI8
            | CoreFunction::SHA3_384HashToI16
            | CoreFunction::SHA3_384HashToI32
            | CoreFunction::SHA3_384HashToI64
            | CoreFunction::SHA3_384HashToI128
            | CoreFunction::SHA3_384HashToU8
            | CoreFunction::SHA3_384HashToU16
            | CoreFunction::SHA3_384HashToU32
            | CoreFunction::SHA3_384HashToU64
            | CoreFunction::SHA3_384HashToU128
            | CoreFunction::SHA3_384HashToScalar
            | CoreFunction::SHA3_512HashToAddress
            | CoreFunction::SHA3_512HashToField
            | CoreFunction::SHA3_512HashToGroup
            | CoreFunction::SHA3_512HashToI8
            | CoreFunction::SHA3_512HashToI16
            | CoreFunction::SHA3_512HashToI32
            | CoreFunction::SHA3_512HashToI64
            | CoreFunction::SHA3_512HashToI128
            | CoreFunction::SHA3_512HashToU8
            | CoreFunction::SHA3_512HashToU16
            | CoreFunction::SHA3_512HashToU32
            | CoreFunction::SHA3_512HashToU64
            | CoreFunction::SHA3_512HashToU128
            | CoreFunction::SHA3_512HashToScalar
            | CoreFunction::GroupToXCoordinate
            | CoreFunction::GroupToYCoordinate
            | CoreFunction::SignatureVerify => false,
        }
    }
}
