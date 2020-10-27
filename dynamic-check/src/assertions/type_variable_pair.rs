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
use leo_static_check::{flatten_array_type, Type, TypeVariable};
use leo_typed::Span;

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
    pub fn new(left: &Type, right: &Type, span: &Span) -> Result<Self, TypeAssertionError> {
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
    pub fn get_pairs(&self) -> &Vec<TypeVariablePair> {
        &self.0
    }

    ///
    /// Pushes a new `TypeVariablePair` struct to self.
    ///
    pub fn push(&mut self, variable: &TypeVariable, type_: &Type) {
        // Create a new type variable -> type pair.
        let pair = TypeVariablePair(variable.clone(), type_.clone());

        // Push the pair to the self vector.
        self.0.push(pair);
    }

    ///
    /// Checks if the given left or right type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    pub fn push_pairs(&mut self, left: &Type, right: &Type, span: &Span) -> Result<(), TypeAssertionError> {
        match (left, right) {
            (Type::TypeVariable(variable), type_) => Ok(self.push(variable, type_)),
            (type_, Type::TypeVariable(variable)) => Ok(self.push(variable, type_)),
            (Type::Array(left_type, left_dimensions), Type::Array(right_type, right_dimensions)) => {
                self.push_pairs_array(left_type, left_dimensions, right_type, right_dimensions, span)
            }
            (Type::Tuple(left_types), Type::Tuple(right_types)) => self.push_pairs_tuple(left_types, right_types, span),
            (_, _) => Ok(()), // No `TypeVariable` found so we do not push any pairs.
        }
    }

    ///
    /// Checks if the given left or right array type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    fn push_pairs_array(
        &mut self,
        left_type: &Type,
        left_dimensions: &Vec<usize>,
        right_type: &Type,
        right_dimensions: &Vec<usize>,
        span: &Span,
    ) -> Result<(), TypeAssertionError> {
        // Flatten the array types to get the element types.
        let (left_type_flat, left_dimensions_flat) = flatten_array_type(left_type, left_dimensions.to_owned());
        let (right_type_flat, right_dimensions_flat) = flatten_array_type(right_type, right_dimensions.to_owned());

        // If the dimensions do not match, then throw an error.
        if left_dimensions_flat.ne(&right_dimensions_flat) {
            return Err(TypeAssertionError::array_dimensions(
                left_dimensions_flat,
                right_dimensions_flat,
                span,
            ));
        }

        // Compare the array element types.
        self.push_pairs(left_type_flat, right_type_flat, span)
    }

    ///
    /// Checks if any given left or right tuple type contains a `TypeVariable`.
    /// If a `TypeVariable` is found, create a new `TypeVariablePair` between the given left
    /// and right type.
    ///
    fn push_pairs_tuple(
        &mut self,
        left_types: &Vec<Type>,
        right_types: &Vec<Type>,
        span: &Span,
    ) -> Result<(), TypeAssertionError> {
        // Iterate over each left == right pair of types.
        for (left, right) in left_types.iter().zip(right_types) {
            // Check for `TypeVariablePair`s.
            self.push_pairs(left, right, span)?;
        }

        Ok(())
    }
}
