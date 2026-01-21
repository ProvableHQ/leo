// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{CompilerState, type_checking::scope_state::ScopeState};

use super::*;

use leo_ast::*;
use leo_errors::{TypeCheckerError, TypeCheckerWarning};
use leo_span::{Span, Symbol};

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use snarkvm::{
    console::algorithms::ECDSASignature,
    prelude::PrivateKey,
    synthesizer::program::{CommitVariant, DeserializeVariant, ECDSAVerifyVariant, HashVariant, SerializeVariant},
};
use std::{ops::Deref, str::FromStr};

pub struct TypeCheckingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The state of the current scope being traversed.
    pub scope_state: ScopeState,
    /// Mapping from async function stub name to the inferred input types.
    pub async_function_input_types: IndexMap<Location, Vec<Type>>,
    /// Mapping from async function name to the names of async transition callers.
    pub async_function_callers: IndexMap<Location, IndexSet<Location>>,
    /// The set of used composites.
    pub used_composites: IndexSet<Vec<Symbol>>,
    /// So we can check if we exceed limits on array size, number of mappings, or number of functions.
    pub limits: TypeCheckingInput,
    /// For detecting the error `TypeCheckerError::async_cannot_assign_outside_conditional`.
    pub conditional_scopes: Vec<IndexSet<Symbol>>,
    /// If we're inside an async block, this is the node ID of the contained `Block`. Otherwise, this is `None`.
    pub async_block_id: Option<NodeID>,
}

impl TypeCheckingVisitor<'_> {
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_existing_scope(Some(id));
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
    pub fn emit_warning(&mut self, warning: TypeCheckerWarning) {
        if self.state.warnings.insert(warning.clone().into()) {
            self.state.handler.emit_warning(warning);
        }
    }

    /// Emits an error if the two given types are not equal.
    pub fn check_eq_types(&self, t1: &Option<Type>, t2: &Option<Type>, span: Span) {
        match (t1, t2) {
            (Some(t1), Some(t2)) if !t1.eq_flat_relaxed(t2) => {
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
        let is_record = |loc: &Location| self.state.symbol_table.lookup_record(loc).is_some();
        if actual != &Type::Err && !actual.can_coerce_to(expected, &is_record) {
            // If `actual` is Err, we will have already reported an error.
            self.emit_err(TypeCheckerError::type_should_be2(actual, format!("type `{expected}`"), span));
        }
    }

    /// Unwraps an optional type to its inner type for use with operands.
    /// If the expected type is `T?`, returns `Some(T)`. Otherwise returns the type as-is.
    pub fn unwrap_optional_type(&self, expected: &Option<Type>) -> Option<Type> {
        match expected {
            Some(Type::Optional(opt_type)) => Some(*opt_type.inner.clone()),
            other => other.clone(),
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

    /// Emits an error if the intrinsic invocation is not valid.
    pub fn get_intrinsic(&mut self, intrinsic_expr: &IntrinsicExpression) -> Option<Intrinsic> {
        // Lookup core struct
        match Intrinsic::from_symbol(intrinsic_expr.name, &intrinsic_expr.type_parameters) {
            None if intrinsic_expr.name == Symbol::intern("__unresolved_get") => {
                let ty = self.visit_expression(&intrinsic_expr.arguments[0], &None);
                self.assert_vector_or_mapping_type(&ty, intrinsic_expr.arguments[0].span());
                match ty {
                    Type::Vector(_) => Some(Intrinsic::VectorGet),
                    Type::Mapping(_) => Some(Intrinsic::MappingGet),
                    _ => None,
                }
            }
            None if intrinsic_expr.name == Symbol::intern("__unresolved_set") => {
                let ty = self.visit_expression(&intrinsic_expr.arguments[0], &None);
                self.assert_vector_or_mapping_type(&ty, intrinsic_expr.arguments[0].span());
                match ty {
                    Type::Vector(_) => Some(Intrinsic::VectorSet),
                    Type::Mapping(_) => Some(Intrinsic::MappingSet),
                    _ => None,
                }
            }
            None => {
                // Not a core library struct.
                self.emit_err(TypeCheckerError::invalid_intrinsic(intrinsic_expr.name, intrinsic_expr.span()));
                None
            }
            intrinsic @ Some(Intrinsic::Deserialize(_, _)) => intrinsic,
            Some(intrinsic) => {
                // Check that the number of type parameters is 0.
                if !intrinsic_expr.type_parameters.is_empty() {
                    self.emit_err(TypeCheckerError::custom(
                        format!("The intrinsic `{}` cannot have type parameters.", intrinsic_expr.name),
                        intrinsic_expr.span(),
                    ));
                    return None;
                };

                Some(intrinsic)
            }
        }
    }

    /// Type checks the inputs to an intrinsic call and returns the expected output type.
    /// Emits an error if the correct number of arguments are not provided.
    /// Emits an error if the arguments are not of the correct type.
    pub fn check_intrinsic(
        &mut self,
        intrinsic: Intrinsic,
        arguments: &[Expression],
        expected: &Option<Type>,
        function_span: Span,
    ) -> Type {
        // Check that the number of arguments is correct.
        if arguments.len() != intrinsic.num_args() {
            self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                intrinsic.num_args(),
                arguments.len(),
                function_span,
            ));
            return Type::Err;
        }

        // Type check and reconstructs the arguments for a given intrinsic call.
        //
        // Depending on the intrinsic, this handles:
        // - Optional operations (`unwrap`, `unwrap_or`) with proper type inference
        // - Container access (`Get`, `Set`) for vectors and mappings
        // - Vector-specific operations (`push`, `swap_remove`)
        // - Default handling for other intrinsics
        //
        // Returns a `Vec<(Type, &Expression)>` pairing each argument with its inferred type, or `Type::Err` if
        // type-checking fails. Argument counts are assumed to be already validated
        let arguments = match intrinsic {
            Intrinsic::OptionalUnwrap => {
                // Expect exactly one argument
                let [opt] = arguments else { panic!("number of arguments is already checked") };

                // If an expected type is provided, wrap it in Optional for type-checking
                let opt_ty = if let Some(expected) = expected {
                    self.visit_expression(
                        opt,
                        &Some(Type::Optional(OptionalType { inner: Box::new(expected.clone()) })),
                    )
                } else {
                    self.visit_expression(opt, &None)
                };

                vec![(opt_ty, opt)]
            }

            Intrinsic::OptionalUnwrapOr => {
                // Expect exactly two arguments: the optional and the fallback value
                let [opt, fallback] = arguments else { panic!("number of arguments is already checked") };

                if let Some(expected) = expected {
                    // Both arguments are typed based on the expected type
                    let opt_ty = self.visit_expression(
                        opt,
                        &Some(Type::Optional(OptionalType { inner: Box::new(expected.clone()) })),
                    );
                    let fallback_ty = self.visit_expression(fallback, &Some(expected.clone()));
                    vec![(opt_ty, opt), (fallback_ty, fallback)]
                } else {
                    // Infer type from the optional argument
                    let opt_ty = self.visit_expression(opt, &None);
                    let fallback_ty = if let Type::Optional(OptionalType { inner }) = &opt_ty {
                        self.visit_expression(fallback, &Some(*inner.clone()))
                    } else {
                        self.visit_expression(fallback, &None)
                    };
                    vec![(opt_ty, opt), (fallback_ty, fallback)]
                }
            }

            Intrinsic::MappingGet => {
                let [container, key_or_index] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                // Key type depends on container type
                let key_or_index_ty = if let Type::Mapping(MappingType { ref key, .. }) = container_ty {
                    self.visit_expression(key_or_index, &Some(*key.clone()))
                } else {
                    self.visit_expression(key_or_index, &None)
                };

                vec![(container_ty, container), (key_or_index_ty, key_or_index)]
            }

            Intrinsic::VectorGet => {
                let [container, key_or_index] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                // Key type depends on container type
                let key_or_index_ty = self.visit_expression_infer_default_u32(key_or_index);

                vec![(container_ty, container), (key_or_index_ty, key_or_index)]
            }

            Intrinsic::MappingSet => {
                let [container, key_or_index, val] = arguments else {
                    panic!("number of arguments is already checked")
                };

                let container_ty = self.visit_expression(container, &None);

                let key_or_index_ty = match container_ty {
                    Type::Vector(_) => self.visit_expression_infer_default_u32(key_or_index),
                    Type::Mapping(MappingType { ref key, .. }) => {
                        self.visit_expression(key_or_index, &Some(*key.clone()))
                    }
                    _ => self.visit_expression(key_or_index, &None),
                };

                let val_ty = if let Type::Mapping(MappingType { ref value, .. }) = container_ty {
                    self.visit_expression(val, &Some(*value.clone()))
                } else {
                    self.visit_expression(val, &None)
                };

                vec![(container_ty, container), (key_or_index_ty, key_or_index), (val_ty, val)]
            }
            Intrinsic::VectorSet => {
                let [container, key_or_index, val] = arguments else {
                    panic!("number of arguments is already checked")
                };

                let container_ty = self.visit_expression(container, &None);

                let key_or_index_ty = self.visit_expression_infer_default_u32(key_or_index);

                let val_ty = if let Type::Vector(VectorType { ref element_type }) = container_ty {
                    self.visit_expression(val, &Some(*element_type.clone()))
                } else {
                    self.visit_expression(val, &None)
                };

                vec![(container_ty, container), (key_or_index_ty, key_or_index), (val_ty, val)]
            }

            Intrinsic::VectorPush => {
                let [vec, val] = arguments else { panic!("number of arguments is already checked") };

                // Check vector type
                let vec_ty = self.visit_expression(vec, &None);

                // Type-check value against vector element type
                let val_ty = if let Type::Vector(VectorType { element_type }) = &vec_ty {
                    self.visit_expression(val, &Some(*element_type.clone()))
                } else {
                    self.visit_expression(val, &None)
                };

                vec![(vec_ty, vec), (val_ty, val)]
            }

            Intrinsic::VectorSwapRemove => {
                let [vec, index] = arguments else { panic!("number of arguments is already checked") };

                let vec_ty = self.visit_expression(vec, &None);

                // Default to u32 index for vector operations
                let index_ty = self.visit_expression_infer_default_u32(index);

                vec![(vec_ty, vec), (index_ty, index)]
            }

            // Default case for other intrinsics
            _ => {
                arguments.iter().map(|arg| (self.visit_expression_reject_numeric(arg, &None), arg)).collect::<Vec<_>>()
            }
        };

        let assert_not_mapping_tuple_unit = |type_: &Type, span: Span| {
            if matches!(type_, Type::Mapping(_) | Type::Tuple(_) | Type::Unit) {
                self.emit_err(TypeCheckerError::type_should_be2(type_, "anything but a mapping, tuple, or unit", span));
            }
        };

        // Make sure the input is no bigger than 64 bits.
        // Due to overhead in the bitwise representations of types in SnarkVM, 64 bit integers
        // input more than 64 bits to a hash function, as do all composites and arrays.
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
        // input more than 128 bits to a hash function, as do most composites and arrays. We could
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

        // Define a regex to match valid program IDs.
        let program_id_regex = regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*\.aleo$").unwrap();

        // Check that the arguments are of the correct type.
        match intrinsic {
            Intrinsic::Commit(variant, type_) => {
                match variant {
                    CommitVariant::CommitPED64 => {
                        assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1.span());
                    }
                    CommitVariant::CommitPED128 => {
                        assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1.span());
                    }
                    _ => {
                        assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1.span());
                    }
                }
                self.assert_type(&arguments[1].0, &Type::Scalar, arguments[1].1.span());
                type_.into()
            }
            Intrinsic::Hash(variant, type_) => {
                // If the hash variant must be byte aligned, check that the number bits of the input is a multiple of 8.
                if variant.requires_byte_alignment() {
                    // Get the input type.
                    let input_type = &arguments[0].0;
                    // Get the size in bits.
                    let size_in_bits = match self.state.network {
                        NetworkName::TestnetV0 => input_type
                            .size_in_bits::<TestnetV0, _>(variant.is_raw(), |_| bail!("structs are not supported")),
                        NetworkName::MainnetV0 => input_type
                            .size_in_bits::<MainnetV0, _>(variant.is_raw(), |_| bail!("structs are not supported")),
                        NetworkName::CanaryV0 => input_type
                            .size_in_bits::<CanaryV0, _>(variant.is_raw(), |_| bail!("structs are not supported")),
                    };
                    if let Ok(size_in_bits) = size_in_bits {
                        // Check that the size in bits is a multiple of 8.
                        if size_in_bits % 8 != 0 {
                            self.emit_err(TypeCheckerError::type_should_be2(
                                input_type,
                                "a type with a size in bits that is a multiple of 8",
                                arguments[0].1.span(),
                            ));
                            return Type::Err;
                        }
                    };
                }
                match variant {
                    HashVariant::HashPED64 => {
                        assert_pedersen_64_bit_input(&arguments[0].0, arguments[0].1.span());
                    }
                    HashVariant::HashPED128 => {
                        assert_pedersen_128_bit_input(&arguments[0].0, arguments[0].1.span());
                    }
                    _ => {
                        assert_not_mapping_tuple_unit(&arguments[0].0, arguments[0].1.span());
                    }
                }
                type_
            }
            Intrinsic::ECDSAVerify(variant) => {
                // Get the expected signature size.
                let signature_size = ECDSASignature::SIGNATURE_SIZE_IN_BYTES;
                // Check that the first input is a 65-byte array.
                let Type::Array(array_type) = &arguments[0].0 else {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        &arguments[0].0,
                        format!("a [u8; {signature_size}]"),
                        arguments[0].1.span(),
                    ));
                    return Type::Err;
                };
                self.assert_type(array_type.element_type(), &Type::Integer(IntegerType::U8), arguments[0].1.span());
                if let Some(length) = array_type.length.as_u32()
                    && length as usize != signature_size
                {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        &arguments[0].0,
                        format!("a [u8; {signature_size}]"),
                        arguments[0].1.span(),
                    ));
                    return Type::Err;
                };

                // Determine whether the intrinsic is Ethereum-specifc.
                let is_eth = match variant {
                    ECDSAVerifyVariant::Digest => false,
                    ECDSAVerifyVariant::DigestEth => true,
                    ECDSAVerifyVariant::HashKeccak256 => false,
                    ECDSAVerifyVariant::HashKeccak256Raw => false,
                    ECDSAVerifyVariant::HashKeccak256Eth => true,
                    ECDSAVerifyVariant::HashKeccak384 => false,
                    ECDSAVerifyVariant::HashKeccak384Raw => false,
                    ECDSAVerifyVariant::HashKeccak384Eth => true,
                    ECDSAVerifyVariant::HashKeccak512 => false,
                    ECDSAVerifyVariant::HashKeccak512Raw => false,
                    ECDSAVerifyVariant::HashKeccak512Eth => true,
                    ECDSAVerifyVariant::HashSha3_256 => false,
                    ECDSAVerifyVariant::HashSha3_256Raw => false,
                    ECDSAVerifyVariant::HashSha3_256Eth => true,
                    ECDSAVerifyVariant::HashSha3_384 => false,
                    ECDSAVerifyVariant::HashSha3_384Raw => false,
                    ECDSAVerifyVariant::HashSha3_384Eth => true,
                    ECDSAVerifyVariant::HashSha3_512 => false,
                    ECDSAVerifyVariant::HashSha3_512Raw => false,
                    ECDSAVerifyVariant::HashSha3_512Eth => true,
                };
                // Get the expected length of the second input.
                let expected_length = if is_eth {
                    ECDSASignature::ETHEREUM_ADDRESS_SIZE_IN_BYTES
                } else {
                    ECDSASignature::VERIFYING_KEY_SIZE_IN_BYTES
                };
                // Check that the second input is a byte array of the expected length.
                let Type::Array(array_type) = &arguments[1].0 else {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        &arguments[1].0,
                        format!("a [u8; {expected_length}]"),
                        arguments[1].1.span(),
                    ));
                    return Type::Err;
                };
                self.assert_type(array_type.element_type(), &Type::Integer(IntegerType::U8), arguments[1].1.span());
                if let Some(length) = array_type.length.as_u32()
                    && length as usize != expected_length
                {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        &arguments[1].0,
                        format!("a [u8; {expected_length}]"),
                        arguments[1].1.span(),
                    ));
                    return Type::Err;
                };

                // Check that the third input is not a mapping nor a tuple.
                if matches!(&arguments[2].0, Type::Mapping(_) | Type::Tuple(_) | Type::Unit) {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        &arguments[2].0,
                        "anything but a mapping, tuple, or unit",
                        arguments[2].1.span(),
                    ));
                }

                // If the variant is a digest variant, check that the third input is a byte array of the correct length.
                if matches!(variant, ECDSAVerifyVariant::Digest | ECDSAVerifyVariant::DigestEth) {
                    // Get the expected length of the third input.
                    let expected_length = ECDSASignature::PREHASH_SIZE_IN_BYTES;
                    // Check that the third input is a byte array of the expected length.
                    let Type::Array(array_type) = &arguments[2].0 else {
                        self.emit_err(TypeCheckerError::type_should_be2(
                            &arguments[2].0,
                            format!("a [u8; {expected_length}]"),
                            arguments[2].1.span(),
                        ));
                        return Type::Err;
                    };
                    self.assert_type(array_type.element_type(), &Type::Integer(IntegerType::U8), arguments[2].1.span());
                    if let Some(length) = array_type.length.as_u32()
                        && length as usize != expected_length
                    {
                        self.emit_err(TypeCheckerError::type_should_be2(
                            &arguments[2].0,
                            format!("a [u8; {expected_length}]"),
                            arguments[2].1.span(),
                        ));
                        return Type::Err;
                    }
                }

                // If the variant requires byte alignment, check that the third input is byte aligned.
                if variant.requires_byte_alignment() {
                    // Get the input type.
                    let input_type = &arguments[2].0;
                    // Get the size in bits.
                    let size_in_bits = match self.state.network {
                        NetworkName::TestnetV0 => input_type
                            .size_in_bits::<TestnetV0, _>(variant.is_raw(), |_| bail!("structs are not supported")),
                        NetworkName::MainnetV0 => input_type
                            .size_in_bits::<MainnetV0, _>(variant.is_raw(), |_| bail!("structs are not supported")),
                        NetworkName::CanaryV0 => input_type
                            .size_in_bits::<CanaryV0, _>(variant.is_raw(), |_| bail!("structs are not supported")),
                    };
                    if let Ok(size_in_bits) = size_in_bits {
                        // Check that the size in bits is a multiple of 8.
                        if size_in_bits % 8 != 0 {
                            self.emit_err(TypeCheckerError::type_should_be2(
                                input_type,
                                "a type with a size in bits that is a multiple of 8",
                                arguments[2].1.span(),
                            ));
                            return Type::Err;
                        }
                    };
                }

                Type::Boolean
            }
            Intrinsic::MappingGet => {
                if let Type::Mapping(MappingType { value, .. }) = &arguments[0].0 {
                    // Check that the operation is invoked in a `finalize` or `async` block.
                    self.check_access_allowed("Mapping::get", true, function_span);

                    *value.clone()
                } else {
                    self.assert_vector_or_mapping_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::VectorGet => {
                if let Type::Vector(VectorType { element_type }) = &arguments[0].0 {
                    // Check that the operation is invoked in a `finalize` or `async` block.
                    self.check_access_allowed("Vector::get", true, function_span);

                    Type::Optional(OptionalType { inner: Box::new(*element_type.clone()) })
                } else {
                    self.assert_vector_or_mapping_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::MappingSet => {
                if let Type::Mapping(_) = &arguments[0].0 {
                    // Check that the operation is invoked in a `finalize` or `async` block.
                    self.check_access_allowed("Mapping::set", true, function_span);

                    Type::Unit
                } else {
                    self.assert_vector_or_mapping_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::VectorSet => {
                if arguments[0].0.is_vector() {
                    // Check that the operation is invoked in a `finalize` or `async` block.
                    self.check_access_allowed("Vector::set", true, function_span);

                    Type::Unit
                } else {
                    self.assert_vector_or_mapping_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::MappingGetOrUse => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::get_or_use", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1.span());

                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                // Check that the second argument matches the key type of the mapping.
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1.span());
                // Check that the third argument matches the value type of the mapping.
                self.assert_type(&arguments[2].0, &mapping_type.value, arguments[2].1.span());

                mapping_type.value.deref().clone()
            }
            Intrinsic::MappingRemove => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::remove", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1.span());

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
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1.span());

                Type::Unit
            }
            Intrinsic::MappingContains => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::contains", true, function_span);
                // Check that the first argument is a mapping.
                self.assert_mapping_type(&arguments[0].0, arguments[0].1.span());

                let Type::Mapping(mapping_type) = &arguments[0].0 else {
                    // We will have already handled the error in the assertion.
                    return Type::Err;
                };

                // Check that the second argument matches the key type of the mapping.
                self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1.span());

                Type::Boolean
            }
            Intrinsic::OptionalUnwrap => {
                // Check that the first argument is an optional.
                self.assert_optional_type(&arguments[0].0, arguments[0].1.span());

                match &arguments[0].0 {
                    Type::Optional(opt) => opt.inner.deref().clone(),
                    _ => Type::Err,
                }
            }
            Intrinsic::OptionalUnwrapOr => {
                // Check that the first argument is an optional.
                self.assert_optional_type(&arguments[0].0, arguments[0].1.span());

                match &arguments[0].0 {
                    Type::Optional(OptionalType { inner }) => {
                        // Ensure that the wrapped type and the fallback type are the same
                        self.assert_type(&arguments[1].0, inner, arguments[1].1.span());
                        inner.deref().clone()
                    }
                    _ => Type::Err,
                }
            }
            Intrinsic::VectorPush => {
                self.check_access_allowed("Vector::push", true, function_span);

                // Check that the first argument is a vector
                match &arguments[0].0 {
                    Type::Vector(VectorType { element_type }) => {
                        // Ensure that the element type and the type of the value to push are the same
                        self.assert_type(&arguments[1].0, element_type, arguments[1].1.span());
                        Type::Unit
                    }
                    _ => {
                        self.assert_vector_type(&arguments[0].0, arguments[0].1.span());
                        Type::Err
                    }
                }
            }
            Intrinsic::VectorLen => {
                self.check_access_allowed("Vector::len", true, function_span);

                if arguments[0].0.is_vector() {
                    Type::Integer(IntegerType::U32)
                } else {
                    self.assert_vector_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::VectorPop => {
                self.check_access_allowed("Vector::pop", true, function_span);

                if let Type::Vector(VectorType { element_type }) = &arguments[0].0 {
                    Type::Optional(OptionalType { inner: Box::new(*element_type.clone()) })
                } else {
                    self.assert_vector_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::VectorSwapRemove => {
                self.check_access_allowed("Vector::swap_remove", true, function_span);

                if let Type::Vector(VectorType { element_type }) = &arguments[0].0 {
                    *element_type.clone()
                } else {
                    self.assert_vector_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::VectorClear => {
                if arguments[0].0.is_vector() {
                    Type::Unit
                } else {
                    self.assert_vector_type(&arguments[0].0, arguments[0].1.span());
                    Type::Err
                }
            }
            Intrinsic::GroupToXCoordinate | Intrinsic::GroupToYCoordinate => {
                // Check that the first argument is a group.
                self.assert_type(&arguments[0].0, &Type::Group, arguments[0].1.span());
                Type::Field
            }
            Intrinsic::ChaChaRand(type_) => type_.into(),
            Intrinsic::SignatureVerify => {
                // Check that the third argument is not a mapping nor a tuple. We have to do this
                // before the other checks below to appease the borrow checker
                assert_not_mapping_tuple_unit(&arguments[2].0, arguments[2].1.span());

                // Check that the first argument is a signature.
                self.assert_type(&arguments[0].0, &Type::Signature, arguments[0].1.span());
                // Check that the second argument is an address.
                self.assert_type(&arguments[1].0, &Type::Address, arguments[1].1.span());
                Type::Boolean
            }
            Intrinsic::FutureAwait => Type::Unit,
            Intrinsic::GroupGen => Type::Group,
            Intrinsic::ProgramChecksum => {
                // Get the argument type, expression, and span.
                let (type_, expression) = &arguments[0];
                let span = expression.span();
                // Check that the expression is a program ID.
                match expression {
                    Expression::Literal(Literal { variant: LiteralVariant::Address(s), .. })
                        if program_id_regex.is_match(s) => {}
                    _ => {
                        self.emit_err(TypeCheckerError::custom(
                            "`Program::checksum` must be called on a program ID, e.g. `foo.aleo`",
                            span,
                        ));
                    }
                }
                // Verify that the argument is a string.
                self.assert_type(type_, &Type::Address, span);
                // Return the type.
                Type::Array(ArrayType::new(
                    Type::Integer(IntegerType::U8),
                    Expression::Literal(Literal::integer(
                        IntegerType::U8,
                        "32".to_string(),
                        Default::default(),
                        Default::default(),
                    )),
                ))
            }
            Intrinsic::ProgramEdition => {
                // Get the argument type, expression, and span.
                let (type_, expression) = &arguments[0];
                let span = expression.span();
                // Check that the expression is a member access.
                match expression {
                    Expression::Literal(Literal { variant: LiteralVariant::Address(s), .. })
                        if program_id_regex.is_match(s) => {}
                    _ => {
                        self.emit_err(TypeCheckerError::custom(
                            "`Program::edition` must be called on a program ID, e.g. `foo.aleo`",
                            span,
                        ));
                    }
                }
                // Verify that the argument is a string.
                self.assert_type(type_, &Type::Address, span);
                // Return the type.
                Type::Integer(IntegerType::U16)
            }
            Intrinsic::ProgramOwner => {
                // Get the argument type, expression, and span.
                let (type_, expression) = &arguments[0];
                let span = expression.span();
                // Check that the expression is a member access.
                match expression {
                    Expression::Literal(Literal { variant: LiteralVariant::Address(s), .. })
                        if program_id_regex.is_match(s) => {}
                    _ => {
                        self.emit_err(TypeCheckerError::custom(
                            "`Program::program_owner` must be called on a program ID, e.g. `foo.aleo`",
                            span,
                        ));
                    }
                }
                // Verify that the argument is a string.
                self.assert_type(type_, &Type::Address, span);
                // Return the type.
                Type::Address
            }
            Intrinsic::Serialize(variant) => {
                // Determine the variant.
                let is_raw = match variant {
                    SerializeVariant::ToBits => false,
                    SerializeVariant::ToBitsRaw => true,
                };
                // Get the input type.
                let input_type = &arguments[0].0;

                // A helper function to check that a type is an allowed literal type.
                let is_allowed_literal_type = |type_: &Type| -> bool {
                    matches!(
                        type_,
                        Type::Boolean
                            | Type::Field
                            | Type::Group
                            | Type::Scalar
                            | Type::Signature
                            | Type::Address
                            | Type::Integer(_)
                            | Type::String
                            | Type::Numeric
                    )
                };

                // Check that the input type is an allowed literal or a (possibly multi-dimensional) array of literals.
                let is_allowed = match input_type {
                    Type::Array(array_type) => is_allowed_literal_type(array_type.base_element_type()),
                    type_ => is_allowed_literal_type(type_),
                };
                if !is_allowed {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        input_type,
                        "a literal type or an (multi-dimensional) array of literal types",
                        arguments[0].1.span(),
                    ));
                    return Type::Err;
                }

                // Get the size in bits.
                let size_in_bits = match self.state.network {
                    NetworkName::TestnetV0 => {
                        input_type.size_in_bits::<TestnetV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                    NetworkName::MainnetV0 => {
                        input_type.size_in_bits::<MainnetV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                    NetworkName::CanaryV0 => {
                        input_type.size_in_bits::<CanaryV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                };

                if let Ok(size_in_bits) = size_in_bits {
                    // Check that the size in bits is valid.
                    let size_in_bits = if size_in_bits > self.limits.max_array_elements {
                        self.emit_err(TypeCheckerError::custom(
                        format!("The input type to `Serialize::*` is too large. Found {size_in_bits} bits, but the maximum allowed is {} bits.", self.limits.max_array_elements),
                        arguments[0].1.span(),
                    ));
                        return Type::Err;
                    } else if size_in_bits == 0 {
                        self.emit_err(TypeCheckerError::custom(
                            "The input type to `Serialize::*` is empty.",
                            arguments[0].1.span(),
                        ));
                        return Type::Err;
                    } else {
                        u32::try_from(size_in_bits).expect("`max_array_elements` should fit in a u32")
                    };

                    // Return the array type.
                    return Type::Array(ArrayType::bit_array(size_in_bits));
                }

                // Could not resolve the size in bits at this time.
                Type::Err
            }
            Intrinsic::Deserialize(variant, type_) => {
                // Determine the variant.
                let is_raw = match variant {
                    DeserializeVariant::FromBits => false,
                    DeserializeVariant::FromBitsRaw => true,
                };
                // Get the input type.
                let input_type = &arguments[0].0;

                // Get the size in bits.
                let size_in_bits = match self.state.network {
                    NetworkName::TestnetV0 => {
                        type_.size_in_bits::<TestnetV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                    NetworkName::MainnetV0 => {
                        type_.size_in_bits::<MainnetV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                    NetworkName::CanaryV0 => {
                        type_.size_in_bits::<CanaryV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                };

                if let Ok(size_in_bits) = size_in_bits {
                    // Check that the size in bits is valid.
                    let size_in_bits = if size_in_bits > self.limits.max_array_elements {
                        self.emit_err(TypeCheckerError::custom(
                        format!("The output type of `Deserialize::*` is too large. Found {size_in_bits} bits, but the maximum allowed is {} bits.", self.limits.max_array_elements),
                        arguments[0].1.span(),
                    ));
                        return Type::Err;
                    } else if size_in_bits == 0 {
                        self.emit_err(TypeCheckerError::custom(
                            "The output type of `Deserialize::*` is empty.",
                            arguments[0].1.span(),
                        ));
                        return Type::Err;
                    } else {
                        u32::try_from(size_in_bits).expect("`max_array_elements` should fit in a u32")
                    };

                    // Check that the input type is an array of the correct size.
                    let expected_type = Type::Array(ArrayType::bit_array(size_in_bits));
                    if !input_type.eq_flat_relaxed(&expected_type) {
                        self.emit_err(TypeCheckerError::type_should_be2(
                            input_type,
                            format!("an array of {size_in_bits} bits"),
                            arguments[0].1.span(),
                        ));
                        return Type::Err;
                    }
                };

                type_.clone()
            }
            Intrinsic::CheatCodePrintMapping => {
                self.assert_mapping_type(&arguments[0].0, arguments[0].1.span());
                Type::Unit
            }
            Intrinsic::CheatCodeSetBlockHeight => {
                self.assert_type(&arguments[0].0, &Type::Integer(IntegerType::U32), arguments[0].1.span());
                Type::Unit
            }
            Intrinsic::CheatCodeSetBlockTimestamp => {
                self.assert_type(&arguments[0].0, &Type::Integer(IntegerType::I64), arguments[0].1.span());
                Type::Unit
            }
            Intrinsic::CheatCodeSetSigner => {
                // Assert that the argument is a string.
                self.assert_type(&arguments[0].0, &Type::String, arguments[0].1.span());
                // Validate that the argument is a valid private key.
                if let Expression::Literal(Literal { variant: LiteralVariant::String(s), .. }) = arguments[0].1 {
                    let s = s.replace("\"", "");
                    let is_err = match self.state.network {
                        NetworkName::TestnetV0 => PrivateKey::<TestnetV0>::from_str(&s).is_err(),
                        NetworkName::MainnetV0 => PrivateKey::<MainnetV0>::from_str(&s).is_err(),
                        NetworkName::CanaryV0 => PrivateKey::<CanaryV0>::from_str(&s).is_err(),
                    };
                    if is_err {
                        self.emit_err(TypeCheckerError::custom(
                            "`CheatCode::set_signer` must be called with a valid private key",
                            arguments[0].1.span(),
                        ));
                    }
                };
                Type::Unit
            }
            Intrinsic::SelfAddress => Type::Address,
            Intrinsic::SelfCaller => {
                // Check that the operation is not invoked in a `finalize` block.
                self.check_access_allowed("self.caller", false, function_span);
                Type::Address
            }
            Intrinsic::SelfChecksum => Type::Array(ArrayType::new(
                Type::Integer(IntegerType::U8),
                Expression::Literal(Literal::integer(
                    IntegerType::U8,
                    "32".to_string(),
                    Default::default(),
                    Default::default(),
                )),
            )),
            Intrinsic::SelfEdition => Type::Integer(IntegerType::U16),
            Intrinsic::SelfId => Type::Address,
            Intrinsic::SelfProgramOwner => {
                // Check that the operation is only invoked in a `finalize` block.
                self.check_access_allowed("program_owner", true, function_span);
                Type::Address
            }
            Intrinsic::SelfSigner => {
                // Check that operation is not invoked in a `finalize` block.
                self.check_access_allowed("self.signer", false, function_span);
                Type::Address
            }
            Intrinsic::BlockHeight => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("block.height", true, function_span);
                Type::Integer(IntegerType::U32)
            }
            Intrinsic::BlockTimestamp => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("block.timestamp", true, function_span);
                Type::Integer(IntegerType::I64)
            }
            Intrinsic::NetworkId => {
                // Check that the operation is not invoked outside a `finalize` block.
                self.check_access_allowed("network.id", true, function_span);
                Type::Integer(IntegerType::U16)
            }
        }
    }

    /// Emits an error if the composite member is a record type.
    pub fn assert_member_is_not_record(&mut self, span: Span, parent: Symbol, type_: &Type) {
        match type_ {
            Type::Composite(composite)
                if self
                    .lookup_composite(composite.path.expect_global_location())
                    .is_some_and(|composite| composite.is_record) =>
            {
                self.emit_err(TypeCheckerError::struct_or_record_cannot_contain_record(
                    parent,
                    composite.path.clone(),
                    span,
                ))
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
            // Check that named composite type has been defined.
            Type::Composite(composite) if self.lookup_composite(composite.path.expect_global_location()).is_none() => {
                self.emit_err(TypeCheckerError::undefined_type(composite.path.clone(), span));
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

                if let Some(length) = array_type.length.as_u32() {
                    if length > self.limits.max_array_elements as u32 {
                        self.emit_err(TypeCheckerError::array_too_large(length, self.limits.max_array_elements, span));
                    }
                } else if let Expression::Literal(_) = &*array_type.length {
                    // Literal, but not valid u32 (e.g. too big or invalid format)
                    self.emit_err(TypeCheckerError::array_too_large_for_u32(span));
                }
                // else: not a literal, so defer for later

                // Check that the array element type is valid.
                match array_type.element_type() {
                    // Array elements cannot be futures.
                    Type::Future(_) => self.emit_err(TypeCheckerError::array_element_cannot_be_future(span)),
                    // Array elements cannot be tuples.
                    Type::Tuple(_) => self.emit_err(TypeCheckerError::array_element_cannot_be_tuple(span)),
                    // Array elements cannot be records.
                    Type::Composite(composite_type) => {
                        // Look up the type.
                        if let Some(composite) = self.lookup_composite(composite_type.path.expect_global_location()) {
                            // Check that the type is not a record.
                            if composite.is_record {
                                self.emit_err(TypeCheckerError::array_element_cannot_be_record(span));
                            }
                        }
                    }
                    _ => {} // Do nothing.
                }
                self.assert_type_is_valid(array_type.element_type(), span);
            }

            Type::Optional(OptionalType { inner }) => {
                // Some types cannot be wrapped in an optional
                if self.disallowed_inside_optional(inner) {
                    self.emit_err(TypeCheckerError::optional_wrapping_unsupported(inner, span));
                }

                // Validate inner type normally
                self.assert_type_is_valid(inner, span);
            }

            Type::Address
            | Type::Boolean
            | Type::Composite(_)
            | Type::Field
            | Type::Future(_)
            | Type::Group
            | Type::Identifier(_)
            | Type::Integer(_)
            | Type::Scalar
            | Type::Signature
            | Type::Vector(_)
            | Type::Numeric
            | Type::Err => {} // Do nothing.
        }
    }

    /// Can type `ty` be used inside an optional?
    fn disallowed_inside_optional(&mut self, ty: &Type) -> bool {
        match ty {
            Type::Unit
            | Type::Err
            | Type::Future(_)
            | Type::Identifier(_)
            | Type::Mapping(_)
            | Type::Optional(_)
            | Type::String
            | Type::Signature
            | Type::Tuple(_)
            | Type::Vector(_) => true,

            Type::Composite(composite_type) => {
                if let Some(composite) = self.lookup_composite(composite_type.path.expect_global_location()) {
                    if composite.is_record {
                        return true;
                    }

                    // recursively check all fields
                    for field in &composite.members {
                        let field_ty = &field.type_;
                        // unwrap optional fields for the check
                        let ty_to_check = match field_ty {
                            Type::Optional(OptionalType { inner }) => inner,
                            _ => field_ty,
                        };
                        if self.disallowed_inside_optional(ty_to_check) {
                            return true;
                        }
                    }
                }
                false
            }

            Type::Array(array_type) => {
                let elem_type = match array_type.element_type() {
                    Type::Optional(OptionalType { inner }) => inner,
                    other => other,
                };
                self.disallowed_inside_optional(elem_type)
            }

            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Integer(_)
            | Type::Numeric
            | Type::Scalar => false,
        }
    }

    /// Ensures the given type is valid for use in storage.
    /// Emits an error if the type or any of its inner types are invalid.
    pub fn assert_storage_type_is_valid(&mut self, type_: &Type, span: Span) {
        if type_.is_empty() {
            self.emit_err(TypeCheckerError::invalid_storage_type("A zero sized type", span));
        }
        match type_ {
            // Prohibited top-level kinds
            Type::Unit => {
                self.emit_err(TypeCheckerError::invalid_storage_type("unit", span));
            }
            Type::String => {
                self.emit_err(TypeCheckerError::invalid_storage_type("string", span));
            }
            Type::Signature => {
                self.emit_err(TypeCheckerError::invalid_storage_type("signature", span));
            }
            Type::Future(_) => {
                self.emit_err(TypeCheckerError::invalid_storage_type("future", span));
            }
            Type::Optional(_) => {
                self.emit_err(TypeCheckerError::invalid_storage_type("optional", span));
            }
            Type::Mapping(_) => {
                self.emit_err(TypeCheckerError::invalid_storage_type("mapping", span));
            }
            Type::Tuple(_) => {
                self.emit_err(TypeCheckerError::invalid_storage_type("tuple", span));
            }

            // Composites
            Type::Composite(composite_type) => {
                if let Some(composite) = self.lookup_composite(composite_type.path.expect_global_location()) {
                    if composite.is_record {
                        self.emit_err(TypeCheckerError::invalid_storage_type("record", span));
                        return;
                    }

                    // Recursively check fields.
                    for field in &composite.members {
                        self.assert_storage_type_is_valid(&field.type_, span);
                    }
                } else {
                    self.emit_err(TypeCheckerError::invalid_storage_type("undefined composite", span));
                }
            }

            // Arrays
            Type::Array(array_type) => {
                if let Some(length) = array_type.length.as_u32()
                    && (length == 0 || length > self.limits.max_array_elements as u32)
                {
                    self.emit_err(TypeCheckerError::invalid_storage_type("array", span));
                }

                let element_ty = array_type.element_type();
                match element_ty {
                    Type::Future(_) => self.emit_err(TypeCheckerError::invalid_storage_type("future", span)),
                    Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_storage_type("tuple", span)),
                    Type::Optional(_) => self.emit_err(TypeCheckerError::invalid_storage_type("optional", span)),
                    _ => {}
                }

                self.assert_storage_type_is_valid(element_ty, span);
            }

            // Everything else (integers, bool, group, etc.)
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Identifier(_)
            | Type::Integer(_)
            | Type::Scalar
            | Type::Numeric
            | Type::Err
            | Type::Vector(_) => {} // valid
        }
    }

    /// Emits an error if the type is not a mapping.
    pub fn assert_mapping_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Mapping(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a mapping", span));
        }
    }

    /// Emits an error if the type is not an optional.
    pub fn assert_optional_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Optional(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "an optional", span));
        }
    }

    /// Emits an error if the type is not a vector
    pub fn assert_vector_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Vector(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a vector", span));
        }
    }

    /// Emits an error if the type is not a vector or a mapping.
    pub fn assert_vector_or_mapping_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Vector(_)) && !matches!(type_, Type::Mapping(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "a vector or a mapping", span));
        }
    }

    pub fn contains_optional_type(&mut self, ty: &Type) -> bool {
        let mut visited_paths = IndexSet::<Vec<Symbol>>::new();
        self.contains_optional_type_inner(ty, &mut visited_paths)
    }

    fn contains_optional_type_inner(&mut self, ty: &Type, visited_paths: &mut IndexSet<Vec<Symbol>>) -> bool {
        match ty {
            Type::Optional(_) => true,

            Type::Tuple(tuple) => tuple.elements.iter().any(|e| self.contains_optional_type_inner(e, visited_paths)),

            Type::Array(array) => self.contains_optional_type_inner(&array.element_type, visited_paths),

            Type::Composite(composite_type) => {
                let composite_location = composite_type.path.expect_global_location();

                // Prevent revisiting the same type
                // TODO: store locations here not just paths. Pending external structs.
                if !visited_paths.insert(composite_location.path.clone()) {
                    return false;
                }

                if let Some(comp) = self.lookup_composite(composite_location) {
                    comp.members
                        .iter()
                        .any(|Member { type_, .. }| self.contains_optional_type_inner(type_, visited_paths))
                } else {
                    false
                }
            }

            _ => false,
        }
    }

    pub fn assert_array_type(&self, type_: &Type, span: Span) {
        if type_ != &Type::Err && !matches!(type_, Type::Array(_)) {
            self.emit_err(TypeCheckerError::type_should_be2(type_, "an array", span));
        }
    }

    /// Helper function to check that the input and output of function are valid
    pub fn check_function_signature(&mut self, function: &Function, is_stub: bool) {
        let function_path = self
            .scope_state
            .module_name
            .iter()
            .cloned()
            .chain(std::iter::once(function.identifier.name))
            .collect::<Vec<Symbol>>();

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
                .get(&Location::new(self.scope_state.program_name.unwrap(), function_path))
                .map(|callers| {
                    callers
                        .iter()
                        .flat_map(|caller| {
                            let caller = Location::new(caller.program, caller.path.clone());
                            self.state.symbol_table.lookup_function(&caller)
                        })
                        .flat_map(|fn_symbol| fn_symbol.finalizer.clone())
                })
                .into_iter()
                .flatten();

            if let Some(first) = caller_finalizers.next() {
                inferred_inputs = first.inferred_inputs.clone();

                // If any input is a future that doesn't have the same member type for all
                // finalizers, set that member to `Type::Err`.
                for finalizer in caller_finalizers {
                    assert_eq!(inferred_inputs.len(), finalizer.inferred_inputs.len());
                    for (t1, t2) in inferred_inputs.iter_mut().zip(finalizer.inferred_inputs.iter()) {
                        self.merge_types(t1, t2);
                    }
                }
            } else {
                self.emit_warning(TypeCheckerWarning::async_function_is_never_called_by_transition_function(
                    function.identifier.name,
                    function.span(),
                ));
            }
        }

        // Ensure that, if the function has generic const paramaters, then it must be an `inline`.
        // Otherwise, emit an error.
        if self.scope_state.variant != Some(Variant::Inline) && !function.const_parameters.is_empty() {
            self.emit_err(TypeCheckerError::only_inline_can_have_const_generics(function.identifier.span()));
        }

        for const_param in &function.const_parameters {
            self.visit_type(const_param.type_());

            // Restrictions for const parameters
            if !matches!(
                const_param.type_(),
                Type::Boolean | Type::Integer(_) | Type::Address | Type::Scalar | Type::Group | Type::Field
            ) {
                self.emit_err(TypeCheckerError::bad_const_generic_type(const_param.type_(), const_param.span()));
            }

            // Set the type of the input in the symbol table.
            self.state.symbol_table.set_local_type(const_param.identifier.name, const_param.type_().clone());

            // Add the input to the type table.
            self.state.type_table.insert(const_param.identifier().id(), const_param.type_().clone());
        }

        // Ensure there aren't too many inputs
        if function.input.len() > self.limits.max_inputs && function.variant != Variant::Inline {
            self.state.handler.emit_err(TypeCheckerError::function_has_too_many_inputs(
                function.variant,
                function.identifier,
                self.limits.max_inputs,
                function.input.len(),
                function.identifier.span,
            ));
        }

        // The inputs should have access to the const parameters, so handle them after.
        for (i, input) in function.input.iter().enumerate() {
            self.visit_type(input.type_());

            // No need to check compatibility of these types; that's already been done
            let table_type = inferred_inputs.get(i).unwrap_or_else(|| input.type_());

            // Check that the type of input parameter is defined.
            self.assert_type_is_valid(table_type, input.span());

            // Check that the type of the input parameter is not a tuple.
            if matches!(table_type, Type::Tuple(_)) {
                self.emit_err(TypeCheckerError::function_cannot_take_tuple_as_input(input.span()))
            }

            // Check that the type of the input parameter does not contain an optional.
            if self.contains_optional_type(table_type)
                && matches!(function.variant, Variant::Transition | Variant::AsyncTransition | Variant::Function)
            {
                self.emit_err(TypeCheckerError::function_cannot_take_option_as_input(
                    input.identifier,
                    table_type,
                    input.span(),
                ))
            }

            // Make sure only transitions can take a record as an input.
            if let Type::Composite(composite) = table_type {
                // Throw error for undefined type.
                if !function.variant.is_transition() {
                    if let Some(elem) = self.lookup_composite(composite.path.expect_global_location()) {
                        if elem.is_record {
                            self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(input.span()))
                        }
                    } else {
                        self.emit_err(TypeCheckerError::undefined_type(composite.path.clone(), input.span()));
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
                // Set the type of the input in the symbol table.
                self.state.symbol_table.set_local_type(input.identifier.name, table_type.clone());

                // Add the input to the type table.
                self.state.type_table.insert(input.identifier().id(), table_type.clone());
            }
        }

        // Ensure there aren't too many outputs
        if function.output.len() > self.limits.max_outputs && function.variant != Variant::Inline {
            self.state.handler.emit_err(TypeCheckerError::function_has_too_many_outputs(
                function.variant,
                function.identifier,
                self.limits.max_outputs,
                function.output.len(),
                function.identifier.span,
            ));
        }

        // Type check the function's return type.
        // Note that checking that each of the component types are defined is sufficient to check that `output_type` is defined.
        function.output.iter().enumerate().for_each(|(index, function_output)| {
            self.visit_type(&function_output.type_);

            // If the function is not a transition function, then it cannot output a record.
            // Note that an external output must always be a record.
            if let Type::Composite(composite) = function_output.type_.clone()
                && let Some(val) = self.lookup_composite(composite.path.expect_global_location())
                && val.is_record
                && !function.variant.is_transition()
            {
                self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(function_output.span));
            }

            // Check that the output type is valid.
            self.assert_type_is_valid(&function_output.type_, function_output.span);

            // Check that the type of the output is not a tuple. This is necessary to forbid nested tuples.
            if matches!(&function_output.type_, Type::Tuple(_)) {
                self.emit_err(TypeCheckerError::nested_tuple_type(function_output.span))
            }

            // Check that the type of the input parameter does not contain an optional.
            if self.contains_optional_type(&function_output.type_)
                && matches!(function.variant, Variant::Transition | Variant::AsyncTransition | Variant::Function)
            {
                self.emit_err(TypeCheckerError::function_cannot_return_option_as_output(
                    &function_output.type_,
                    function_output.span(),
                ))
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

        self.visit_type(&function.output_type);
    }

    /// Merge inferred types into `lhs`.
    ///
    /// That is, if `lhs` and `rhs` aren't equal, set `lhs` to Type::Err;
    /// or, if they're both futures, set any member of `lhs` that isn't
    /// equal to the equivalent member of `rhs` to `Type::Err`.
    fn merge_types(&self, lhs: &mut Type, rhs: &Type) {
        let is_record = |loc: &Location| self.state.symbol_table.lookup_record(loc).is_some();
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
        } else if !lhs.eq_user(rhs, &is_record) {
            *lhs = Type::Err;
        }
    }

    /// Wrapper around lookup_struct and lookup_record that additionally records all structs and records that are
    /// used in the program.
    pub fn lookup_composite(&mut self, loc: &Location) -> Option<Composite> {
        let record_comp = self.state.symbol_table.lookup_record(loc);
        let comp = record_comp.or_else(|| self.state.symbol_table.lookup_struct(&loc.path));
        // Record the usage.
        if let Some(s) = comp {
            // If it's a struct or internal record, mark it used.
            if !s.is_record || Some(loc.program) == self.scope_state.program_name {
                self.used_composites.insert(loc.path.clone());
            }
        }
        comp.cloned()
    }

    /// Sets the type of a variable in the symbol table.
    pub fn set_local_type(&mut self, inferred_type: Option<Type>, name: &Identifier, type_: Type) {
        self.insert_symbol_conditional_scope(name.name);

        let is_future = match &type_ {
            Type::Future(..) => true,
            Type::Tuple(tuple_type) if matches!(tuple_type.elements().last(), Some(Type::Future(..))) => true,
            _ => false,
        };

        if is_future {
            // It can happen that the call location has not been set if there was an error
            // in the call that produced the Future.
            if let Some(call_location) = &self.scope_state.call_location {
                self.scope_state.futures.insert(name.name, call_location.clone());
            }
        }

        let ty = match (is_future, inferred_type) {
            (false, _) => type_,
            (true, Some(inferred)) => inferred,
            (true, None) => unreachable!("Type checking guarantees the inferred type is present"),
        };

        self.state.symbol_table.set_local_type(name.name, ty.clone());
    }

    // Validates whether an access operation is allowed in the current function or block context.
    // This prevents illegal use of certain operations depending on whether the code is inside
    // an async function, an async block, or a finalize block.
    pub fn check_access_allowed(&mut self, name: &str, finalize_op: bool, span: Span) {
        // Case 1: Operation is not a finalize op, and we're inside an `async` function.
        if self.scope_state.variant == Some(Variant::AsyncFunction) && !finalize_op {
            self.state.handler.emit_err(TypeCheckerError::invalid_operation_inside_finalize(name, span));
        }
        // Case 2: Operation is not a finalize op, and we're inside an `async` block.
        else if self.async_block_id.is_some() && !finalize_op {
            self.state.handler.emit_err(TypeCheckerError::invalid_operation_inside_async_block(name, span));
        }
        // Case 3: Operation *is* a finalize op, but we're *not* inside an async context.
        else if !matches!(self.scope_state.variant, Some(Variant::AsyncFunction) | Some(Variant::Script))
            && self.async_block_id.is_none()
            && finalize_op
        {
            self.state.handler.emit_err(TypeCheckerError::invalid_operation_outside_finalize(name, span));
        }
    }

    pub fn is_external_record(&self, ty: &Type) -> bool {
        if let Type::Composite(typ) = &ty {
            let this_program = self.scope_state.program_name.unwrap();
            let composite_location = typ.path.expect_global_location();
            composite_location.program != this_program
                && self.state.symbol_table.lookup_record(composite_location).is_some()
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
    pub fn check_numeric_literal(&self, input: &Literal, ty: &Type) -> bool {
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
                        return false;
                    } else {
                        let trimmed = s.trim_start_matches('-').trim_start_matches('0');
                        if !trimmed.is_empty()
                            && format!("{trimmed}group")
                                .parse::<snarkvm::prelude::Group<snarkvm::prelude::TestnetV0>>()
                                .is_err()
                        {
                            self.emit_err(TypeCheckerError::invalid_int_value(trimmed, "group", span));
                            return false;
                        }
                    }
                }
                Type::Field | Type::Scalar => {
                    if has_nondecimal_prefix(s) {
                        // This is not checked in the parser for unsuffixed numerals. So do that here.
                        self.emit_err(TypeCheckerError::hexbin_literal_nonintegers(span));
                        return false;
                    }
                }
                _ => {
                    // Other types aren't expected here
                }
            }
        }
        true
    }
}
