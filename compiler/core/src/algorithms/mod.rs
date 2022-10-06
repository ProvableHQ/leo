// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_ast::Type;
use leo_span::{sym, Symbol};

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
    Poseidon4Hash,
    Poseidon8Hash,
}

impl CoreInstruction {
    /// Returns a `CoreInstruction` from the given module and method symbols.
    pub fn from_symbols(module: Symbol, function: Symbol) -> Option<Self> {
        Some(match (module, function) {
            (sym::BHP256, sym::commit) => Self::BHP256Commit,
            (sym::BHP256, sym::hash) => Self::BHP256Hash,
            (sym::BHP512, sym::commit) => Self::BHP512Commit,
            (sym::BHP512, sym::hash) => Self::BHP512Hash,
            (sym::BHP768, sym::commit) => Self::BHP768Commit,
            (sym::BHP768, sym::hash) => Self::BHP768Hash,
            (sym::BHP1024, sym::commit) => Self::BHP1024Commit,
            (sym::BHP1024, sym::hash) => Self::BHP1024Hash,

            (sym::Pedersen64, sym::commit) => Self::Pedersen64Commit,
            (sym::Pedersen64, sym::hash) => Self::Pedersen64Hash,
            (sym::Pedersen128, sym::commit) => Self::Pedersen128Commit,
            (sym::Pedersen128, sym::hash) => Self::Pedersen128Hash,

            (sym::Poseidon2, sym::hash) => Self::Poseidon2Hash,
            (sym::Poseidon4, sym::hash) => Self::Poseidon4Hash,
            (sym::Poseidon8, sym::hash) => Self::Poseidon8Hash,
            _ => return None,
        })
    }

    /// Returns the number of arguments required by the instruction.
    pub fn num_args(&self) -> usize {
        match self {
            Self::BHP256Commit => BHP256Commit::NUM_ARGS,
            Self::BHP256Hash => BHP256Hash::NUM_ARGS,
            Self::BHP512Commit => BHP512Commit::NUM_ARGS,
            Self::BHP512Hash => BHP512Hash::NUM_ARGS,
            Self::BHP768Commit => BHP768Commit::NUM_ARGS,
            Self::BHP768Hash => BHP768Hash::NUM_ARGS,
            Self::BHP1024Commit => BHP1024Commit::NUM_ARGS,
            Self::BHP1024Hash => BHP1024Hash::NUM_ARGS,

            Self::Pedersen64Commit => Pedersen64Commit::NUM_ARGS,
            Self::Pedersen64Hash => Pedersen64Hash::NUM_ARGS,
            Self::Pedersen128Commit => Pedersen128Commit::NUM_ARGS,
            Self::Pedersen128Hash => Pedersen128Hash::NUM_ARGS,

            Self::Poseidon2Hash => Poseidon2Hash::NUM_ARGS,
            Self::Poseidon4Hash => Poseidon4Hash::NUM_ARGS,
            Self::Poseidon8Hash => Poseidon8Hash::NUM_ARGS,
        }
    }

    /// Returns whether or not the first argument is an allowed type.
    pub fn first_arg_is_allowed_type(&self, type_: &Type) -> bool {
        match self {
            CoreInstruction::BHP256Commit => BHP256Commit::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP256Hash => BHP256Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP512Commit => BHP512Commit::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP512Hash => BHP512Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP768Commit => BHP768Commit::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP768Hash => BHP768Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP1024Commit => BHP1024Commit::first_arg_is_allowed_type(type_),
            CoreInstruction::BHP1024Hash => BHP1024Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen64Commit => Pedersen64Commit::first_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen64Hash => Pedersen64Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen128Commit => Pedersen128Commit::first_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen128Hash => Pedersen128Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::Poseidon2Hash => Poseidon2Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::Poseidon4Hash => Poseidon4Hash::first_arg_is_allowed_type(type_),
            CoreInstruction::Poseidon8Hash => Poseidon8Hash::first_arg_is_allowed_type(type_),
        }
    }

    /// Returns whether or not the second argument is an allowed type.
    pub fn second_arg_is_allowed_type(&self, type_: &Type) -> bool {
        match self {
            CoreInstruction::BHP256Commit => BHP256Commit::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP256Hash => BHP256Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP512Commit => BHP512Commit::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP512Hash => BHP512Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP768Commit => BHP768Commit::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP768Hash => BHP768Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP1024Commit => BHP1024Commit::second_arg_is_allowed_type(type_),
            CoreInstruction::BHP1024Hash => BHP1024Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen64Commit => Pedersen64Commit::second_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen64Hash => Pedersen64Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen128Commit => Pedersen128Commit::second_arg_is_allowed_type(type_),
            CoreInstruction::Pedersen128Hash => Pedersen128Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::Poseidon2Hash => Poseidon2Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::Poseidon4Hash => Poseidon4Hash::second_arg_is_allowed_type(type_),
            CoreInstruction::Poseidon8Hash => Poseidon8Hash::second_arg_is_allowed_type(type_),
        }
    }

    /// The type of the instruction output.
    pub fn return_type(&self) -> Type {
        match self {
            Self::BHP256Commit => BHP256Commit::return_type(),
            Self::BHP256Hash => BHP256Hash::return_type(),
            Self::BHP512Commit => BHP512Commit::return_type(),
            Self::BHP512Hash => BHP512Hash::return_type(),
            Self::BHP768Commit => BHP768Commit::return_type(),
            Self::BHP768Hash => BHP768Hash::return_type(),
            Self::BHP1024Commit => BHP1024Commit::return_type(),
            Self::BHP1024Hash => BHP1024Hash::return_type(),

            Self::Pedersen64Commit => Pedersen64Commit::return_type(),
            Self::Pedersen64Hash => Pedersen64Hash::return_type(),
            Self::Pedersen128Commit => Pedersen128Commit::return_type(),
            Self::Pedersen128Hash => Pedersen128Hash::return_type(),

            Self::Poseidon2Hash => Poseidon2Hash::return_type(),
            Self::Poseidon4Hash => Poseidon4Hash::return_type(),
            Self::Poseidon8Hash => Poseidon8Hash::return_type(),
        }
    }
}

/// A core function of a core struct, e.g. `hash` or `commit`
/// Provides required type information to the type checker.
trait CoreFunction {
    const NUM_ARGS: usize;

    /// Returns whether or not the first argument is an allowed type.
    fn first_arg_is_allowed_type(_: &Type) -> bool {
        false
    }

    /// Returns whether or not the second argument is an allowed type.
    fn second_arg_is_allowed_type(_: &Type) -> bool {
        false
    }

    /// The return type of the core function.
    fn return_type() -> Type;
}
