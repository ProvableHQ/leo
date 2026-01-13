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

use crate::{Expression, IntegerType, Literal, LiteralVariant, Type};
use snarkvm::console::program::ArrayType as ConsoleArrayType;

use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use snarkvm::prelude::Network;
use std::fmt;

/// An array type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayType {
    pub element_type: Box<Type>,
    pub length: Box<Expression>,
}

impl ArrayType {
    /// Creates a new array type.
    pub fn new(element: Type, length: Expression) -> Self {
        Self { element_type: Box::new(element), length: Box::new(length) }
    }

    /// Creates a new bit array type.
    pub fn bit_array(length: u32) -> Self {
        Self {
            element_type: Box::new(Type::Boolean),
            length: Box::new(Expression::Literal(Literal {
                variant: LiteralVariant::Integer(IntegerType::U32, length.to_string()),
                id: Default::default(),
                span: Span::default(),
            })),
        }
    }

    /// Returns the element type of the array.
    pub fn element_type(&self) -> &Type {
        &self.element_type
    }

    /// Returns the base element type of the array.
    pub fn base_element_type(&self) -> &Type {
        match self.element_type.as_ref() {
            Type::Array(array_type) => array_type.base_element_type(),
            type_ => type_,
        }
    }

    pub fn from_snarkvm<N: Network>(array_type: &ConsoleArrayType<N>, program: Symbol) -> Self {
        Self {
            element_type: Box::new(Type::from_snarkvm(array_type.next_element_type(), program)),
            length: Box::new(Expression::Literal(Literal {
                variant: LiteralVariant::Integer(IntegerType::U32, array_type.length().to_string().replace("u32", "")),
                id: Default::default(),
                span: Span::default(),
            })),
        }
    }

    pub fn to_snarkvm<N: Network>(&self) -> anyhow::Result<ConsoleArrayType<N>> {
        let length = if let Expression::Literal(literal) = &*self.length {
            match &literal.variant {
                LiteralVariant::Integer(_, s) => s.parse::<u32>().unwrap(),
                LiteralVariant::Unsuffixed(s) => s.parse::<u32>().unwrap(),
                _ => anyhow::bail!("Array length must be an integer literal"),
            }
        } else {
            anyhow::bail!("Array length must be an integer literal")
        };

        ConsoleArrayType::new(self.element_type.to_snarkvm()?, vec![snarkvm::console::types::U32::new(length)])
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // For display purposes (in error messages for example.), do not include the type suffix.
        if let Expression::Literal(literal) = &*self.length
            && let LiteralVariant::Integer(_, s) = &literal.variant
        {
            return write!(f, "[{}; {s}]", self.element_type);
        }

        write!(f, "[{}; {}]", self.element_type, self.length)
    }
}

impl From<ArrayType> for Type {
    fn from(value: ArrayType) -> Self {
        Type::Array(value)
    }
}
