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
mod bhp;
pub use bhp::*;

mod pedersen;
pub use pedersen::*;

mod poseidon;
pub use poseidon::*;

use crate::Types;
use leo_ast::{IntegerType, Type};
use leo_span::{sym, Symbol};

use indexmap::IndexSet;

// /// Returns `true` if the given symbol matches the name of a core circuit.
// pub fn is_core_circuit(circuit: Symbol) -> bool {
//     match circuit {
//         sym::bhp256
//         | sym::bhp512
//         | sym::bhp768
//         | sym::bhp1024
//         | sym::ped64
//         | sym::ped128
//         | sym::psd2
//         | sym::psd4
//         | sym::psd8 => true,
//         _ => false,
//     }
// }

/// A core instruction that maps directly to an AVM bytecode instruction.
#[derive(Clone, PartialEq, Eq)]
pub enum CoreInstruction {
    BHP256Commit,
    BHP256Hash,
    BHP512Commit,
    BHP512Hash,
    BHP768Commit,
    BHP768Hash,
    BHP1024Commit,
    BHP1024Hash,

    Pedersen64Commit,
    Pedersen64Hash,
    Pedersen128Commit,
    Pedersen128Hash,

    Poseidon2Hash,
    Poseidon2PRF,
    Poseidon4Hash,
    Poseidon4PRF,
    Poseidon8Hash,
    Poseidon8PRF,
}

impl CoreInstruction {
    /// Returns a `CoreInstruction` from the given circuit and method symbols.
    pub fn from_symbols(circuit: Symbol, function: Symbol) -> Option<Self> {
        Some(match (circuit, function) {
            (sym::bhp256, sym::commit) => Self::BHP256Commit,
            (sym::bhp256, sym::hash) => Self::BHP256Hash,
            (sym::bhp512, sym::commit) => Self::BHP512Commit,
            (sym::bhp512, sym::hash) => Self::BHP512Hash,
            (sym::bhp768, sym::commit) => Self::BHP768Commit,
            (sym::bhp768, sym::hash) => Self::BHP768Hash,
            (sym::bhp1024, sym::commit) => Self::BHP1024Commit,
            (sym::bhp1024, sym::hash) => Self::BHP1024Hash,

            (sym::ped64, sym::commit) => Self::Pedersen64Commit,
            (sym::ped64, sym::hash) => Self::Pedersen64Hash,
            (sym::ped128, sym::commit) => Self::Pedersen128Commit,
            (sym::ped128, sym::hash) => Self::Pedersen128Hash,

            (sym::psd2, sym::hash) => Self::Poseidon2Hash,
            (sym::psd2, sym::prf) => Self::Poseidon2PRF,
            (sym::psd4, sym::hash) => Self::Poseidon4Hash,
            (sym::psd4, sym::prf) => Self::Poseidon4PRF,
            (sym::psd8, sym::hash) => Self::Poseidon8Hash,
            (sym::psd8, sym::prf) => Self::Poseidon8PRF,
            _ => return None,
        })
    }

    /// Returns the number of arguments required by the instruction.
    pub fn num_args(&self) -> usize {
        match self {
            CoreInstruction::BHP256Commit => BHP256Commit::NUM_ARGS,
            CoreInstruction::BHP256Hash => BHP256Hash::NUM_ARGS,
            CoreInstruction::BHP512Commit => BHP512Commit::NUM_ARGS,
            CoreInstruction::BHP512Hash => BHP512Hash::NUM_ARGS,
            CoreInstruction::BHP768Commit => BHP768Commit::NUM_ARGS,
            CoreInstruction::BHP768Hash => BHP768Hash::NUM_ARGS,
            CoreInstruction::BHP1024Commit => BHP1024Commit::NUM_ARGS,
            CoreInstruction::BHP1024Hash => BHP1024Hash::NUM_ARGS,

            CoreInstruction::Pedersen64Commit => Pedersen64Commit::NUM_ARGS,
            CoreInstruction::Pedersen64Hash => Pedersen64Hash::NUM_ARGS,
            CoreInstruction::Pedersen128Commit => Pedersen128Commit::NUM_ARGS,
            CoreInstruction::Pedersen128Hash => Pedersen128Hash::NUM_ARGS,

            CoreInstruction::Poseidon2Hash => Poseidon2Hash::NUM_ARGS,
            CoreInstruction::Poseidon2PRF => Poseidon2PRF::NUM_ARGS,
            CoreInstruction::Poseidon4Hash => Poseidon4Hash::NUM_ARGS,
            CoreInstruction::Poseidon4PRF => Poseidon4PRF::NUM_ARGS,
            CoreInstruction::Poseidon8Hash => Poseidon8Hash::NUM_ARGS,
            CoreInstruction::Poseidon8PRF => Poseidon8PRF::NUM_ARGS,
        }
    }

    /// The allowed types for the first argument of the instruction.
    pub fn first_arg_types(&self) -> &'static [Type] {
        match self {
            CoreInstruction::BHP256Commit => BHP256Commit::first_arg_types(),
            CoreInstruction::BHP256Hash => BHP256Hash::first_arg_types(),
            CoreInstruction::BHP512Commit => BHP512Commit::first_arg_types(),
            CoreInstruction::BHP512Hash => BHP512Hash::first_arg_types(),
            CoreInstruction::BHP768Commit => BHP768Commit::first_arg_types(),
            CoreInstruction::BHP768Hash => BHP768Hash::first_arg_types(),
            CoreInstruction::BHP1024Commit => BHP1024Commit::first_arg_types(),
            CoreInstruction::BHP1024Hash => BHP1024Hash::first_arg_types(),

            CoreInstruction::Pedersen64Commit => Pedersen64Commit::first_arg_types(),
            CoreInstruction::Pedersen64Hash => Pedersen64Hash::first_arg_types(),
            CoreInstruction::Pedersen128Commit => Pedersen128Commit::first_arg_types(),
            CoreInstruction::Pedersen128Hash => Pedersen128Hash::first_arg_types(),

            CoreInstruction::Poseidon2Hash => Poseidon2Hash::first_arg_types(),
            CoreInstruction::Poseidon2PRF => Poseidon2PRF::first_arg_types(),
            CoreInstruction::Poseidon4Hash => Poseidon4Hash::first_arg_types(),
            CoreInstruction::Poseidon4PRF => Poseidon4PRF::first_arg_types(),
            CoreInstruction::Poseidon8Hash => Poseidon8Hash::first_arg_types(),
            CoreInstruction::Poseidon8PRF => Poseidon8PRF::first_arg_types(),
        }
    }

    /// The allowed types for the second argument of the instruction.
    pub fn second_arg_types(&self) -> &'static [Type] {
        match self {
            CoreInstruction::BHP256Commit => BHP256Commit::second_arg_types(),
            CoreInstruction::BHP256Hash => BHP256Hash::second_arg_types(),
            CoreInstruction::BHP512Commit => BHP512Commit::second_arg_types(),
            CoreInstruction::BHP512Hash => BHP512Hash::second_arg_types(),
            CoreInstruction::BHP768Commit => BHP768Commit::second_arg_types(),
            CoreInstruction::BHP768Hash => BHP768Hash::second_arg_types(),
            CoreInstruction::BHP1024Commit => BHP1024Commit::second_arg_types(),
            CoreInstruction::BHP1024Hash => BHP1024Hash::second_arg_types(),

            CoreInstruction::Pedersen64Commit => Pedersen64Commit::second_arg_types(),
            CoreInstruction::Pedersen64Hash => Pedersen64Hash::second_arg_types(),
            CoreInstruction::Pedersen128Commit => Pedersen128Commit::second_arg_types(),
            CoreInstruction::Pedersen128Hash => Pedersen128Hash::second_arg_types(),

            CoreInstruction::Poseidon2Hash => Poseidon2Hash::second_arg_types(),
            CoreInstruction::Poseidon2PRF => Poseidon2PRF::second_arg_types(),
            CoreInstruction::Poseidon4Hash => Poseidon4Hash::second_arg_types(),
            CoreInstruction::Poseidon4PRF => Poseidon4PRF::second_arg_types(),
            CoreInstruction::Poseidon8Hash => Poseidon8Hash::second_arg_types(),
            CoreInstruction::Poseidon8PRF => Poseidon8PRF::second_arg_types(),
        }
    }

    /// The type of the instruction output.
    pub fn return_type(&self) -> Type {
        match self {
            CoreInstruction::BHP256Commit => BHP256Commit::return_type(),
            CoreInstruction::BHP256Hash => BHP256Hash::return_type(),
            CoreInstruction::BHP512Commit => BHP512Commit::return_type(),
            CoreInstruction::BHP512Hash => BHP512Hash::return_type(),
            CoreInstruction::BHP768Commit => BHP768Commit::return_type(),
            CoreInstruction::BHP768Hash => BHP768Hash::return_type(),
            CoreInstruction::BHP1024Commit => BHP1024Commit::return_type(),
            CoreInstruction::BHP1024Hash => BHP1024Hash::return_type(),

            CoreInstruction::Pedersen64Commit => Pedersen64Commit::return_type(),
            CoreInstruction::Pedersen64Hash => Pedersen64Hash::return_type(),
            CoreInstruction::Pedersen128Commit => Pedersen128Commit::return_type(),
            CoreInstruction::Pedersen128Hash => Pedersen128Hash::return_type(),

            CoreInstruction::Poseidon2Hash => Poseidon2Hash::return_type(),
            CoreInstruction::Poseidon2PRF => Poseidon2PRF::return_type(),
            CoreInstruction::Poseidon4Hash => Poseidon4Hash::return_type(),
            CoreInstruction::Poseidon4PRF => Poseidon4PRF::return_type(),
            CoreInstruction::Poseidon8Hash => Poseidon8Hash::return_type(),
            CoreInstruction::Poseidon8PRF => Poseidon8PRF::return_type(),
        }
    }
}

/// A core function of a core circuit, e.g. `hash` or `commit`
/// Provides required type information to the type checker.
trait CoreFunction {
    const NUM_ARGS: usize;

    /// Returns first argument allowed types.
    fn first_arg_types() -> &'static [Type];

    /// Returns the second argument allowed types.
    /// Implementing this method is optional since some functions may not require a second argument.
    fn second_arg_types() -> &'static [Type] {
        &[]
    }

    /// The return type of the core function.
    fn return_type() -> Type;
}

const ALL_TYPES: [Type; 16] = [
    Type::Address,
    Type::Boolean,
    Type::Field,
    Type::Group,
    Type::IntegerType(IntegerType::I8),
    Type::IntegerType(IntegerType::I16),
    Type::IntegerType(IntegerType::I32),
    Type::IntegerType(IntegerType::I64),
    Type::IntegerType(IntegerType::I128),
    Type::IntegerType(IntegerType::U8),
    Type::IntegerType(IntegerType::U16),
    Type::IntegerType(IntegerType::U32),
    Type::IntegerType(IntegerType::U64),
    Type::IntegerType(IntegerType::U128),
    Type::Scalar,
    Type::String,
];

const BOOL_INT_STRING_TYPES: [Type; 12] = [
    Type::Boolean,
    Type::IntegerType(IntegerType::I8),
    Type::IntegerType(IntegerType::I16),
    Type::IntegerType(IntegerType::I32),
    Type::IntegerType(IntegerType::I64),
    Type::IntegerType(IntegerType::I128),
    Type::IntegerType(IntegerType::U8),
    Type::IntegerType(IntegerType::U16),
    Type::IntegerType(IntegerType::U32),
    Type::IntegerType(IntegerType::U64),
    Type::IntegerType(IntegerType::U128),
    Type::String,
];

// todo (collin): deprecate this code
pub struct Algorithms;

impl Types for Algorithms {
    fn types() -> IndexSet<Symbol> {
        IndexSet::from([Symbol::intern("Poseidon")])
    }
}
