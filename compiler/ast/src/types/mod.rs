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

mod array;
pub use array::*;

mod core_constant;
pub use core_constant::*;

mod future;
pub use future::*;

mod integer_type;
pub use integer_type::*;

mod optional;
pub use optional::*;

mod mapping;
pub use mapping::*;

mod struct_type;
pub use struct_type::*;

mod tuple;
pub use tuple::*;

mod type_;
pub use type_::*;

use snarkvm::prelude::LiteralType;

impl From<LiteralType> for Type {
    fn from(literal_type: LiteralType) -> Self {
        match literal_type {
            LiteralType::Boolean => Type::Boolean,
            LiteralType::Field => Type::Field,
            LiteralType::Group => Type::Group,
            LiteralType::I8 => Type::Integer(IntegerType::I8),
            LiteralType::I16 => Type::Integer(IntegerType::I16),
            LiteralType::I32 => Type::Integer(IntegerType::I32),
            LiteralType::I64 => Type::Integer(IntegerType::I64),
            LiteralType::I128 => Type::Integer(IntegerType::I128),
            LiteralType::U8 => Type::Integer(IntegerType::U8),
            LiteralType::U16 => Type::Integer(IntegerType::U16),
            LiteralType::U32 => Type::Integer(IntegerType::U32),
            LiteralType::U64 => Type::Integer(IntegerType::U64),
            LiteralType::U128 => Type::Integer(IntegerType::U128),
            LiteralType::String => Type::String,
            LiteralType::Address => Type::Address,
            LiteralType::Scalar => Type::Scalar,
            LiteralType::Signature => Type::Signature,
        }
    }
}
