// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::TypeAssertionError;
use leo_ast::Span;
use leo_symbol_table::{get_array_element_type, Type, TypeVariable};

/// A type variable -> type pair.
pub struct TypeVariablePair(TypeVariable, Type);

impl TypeVariablePair {
    pub fn first(&self) -> &TypeVariable {
        &self.0
    }

    pub fn second(&self) -> &Type {
        &self.1
    }
}

/// A vector of `TypeVariablePair`s.
pub struct TypeVariablePairs(Vec<TypeVariablePair>);

impl Default for TypeVariablePairs {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl TypeVariablePairs {
    ///
    /// Returns a new `TypeVariablePairs` struct from the given left and right types.
    ///
    pub fn new(left: Type, right: Type, span: &Span) -> Result<Self, TypeAssertionError> {
        let mut pairs = Self::default();

        // Push all `TypeVariablePair`s.
        pairs.push_pairs(left, right, span)?;

        Ok(pairs)
    }

    ///
    /// Returns true if the self vector has no pairs.
    ///
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Returns the self vector of pairs.
    ///
    pub fn get_pairs(&self) -> &[TypeVariablePair] {
        &self.0
    }

    ///
    /// Pushes a new `TypeVariablePair` struct to self.
    ///
    pub fn push(&mut self, variable: TypeVariable, type_: Type) {
        // Create a new type variable -> type pair.
        let pair = TypeVariablePair(variable, type_);

        // Push the pair to the self vector.
        self.0.push(pair);
    }

    ///
    /// Checks if the given left or right type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    pub fn push_pairs(&mut self, left: Type, right: Type, span: &Span) -> Result<(), TypeAssertionError> {
        match (left, right) {
            (Type::TypeVariable(variable), type_) => {
                self.push(variable, type_);
                Ok(())
            }
            (type_, Type::TypeVariable(variable)) => {
                self.push(variable, type_);
                Ok(())
            }
            (Type::Array(left_type), Type::Array(right_type)) => self.push_pairs_array(*left_type, *right_type, span),
            (Type::Tuple(left_types), Type::Tuple(right_types)) => {
                self.push_pairs_tuple(left_types.into_iter(), right_types.into_iter(), span)
            }
            (_, _) => Ok(()), // No `TypeVariable` found so we do not push any pairs.
        }
    }

    ///
    /// Checks if the given left or right array type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    fn push_pairs_array(&mut self, left_type: Type, right_type: Type, span: &Span) -> Result<(), TypeAssertionError> {
        // Get both array element types before comparison.
        let array1_element = get_array_element_type(&left_type);
        let array2_element = get_array_element_type(&right_type);

        // Compare the array element types.
        self.push_pairs(array1_element.to_owned(), array2_element.to_owned(), span)
    }

    ///
    /// Checks if any given left or right tuple type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    fn push_pairs_tuple(
        &mut self,
        left_types: impl Iterator<Item = Type>,
        right_types: impl Iterator<Item = Type>,
        span: &Span,
    ) -> Result<(), TypeAssertionError> {
        // Iterate over each left == right pair of types.
        for (left, right) in left_types.into_iter().zip(right_types) {
            // Check for `TypeVariablePair`s.
            self.push_pairs(left, right, span)?;
        }

        Ok(())
    }
}
