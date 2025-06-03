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

use crate::{CompilerState, VariableSymbol, VariableType, type_checking::scope_state::ScopeState};

use super::TypeCheckingInput;

use leo_ast::*;
use leo_errors::{TypeCheckerError, TypeCheckerWarning};
use leo_span::{Span, Symbol};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::ops::Deref;

pub struct TypeCheckingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The state of the current scope being traversed.
    pub scope_state: ScopeState,
    /// Mapping from async function stub name to the inferred input types.
    pub async_function_input_types: IndexMap<Location, Vec<Type>>,
    /// Mapping from async function name to the names of async transition callers.
    pub async_function_callers: IndexMap<Location, IndexSet<Location>>,
    /// The set of used composites.
    pub used_structs: IndexSet<Symbol>,
    /// So we can check if we exceed limits on array size, number of mappings, or number of functions.
    pub limits: TypeCheckingInput,
    /// For detecting the error `TypeCheckerError::async_cannot_assign_outside_conditional`.
    pub conditional_scopes: Vec<IndexSet<Symbol>>,
}

impl TypeCheckingVisitor<'_> {
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }

    pub fn in_conditional_scope<T>(&mut self, func: impl FnOnce(&mut Self) -> T) -> T {
        self.conditional_scopes.push(Default::default());
        let result = func(self);
        self.conditional_scopes.pop();
        result
    }

    pub fn insert_symbol_conditional_scope(&mut self, symbol: Symbol) {
        self.conditional_scopes.last_mut().expect("A conditional scope must be present.").insert(symbol);
    }

    pub fn symbol_in_conditional_scope(&mut self, symbol: Symbol) -> bool {
        self.conditional_scopes.last().map(|set| set.contains(&symbol)).unwrap_or(false)
    }

    /// Emits a type checker error.
    pub fn emit_err(&self, err: TypeCheckerError) {
        self.state.handler.emit_err(err);
    }

    /// Emits a type checker warning
    pub fn emit_warning(&self, warning: TypeCheckerWarning) {
        self.state.handler.emit_warning(warning.into());
    }

    /// Emits an error if the two given types are not equal.
    pub fn check_eq_types(&self, t1: &Option<Type>, t2: &Option<Type>, span: Span) {
        match (t1, t2) {
            (Some(t1), Some(t2)) if !Type::eq_flat_relaxed(t1, t2) => {
                // If both types are futures, print them out.
                if let (Type::Future(f1), Type::Future(f2)) = (t1, t2) {
                    println!("Future 1: {:?}", f1);
                    println!("Future 2: {:?}", f2);
                }
                self.emit_err(TypeCheckerError::type_should_be(t1, t2, span))
            }
            (Some(type_), None) | (None, Some(type_)) => {
                self.emit_err(TypeCheckerError::type_should_be("no type", type_, span))
            }
            _ => {}
        }
    }

    /// Use this method when you know the actual type.
    /// Emits an error to the handler if the `actual` type is not equal to the `expected` type.
    pub fn assert_and_return_type(&mut self, actual: Type, expected: &Option<Type>, span: Span) -> Type {
        // If expected is `Type::Err`, we don't want to actually report a redundant error.
        if expected.is_some() && !matches!(expected, Some(Type::Err)) {
            self.check_eq_types(&Some(actual.clone()), expected, span);
        }
        actual
    }

    pub fn maybe_assert_type(&mut self, actual: &Type, expected: &Option<Type>, span: Span) {
        if let Some(expected) = expected {
            self.assert_type(actual, expected, span);
        }
    }

    pub fn assert_type(&mut self, actual: &Type, expected: &Type, span: Span) {
        if actual != &Type::Err && !self.eq_user(actual, expected) {
            // If `actual` is Err, we will have already reported an error.
            self.emit_err(TypeCheckerError::type_should_be2(actual, format!("type `{expected}`"), span));
        }
    }

    pub fn assert_int_type(&self, type_: &Type, span: Span) {
        if !matches!(type_, Type::Err | Type::Integer(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "an integer", span));
        }
    }

    pub fn assert_unsigned_type(&self, type_: &Type, span: Span) {
        if !matches!(
            type_,
            Type::Err
                | Type::Integer(IntegerType::U8)
                | Type::Integer(IntegerType::U16)
                | Type::Integer(IntegerType::U32)
                | Type::Integer(IntegerType::U64)
                | Type::Integer(IntegerType::U128)
        ) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "an unsigned integer", span));
        }
    }

    pub fn assert_bool_int_type(&self, type_: &Type, span: Span) {
        if !matches!(
            type_,
            Type::Err
                | Type::Boolean
                | Type::Integer(IntegerType::U8)
                | Type::Integer(IntegerType::U16)
                | Type::Integer(IntegerType::U32)
                | Type::Integer(IntegerType::U64)
                | Type::Integer(IntegerType::U128)
                | Type::Integer(IntegerType::I8)
                | Type::Integer(IntegerType::I16)
                | Type::Integer(IntegerType::I32)
                | Type::Integer(IntegerType::I64)
                | Type::Integer(IntegerType::I128)
        ) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a bool or integer", span));
        }
    }

    pub fn assert_field_int_type(&self, type_: &Type, span: Span) {
        if !matches!(
            type_,
            Type::Err
                | Type::Field
                | Type::Integer(IntegerType::U8)
                | Type::Integer(IntegerType::U16)
                | Type::Integer(IntegerType::U32)
                | Type::Integer(IntegerType::U64)
                | Type::Integer(IntegerType::U128)
                | Type::Integer(IntegerType::I8)
                | Type::Integer(IntegerType::I16)
                | Type::Integer(IntegerType::I32)
                | Type::Integer(IntegerType::I64)
                | Type::Integer(IntegerType::I128)
        ) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a field or integer", span));
        }
    }

    pub fn assert_field_group_int_type(&self, type_: &Type, span: Span) {
        if !matches!(type_, Type::Err | Type::Field | Type::Group | Type::Integer(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a field, group, or integer", span));
        }
    }

    /// Type checks the inputs to an associated constant and returns the expected output type.
    pub fn get_core_constant(&self, type_: &Type, constant: &Identifier) -> Option<CoreConstant> {
        if let Type::Identifier(ident) = type_ {
            // Lookup core constant
            match CoreConstant::from_symbols(ident.name, constant.name) {
                None => {
                    // Not a core constant.
                    self.emit_err(TypeCheckerError::invalid_core_constant(ident.name, constant.name, ident.span()));
                }
                Some(core_constant) => return Some(core_constant),
            }
        }
        None
    }

    /// Emits an error if the `struct` is not a core library struct.
    /// Emits an error if the `function` is not supported by the struct.
    pub fn get_core_function_call(&self, struct_: &Identifier, function: &Identifier) -> Option<CoreFunction> {
        // Lookup core struct
        match CoreFunction::from_symbols(struct_.name, function.name) {
            None => {
                // Not a core library struct.
                self.emit_err(TypeCheckerError::invalid_core_function(struct_.name, function.name, struct_.span()));
                None
            }
            Some(core_instruction) => Some(core_instruction),
        }
    }

    /// Type checks the inputs to a core function call and returns the expected output type.
    /// Emits an error if the correct number of arguments are not provided.
    /// Emits an error if the arguments are not of the correct type.
    pub fn check_core_function_call(
        &mut self,
        core_function: CoreFunction,
        arguments: &[(Type, Span)],
        function_span: Span,
    ) -> Type {
        // Check that the number of arguments is correct.
        if arguments.len() != core_function.num_args() {
            self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                core_function.num_args(),
                arguments.len(),
                function_span,
            ));
            return Type::Err;
        }

        let assert_not_mapping_tuple_unit = |type_: &Type, span: Span| {
            if matches!(type_, Type::Mapping(_) | Type::Tuple(_) | Type::Unit) {
                self.emit_err(TypeCheckerError::type_should_be2(type_, "anything but a mapping, tuple, or unit", span));
            }
        };

        // Make sure the input is no bigger than 64 bits.
        // Due to overhead in the bitwise representations of types in SnarkVM, 64 bit integers
        // input more than 64 bits to a hash function, as do all structs and arrays.
        let assert_pedersen_64_bit_input = |type_: &Type, span: Span| {
            if !matches!(
                type_,
                Type::Integer(IntegerType::U8)
                    | Type::Integer(IntegerType::U16)
                    | Type::Integer(IntegerType::U32)
                    | Type::Integer(IntegerType::I8)
                    | Type::Integer(IntegerType::I16)
                    | Type::Integer(IntegerType::I32)
                    | Type::Boolean
                    | Type::Err
            ) {
                self.emit_err(TypeCheckerError::type_should_be2(
                    type_,
                    "an integer of less than 64 bits or a bool",
                    span,
                ));
            }
        };

        // Make sure the input is no bigger than 128 bits.
        //
        // Due to overhead in the bitwise representations of types in SnarkVM, 128 bit integers
        // input more than 128 bits to a hash function, as do most structs and arrays. We could
        // actually allow arrays with a single element of type smaller than 64 bits, but it
        // seems most understandable to the user to simply disallow composite types entirely.
        let assert_pedersen_128_bit_input = |type_: &Type, span: Span| {
            if !matches!(
                type_,
                Type::Integer(IntegerType::U8)
                    | Type::Integer(IntegerType::U16)
                    | Type::Integer(IntegerType::U32)
                    | Type::Integer(IntegerType::U64)
                    | Type::Integer(IntegerType::I8)
                    | Type::Integer(IntegerType::I16)
                    | Type::Integer(IntegerType::I32)
                    | Type::Integer(IntegerType::I64)
                    | Type::Boolean
                    | Type::Err
            ) {
                self.emit_err(TypeCheckerError::type_should_be2(
                    type_,
                    "an integer of less than 128 bits or a bool",
                    span,
                ));
            }
        };

        // Check that the arguments are of the correct type.
        match core_function {
            CoreFunction::BHP256CommitToAddress
            | CoreFunction::BHP512CommitToAddress
            | CoreFunction::BHP768CommitToAddress
            | CoreFunction::BHP1024CommitToAddress => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Address
            }
            CoreFunction::BHP256CommitToField
            | CoreFunction::BHP512CommitToField
            | CoreFunction::BHP768CommitToField
            | CoreFunction::BHP1024CommitToField => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Field
            }
            CoreFunction::BHP256CommitToGroup
            | CoreFunction::BHP512CommitToGroup
            | CoreFunction::BHP768CommitToGroup
            | CoreFunction::BHP1024CommitToGroup => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Group
            }
            CoreFunction::BHP256HashToAddress
            | CoreFunction::BHP512HashToAddress
            | CoreFunction::BHP768HashToAddress
            | CoreFunction::BHP1024HashToAddress
            | CoreFunction::Keccak256HashToAddress
            | CoreFunction::Keccak384HashToAddress
            | CoreFunction::Keccak512HashToAddress
            | CoreFunction::Poseidon2HashToAddress
            | CoreFunction::Poseidon4HashToAddress
            | CoreFunction::Poseidon8HashToAddress
            | CoreFunction::SHA3_256HashToAddress
            | CoreFunction::SHA3_384HashToAddress
            | CoreFunction::SHA3_512HashToAddress => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Address
            }
            CoreFunction::BHP256HashToField
            | CoreFunction::BHP512HashToField
            | CoreFunction::BHP768HashToField
            | CoreFunction::BHP1024HashToField
            | CoreFunction::Keccak256HashToField
            | CoreFunction::Keccak384HashToField
            | CoreFunction::Keccak512HashToField
            | CoreFunction::Poseidon2HashToField
            | CoreFunction::Poseidon4HashToField
            | CoreFunction::Poseidon8HashToField
            | CoreFunction::SHA3_256HashToField
            | CoreFunction::SHA3_384HashToField
            | CoreFunction::SHA3_512HashToField => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Field
            }
            CoreFunction::BHP256HashToGroup
            | CoreFunction::BHP512HashToGroup
            | CoreFunction::BHP768HashToGroup
            | CoreFunction::BHP1024HashToGroup
            | CoreFunction::Keccak256HashToGroup
            | CoreFunction::Keccak384HashToGroup
            | CoreFunction::Keccak512HashToGroup
            | CoreFunction::Poseidon2HashToGroup
            | CoreFunction::Poseidon4HashToGroup
            | CoreFunction::Poseidon8HashToGroup
            | CoreFunction::SHA3_256HashToGroup
            | CoreFunction::SHA3_384HashToGroup
            | CoreFunction::SHA3_512HashToGroup => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Group
            }
            CoreFunction::BHP256HashToI8
            | CoreFunction::BHP512HashToI8
            | CoreFunction::BHP768HashToI8
            | CoreFunction::BHP1024HashToI8
            | CoreFunction::Keccak256HashToI8
            | CoreFunction::Keccak384HashToI8
            | CoreFunction::Keccak512HashToI8
            | CoreFunction::Poseidon2HashToI8
            | CoreFunction::Poseidon4HashToI8
            | CoreFunction::Poseidon8HashToI8
            | CoreFunction::SHA3_256HashToI8
            | CoreFunction::SHA3_384HashToI8
            | CoreFunction::SHA3_512HashToI8 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I8)
            }
            CoreFunction::BHP256HashToI16
            | CoreFunction::BHP512HashToI16
            | CoreFunction::BHP768HashToI16
            | CoreFunction::BHP1024HashToI16
            | CoreFunction::Keccak256HashToI16
            | CoreFunction::Keccak384HashToI16
            | CoreFunction::Keccak512HashToI16
            | CoreFunction::Poseidon2HashToI16
            | CoreFunction::Poseidon4HashToI16
            | CoreFunction::Poseidon8HashToI16
            | CoreFunction::SHA3_256HashToI16
            | CoreFunction::SHA3_384HashToI16
            | CoreFunction::SHA3_512HashToI16 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I16)
            }
            CoreFunction::BHP256HashToI32
            | CoreFunction::BHP512HashToI32
            | CoreFunction::BHP768HashToI32
            | CoreFunction::BHP1024HashToI32
            | CoreFunction::Keccak256HashToI32
            | CoreFunction::Keccak384HashToI32
            | CoreFunction::Keccak512HashToI32
            | CoreFunction::Poseidon2HashToI32
            | CoreFunction::Poseidon4HashToI32
            | CoreFunction::Poseidon8HashToI32
            | CoreFunction::SHA3_256HashToI32
            | CoreFunction::SHA3_384HashToI32
            | CoreFunction::SHA3_512HashToI32 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I32)
            }
            CoreFunction::BHP256HashToI64
            | CoreFunction::BHP512HashToI64
            | CoreFunction::BHP768HashToI64
            | CoreFunction::BHP1024HashToI64
            | CoreFunction::Keccak256HashToI64
            | CoreFunction::Keccak384HashToI64
            | CoreFunction::Keccak512HashToI64
            | CoreFunction::Poseidon2HashToI64
            | CoreFunction::Poseidon4HashToI64
            | CoreFunction::Poseidon8HashToI64
            | CoreFunction::SHA3_256HashToI64
            | CoreFunction::SHA3_384HashToI64
            | CoreFunction::SHA3_512HashToI64 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I64)
            }
            CoreFunction::BHP256HashToI128
            | CoreFunction::BHP512HashToI128
            | CoreFunction::BHP768HashToI128
            | CoreFunction::BHP1024HashToI128
            | CoreFunction::Keccak256HashToI128
            | CoreFunction::Keccak384HashToI128
            | CoreFunction::Keccak512HashToI128
            | CoreFunction::Poseidon2HashToI128
            | CoreFunction::Poseidon4HashToI128
            | CoreFunction::Poseidon8HashToI128
            | CoreFunction::SHA3_256HashToI128
            | CoreFunction::SHA3_384HashToI128
            | CoreFunction::SHA3_512HashToI128 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I128)
            }
            CoreFunction::BHP256HashToU8
            | CoreFunction::BHP512HashToU8
            | CoreFunction::BHP768HashToU8
            | CoreFunction::BHP1024HashToU8
            | CoreFunction::Keccak256HashToU8
            | CoreFunction::Keccak384HashToU8
            | CoreFunction::Keccak512HashToU8
            | CoreFunction::Poseidon2HashToU8
            | CoreFunction::Poseidon4HashToU8
            | CoreFunction::Poseidon8HashToU8
            | CoreFunction::SHA3_256HashToU8
            | CoreFunction::SHA3_384HashToU8
            | CoreFunction::SHA3_512HashToU8 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U8)
            }
            CoreFunction::BHP256HashToU16
            | CoreFunction::BHP512HashToU16
            | CoreFunction::BHP768HashToU16
            | CoreFunction::BHP1024HashToU16
            | CoreFunction::Keccak256HashToU16
            | CoreFunction::Keccak384HashToU16
            | CoreFunction::Keccak512HashToU16
            | CoreFunction::Poseidon2HashToU16
            | CoreFunction::Poseidon4HashToU16
            | CoreFunction::Poseidon8HashToU16
            | CoreFunction::SHA3_256HashToU16
            | CoreFunction::SHA3_384HashToU16
            | CoreFunction::SHA3_512HashToU16 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U16)
            }
            CoreFunction::BHP256HashToU32
            | CoreFunction::BHP512HashToU32
            | CoreFunction::BHP768HashToU32
            | CoreFunction::BHP1024HashToU32
            | CoreFunction::Keccak256HashToU32
            | CoreFunction::Keccak384HashToU32
            | CoreFunction::Keccak512HashToU32
            | CoreFunction::Poseidon2HashToU32
            | CoreFunction::Poseidon4HashToU32
            | CoreFunction::Poseidon8HashToU32
            | CoreFunction::SHA3_256HashToU32
            | CoreFunction::SHA3_384HashToU32
            | CoreFunction::SHA3_512HashToU32 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U32)
            }
            CoreFunction::BHP256HashToU64
            | CoreFunction::BHP512HashToU64
            | CoreFunction::BHP768HashToU64
            | CoreFunction::BHP1024HashToU64
            | CoreFunction::Keccak256HashToU64
            | CoreFunction::Keccak384HashToU64
            | CoreFunction::Keccak512HashToU64
            | CoreFunction::Poseidon2HashToU64
            | CoreFunction::Poseidon4HashToU64
            | CoreFunction::Poseidon8HashToU64
            | CoreFunction::SHA3_256HashToU64
            | CoreFunction::SHA3_384HashToU64
            | CoreFunction::SHA3_512HashToU64 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U64)
            }
            CoreFunction::BHP256HashToU128
            | CoreFunction::BHP512HashToU128
            | CoreFunction::BHP768HashToU128
            | CoreFunction::BHP1024HashToU128
            | CoreFunction::Keccak256HashToU128
            | CoreFunction::Keccak384HashToU128
            | CoreFunction::Keccak512HashToU128
            | CoreFunction::Poseidon2HashToU128
            | CoreFunction::Poseidon4HashToU128
            | CoreFunction::Poseidon8HashToU128
            | CoreFunction::SHA3_256HashToU128
            | CoreFunction::SHA3_384HashToU128
            | CoreFunction::SHA3_512HashToU128 => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U128)
            }
            CoreFunction::BHP256HashToScalar
            | CoreFunction::BHP512HashToScalar
            | CoreFunction::BHP768HashToScalar
            | CoreFunction::BHP1024HashToScalar
            | CoreFunction::Keccak256HashToScalar
            | CoreFunction::Keccak384HashToScalar
            | CoreFunction::Keccak512HashToScalar
            | CoreFunction::Poseidon2HashToScalar
            | CoreFunction::Poseidon4HashToScalar
            | CoreFunction::Poseidon8HashToScalar
            | CoreFunction::SHA3_256HashToScalar
            | CoreFunction::SHA3_384HashToScalar
            | CoreFunction::SHA3_512HashToScalar => {
                assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1);
                Type::Scalar
            }
            CoreFunction::Pedersen64CommitToAddress => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Address
            }
            CoreFunction::Pedersen64CommitToField => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Field
            }
            CoreFunction::Pedersen64CommitToGroup => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Group
            }
            CoreFunction::Pedersen64HashToAddress => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Address
            }
            CoreFunction::Pedersen64HashToField => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Field
            }
            CoreFunction::Pedersen64HashToGroup => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Group
            }
            CoreFunction::Pedersen64HashToI8 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I8)
            }
            CoreFunction::Pedersen64HashToI16 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I16)
            }
            CoreFunction::Pedersen64HashToI32 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I32)
            }
            CoreFunction::Pedersen64HashToI64 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I64)
            }
            CoreFunction::Pedersen64HashToI128 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I128)
            }
            CoreFunction::Pedersen64HashToU8 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U8)
            }
            CoreFunction::Pedersen64HashToU16 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U16)
            }
            CoreFunction::Pedersen64HashToU32 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U32)
            }
            CoreFunction::Pedersen64HashToU64 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U64)
            }
            CoreFunction::Pedersen64HashToU128 => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U128)
            }
            CoreFunction::Pedersen64HashToScalar => {
                assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1);
                Type::Scalar
            }
            CoreFunction::Pedersen128CommitToAddress => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Address
            }
            CoreFunction::Pedersen128CommitToField => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Field
            }
            CoreFunction::Pedersen128CommitToGroup => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1);
                Type::Group
            }
            CoreFunction::Pedersen128HashToAddress => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Address
            }
            CoreFunction::Pedersen128HashToField => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Field
            }
            CoreFunction::Pedersen128HashToGroup => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Group
            }
            CoreFunction::Pedersen128HashToI8 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I8)
            }
            CoreFunction::Pedersen128HashToI16 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I16)
            }
            CoreFunction::Pedersen128HashToI32 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I32)
            }
            CoreFunction::Pedersen128HashToI64 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I64)
            }
            CoreFunction::Pedersen128HashToI128 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::I128)
            }
            CoreFunction::Pedersen128HashToU8 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U8)
            }
            CoreFunction::Pedersen128HashToU16 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U16)
            }
            CoreFunction::Pedersen128HashToU32 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U32)
            }
            CoreFunction::Pedersen128HashToU64 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U64)
            }
            CoreFunction::Pedersen128HashToU128 => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Integer(IntegerType::U128)
            }
            CoreFunction::Pedersen128HashToScalar => {
                assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1);
                Type::Scalar
            }
            CoreFunction::MappingGet => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::get", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1);
                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);

                mapping_type.value.deref().clone()
            }
            CoreFunction::MappingGetOrUse => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::get_or", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1);

                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                // Check that the second argument matches the key type of the mapping.
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                // Check that the third argument matches the value type of the mapping.
                self.assert_type(&arguments[2].0, &mapping_type.value, arguments[2].1);

                mapping_type.value.deref().clone()
            }
            CoreFunction::MappingSet => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::set", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1);

                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                // Check that the second argument matches the key type of the mapping.
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                // Check that the third argument matches the value type of the mapping.
                self.assert_type(&arguments[2].0, &mapping_type.value, arguments[2].1);

                Type::Unit
            }
            CoreFunction::MappingRemove => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::remove", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1);

                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                // Cannot modify external mappings.
                if mapping_type.program != self.scope_state.program_name.unwrap() {
                    self.state
                        .handler
                        .emit_err(TypeCheckerError::cannot_modify_external_mapping("remove", function_span));
                }

                // Check that the second argument matches the key type of the mapping.
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);

                Type::Unit
            }
            CoreFunction::MappingContains => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::contains", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1);

                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                // Check that the second argument matches the key type of the mapping.
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);

                Type::Boolean
            }
            CoreFunction::GroupToXCoordinate | CoreFunction::GroupToYCoordinate => {
                // Check that the first argument is a group.
                self.assert_type(&arguments[0].0, &Type::Group, arguments[0].1);
                Type::Field
            }
            CoreFunction::ChaChaRandAddress => Type::Address,
            CoreFunction::ChaChaRandBool => Type::Boolean,
            CoreFunction::ChaChaRandField => Type::Field,
            CoreFunction::ChaChaRandGroup => Type::Group,
            CoreFunction::ChaChaRandI8 => Type::Integer(IntegerType::I8),
            CoreFunction::ChaChaRandI16 => Type::Integer(IntegerType::I16),
            CoreFunction::ChaChaRandI32 => Type::Integer(IntegerType::I32),
            CoreFunction::ChaChaRandI64 => Type::Integer(IntegerType::I64),
            CoreFunction::ChaChaRandI128 => Type::Integer(IntegerType::I128),
            CoreFunction::ChaChaRandScalar => Type::Scalar,
            CoreFunction::ChaChaRandU8 => Type::Integer(IntegerType::U8),
            CoreFunction::ChaChaRandU16 => Type::Integer(IntegerType::U16),
            CoreFunction::ChaChaRandU32 => Type::Integer(IntegerType::U32),
            CoreFunction::ChaChaRandU64 => Type::Integer(IntegerType::U64),
            CoreFunction::ChaChaRandU128 => Type::Integer(IntegerType::U128),
            CoreFunction::SignatureVerify => {
                // Check that the third argument is not a mapping nor a tuple. We have to do this
                // before the other checks below to appease the borrow checker
                assert_not_mapping_tuple_unit(&arguments[2].0, arguments[2].1);

                // Check that the first argument is a signature.
                self.assert_type(&arguments[0].0, &Type::Signature, arguments[0].1);
                // Check that the second argument is an address.
                self.assert_type(&arguments[1].0, &Type::Address, arguments[1].1);
                Type::Boolean
            }
            CoreFunction::FutureAwait => Type::Unit,
            CoreFunction::CheatCodePrintMapping => {
                self.assert_mapping_type(&arguments[0].0, arguments[0].1);
                Type::Unit
            }
            CoreFunction::CheatCodeSetBlockHeight => {
                self.assert_type(&arguments[0].0, &Type::Integer(IntegerType::U32), arguments[0].1);
                Type::Unit
            }
        }
    }

    /// Emits an error if the struct member is a record type.
    pub fn assert_member_is_not_record(&mut self, span: Span, parent: Symbol, type_: &Type) {
        match type_ {
            Type::Composite(struct_)
                if self
                    .lookup_struct(struct_.program.or(self.scope_state.program_name), struct_.id.name)
                    .is_some_and(|struct_| struct_.is_record) =>
            {
                self.emit_err(TypeCheckerError::struct_or_record_cannot_contain_record(parent, struct_.id.name, span))
            }
            Type::Tuple(tuple_type) => {
                for type_ in tuple_type.elements().iter() {
                    self.assert_member_is_not_record(span, parent, type_)
                }
            }
            _ => {} // Do nothing.
        }
    }

    /// Emits an error if the type or its constituent types is not valid.
    pub fn assert_type_is_valid(&mut self, type_: &Type, span: Span) {
        match type_ {
            // Unit types may only appear as the return type of a function.
            Type::Unit => {
                self.emit_err(TypeCheckerError::unit_type_only_return(span));
            }
            // String types are temporarily disabled.
            Type::String => {
                self.emit_err(TypeCheckerError::strings_are_not_supported(span));
            }
            // Check that the named composite type has been defined.
            Type::Composite(struct_)
                if self.lookup_struct(struct_.program.or(self.scope_state.program_name), struct_.id.name).is_none() =>
            {
                self.emit_err(TypeCheckerError::undefined_type(struct_.id.name, span));
            }
            // Check that the constituent types of the tuple are valid.
            Type::Tuple(tuple_type) => {
                for type_ in tuple_type.elements().iter() {
                    self.assert_type_is_valid(type_, span);
                }
            }
            // Check that the constituent types of mapping are valid.
            Type::Mapping(mapping_type) => {
                self.assert_type_is_valid(&mapping_type.key, span);
                self.assert_type_is_valid(&mapping_type.value, span);
            }
            // Check that the array element types are valid.
            Type::Array(array_type) => {
                // Check that the array length is valid.
                match array_type.length() {
                    0 => self.emit_err(TypeCheckerError::array_empty(span)),
                    length => {
                        if length > self.limits.max_array_elements {
                            self.emit_err(TypeCheckerError::array_too_large(
                                length,
                                self.limits.max_array_elements,
                                span,
                            ))
                        }
                    }
                }
                // Check that the array element type is valid.
                match array_type.element_type() {
                    // Array elements cannot be futures.
                    Type::Future(_) => self.emit_err(TypeCheckerError::array_element_cannot_be_future(span)),
                    // Array elements cannot be tuples.
                    Type::Tuple(_) => self.emit_err(TypeCheckerError::array_element_cannot_be_tuple(span)),
                    // Array elements cannot be records.
                    Type::Composite(struct_type) => {
                        // Look up the type.
                        if let Some(struct_) = self
                            .lookup_struct(struct_type.program.or(self.scope_state.program_name), struct_type.id.name)
                        {
                            // Check that the type is not a record.
                            if struct_.is_record {
                                self.emit_err(TypeCheckerError::array_element_cannot_be_record(span));
                            }
                        }
                    }
                    _ => {} // Do nothing.
                }
                self.assert_type_is_valid(array_type.element_type(), span);
            }
            _ => {} // Do nothing.
        }
    }

    /// Emits an error if the type is not a mapping.
    pub fn assert_mapping_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Mapping(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a mapping", span));
        }
    }

    pub fn assert_array_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Array(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "an array", span));
        }
    }

    /// Helper function to check that the input and output of function are valid
    pub fn check_function_signature(&mut self, function: &Function, is_stub: bool) {
        self.scope_state.variant = Some(function.variant);

        let mut inferred_inputs: Vec<Type> = Vec::new();

        if self.scope_state.variant == Some(Variant::AsyncFunction) && !self.scope_state.is_stub {
            // Async functions are not allowed to return values.
            if !function.output.is_empty() {
                self.emit_err(TypeCheckerError::async_function_cannot_return_value(function.span()));
            }

            // Iterator over the `finalize` member (type Finalizer) of each async transition that calls
            // this async function.
            let mut caller_finalizers = self
                .async_function_callers
                .get(&Location::new(self.scope_state.program_name.unwrap(), function.identifier.name))
                .map(|callers| {
                    callers
                        .iter()
                        .flat_map(|caller| {
                            let caller = Location::new(caller.program, caller.name);
                            self.state.symbol_table.lookup_function(caller)
                        })
                        .flat_map(|fn_symbol| fn_symbol.finalizer.clone())
                })
                .into_iter()
                .flatten();

            if let Some(first) = caller_finalizers.next() {
                inferred_inputs = first.inferred_inputs.clone();
            } else {
                self.emit_warning(TypeCheckerWarning::async_function_is_never_called_by_transition_function(
                    function.identifier.name,
                    function.span(),
                ));
            }

            // If any input is a future that doesn't have the same member type for all
            // finalizers, set that member to `Type::Err`.
            for finalizer in caller_finalizers {
                assert_eq!(inferred_inputs.len(), finalizer.inferred_inputs.len());
                for (t1, t2) in inferred_inputs.iter_mut().zip(finalizer.inferred_inputs.iter()) {
                    self.merge_types(t1, t2);
                }
            }
        }

        // Ensure that, if the function has generic const paramaters, then it must be an `inline`.
        // Otherwise, emit an error.
        if self.scope_state.variant != Some(Variant::Inline) && !function.const_parameters.is_empty() {
            self.emit_err(TypeCheckerError::only_inline_can_have_const_generics(function.identifier.span()));
        }

        for const_param in &function.const_parameters {
            // Restrictions for const parameters
            if !matches!(
                const_param.type_(),
                Type::Boolean | Type::Integer(_) | Type::Address | Type::Scalar | Type::Group | Type::Field
            ) {
                self.emit_err(TypeCheckerError::bad_const_generic_type(const_param.type_(), const_param.span()));
            }

            // Add the input to the symbol table.
            if let Err(err) = self.state.symbol_table.insert_variable(
                self.scope_state.program_name.unwrap(),
                const_param.identifier().name,
                VariableSymbol {
                    type_: const_param.type_().clone(),
                    span: const_param.identifier.span(),
                    declaration: VariableType::ConstParameter,
                },
            ) {
                self.state.handler.emit_err(err);
            }

            // Add the input to the type table.
            self.state.type_table.insert(const_param.identifier().id(), const_param.type_().clone());
        }

        for (i, input) in function.input.iter().enumerate() {
            // No need to check compatibility of these types; that's already been done
            let table_type = inferred_inputs.get(i).unwrap_or_else(|| input.type_());

            // Check that the type of input parameter is defined.
            self.assert_type_is_valid(table_type, input.span());

            // Check that the type of the input parameter is not a tuple.
            if matches!(table_type, Type::Tuple(_)) {
                self.emit_err(TypeCheckerError::function_cannot_take_tuple_as_input(input.span()))
            }

            // Make sure only transitions can take a record as an input.
            if let Type::Composite(struct_) = table_type {
                // Throw error for undefined type.
                if !function.variant.is_transition() {
                    if let Some(elem) =
                        self.lookup_struct(struct_.program.or(self.scope_state.program_name), struct_.id.name)
                    {
                        if elem.is_record {
                            self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(input.span()))
                        }
                    } else {
                        self.emit_err(TypeCheckerError::undefined_type(struct_.id, input.span()));
                    }
                }
            }

            // This unwrap works since we assign to `variant` above.
            match self.scope_state.variant.unwrap() {
                // If the function is a transition function, then check that the parameter mode is not a constant.
                Variant::Transition | Variant::AsyncTransition if input.mode() == Mode::Constant => {
                    self.emit_err(TypeCheckerError::transition_function_inputs_cannot_be_const(input.span()))
                }
                // If the function is standard function or inline, then check that the parameters do not have an associated mode.
                Variant::Function | Variant::Inline if input.mode() != Mode::None => {
                    self.emit_err(TypeCheckerError::regular_function_inputs_cannot_have_modes(input.span()))
                }
                // If the function is an async function, then check that the input parameter is not constant or private.
                Variant::AsyncFunction if matches!(input.mode(), Mode::Constant | Mode::Private) => {
                    self.emit_err(TypeCheckerError::async_function_input_must_be_public(input.span()));
                }
                _ => {} // Do nothing.
            }

            if matches!(table_type, Type::Future(..)) {
                // Future parameters may only appear in async functions.
                if !matches!(self.scope_state.variant, Some(Variant::AsyncFunction)) {
                    self.emit_err(TypeCheckerError::no_future_parameters(input.span()));
                }
            }

            if !is_stub {
                // Add the input to the symbol table.
                if let Err(err) = self.state.symbol_table.insert_variable(
                    self.scope_state.program_name.unwrap(),
                    input.identifier().name,
                    VariableSymbol {
                        type_: table_type.clone(),
                        span: input.identifier.span(),
                        declaration: VariableType::Input(input.mode()),
                    },
                ) {
                    self.state.handler.emit_err(err);
                }

                // Add the input to the type table.
                self.state.type_table.insert(input.identifier().id(), table_type.clone());
            }
        }

        // Type check the function's return type.
        // Note that checking that each of the component types are defined is sufficient to check that `output_type` is defined.
        function.output.iter().enumerate().for_each(|(index, function_output)| {
            // If the function is not a transition function, then it cannot output a record.
            // Note that an external output must always be a record.
            if let Type::Composite(struct_) = function_output.type_.clone() {
                if let Some(val) =
                    self.lookup_struct(struct_.program.or(self.scope_state.program_name), struct_.id.name)
                {
                    if val.is_record && !function.variant.is_transition() {
                        self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(function_output.span));
                    }
                }
            }

            // Check that the output type is valid.
            self.assert_type_is_valid(&function_output.type_, function_output.span);

            // Check that the type of the output is not a tuple. This is necessary to forbid nested tuples.
            if matches!(&function_output.type_, Type::Tuple(_)) {
                self.emit_err(TypeCheckerError::nested_tuple_type(function_output.span))
            }
            // Check that the mode of the output is valid.
            // For functions, only public and private outputs are allowed
            if function_output.mode == Mode::Constant {
                self.emit_err(TypeCheckerError::cannot_have_constant_output_mode(function_output.span));
            }
            // Async transitions must return exactly one future, and it must be in the last position.
            if self.scope_state.variant == Some(Variant::AsyncTransition)
                && ((index < function.output.len() - 1 && matches!(function_output.type_, Type::Future(_)))
                    || (index == function.output.len() - 1 && !matches!(function_output.type_, Type::Future(_))))
            {
                self.emit_err(TypeCheckerError::async_transition_invalid_output(function_output.span));
            }
            // If the function is not an async transition, then it cannot have a future as output.
            if !matches!(self.scope_state.variant, Some(Variant::AsyncTransition) | Some(Variant::Script))
                && matches!(function_output.type_, Type::Future(_))
            {
                self.emit_err(TypeCheckerError::only_async_transition_can_return_future(function_output.span));
            }
        });
    }

    /// Merge inferred types into `lhs`.
    ///
    /// That is, if `lhs` and `rhs` aren't equal, set `lhs` to Type::Err;
    /// or, if they're both futures, set any member of `lhs` that isn't
    /// equal to the equivalent member of `rhs` to `Type::Err`.
    fn merge_types(&self, lhs: &mut Type, rhs: &Type) {
        if let Type::Future(f1) = lhs {
            if let Type::Future(f2) = rhs {
                for (i, type_) in f2.inputs.iter().enumerate() {
                    if let Some(lhs_type) = f1.inputs.get_mut(i) {
                        self.merge_types(lhs_type, type_);
                    } else {
                        f1.inputs.push(Type::Err);
                    }
                }
            } else {
                *lhs = Type::Err;
            }
        } else if !self.eq_user(lhs, rhs) {
            *lhs = Type::Err;
        }
    }

    /// Are the types considered equal as far as the Leo user is concerned?
    ///
    /// In particular, any comparison involving an `Err` is `true`,
    /// composite types are resolved to the current program if not specified,
    /// and Futures which aren't explicit compare equal to other Futures.
    pub fn eq_user(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (Type::Err, _)
            | (_, Type::Err)
            | (Type::Address, Type::Address)
            | (Type::Boolean, Type::Boolean)
            | (Type::Field, Type::Field)
            | (Type::Group, Type::Group)
            | (Type::Scalar, Type::Scalar)
            | (Type::Signature, Type::Signature)
            | (Type::String, Type::String)
            | (Type::Unit, Type::Unit) => true,
            (Type::Array(left), Type::Array(right)) => {
                left.length() == right.length() && self.eq_user(left.element_type(), right.element_type())
            }
            (Type::Identifier(left), Type::Identifier(right)) => left.name == right.name,
            (Type::Integer(left), Type::Integer(right)) => left == right,
            (Type::Mapping(left), Type::Mapping(right)) => {
                self.eq_user(&left.key, &right.key) && self.eq_user(&left.value, &right.value)
            }
            (Type::Tuple(left), Type::Tuple(right)) if left.length() == right.length() => left
                .elements()
                .iter()
                .zip_eq(right.elements().iter())
                .all(|(left_type, right_type)| self.eq_user(left_type, right_type)),
            (Type::Composite(left), Type::Composite(right)) => {
                let left_program = left.program.or(self.scope_state.program_name);
                let right_program = right.program.or(self.scope_state.program_name);

                left.id.name == right.id.name && left_program == right_program
            }
            (Type::Future(left), Type::Future(right)) if !left.is_explicit || !right.is_explicit => true,
            (Type::Future(left), Type::Future(right)) if left.inputs.len() == right.inputs.len() => left
                .inputs()
                .iter()
                .zip_eq(right.inputs().iter())
                .all(|(left_type, right_type)| self.eq_user(left_type, right_type)),
            _ => false,
        }
    }

    /// Wrapper around lookup_struct that additionally records all structs that are used in the program.
    pub fn lookup_struct(&mut self, program: Option<Symbol>, name: Symbol) -> Option<Composite> {
        let record_comp = program.and_then(|prog| self.state.symbol_table.lookup_record(Location::new(prog, name)));
        let comp = record_comp.or_else(|| self.state.symbol_table.lookup_struct(name));
        // Record the usage.
        if let Some(s) = comp {
            // If it's a struct or internal record, mark it used.
            if !s.is_record || program == self.scope_state.program_name {
                self.used_structs.insert(s.identifier.name);
            }
        }
        comp.cloned()
    }

    /// Inserts variable to symbol table.
    pub fn insert_variable(&mut self, inferred_type: Option<Type>, name: &Identifier, type_: Type, span: Span) {
        self.insert_symbol_conditional_scope(name.name);

        let is_future = match &type_ {
            Type::Future(..) => true,
            Type::Tuple(tuple_type) if matches!(tuple_type.elements().last(), Some(Type::Future(..))) => true,
            _ => false,
        };

        if is_future {
            // It can happen that the call location has not been set if there was an error
            // in the call that produced the Future.
            if let Some(call_location) = self.scope_state.call_location {
                self.scope_state.futures.insert(name.name, call_location);
            }
        }

        let ty = match (is_future, inferred_type) {
            (false, _) => type_,
            (true, Some(inferred)) => inferred,
            (true, None) => unreachable!("Type checking guarantees the inferred type is present"),
        };

        // Insert the variable into the symbol table.
        if let Err(err) =
            self.state.symbol_table.insert_variable(self.scope_state.program_name.unwrap(), name.name, VariableSymbol {
                type_: ty.clone(),
                span,
                declaration: VariableType::Mut,
            })
        {
            self.state.handler.emit_err(err);
        }
    }

    // Checks if the access operation is valid inside the current function variant.
    pub fn check_access_allowed(&mut self, name: &str, finalize_op: bool, span: Span) {
        // Check that the function context matches.
        if self.scope_state.variant == Some(Variant::AsyncFunction) && !finalize_op {
            self.state.handler.emit_err(TypeCheckerError::invalid_operation_inside_finalize(name, span))
        } else if !matches!(self.scope_state.variant, Some(Variant::AsyncFunction) | Some(Variant::Script))
            && finalize_op
        {
            self.state.handler.emit_err(TypeCheckerError::invalid_operation_outside_finalize(name, span))
        }
    }

    pub fn is_external_record(&self, ty: &Type) -> bool {
        if let Type::Composite(typ) = &ty {
            let this_program = self.scope_state.program_name.unwrap();
            let program = typ.program.unwrap_or(this_program);
            program != this_program
                && self.state.symbol_table.lookup_record(Location::new(program, typ.id.name)).is_some()
        } else {
            false
        }
    }

    pub fn parse_integer_literal<I: FromStrRadix>(&self, raw_string: &str, span: Span, type_string: &str) {
        let string = raw_string.replace('_', "");
        if I::from_str_by_radix(&string).is_err() {
            self.state.handler.emit_err(TypeCheckerError::invalid_int_value(string, type_string, span));
        }
    }

    // Emit an error and update `ty` to be `Type::Err` indicating that the type of the expression could not be inferred.
    // Also update `type_table` accordingly
    pub fn emit_inference_failure_error(&self, ty: &mut Type, expr: &Expression) {
        self.emit_err(TypeCheckerError::could_not_determine_type(expr.clone(), expr.span()));
        *ty = Type::Err;
        self.state.type_table.insert(expr.id(), Type::Err);
    }

    // Given a `Literal` and its type, if the literal is a numeric `Unsuffixed` literal, ensure it's a valid literal
    // given the type. E.g., a `256` is not a valid `u8`.
    pub fn check_numeric_literal(&self, input: &Literal, ty: &Type) {
        if let Literal { variant: LiteralVariant::Unsuffixed(s), .. } = input {
            let span = input.span();
            let has_nondecimal_prefix =
                |s: &str| ["0x", "0o", "0b", "-0x", "-0o", "-0b"].iter().any(|p| s.starts_with(p));

            macro_rules! parse_int {
                ($t:ty, $name:expr) => {
                    self.parse_integer_literal::<$t>(s, span, $name)
                };
            }

            match ty {
                Type::Integer(kind) => match kind {
                    IntegerType::U8 => parse_int!(u8, "u8"),
                    IntegerType::U16 => parse_int!(u16, "u16"),
                    IntegerType::U32 => parse_int!(u32, "u32"),
                    IntegerType::U64 => parse_int!(u64, "u64"),
                    IntegerType::U128 => parse_int!(u128, "u128"),
                    IntegerType::I8 => parse_int!(i8, "i8"),
                    IntegerType::I16 => parse_int!(i16, "i16"),
                    IntegerType::I32 => parse_int!(i32, "i32"),
                    IntegerType::I64 => parse_int!(i64, "i64"),
                    IntegerType::I128 => parse_int!(i128, "i128"),
                },
                Type::Group => {
                    if has_nondecimal_prefix(s) {
                        // This is not checked in the parser for unsuffixed numerals. So do that here.
                        self.emit_err(TypeCheckerError::hexbin_literal_nonintegers(span));
                    } else {
                        let trimmed = s.trim_start_matches('-').trim_start_matches('0');
                        if !trimmed.is_empty()
                            && format!("{trimmed}group")
                                .parse::<snarkvm::prelude::Group<snarkvm::prelude::TestnetV0>>()
                                .is_err()
                        {
                            self.emit_err(TypeCheckerError::invalid_int_value(trimmed, "group", span));
                        }
                    }
                }
                Type::Field | Type::Scalar => {
                    if has_nondecimal_prefix(s) {
                        // This is not checked in the parser for unsuffixed numerals. So do that here.
                        self.emit_err(TypeCheckerError::hexbin_literal_nonintegers(span));
                    }
                }
                _ => {
                    // Other types aren't expected here
                }
            }
        }
    }
}
