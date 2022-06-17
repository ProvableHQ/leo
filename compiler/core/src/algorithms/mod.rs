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

use crate::Types;
use leo_span::{sym, Symbol};

use indexmap::IndexSet;
use leo_ast::{IntegerType, Type};

/// A core instruction that maps directly to an AVM bytecode instruction.
#[derive(Clone, PartialEq, Eq)]
pub enum CoreInstruction {
    BHP256Hash,
    BHP512,
    BHP768,
    BHP1024,
    Pedersen64,
    Pedersen128,
    Poseidon2,
    Poseidon4,
    Poseidon8,
}

impl CoreInstruction {
    /// Returns a `CoreInstruction` from the given circuit and method symbols.
    pub fn from_symbols(circuit: Symbol, function: Symbol) -> Option<Self> {
        Some(match (circuit, function) {
            (sym::bhp256, sym::hash) => Self::BHP256Hash,
            _ => return None,
        })
    }

    pub fn num_args(&self) -> usize {
        match self {
            CoreInstruction::BHP256Hash => BHP256Hash::NUM_ARGS,
            _ => unimplemented!(),
        }
    }

    pub fn first_arg_types(&self) -> &'static [Type] {
        match self {
            CoreInstruction::BHP256Hash => BHP256Hash::first_arg_types(),
            _ => unimplemented!(),
        }
    }

    pub fn second_arg_types(&self) -> &'static [Type] {
        match self {
            _ => unimplemented!(),
        }
    }

    pub fn return_type(&self) -> Type {
        match self {
            CoreInstruction::BHP256Hash => BHP256Hash::return_type(),
            _ => unimplemented!(),
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

// todo (collin): deprecate this code
pub struct Algorithms;

impl Types for Algorithms {
    fn types() -> IndexSet<Symbol> {
        IndexSet::from([Symbol::intern("Poseidon")])
    }
}
