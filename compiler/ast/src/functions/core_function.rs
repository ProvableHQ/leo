// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{ArrayType, AssociatedFunctionExpression, IntegerType, Type};

use leo_span::{Symbol, sym};

use snarkvm::{
    prelude::LiteralType,
    synthesizer::program::{CommitVariant, DeserializeVariant, ECDSAVerifyVariant, HashVariant, SerializeVariant},
};

/// A core instruction that maps directly to an AVM bytecode instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreFunction {
    // ChaCha random value of type (LiteralType).
    ChaChaRand(LiteralType),
    // Commitment to a value using hash (HashVariant) returning value of type (LiteralType).
    Commit(CommitVariant, LiteralType),
    // ECDSA verify with hash (HashVariant) and the ETH variant (bool).
    ECDSAVerify(ECDSAVerifyVariant),
    // Hash function with variant (HashVariant) returning value of type (LiteralType).
    Hash(HashVariant, Type),

    // These are used for both mappings and vectors
    Get,
    Set,

    MappingGetOrUse,
    MappingRemove,
    MappingContains,

    OptionalUnwrap,
    OptionalUnwrapOr,

    VectorPush,
    VectorLen,
    VectorClear,
    VectorPop,
    VectorSwapRemove,

    GroupToXCoordinate,
    GroupToYCoordinate,

    // Schnorr signature verification.
    SignatureVerify,

    FutureAwait,

    ProgramChecksum,
    ProgramEdition,
    ProgramOwner,

    Serialize(SerializeVariant),

    // Note. `Deserialize` cannot be instantiated via `from_symbols` as it requires a type argument.
    Deserialize(DeserializeVariant, Type),

    CheatCodePrintMapping,
    CheatCodeSetBlockHeight,
}

impl CoreFunction {
    /// Returns a `CoreFunction` from the given module and method symbols.
    #[rustfmt::skip]
    pub fn from_symbols(module: Symbol, function: Symbol) -> Option<Self> {
        Some(match (module, function) {
            (sym::ChaCha, sym::rand_address) => Self::ChaChaRand(LiteralType::Address),
            (sym::ChaCha, sym::rand_bool)    => Self::ChaChaRand(LiteralType::Boolean),
            (sym::ChaCha, sym::rand_field)   => Self::ChaChaRand(LiteralType::Field),
            (sym::ChaCha, sym::rand_group)   => Self::ChaChaRand(LiteralType::Group),
            (sym::ChaCha, sym::rand_i8)      => Self::ChaChaRand(LiteralType::I8),
            (sym::ChaCha, sym::rand_i16)     => Self::ChaChaRand(LiteralType::I16),
            (sym::ChaCha, sym::rand_i32)     => Self::ChaChaRand(LiteralType::I32),
            (sym::ChaCha, sym::rand_i64)     => Self::ChaChaRand(LiteralType::I64),
            (sym::ChaCha, sym::rand_i128)    => Self::ChaChaRand(LiteralType::I128),
            (sym::ChaCha, sym::rand_u8)      => Self::ChaChaRand(LiteralType::U8),
            (sym::ChaCha, sym::rand_u16)     => Self::ChaChaRand(LiteralType::U16),
            (sym::ChaCha, sym::rand_u32)     => Self::ChaChaRand(LiteralType::U32),
            (sym::ChaCha, sym::rand_u64)     => Self::ChaChaRand(LiteralType::U64),
            (sym::ChaCha, sym::rand_u128)    => Self::ChaChaRand(LiteralType::U128),
            (sym::ChaCha, sym::rand_scalar)  => Self::ChaChaRand(LiteralType::Scalar),

            (sym::BHP256, sym::commit_to_address)   => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Address),
            (sym::BHP256, sym::commit_to_field)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Field),
            (sym::BHP256, sym::commit_to_group)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Group),
            (sym::BHP512, sym::commit_to_address)   => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Address),
            (sym::BHP512, sym::commit_to_field)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Field),
            (sym::BHP512, sym::commit_to_group)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Group),
            (sym::BHP768, sym::commit_to_address)   => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Address),
            (sym::BHP768, sym::commit_to_field)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Field),
            (sym::BHP768, sym::commit_to_group)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Group),
            (sym::BHP1024, sym::commit_to_address)   => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Address),
            (sym::BHP1024, sym::commit_to_field)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Field),
            (sym::BHP1024, sym::commit_to_group)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Group),
            (sym::Pedersen64, sym::commit_to_address)   => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Address),
            (sym::Pedersen64, sym::commit_to_field)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Field),
            (sym::Pedersen64, sym::commit_to_group)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Group),
            (sym::Pedersen128, sym::commit_to_address)   => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Address),
            (sym::Pedersen128, sym::commit_to_field)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Field),
            (sym::Pedersen128, sym::commit_to_group)     => Self::Commit(CommitVariant::CommitBHP256, LiteralType::Group),

            (sym::BHP256, sym::hash_to_address)     => Self::Hash(HashVariant::HashBHP256, Type::Address),
            (sym::BHP256, sym::hash_to_field)       => Self::Hash(HashVariant::HashBHP256, Type::Field),
            (sym::BHP256, sym::hash_to_group)       => Self::Hash(HashVariant::HashBHP256, Type::Group),
            (sym::BHP256, sym::hash_to_i8)          => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::I8)),
            (sym::BHP256, sym::hash_to_i16)         => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::I16)),
            (sym::BHP256, sym::hash_to_i32)         => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::I32)),
            (sym::BHP256, sym::hash_to_i64)         => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::I64)),
            (sym::BHP256, sym::hash_to_i128)        => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::I128)),
            (sym::BHP256, sym::hash_to_u8)          => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::U8)),
            (sym::BHP256, sym::hash_to_u16)         => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::U16)),
            (sym::BHP256, sym::hash_to_u32)         => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::U32)),
            (sym::BHP256, sym::hash_to_u64)         => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::U64)),
            (sym::BHP256, sym::hash_to_u128)        => Self::Hash(HashVariant::HashBHP256, Type::Integer(IntegerType::U128)),
            (sym::BHP256, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashBHP256, Type::Scalar),
            (sym::BHP256, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashBHP256Raw, Type::Address),
            (sym::BHP256, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashBHP256Raw, Type::Field),
            (sym::BHP256, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashBHP256Raw, Type::Group),
            (sym::BHP256, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::I8)),
            (sym::BHP256, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::I16)),
            (sym::BHP256, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::I32)),
            (sym::BHP256, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::I64)),
            (sym::BHP256, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::I128)),
            (sym::BHP256, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::U8)),
            (sym::BHP256, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::U16)),
            (sym::BHP256, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::U32)),
            (sym::BHP256, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::U64)),
            (sym::BHP256, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashBHP256Raw, Type::Integer(IntegerType::U128)),
            (sym::BHP256, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashBHP256Raw, Type::Scalar),

            (sym::BHP512, sym::hash_to_address)     => Self::Hash(HashVariant::HashBHP512, Type::Address),
            (sym::BHP512, sym::hash_to_field)       => Self::Hash(HashVariant::HashBHP512, Type::Field),
            (sym::BHP512, sym::hash_to_group)       => Self::Hash(HashVariant::HashBHP512, Type::Group),
            (sym::BHP512, sym::hash_to_i8)          => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::I8)),
            (sym::BHP512, sym::hash_to_i16)         => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::I16)),
            (sym::BHP512, sym::hash_to_i32)         => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::I32)),
            (sym::BHP512, sym::hash_to_i64)         => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::I64)),
            (sym::BHP512, sym::hash_to_i128)        => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::I128)),
            (sym::BHP512, sym::hash_to_u8)          => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::U8)),
            (sym::BHP512, sym::hash_to_u16)         => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::U16)),
            (sym::BHP512, sym::hash_to_u32)         => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::U32)),
            (sym::BHP512, sym::hash_to_u64)         => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::U64)),
            (sym::BHP512, sym::hash_to_u128)        => Self::Hash(HashVariant::HashBHP512, Type::Integer(IntegerType::U128)),
            (sym::BHP512, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashBHP512, Type::Scalar),
            (sym::BHP512, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashBHP512Raw, Type::Address),
            (sym::BHP512, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashBHP512Raw, Type::Field),
            (sym::BHP512, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashBHP512Raw, Type::Group),
            (sym::BHP512, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::I8)),
            (sym::BHP512, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::I16)),
            (sym::BHP512, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::I32)),
            (sym::BHP512, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::I64)),
            (sym::BHP512, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::I128)),
            (sym::BHP512, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::U8)),
            (sym::BHP512, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::U16)),
            (sym::BHP512, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::U32)),
            (sym::BHP512, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::U64)),
            (sym::BHP512, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashBHP512Raw, Type::Integer(IntegerType::U128)),
            (sym::BHP512, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashBHP512Raw, Type::Scalar),

            (sym::BHP768, sym::hash_to_address)     => Self::Hash(HashVariant::HashBHP768, Type::Address),
            (sym::BHP768, sym::hash_to_field)       => Self::Hash(HashVariant::HashBHP768, Type::Field),
            (sym::BHP768, sym::hash_to_group)       => Self::Hash(HashVariant::HashBHP768, Type::Group),
            (sym::BHP768, sym::hash_to_i8)          => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::I8)),
            (sym::BHP768, sym::hash_to_i16)         => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::I16)),
            (sym::BHP768, sym::hash_to_i32)         => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::I32)),
            (sym::BHP768, sym::hash_to_i64)         => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::I64)),
            (sym::BHP768, sym::hash_to_i128)        => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::I128)),
            (sym::BHP768, sym::hash_to_u8)          => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::U8)),
            (sym::BHP768, sym::hash_to_u16)         => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::U16)),
            (sym::BHP768, sym::hash_to_u32)         => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::U32)),
            (sym::BHP768, sym::hash_to_u64)         => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::U64)),
            (sym::BHP768, sym::hash_to_u128)        => Self::Hash(HashVariant::HashBHP768, Type::Integer(IntegerType::U128)),
            (sym::BHP768, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashBHP768, Type::Scalar),
            (sym::BHP768, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashBHP768Raw, Type::Address),
            (sym::BHP768, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashBHP768Raw, Type::Field),
            (sym::BHP768, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashBHP768Raw, Type::Group),
            (sym::BHP768, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::I8)),
            (sym::BHP768, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::I16)),
            (sym::BHP768, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::I32)),
            (sym::BHP768, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::I64)),
            (sym::BHP768, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::I128)),
            (sym::BHP768, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::U8)),
            (sym::BHP768, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::U16)),
            (sym::BHP768, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::U32)),
            (sym::BHP768, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::U64)),
            (sym::BHP768, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashBHP768Raw, Type::Integer(IntegerType::U128)),
            (sym::BHP768, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashBHP768Raw, Type::Scalar),

            (sym::BHP1024, sym::hash_to_address)     => Self::Hash(HashVariant::HashBHP1024, Type::Address),
            (sym::BHP1024, sym::hash_to_field)       => Self::Hash(HashVariant::HashBHP1024, Type::Field),
            (sym::BHP1024, sym::hash_to_group)       => Self::Hash(HashVariant::HashBHP1024, Type::Group),
            (sym::BHP1024, sym::hash_to_i8)          => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::I8)),
            (sym::BHP1024, sym::hash_to_i16)         => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::I16)),
            (sym::BHP1024, sym::hash_to_i32)         => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::I32)),
            (sym::BHP1024, sym::hash_to_i64)         => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::I64)),
            (sym::BHP1024, sym::hash_to_i128)        => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::I128)),
            (sym::BHP1024, sym::hash_to_u8)          => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::U8)),
            (sym::BHP1024, sym::hash_to_u16)         => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::U16)),
            (sym::BHP1024, sym::hash_to_u32)         => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::U32)),
            (sym::BHP1024, sym::hash_to_u64)         => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::U64)),
            (sym::BHP1024, sym::hash_to_u128)        => Self::Hash(HashVariant::HashBHP1024, Type::Integer(IntegerType::U128)),
            (sym::BHP1024, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashBHP1024, Type::Scalar),
            (sym::BHP1024, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashBHP1024Raw, Type::Address),
            (sym::BHP1024, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashBHP1024Raw, Type::Field),
            (sym::BHP1024, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashBHP1024Raw, Type::Group),
            (sym::BHP1024, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::I8)),
            (sym::BHP1024, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::I16)),
            (sym::BHP1024, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::I32)),
            (sym::BHP1024, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::I64)),
            (sym::BHP1024, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::I128)),
            (sym::BHP1024, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::U8)),
            (sym::BHP1024, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::U16)),
            (sym::BHP1024, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::U32)),
            (sym::BHP1024, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::U64)),
            (sym::BHP1024, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashBHP1024Raw, Type::Integer(IntegerType::U128)),
            (sym::BHP1024, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashBHP1024Raw, Type::Scalar),

            (sym::Keccak256, sym::hash_to_address)     => Self::Hash(HashVariant::HashKeccak256, Type::Address),
            (sym::Keccak256, sym::hash_to_field)       => Self::Hash(HashVariant::HashKeccak256, Type::Field),
            (sym::Keccak256, sym::hash_to_group)       => Self::Hash(HashVariant::HashKeccak256, Type::Group),
            (sym::Keccak256, sym::hash_to_i8)          => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::I8)),
            (sym::Keccak256, sym::hash_to_i16)         => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::I16)),
            (sym::Keccak256, sym::hash_to_i32)         => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::I32)),
            (sym::Keccak256, sym::hash_to_i64)         => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::I64)),
            (sym::Keccak256, sym::hash_to_i128)        => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::I128)),
            (sym::Keccak256, sym::hash_to_u8)          => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::U8)),
            (sym::Keccak256, sym::hash_to_u16)         => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::U16)),
            (sym::Keccak256, sym::hash_to_u32)         => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::U32)),
            (sym::Keccak256, sym::hash_to_u64)         => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::U64)),
            (sym::Keccak256, sym::hash_to_u128)        => Self::Hash(HashVariant::HashKeccak256, Type::Integer(IntegerType::U128)),
            (sym::Keccak256, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashKeccak256, Type::Scalar),
            (sym::Keccak256, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashKeccak256Raw, Type::Address),
            (sym::Keccak256, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashKeccak256Raw, Type::Field),
            (sym::Keccak256, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashKeccak256Raw, Type::Group),
            (sym::Keccak256, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::I8)),
            (sym::Keccak256, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::I16)),
            (sym::Keccak256, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::I32)),
            (sym::Keccak256, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::I64)),
            (sym::Keccak256, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::I128)),
            (sym::Keccak256, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::U8)),
            (sym::Keccak256, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::U16)),
            (sym::Keccak256, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::U32)),
            (sym::Keccak256, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::U64)),
            (sym::Keccak256, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashKeccak256Raw, Type::Integer(IntegerType::U128)),
            (sym::Keccak256, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashKeccak256Raw, Type::Scalar),
            (sym::Keccak256, sym::hash_native)         => Self::Hash(HashVariant::HashKeccak256Native, Type::Array(ArrayType::bit_array(256))),
            (sym::Keccak256, sym::hash_native_raw)     => Self::Hash(HashVariant::HashKeccak256NativeRaw, Type::Array(ArrayType::bit_array(256))),

            (sym::Keccak384, sym::hash_to_address)     => Self::Hash(HashVariant::HashKeccak384, Type::Address),
            (sym::Keccak384, sym::hash_to_field)       => Self::Hash(HashVariant::HashKeccak384, Type::Field),
            (sym::Keccak384, sym::hash_to_group)       => Self::Hash(HashVariant::HashKeccak384, Type::Group),
            (sym::Keccak384, sym::hash_to_i8)          => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::I8)),
            (sym::Keccak384, sym::hash_to_i16)         => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::I16)),
            (sym::Keccak384, sym::hash_to_i32)         => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::I32)),
            (sym::Keccak384, sym::hash_to_i64)         => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::I64)),
            (sym::Keccak384, sym::hash_to_i128)        => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::I128)),
            (sym::Keccak384, sym::hash_to_u8)          => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::U8)),
            (sym::Keccak384, sym::hash_to_u16)         => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::U16)),
            (sym::Keccak384, sym::hash_to_u32)         => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::U32)),
            (sym::Keccak384, sym::hash_to_u64)         => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::U64)),
            (sym::Keccak384, sym::hash_to_u128)        => Self::Hash(HashVariant::HashKeccak384, Type::Integer(IntegerType::U128)),
            (sym::Keccak384, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashKeccak384, Type::Scalar),
            (sym::Keccak384, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashKeccak384Raw, Type::Address),
            (sym::Keccak384, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashKeccak384Raw, Type::Field),
            (sym::Keccak384, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashKeccak384Raw, Type::Group),
            (sym::Keccak384, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::I8)),
            (sym::Keccak384, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::I16)),
            (sym::Keccak384, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::I32)),
            (sym::Keccak384, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::I64)),
            (sym::Keccak384, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::I128)),
            (sym::Keccak384, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::U8)),
            (sym::Keccak384, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::U16)),
            (sym::Keccak384, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::U32)),
            (sym::Keccak384, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::U64)),
            (sym::Keccak384, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashKeccak384Raw, Type::Integer(IntegerType::U128)),
            (sym::Keccak384, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashKeccak384Raw, Type::Scalar),
            (sym::Keccak384, sym::hash_native)         => Self::Hash(HashVariant::HashKeccak384Native, Type::Array(ArrayType::bit_array(384))),
            (sym::Keccak384, sym::hash_native_raw)     => Self::Hash(HashVariant::HashKeccak384NativeRaw, Type::Array(ArrayType::bit_array(384))),

            (sym::Keccak512, sym::hash_to_address)     => Self::Hash(HashVariant::HashKeccak512, Type::Address),
            (sym::Keccak512, sym::hash_to_field)       => Self::Hash(HashVariant::HashKeccak512, Type::Field),
            (sym::Keccak512, sym::hash_to_group)       => Self::Hash(HashVariant::HashKeccak512, Type::Group),
            (sym::Keccak512, sym::hash_to_i8)          => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::I8)),
            (sym::Keccak512, sym::hash_to_i16)         => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::I16)),
            (sym::Keccak512, sym::hash_to_i32)         => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::I32)),
            (sym::Keccak512, sym::hash_to_i64)         => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::I64)),
            (sym::Keccak512, sym::hash_to_i128)        => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::I128)),
            (sym::Keccak512, sym::hash_to_u8)          => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::U8)),
            (sym::Keccak512, sym::hash_to_u16)         => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::U16)),
            (sym::Keccak512, sym::hash_to_u32)         => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::U32)),
            (sym::Keccak512, sym::hash_to_u64)         => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::U64)),
            (sym::Keccak512, sym::hash_to_u128)        => Self::Hash(HashVariant::HashKeccak512, Type::Integer(IntegerType::U128)),
            (sym::Keccak512, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashKeccak512, Type::Scalar),
            (sym::Keccak512, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashKeccak512Raw, Type::Address),
            (sym::Keccak512, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashKeccak512Raw, Type::Field),
            (sym::Keccak512, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashKeccak512Raw, Type::Group),
            (sym::Keccak512, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::I8)),
            (sym::Keccak512, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::I16)),
            (sym::Keccak512, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::I32)),
            (sym::Keccak512, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::I64)),
            (sym::Keccak512, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::I128)),
            (sym::Keccak512, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::U8)),
            (sym::Keccak512, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::U16)),
            (sym::Keccak512, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::U32)),
            (sym::Keccak512, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::U64)),
            (sym::Keccak512, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashKeccak512Raw, Type::Integer(IntegerType::U128)),
            (sym::Keccak512, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashKeccak512Raw, Type::Scalar),
            (sym::Keccak512, sym::hash_native)         => Self::Hash(HashVariant::HashKeccak512Native, Type::Array(ArrayType::bit_array(512))),
            (sym::Keccak512, sym::hash_native_raw)     => Self::Hash(HashVariant::HashKeccak512NativeRaw, Type::Array(ArrayType::bit_array(512))),

            (sym::Pedersen64, sym::hash_to_address)     => Self::Hash(HashVariant::HashPED64, Type::Address),
            (sym::Pedersen64, sym::hash_to_field)       => Self::Hash(HashVariant::HashPED64, Type::Field),
            (sym::Pedersen64, sym::hash_to_group)       => Self::Hash(HashVariant::HashPED64, Type::Group),
            (sym::Pedersen64, sym::hash_to_i8)          => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::I8)),
            (sym::Pedersen64, sym::hash_to_i16)         => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::I16)),
            (sym::Pedersen64, sym::hash_to_i32)         => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::I32)),
            (sym::Pedersen64, sym::hash_to_i64)         => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::I64)),
            (sym::Pedersen64, sym::hash_to_i128)        => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::I128)),
            (sym::Pedersen64, sym::hash_to_u8)          => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::U8)),
            (sym::Pedersen64, sym::hash_to_u16)         => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::U16)),
            (sym::Pedersen64, sym::hash_to_u32)         => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::U32)),
            (sym::Pedersen64, sym::hash_to_u64)         => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::U64)),
            (sym::Pedersen64, sym::hash_to_u128)        => Self::Hash(HashVariant::HashPED64, Type::Integer(IntegerType::U128)),
            (sym::Pedersen64, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashPED64, Type::Scalar),
            (sym::Pedersen64, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashPED64Raw, Type::Address),
            (sym::Pedersen64, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashPED64Raw, Type::Field),
            (sym::Pedersen64, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashPED64Raw, Type::Group),
            (sym::Pedersen64, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::I8)),
            (sym::Pedersen64, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::I16)),
            (sym::Pedersen64, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::I32)),
            (sym::Pedersen64, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::I64)),
            (sym::Pedersen64, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::I128)),
            (sym::Pedersen64, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::U8)),
            (sym::Pedersen64, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::U16)),
            (sym::Pedersen64, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::U32)),
            (sym::Pedersen64, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::U64)),
            (sym::Pedersen64, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashPED64Raw, Type::Integer(IntegerType::U128)),
            (sym::Pedersen64, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashPED64Raw, Type::Scalar),

            (sym::Pedersen128, sym::hash_to_address)     => Self::Hash(HashVariant::HashPED128, Type::Address),
            (sym::Pedersen128, sym::hash_to_field)       => Self::Hash(HashVariant::HashPED128, Type::Field),
            (sym::Pedersen128, sym::hash_to_group)       => Self::Hash(HashVariant::HashPED128, Type::Group),
            (sym::Pedersen128, sym::hash_to_i8)          => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::I8)),
            (sym::Pedersen128, sym::hash_to_i16)         => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::I16)),
            (sym::Pedersen128, sym::hash_to_i32)         => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::I32)),
            (sym::Pedersen128, sym::hash_to_i64)         => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::I64)),
            (sym::Pedersen128, sym::hash_to_i128)        => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::I128)),
            (sym::Pedersen128, sym::hash_to_u8)          => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::U8)),
            (sym::Pedersen128, sym::hash_to_u16)         => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::U16)),
            (sym::Pedersen128, sym::hash_to_u32)         => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::U32)),
            (sym::Pedersen128, sym::hash_to_u64)         => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::U64)),
            (sym::Pedersen128, sym::hash_to_u128)        => Self::Hash(HashVariant::HashPED128, Type::Integer(IntegerType::U128)),
            (sym::Pedersen128, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashPED128, Type::Scalar),
            (sym::Pedersen128, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashPED128Raw, Type::Address),
            (sym::Pedersen128, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashPED128Raw, Type::Field),
            (sym::Pedersen128, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashPED128Raw, Type::Group),
            (sym::Pedersen128, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::I8)),
            (sym::Pedersen128, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::I16)),
            (sym::Pedersen128, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::I32)),
            (sym::Pedersen128, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::I64)),
            (sym::Pedersen128, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::I128)),
            (sym::Pedersen128, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::U8)),
            (sym::Pedersen128, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::U16)),
            (sym::Pedersen128, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::U32)),
            (sym::Pedersen128, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::U64)),
            (sym::Pedersen128, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashPED128Raw, Type::Integer(IntegerType::U128)),
            (sym::Pedersen128, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashPED128Raw, Type::Scalar),

            (sym::Poseidon2, sym::hash_to_address)     => Self::Hash(HashVariant::HashPSD2, Type::Address),
            (sym::Poseidon2, sym::hash_to_field)       => Self::Hash(HashVariant::HashPSD2, Type::Field),
            (sym::Poseidon2, sym::hash_to_group)       => Self::Hash(HashVariant::HashPSD2, Type::Group),
            (sym::Poseidon2, sym::hash_to_i8)          => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::I8)),
            (sym::Poseidon2, sym::hash_to_i16)         => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::I16)),
            (sym::Poseidon2, sym::hash_to_i32)         => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::I32)),
            (sym::Poseidon2, sym::hash_to_i64)         => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::I64)),
            (sym::Poseidon2, sym::hash_to_i128)        => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::I128)),
            (sym::Poseidon2, sym::hash_to_u8)          => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::U8)),
            (sym::Poseidon2, sym::hash_to_u16)         => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::U16)),
            (sym::Poseidon2, sym::hash_to_u32)         => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::U32)),
            (sym::Poseidon2, sym::hash_to_u64)         => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::U64)),
            (sym::Poseidon2, sym::hash_to_u128)        => Self::Hash(HashVariant::HashPSD2, Type::Integer(IntegerType::U128)),
            (sym::Poseidon2, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashPSD2, Type::Scalar),
            (sym::Poseidon2, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashPSD2Raw, Type::Address),
            (sym::Poseidon2, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashPSD2Raw, Type::Field),
            (sym::Poseidon2, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashPSD2Raw, Type::Group),
            (sym::Poseidon2, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::I8)),
            (sym::Poseidon2, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::I16)),
            (sym::Poseidon2, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::I32)),
            (sym::Poseidon2, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::I64)),
            (sym::Poseidon2, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::I128)),
            (sym::Poseidon2, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::U8)),
            (sym::Poseidon2, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::U16)),
            (sym::Poseidon2, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::U32)),
            (sym::Poseidon2, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::U64)),
            (sym::Poseidon2, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashPSD2Raw, Type::Integer(IntegerType::U128)),
            (sym::Poseidon2, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashPSD2Raw, Type::Scalar),

            (sym::Poseidon4, sym::hash_to_address)     => Self::Hash(HashVariant::HashPSD4, Type::Address),
            (sym::Poseidon4, sym::hash_to_field)       => Self::Hash(HashVariant::HashPSD4, Type::Field),
            (sym::Poseidon4, sym::hash_to_group)       => Self::Hash(HashVariant::HashPSD4, Type::Group),
            (sym::Poseidon4, sym::hash_to_i8)          => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::I8)),
            (sym::Poseidon4, sym::hash_to_i16)         => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::I16)),
            (sym::Poseidon4, sym::hash_to_i32)         => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::I32)),
            (sym::Poseidon4, sym::hash_to_i64)         => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::I64)),
            (sym::Poseidon4, sym::hash_to_i128)        => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::I128)),
            (sym::Poseidon4, sym::hash_to_u8)          => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::U8)),
            (sym::Poseidon4, sym::hash_to_u16)         => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::U16)),
            (sym::Poseidon4, sym::hash_to_u32)         => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::U32)),
            (sym::Poseidon4, sym::hash_to_u64)         => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::U64)),
            (sym::Poseidon4, sym::hash_to_u128)        => Self::Hash(HashVariant::HashPSD4, Type::Integer(IntegerType::U128)),
            (sym::Poseidon4, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashPSD4, Type::Scalar),
            (sym::Poseidon4, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashPSD4Raw, Type::Address),
            (sym::Poseidon4, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashPSD4Raw, Type::Field),
            (sym::Poseidon4, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashPSD4Raw, Type::Group),
            (sym::Poseidon4, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::I8)),
            (sym::Poseidon4, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::I16)),
            (sym::Poseidon4, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::I32)),
            (sym::Poseidon4, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::I64)),
            (sym::Poseidon4, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::I128)),
            (sym::Poseidon4, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::U8)),
            (sym::Poseidon4, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::U16)),
            (sym::Poseidon4, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::U32)),
            (sym::Poseidon4, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::U64)),
            (sym::Poseidon4, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashPSD4Raw, Type::Integer(IntegerType::U128)),
            (sym::Poseidon4, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashPSD4Raw, Type::Scalar),

            (sym::Poseidon8, sym::hash_to_address)     => Self::Hash(HashVariant::HashPSD8, Type::Address),
            (sym::Poseidon8, sym::hash_to_field)       => Self::Hash(HashVariant::HashPSD8, Type::Field),
            (sym::Poseidon8, sym::hash_to_group)       => Self::Hash(HashVariant::HashPSD8, Type::Group),
            (sym::Poseidon8, sym::hash_to_i8)          => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::I8)),
            (sym::Poseidon8, sym::hash_to_i16)         => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::I16)),
            (sym::Poseidon8, sym::hash_to_i32)         => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::I32)),
            (sym::Poseidon8, sym::hash_to_i64)         => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::I64)),
            (sym::Poseidon8, sym::hash_to_i128)        => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::I128)),
            (sym::Poseidon8, sym::hash_to_u8)          => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::U8)),
            (sym::Poseidon8, sym::hash_to_u16)         => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::U16)),
            (sym::Poseidon8, sym::hash_to_u32)         => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::U32)),
            (sym::Poseidon8, sym::hash_to_u64)         => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::U64)),
            (sym::Poseidon8, sym::hash_to_u128)        => Self::Hash(HashVariant::HashPSD8, Type::Integer(IntegerType::U128)),
            (sym::Poseidon8, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashPSD8, Type::Scalar),
            (sym::Poseidon8, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashPSD8Raw, Type::Address),
            (sym::Poseidon8, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashPSD8Raw, Type::Field),
            (sym::Poseidon8, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashPSD8Raw, Type::Group),
            (sym::Poseidon8, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::I8)),
            (sym::Poseidon8, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::I16)),
            (sym::Poseidon8, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::I32)),
            (sym::Poseidon8, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::I64)),
            (sym::Poseidon8, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::I128)),
            (sym::Poseidon8, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::U8)),
            (sym::Poseidon8, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::U16)),
            (sym::Poseidon8, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::U32)),
            (sym::Poseidon8, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::U64)),
            (sym::Poseidon8, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashPSD8Raw, Type::Integer(IntegerType::U128)),
            (sym::Poseidon8, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashPSD8Raw, Type::Scalar),

            (sym::SHA3_256, sym::hash_to_address)     => Self::Hash(HashVariant::HashSha3_256, Type::Address),
            (sym::SHA3_256, sym::hash_to_field)       => Self::Hash(HashVariant::HashSha3_256, Type::Field),
            (sym::SHA3_256, sym::hash_to_group)       => Self::Hash(HashVariant::HashSha3_256, Type::Group),
            (sym::SHA3_256, sym::hash_to_i8)          => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::I8)),
            (sym::SHA3_256, sym::hash_to_i16)         => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::I16)),
            (sym::SHA3_256, sym::hash_to_i32)         => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::I32)),
            (sym::SHA3_256, sym::hash_to_i64)         => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::I64)),
            (sym::SHA3_256, sym::hash_to_i128)        => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::I128)),
            (sym::SHA3_256, sym::hash_to_u8)          => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::U8)),
            (sym::SHA3_256, sym::hash_to_u16)         => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::U16)),
            (sym::SHA3_256, sym::hash_to_u32)         => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::U32)),
            (sym::SHA3_256, sym::hash_to_u64)         => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::U64)),
            (sym::SHA3_256, sym::hash_to_u128)        => Self::Hash(HashVariant::HashSha3_256, Type::Integer(IntegerType::U128)),
            (sym::SHA3_256, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashSha3_256, Type::Scalar),
            (sym::SHA3_256, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashSha3_256Raw, Type::Address),
            (sym::SHA3_256, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashSha3_256Raw, Type::Field),
            (sym::SHA3_256, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashSha3_256Raw, Type::Group),
            (sym::SHA3_256, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::I8)),
            (sym::SHA3_256, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::I16)),
            (sym::SHA3_256, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::I32)),
            (sym::SHA3_256, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::I64)),
            (sym::SHA3_256, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::I128)),
            (sym::SHA3_256, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::U8)),
            (sym::SHA3_256, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::U16)),
            (sym::SHA3_256, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::U32)),
            (sym::SHA3_256, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::U64)),
            (sym::SHA3_256, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashSha3_256Raw, Type::Integer(IntegerType::U128)),
            (sym::SHA3_256, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashSha3_256Raw, Type::Scalar),
            (sym::SHA3_256, sym::hash_native)         => Self::Hash(HashVariant::HashSha3_256Native, Type::Array(ArrayType::bit_array(256))),
            (sym::SHA3_256, sym::hash_native_raw)     => Self::Hash(HashVariant::HashSha3_256NativeRaw, Type::Array(ArrayType::bit_array(256))),

            (sym::SHA3_384, sym::hash_to_address)     => Self::Hash(HashVariant::HashSha3_384, Type::Address),
            (sym::SHA3_384, sym::hash_to_field)       => Self::Hash(HashVariant::HashSha3_384, Type::Field),
            (sym::SHA3_384, sym::hash_to_group)       => Self::Hash(HashVariant::HashSha3_384, Type::Group),
            (sym::SHA3_384, sym::hash_to_i8)          => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::I8)),
            (sym::SHA3_384, sym::hash_to_i16)         => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::I16)),
            (sym::SHA3_384, sym::hash_to_i32)         => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::I32)),
            (sym::SHA3_384, sym::hash_to_i64)         => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::I64)),
            (sym::SHA3_384, sym::hash_to_i128)        => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::I128)),
            (sym::SHA3_384, sym::hash_to_u8)          => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::U8)),
            (sym::SHA3_384, sym::hash_to_u16)         => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::U16)),
            (sym::SHA3_384, sym::hash_to_u32)         => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::U32)),
            (sym::SHA3_384, sym::hash_to_u64)         => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::U64)),
            (sym::SHA3_384, sym::hash_to_u128)        => Self::Hash(HashVariant::HashSha3_384, Type::Integer(IntegerType::U128)),
            (sym::SHA3_384, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashSha3_384, Type::Scalar),
            (sym::SHA3_384, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashSha3_384Raw, Type::Address),
            (sym::SHA3_384, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashSha3_384Raw, Type::Field),
            (sym::SHA3_384, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashSha3_384Raw, Type::Group),
            (sym::SHA3_384, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::I8)),
            (sym::SHA3_384, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::I16)),
            (sym::SHA3_384, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::I32)),
            (sym::SHA3_384, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::I64)),
            (sym::SHA3_384, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::I128)),
            (sym::SHA3_384, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::U8)),
            (sym::SHA3_384, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::U16)),
            (sym::SHA3_384, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::U32)),
            (sym::SHA3_384, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::U64)),
            (sym::SHA3_384, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashSha3_384Raw, Type::Integer(IntegerType::U128)),
            (sym::SHA3_384, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashSha3_384Raw, Type::Scalar),
            (sym::SHA3_384, sym::hash_native)         => Self::Hash(HashVariant::HashSha3_384Native, Type::Array(ArrayType::bit_array(384))),
            (sym::SHA3_384, sym::hash_native_raw)     => Self::Hash(HashVariant::HashSha3_384NativeRaw, Type::Array(ArrayType::bit_array(384))),

            (sym::SHA3_512, sym::hash_to_address)     => Self::Hash(HashVariant::HashSha3_512, Type::Address),
            (sym::SHA3_512, sym::hash_to_field)       => Self::Hash(HashVariant::HashSha3_512, Type::Field),
            (sym::SHA3_512, sym::hash_to_group)       => Self::Hash(HashVariant::HashSha3_512, Type::Group),
            (sym::SHA3_512, sym::hash_to_i8)          => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::I8)),
            (sym::SHA3_512, sym::hash_to_i16)         => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::I16)),
            (sym::SHA3_512, sym::hash_to_i32)         => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::I32)),
            (sym::SHA3_512, sym::hash_to_i64)         => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::I64)),
            (sym::SHA3_512, sym::hash_to_i128)        => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::I128)),
            (sym::SHA3_512, sym::hash_to_u8)          => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::U8)),
            (sym::SHA3_512, sym::hash_to_u16)         => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::U16)),
            (sym::SHA3_512, sym::hash_to_u32)         => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::U32)),
            (sym::SHA3_512, sym::hash_to_u64)         => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::U64)),
            (sym::SHA3_512, sym::hash_to_u128)        => Self::Hash(HashVariant::HashSha3_512, Type::Integer(IntegerType::U128)),
            (sym::SHA3_512, sym::hash_to_scalar)      => Self::Hash(HashVariant::HashSha3_512, Type::Scalar),
            (sym::SHA3_512, sym::hash_to_address_raw) => Self::Hash(HashVariant::HashSha3_512Raw, Type::Address),
            (sym::SHA3_512, sym::hash_to_field_raw)   => Self::Hash(HashVariant::HashSha3_512Raw, Type::Field),
            (sym::SHA3_512, sym::hash_to_group_raw)   => Self::Hash(HashVariant::HashSha3_512Raw, Type::Group),
            (sym::SHA3_512, sym::hash_to_i8_raw)      => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::I8)),
            (sym::SHA3_512, sym::hash_to_i16_raw)     => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::I16)),
            (sym::SHA3_512, sym::hash_to_i32_raw)     => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::I32)),
            (sym::SHA3_512, sym::hash_to_i64_raw)     => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::I64)),
            (sym::SHA3_512, sym::hash_to_i128_raw)    => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::I128)),
            (sym::SHA3_512, sym::hash_to_u8_raw)      => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::U8)),
            (sym::SHA3_512, sym::hash_to_u16_raw)     => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::U16)),
            (sym::SHA3_512, sym::hash_to_u32_raw)     => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::U32)),
            (sym::SHA3_512, sym::hash_to_u64_raw)     => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::U64)),
            (sym::SHA3_512, sym::hash_to_u128_raw)    => Self::Hash(HashVariant::HashSha3_512Raw, Type::Integer(IntegerType::U128)),
            (sym::SHA3_512, sym::hash_to_scalar_raw)  => Self::Hash(HashVariant::HashSha3_512Raw, Type::Scalar),
            (sym::SHA3_512, sym::hash_native)         => Self::Hash(HashVariant::HashSha3_512Native, Type::Array(ArrayType::bit_array(512))),
            (sym::SHA3_512, sym::hash_native_raw)     => Self::Hash(HashVariant::HashSha3_512NativeRaw, Type::Array(ArrayType::bit_array(512))),

            (sym::ECDSA, sym::verify_digest)          => Self::ECDSAVerify(ECDSAVerifyVariant::Digest),
            (sym::ECDSA, sym::verify_digest_eth)      => Self::ECDSAVerify(ECDSAVerifyVariant::DigestEth),
            (sym::ECDSA, sym::verify_keccak256)       => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak256),
            (sym::ECDSA, sym::verify_keccak256_raw)   => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak256Raw),
            (sym::ECDSA, sym::verify_keccak256_eth)   => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak256Eth),
            (sym::ECDSA, sym::verify_keccak384)       => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak384),
            (sym::ECDSA, sym::verify_keccak384_raw)   => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak384Raw),
            (sym::ECDSA, sym::verify_keccak384_eth)   => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak384Eth),
            (sym::ECDSA, sym::verify_keccak512)       => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak512),
            (sym::ECDSA, sym::verify_keccak512_raw)   => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak512Raw),
            (sym::ECDSA, sym::verify_keccak512_eth)   => Self::ECDSAVerify(ECDSAVerifyVariant::HashKeccak512Eth),
            (sym::ECDSA, sym::verify_sha3_256)        => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_256),
            (sym::ECDSA, sym::verify_sha3_256_raw)    => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_256Raw),
            (sym::ECDSA, sym::verify_sha3_256_eth)    => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_256Eth),
            (sym::ECDSA, sym::verify_sha3_384)        => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_384),
            (sym::ECDSA, sym::verify_sha3_384_raw)    => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_384Raw),
            (sym::ECDSA, sym::verify_sha3_384_eth)    => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_384Eth),
            (sym::ECDSA, sym::verify_sha3_512)        => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_512),
            (sym::ECDSA, sym::verify_sha3_512_raw)    => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_512Raw),
            (sym::ECDSA, sym::verify_sha3_512_eth)    => Self::ECDSAVerify(ECDSAVerifyVariant::HashSha3_512Eth),

            (_, sym::get) => Self::Get,
            (_, sym::set) => Self::Set,

            (sym::Mapping, sym::get_or_use) => Self::MappingGetOrUse,
            (sym::Mapping, sym::remove) => Self::MappingRemove,
            (sym::Mapping, sym::contains) => Self::MappingContains,

            (sym::Optional, sym::unwrap) => Self::OptionalUnwrap,
            (sym::Optional, sym::unwrap_or) => Self::OptionalUnwrapOr,

            (sym::Vector, sym::push) => Self::VectorPush,
            (sym::Vector, sym::len) => Self::VectorLen,
            (sym::Vector, sym::clear) => Self::VectorClear,
            (sym::Vector, sym::pop) => Self::VectorPop,
            (sym::Vector, sym::swap_remove) => Self::VectorSwapRemove,

            (sym::group, sym::to_x_coordinate) => Self::GroupToXCoordinate,
            (sym::group, sym::to_y_coordinate) => Self::GroupToYCoordinate,

            (sym::ProgramCore, sym::checksum) => Self::ProgramChecksum,
            (sym::ProgramCore, sym::edition) => Self::ProgramEdition,
            (sym::ProgramCore, sym::program_owner) => Self::ProgramOwner,

            (sym::signature, sym::verify) => Self::SignatureVerify,
            (sym::Future, sym::Await) => Self::FutureAwait,

            (sym::Serialize, sym::to_bits) => Self::Serialize(SerializeVariant::ToBits),
            (sym::Serialize, sym::to_bits_raw) => Self::Serialize(SerializeVariant::ToBitsRaw),

            (sym::CheatCode, sym::print_mapping) => Self::CheatCodePrintMapping,
            (sym::CheatCode, sym::set_block_height) => Self::CheatCodeSetBlockHeight,
            _ => return None,
        })
    }

    /// Returns the number of arguments required by the instruction.
    pub fn num_args(&self) -> usize {
        match self {
            Self::ChaChaRand(_) => 0,
            Self::Commit(_, _) => 2,
            Self::Hash(_, _) => 1,
            Self::ECDSAVerify(_) => 3,

            Self::Get => 2,
            Self::Set => 3,

            Self::MappingGetOrUse => 3,
            Self::MappingRemove => 2,
            Self::MappingContains => 2,

            Self::OptionalUnwrap => 1,
            Self::OptionalUnwrapOr => 2,

            Self::VectorPush => 2,
            Self::VectorLen => 1,
            Self::VectorClear => 1,
            Self::VectorPop => 1,
            Self::VectorSwapRemove => 2,

            Self::GroupToXCoordinate => 1,
            Self::GroupToYCoordinate => 1,

            Self::SignatureVerify => 3,
            Self::FutureAwait => 1,

            Self::ProgramChecksum => 1,
            Self::ProgramEdition => 1,
            Self::ProgramOwner => 1,

            Self::Serialize(_) => 1,
            Self::Deserialize(_, _) => 1,

            Self::CheatCodePrintMapping => 1,
            Self::CheatCodeSetBlockHeight => 1,
        }
    }

    /// Returns whether or not this function is finalize command.
    pub fn is_finalize_command(&self) -> bool {
        match self {
            CoreFunction::FutureAwait
            | CoreFunction::ChaChaRand(_)
            | CoreFunction::ECDSAVerify(_)
            | CoreFunction::Get
            | CoreFunction::MappingGetOrUse
            | CoreFunction::Set
            | CoreFunction::MappingRemove
            | CoreFunction::MappingContains
            | CoreFunction::ProgramChecksum
            | CoreFunction::ProgramEdition
            | CoreFunction::ProgramOwner
            | CoreFunction::VectorPush
            | CoreFunction::VectorLen
            | CoreFunction::VectorClear
            | CoreFunction::VectorPop
            | CoreFunction::VectorSwapRemove => true,
            CoreFunction::Commit(_, _)
            | CoreFunction::Hash(_, _)
            | CoreFunction::OptionalUnwrap
            | CoreFunction::OptionalUnwrapOr
            | CoreFunction::GroupToXCoordinate
            | CoreFunction::GroupToYCoordinate
            | CoreFunction::SignatureVerify
            | CoreFunction::Serialize(_)
            | CoreFunction::Deserialize(_, _)
            | CoreFunction::CheatCodePrintMapping
            | CoreFunction::CheatCodeSetBlockHeight => false,
        }
    }
}

impl TryFrom<&AssociatedFunctionExpression> for CoreFunction {
    type Error = anyhow::Error;

    fn try_from(associated_function: &AssociatedFunctionExpression) -> anyhow::Result<Self> {
        match CoreFunction::from_symbols(associated_function.variant.name, associated_function.name.name) {
            Some(core_function) => Ok(core_function),
            // Attempt to handle `Deserialize::from_bits::[T](..)`
            None if associated_function.variant.name == sym::Deserialize => {
                // Get the variant.
                let variant = match associated_function.name.name {
                    sym::from_bits => DeserializeVariant::FromBits,
                    sym::from_bits_raw => DeserializeVariant::FromBitsRaw,
                    _ => anyhow::bail!(
                        "Unknown associated function: {}::{}",
                        associated_function.variant.name,
                        associated_function.name.name
                    ),
                };
                // Get the type parameter.
                anyhow::ensure!(
                    associated_function.type_parameters.len() == 1,
                    "Expected exactly one type argument for Deserialize::{}",
                    associated_function.name.name
                );
                let type_parameter = associated_function.type_parameters[0].0.clone();
                Ok(Self::Deserialize(variant, type_parameter))
            }
            _ => anyhow::bail!(
                "Unknown associated function: {}::{}",
                associated_function.variant.name,
                associated_function.name.name
            ),
        }
    }
}
