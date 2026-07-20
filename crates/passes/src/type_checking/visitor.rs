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
use leo_errors::LeoError;
use leo_span::{Span, Symbol, sym};

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use snarkvm::{
    console::algorithms::ECDSASignature,
    synthesizer::program::{
        CommitVariant,
        DeserializeVariant,
        ECDSAVerifyVariant,
        HashVariant,
        MAX_SNARK_VERIFY_CIRCUITS,
        MAX_SNARK_VERIFY_INSTANCES,
        SerializeVariant,
    },
};
use std::ops::Deref;

/// Where in the program text a snarkVM-bound access operation is permitted.
///
/// Each access intrinsic (e.g. mapping reads, mapping writes, `self.caller`) falls into one
/// of these scopes. `check_access_allowed` enforces the corresponding context rules.
#[derive(Copy, Clone, Debug)]
pub enum AccessScope {
    /// Read-only finalize op: permitted inside `final fn` / `final {}` blocks and inside
    /// `view fn` bodies. Examples: `Mapping::get`, `Vector::len`, `block.height`,
    /// `block.timestamp`, `Snark::verify`, `self.program_owner`.
    FinalizeRead,
    /// Mutating finalize op: permitted inside `final fn` / `final {}` blocks only. Examples:
    /// `Mapping::set`, `Vector::push`, storage writes.
    FinalizeWrite,
    /// Caller-context op: permitted inside regular `fn` and entry-point `fn` bodies only.
    /// Examples: `self.caller`, `self.signer`.
    OffchainCaller,
}

pub struct TypeCheckingVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The state of the current scope being traversed.
    pub scope_state: ScopeState,
    /// Mapping from async function stub name to the inferred input types.
    pub async_function_input_types: IndexMap<Location, Vec<TypeKind>>,
    /// Mapping from async function name to the names of async transition callers.
    pub async_function_callers: IndexMap<Location, IndexSet<Location>>,
    /// The set of used composites.
    pub used_composites: IndexSet<Location>,
    /// So we can check if we exceed limits on array size, number of mappings, or number of functions.
    pub limits: TypeCheckingInput,
    /// For detecting the error `crate::errors::type_checker::async_cannot_assign_outside_conditional`.
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

    /// Emit `inaccessible_item` for each parent interface in `parents` whose declaration is
    /// not visible to the current scope. Parents that don't resolve to a global location, or
    /// whose target isn't found in the interface table, are skipped. Those are reported by
    /// `check_interfaces`.
    pub fn check_parent_interface_accessibility(&mut self, parents: &[(Span, leo_ast::TypeKind)]) {
        let current_program = self.scope_state.unit_name.expect("must be inside a compilation unit");
        for (parent_span, parent_type) in parents {
            let leo_ast::TypeKind::Composite(leo_ast::CompositeType { path, .. }) = parent_type else { continue };
            let Some(loc) = path.try_global_location() else { continue };
            let Some(interface) = self.state.symbol_table.lookup_interface(current_program, loc) else { continue };
            if !self.scope_state.is_accessible(loc, interface.is_exported) {
                let name = interface.identifier.name;
                self.emit_err(crate::errors::type_checker::inaccessible_item("interface", name, *parent_span));
            }
        }
    }

    /// Emits a type checker error.
    pub fn emit_err(&self, err: impl Into<LeoError>) {
        self.state.handler.emit_err(err);
    }

    /// Returns `true` if `expr` is a path receiver; otherwise emits a diagnostic and returns `false`.
    /// Storage `Vector::*` and `Mapping::*` operations require a path receiver because downstream
    /// passes look up the backing mappings by name (storage_lowering for vectors, codegen for
    /// mappings).
    fn check_path_receiver(&self, module: &str, operation: &str, kind: &str, expr: &Expression) -> bool {
        if matches!(expr, Expression::Path(_)) {
            return true;
        }
        self.emit_err(crate::errors::type_checker::storage_op_requires_path_receiver(
            module,
            operation,
            kind,
            expr.span(),
        ));
        false
    }

    /// Emits an error if the two given types are not equal.
    pub fn check_eq_types(&self, t1: &Option<TypeKind>, t2: &Option<TypeKind>, span: Span) {
        match (t1, t2) {
            (Some(t1), Some(t2)) if !t1.types_equivalent(t2) => {
                self.emit_err(crate::errors::type_checker::type_should_be(t1, t2, span))
            }
            (Some(type_), None) | (None, Some(type_)) => {
                self.emit_err(crate::errors::type_checker::type_should_be("no type", type_, span))
            }
            _ => {}
        }
    }

    /// Use this method when you know the actual type.
    /// Emits an error to the handler if the `actual` type is not equal to the `expected` type.
    pub fn assert_and_return_type(&mut self, actual: TypeKind, expected: &Option<TypeKind>, span: Span) -> TypeKind {
        // If expected is `TypeKind::Err`, we don't want to actually report a redundant error.
        if expected.is_some() && !matches!(expected, Some(TypeKind::Err)) {
            self.check_eq_types(&Some(actual.clone()), expected, span);
        }
        actual
    }

    pub fn maybe_assert_type(&mut self, actual: &TypeKind, expected: &Option<TypeKind>, span: Span) {
        if let Some(expected) = expected {
            self.assert_type(actual, expected, span);
        }
    }

    pub fn assert_type(&mut self, actual: &TypeKind, expected: &TypeKind, span: Span) {
        if actual != &TypeKind::Err && !actual.can_coerce_to(expected) {
            // If `actual` is Err, we will have already reported an error.
            self.emit_err(crate::errors::type_checker::type_should_be2(actual, format!("type `{expected}`"), span));
        }
    }

    /// Unwraps an optional type to its inner type for use with operands.
    /// If the expected type is `T?`, returns `Some(T)`. Otherwise returns the type as-is.
    pub fn unwrap_optional_type(&self, expected: &Option<TypeKind>) -> Option<TypeKind> {
        match expected {
            Some(TypeKind::Optional(opt_type)) => Some(*opt_type.inner.clone()),
            other => other.clone(),
        }
    }

    pub fn assert_int_type(&self, type_: &TypeKind, span: Span) {
        if !matches!(type_, TypeKind::Err | TypeKind::Integer(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "an integer", span));
        }
    }

    pub fn assert_unsigned_type(&self, type_: &TypeKind, span: Span) {
        if !matches!(
            type_,
            TypeKind::Err
                | TypeKind::Integer(IntegerType::U8)
                | TypeKind::Integer(IntegerType::U16)
                | TypeKind::Integer(IntegerType::U32)
                | TypeKind::Integer(IntegerType::U64)
                | TypeKind::Integer(IntegerType::U128)
        ) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "an unsigned integer", span));
        }
    }

    pub fn assert_bool_int_type(&self, type_: &TypeKind, span: Span) {
        if !matches!(
            type_,
            TypeKind::Err
                | TypeKind::Boolean
                | TypeKind::Integer(IntegerType::U8)
                | TypeKind::Integer(IntegerType::U16)
                | TypeKind::Integer(IntegerType::U32)
                | TypeKind::Integer(IntegerType::U64)
                | TypeKind::Integer(IntegerType::U128)
                | TypeKind::Integer(IntegerType::I8)
                | TypeKind::Integer(IntegerType::I16)
                | TypeKind::Integer(IntegerType::I32)
                | TypeKind::Integer(IntegerType::I64)
                | TypeKind::Integer(IntegerType::I128)
        ) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "a bool or integer", span));
        }
    }

    pub fn assert_field_int_type(&self, type_: &TypeKind, span: Span) {
        if !matches!(
            type_,
            TypeKind::Err
                | TypeKind::Field
                | TypeKind::Integer(IntegerType::U8)
                | TypeKind::Integer(IntegerType::U16)
                | TypeKind::Integer(IntegerType::U32)
                | TypeKind::Integer(IntegerType::U64)
                | TypeKind::Integer(IntegerType::U128)
                | TypeKind::Integer(IntegerType::I8)
                | TypeKind::Integer(IntegerType::I16)
                | TypeKind::Integer(IntegerType::I32)
                | TypeKind::Integer(IntegerType::I64)
                | TypeKind::Integer(IntegerType::I128)
        ) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "a field or integer", span));
        }
    }

    pub fn assert_field_group_int_type(&self, type_: &TypeKind, span: Span) {
        if !matches!(type_, TypeKind::Err | TypeKind::Field | TypeKind::Group | TypeKind::Integer(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "a field, group, or integer", span));
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
                    TypeKind::Vector(_) => Some(Intrinsic::VectorGet),
                    TypeKind::Mapping(_) => Some(Intrinsic::MappingGet),
                    _ => None,
                }
            }
            None if intrinsic_expr.name == Symbol::intern("__unresolved_set") => {
                let ty = self.visit_expression(&intrinsic_expr.arguments[0], &None);
                self.assert_vector_or_mapping_type(&ty, intrinsic_expr.arguments[0].span());
                match ty {
                    TypeKind::Vector(_) => Some(Intrinsic::VectorSet),
                    TypeKind::Mapping(_) => Some(Intrinsic::MappingSet),
                    _ => None,
                }
            }
            None => {
                // Not a core library struct.
                self.emit_err(crate::errors::type_checker::invalid_intrinsic(
                    intrinsic_expr.name,
                    intrinsic_expr.span(),
                ));
                None
            }
            // Deserialize intrinsics require exactly one type parameter.
            Some(Intrinsic::Deserialize(variant, _)) if intrinsic_expr.type_parameters.len() != 1 => {
                let name = match variant {
                    DeserializeVariant::FromBits => "Deserialize::from_bits",
                    DeserializeVariant::FromBitsRaw => "Deserialize::from_bits_raw",
                };
                self.emit_err(crate::errors::type_checker::dynamic_intrinsic_missing_type_param(
                    name,
                    intrinsic_expr.span(),
                ));
                None
            }
            intrinsic @ Some(Intrinsic::Deserialize(_, _)) => intrinsic,
            // Dynamic dispatch intrinsics may have type parameters.
            intrinsic @ Some(
                Intrinsic::DynamicCall
                | Intrinsic::DynamicContains
                | Intrinsic::DynamicGet
                | Intrinsic::DynamicGetOrUse,
            ) => intrinsic,
            Some(intrinsic) => {
                // Check that the number of type parameters is 0.
                if !intrinsic_expr.type_parameters.is_empty() {
                    self.emit_err(crate::errors::type_checker::custom(
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
        expected: &Option<TypeKind>,
        function_span: Span,
    ) -> TypeKind {
        // Check that the number of arguments is correct.
        if arguments.len() != intrinsic.num_args() {
            self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(
                intrinsic.num_args(),
                arguments.len(),
                function_span,
            ));
            return TypeKind::Err;
        }

        // Type check and reconstructs the arguments for a given intrinsic call.
        //
        // Depending on the intrinsic, this handles:
        // - Optional operations (`unwrap`, `unwrap_or`) with proper type inference
        // - Container access (`Get`, `Set`) for vectors and mappings
        // - Vector-specific operations (`push`, `swap_remove`)
        // - Default handling for other intrinsics
        //
        // Returns a `Vec<(TypeKind, &Expression)>` pairing each argument with its inferred type, or `TypeKind::Err` if
        // type-checking fails. Argument counts are assumed to be already validated
        let arguments = match intrinsic {
            Intrinsic::OptionalUnwrap => {
                // Expect exactly one argument
                let [opt] = arguments else { panic!("number of arguments is already checked") };

                // If an expected type is provided, wrap it in Optional for type-checking
                let opt_ty = if let Some(expected) = expected {
                    self.visit_expression(
                        opt,
                        &Some(TypeKind::Optional(OptionalType { inner: Box::new(expected.clone()) })),
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
                        &Some(TypeKind::Optional(OptionalType { inner: Box::new(expected.clone()) })),
                    );
                    let fallback_ty = self.visit_expression(fallback, &Some(expected.clone()));
                    vec![(opt_ty, opt), (fallback_ty, fallback)]
                } else {
                    // Infer type from the optional argument
                    let opt_ty = self.visit_expression(opt, &None);
                    let fallback_ty = if let TypeKind::Optional(OptionalType { inner }) = &opt_ty {
                        self.visit_expression(fallback, &Some(*inner.clone()))
                    } else {
                        self.visit_expression(fallback, &None)
                    };
                    vec![(opt_ty, opt), (fallback_ty, fallback)]
                }
            }

            Intrinsic::MappingGet => {
                let [container, key] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                // Key type is obtained from the mapping type. Otherwise, don't do anything. Error
                // emission is deferred.
                let key_ty = if let TypeKind::Mapping(MappingType { key: ref key_ty, .. }) = container_ty {
                    self.visit_expression(key, &Some(*key_ty.clone()))
                } else {
                    self.visit_expression(key, &None)
                };

                vec![(container_ty, container), (key_ty, key)]
            }

            Intrinsic::MappingSet => {
                let [container, key, val] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                let (key_ty, val_ty) =
                    if let TypeKind::Mapping(MappingType { key: ref key_ty, value: ref value_ty, .. }) = container_ty {
                        (
                            self.visit_expression(key, &Some(*key_ty.clone())),
                            self.visit_expression(val, &Some(*value_ty.clone())),
                        )
                    } else {
                        (self.visit_expression(key, &None), self.visit_expression(val, &None))
                    };

                vec![(container_ty, container), (key_ty, key), (val_ty, val)]
            }

            Intrinsic::MappingGetOrUse => {
                let [container, key, default] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                let (key_ty, default_ty) =
                    if let TypeKind::Mapping(MappingType { key: ref key_ty, value: ref value_ty, .. }) = container_ty {
                        (
                            self.visit_expression(key, &Some(*key_ty.clone())),
                            self.visit_expression(default, &Some(*value_ty.clone())),
                        )
                    } else {
                        (self.visit_expression(key, &None), self.visit_expression(default, &None))
                    };

                vec![(container_ty, container), (key_ty, key), (default_ty, default)]
            }

            Intrinsic::MappingRemove => {
                let [container, key] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                let key_ty = if let TypeKind::Mapping(MappingType { key: ref key_ty, .. }) = container_ty {
                    self.visit_expression(key, &Some(*key_ty.clone()))
                } else {
                    self.visit_expression(key, &None)
                };

                vec![(container_ty, container), (key_ty, key)]
            }

            Intrinsic::MappingContains => {
                let [container, key] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                let key_ty = if let TypeKind::Mapping(MappingType { key: ref key_ty, .. }) = container_ty {
                    self.visit_expression(key, &Some(*key_ty.clone()))
                } else {
                    self.visit_expression(key, &None)
                };

                vec![(container_ty, container), (key_ty, key)]
            }

            Intrinsic::VectorGet => {
                let [container, index] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                // Indices default to `u32` when numeric.
                let index_ty = self.visit_expression_infer_default_u32(index);

                vec![(container_ty, container), (index_ty, index)]
            }

            Intrinsic::VectorSet => {
                let [container, index, val] = arguments else { panic!("number of arguments is already checked") };

                let container_ty = self.visit_expression(container, &None);

                // Indices default to `u32` when numeric.
                let index_ty = self.visit_expression_infer_default_u32(index);

                let val_ty = if let TypeKind::Vector(VectorType { ref element_type }) = container_ty {
                    self.visit_expression(val, &Some(*element_type.clone()))
                } else {
                    self.visit_expression(val, &None)
                };

                vec![(container_ty, container), (index_ty, index), (val_ty, val)]
            }

            Intrinsic::VectorPush => {
                let [vec, val] = arguments else { panic!("number of arguments is already checked") };

                // Check vector type
                let vec_ty = self.visit_expression(vec, &None);

                // Type-check value against vector element type
                let val_ty = if let TypeKind::Vector(VectorType { ref element_type }) = vec_ty {
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

        let assert_not_mapping_tuple_unit = |type_: &TypeKind, span: Span| {
            if matches!(type_, TypeKind::Mapping(_) | TypeKind::Tuple(_) | TypeKind::Unit) {
                self.emit_err(crate::errors::type_checker::type_should_be2(
                    type_,
                    "anything but a mapping, tuple, or unit",
                    span,
                ));
            }
        };

        // Make sure the input is no bigger than 64 bits.
        // Due to overhead in the bitwise representations of types in SnarkVM, 64 bit integers
        // input more than 64 bits to a hash function, as do all composites and arrays.
        let assert_pedersen_64_bit_input = |type_: &TypeKind, span: Span| {
            if !matches!(
                type_,
                TypeKind::Integer(IntegerType::U8)
                    | TypeKind::Integer(IntegerType::U16)
                    | TypeKind::Integer(IntegerType::U32)
                    | TypeKind::Integer(IntegerType::I8)
                    | TypeKind::Integer(IntegerType::I16)
                    | TypeKind::Integer(IntegerType::I32)
                    | TypeKind::Boolean
                    | TypeKind::Err
            ) {
                self.emit_err(crate::errors::type_checker::type_should_be2(
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
        let assert_pedersen_128_bit_input = |type_: &TypeKind, span: Span| {
            if !matches!(
                type_,
                TypeKind::Integer(IntegerType::U8)
                    | TypeKind::Integer(IntegerType::U16)
                    | TypeKind::Integer(IntegerType::U32)
                    | TypeKind::Integer(IntegerType::U64)
                    | TypeKind::Integer(IntegerType::I8)
                    | TypeKind::Integer(IntegerType::I16)
                    | TypeKind::Integer(IntegerType::I32)
                    | TypeKind::Integer(IntegerType::I64)
                    | TypeKind::Boolean
                    | TypeKind::Err
            ) {
                self.emit_err(crate::errors::type_checker::type_should_be2(
                    type_,
                    "an integer of less than 128 bits or a bool",
                    span,
                ));
            }
        };

        // Define a regex to match valid program IDs.
        let program_id_regex = regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{0,30}\.aleo$").unwrap();

        fn struct_not_supported<T, U>(_: &T) -> anyhow::Result<U> {
            bail!("structs are not supported")
        }

        let current_program = self.scope_state.unit_name.unwrap();
        // Returns true if the given expression is a local path in the current program.
        let is_local_path = |expr: &Expression| -> bool {
            matches!(expr, Expression::Path(path)
        if path.program().is_none() || path.program() == Some(current_program))
        };

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
                self.assert_type(&arguments[1].0, &TypeKind::Scalar, arguments[1].1.span());
                type_.into()
            }
            Intrinsic::Hash(variant, type_) => {
                // If the hash variant must be byte aligned, check that the number bits of the input is a multiple of 8.
                if variant.requires_byte_alignment() {
                    // Get the input type.
                    let input_type = &arguments[0].0;
                    // Get the size in bits.
                    let size_in_bits = match self.state.network {
                        NetworkName::TestnetV0 => input_type.size_in_bits::<TestnetV0, _, _>(
                            variant.is_raw(),
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                        NetworkName::MainnetV0 => input_type.size_in_bits::<MainnetV0, _, _>(
                            variant.is_raw(),
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                        NetworkName::CanaryV0 => input_type.size_in_bits::<CanaryV0, _, _>(
                            variant.is_raw(),
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                    };
                    if let Ok(size_in_bits) = size_in_bits {
                        // Check that the size in bits is a multiple of 8.
                        if size_in_bits % 8 != 0 {
                            self.emit_err(crate::errors::type_checker::type_should_be2(
                                input_type,
                                "a type with a size in bits that is a multiple of 8",
                                arguments[0].1.span(),
                            ));
                            return TypeKind::Err;
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
                let TypeKind::Array(array_type) = &arguments[0].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        format!("a [u8; {signature_size}]"),
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                };
                self.assert_type(array_type.element_type(), &TypeKind::Integer(IntegerType::U8), arguments[0].1.span());
                if let Some(length) = array_type.length.as_u32()
                    && length as usize != signature_size
                {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        format!("a [u8; {signature_size}]"),
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
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
                let TypeKind::Array(array_type) = &arguments[1].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[1].0,
                        format!("a [u8; {expected_length}]"),
                        arguments[1].1.span(),
                    ));
                    return TypeKind::Err;
                };
                self.assert_type(array_type.element_type(), &TypeKind::Integer(IntegerType::U8), arguments[1].1.span());
                if let Some(length) = array_type.length.as_u32()
                    && length as usize != expected_length
                {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[1].0,
                        format!("a [u8; {expected_length}]"),
                        arguments[1].1.span(),
                    ));
                    return TypeKind::Err;
                };

                // Check that the third input is not a mapping nor a tuple.
                if matches!(&arguments[2].0, TypeKind::Mapping(_) | TypeKind::Tuple(_) | TypeKind::Unit) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
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
                    let TypeKind::Array(array_type) = &arguments[2].0 else {
                        self.emit_err(crate::errors::type_checker::type_should_be2(
                            &arguments[2].0,
                            format!("a [u8; {expected_length}]"),
                            arguments[2].1.span(),
                        ));
                        return TypeKind::Err;
                    };
                    self.assert_type(
                        array_type.element_type(),
                        &TypeKind::Integer(IntegerType::U8),
                        arguments[2].1.span(),
                    );
                    if let Some(length) = array_type.length.as_u32()
                        && length as usize != expected_length
                    {
                        self.emit_err(crate::errors::type_checker::type_should_be2(
                            &arguments[2].0,
                            format!("a [u8; {expected_length}]"),
                            arguments[2].1.span(),
                        ));
                        return TypeKind::Err;
                    }
                }

                // If the variant requires byte alignment, check that the third input is byte aligned.
                if variant.requires_byte_alignment() {
                    // Get the input type.
                    let input_type = &arguments[2].0;
                    // Get the size in bits.
                    let size_in_bits = match self.state.network {
                        NetworkName::TestnetV0 => input_type.size_in_bits::<TestnetV0, _, _>(
                            variant.is_raw(),
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                        NetworkName::MainnetV0 => input_type.size_in_bits::<MainnetV0, _, _>(
                            variant.is_raw(),
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                        NetworkName::CanaryV0 => input_type.size_in_bits::<CanaryV0, _, _>(
                            variant.is_raw(),
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                    };
                    if let Ok(size_in_bits) = size_in_bits {
                        // Check that the size in bits is a multiple of 8.
                        if size_in_bits % 8 != 0 {
                            self.emit_err(crate::errors::type_checker::type_should_be2(
                                input_type,
                                "a type with a size in bits that is a multiple of 8",
                                arguments[2].1.span(),
                            ));
                            return TypeKind::Err;
                        }
                    };
                }

                TypeKind::Boolean
            }
            Intrinsic::SnarkVerify => {
                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                // `snark.verify` is a pure instruction (no state mutation), so views may call it.
                self.check_access_allowed("Snark::verify", AccessScope::FinalizeRead, function_span);

                // arg0: [u8; N] — verifying key (1D byte array)
                let TypeKind::Array(vk_arr) = &arguments[0].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        "a [u8; N]",
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                };
                if matches!(vk_arr.element_type(), TypeKind::Array(..)) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        "a [u8; N]",
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                }
                self.assert_type(vk_arr.element_type(), &TypeKind::Integer(IntegerType::U8), arguments[0].1.span());

                // arg1: u8 — varuna version
                self.assert_type(&arguments[1].0, &TypeKind::Integer(IntegerType::U8), arguments[1].1.span());

                // arg2: [field; N] — public inputs (1D field array)
                let TypeKind::Array(inputs_arr) = &arguments[2].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[2].0,
                        "a [field; N]",
                        arguments[2].1.span(),
                    ));
                    return TypeKind::Err;
                };
                if matches!(inputs_arr.element_type(), TypeKind::Array(..)) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[2].0,
                        "a [field; N]",
                        arguments[2].1.span(),
                    ));
                    return TypeKind::Err;
                }
                self.assert_type(inputs_arr.element_type(), &TypeKind::Field, arguments[2].1.span());

                // arg3: [u8; N] — proof (1D byte array)
                let TypeKind::Array(proof_arr) = &arguments[3].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[3].0,
                        "a [u8; N]",
                        arguments[3].1.span(),
                    ));
                    return TypeKind::Err;
                };
                if matches!(proof_arr.element_type(), TypeKind::Array(..)) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[3].0,
                        "a [u8; N]",
                        arguments[3].1.span(),
                    ));
                    return TypeKind::Err;
                }
                self.assert_type(proof_arr.element_type(), &TypeKind::Integer(IntegerType::U8), arguments[3].1.span());

                TypeKind::Boolean
            }
            Intrinsic::SnarkVerifyBatch => {
                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                // `snark.verify` is a pure instruction (no state mutation), so views may call it.
                self.check_access_allowed("Snark::verify_batch", AccessScope::FinalizeRead, function_span);

                // arg0: [[u8; N]; M] — verifying keys (2D byte array)
                let TypeKind::Array(vks_outer) = &arguments[0].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        "a [[u8; N]; M]",
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                };
                let TypeKind::Array(vks_inner) = vks_outer.element_type() else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        "a [[u8; N]; M]",
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                };
                // Reject 3D arrays — the inner dimension must be strictly 1D bytes.
                if matches!(vks_inner.element_type(), TypeKind::Array(..)) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[0].0,
                        "a [[u8; N]; M]",
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                }
                self.assert_type(vks_inner.element_type(), &TypeKind::Integer(IntegerType::U8), arguments[0].1.span());

                // arg1: u8 — varuna version
                self.assert_type(&arguments[1].0, &TypeKind::Integer(IntegerType::U8), arguments[1].1.span());

                // arg2: [[[field; N]; M]; K] — public inputs (3D field array)
                let TypeKind::Array(inputs_d1) = &arguments[2].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[2].0,
                        "a [[[field; N]; M]; K]",
                        arguments[2].1.span(),
                    ));
                    return TypeKind::Err;
                };
                let TypeKind::Array(inputs_d2) = inputs_d1.element_type() else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[2].0,
                        "a [[[field; N]; M]; K]",
                        arguments[2].1.span(),
                    ));
                    return TypeKind::Err;
                };
                let TypeKind::Array(inputs_d3) = inputs_d2.element_type() else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[2].0,
                        "a [[[field; N]; M]; K]",
                        arguments[2].1.span(),
                    ));
                    return TypeKind::Err;
                };
                // Reject 4D arrays — the innermost dimension must be strictly 1D fields.
                if matches!(inputs_d3.element_type(), TypeKind::Array(..)) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[2].0,
                        "a [[[field; N]; M]; K]",
                        arguments[2].1.span(),
                    ));
                    return TypeKind::Err;
                }
                self.assert_type(inputs_d3.element_type(), &TypeKind::Field, arguments[2].1.span());

                // Validate dimension match: num_vks (M) must equal num_circuits (K).
                // These limits match `MAX_SNARK_VERIFY_CIRCUITS` and `MAX_SNARK_VERIFY_INSTANCES` from snarkVM.
                if let (Some(num_vks), Some(num_circuits)) = (vks_outer.length.as_u32(), inputs_d1.length.as_u32()) {
                    if num_vks != num_circuits {
                        self.emit_err(crate::errors::type_checker::custom(
                            format!(
                                "The number of verifying keys ({num_vks}) must match the number of circuits in the inputs ({num_circuits})."
                            ),
                            arguments[0].1.span()));
                    }

                    if num_circuits > MAX_SNARK_VERIFY_CIRCUITS {
                        self.emit_err(crate::errors::type_checker::array_too_large(
                            num_circuits,
                            MAX_SNARK_VERIFY_CIRCUITS,
                            arguments[2].1.span(),
                        ));
                    }

                    if let Some(instances_per_circuit) = inputs_d2.length.as_u32() {
                        let total_instances = num_circuits.saturating_mul(instances_per_circuit);
                        if total_instances > MAX_SNARK_VERIFY_INSTANCES {
                            self.emit_err(crate::errors::type_checker::array_too_large(
                                total_instances,
                                MAX_SNARK_VERIFY_INSTANCES,
                                arguments[2].1.span(),
                            ));
                        }
                    }
                } else {
                    // Array lengths in Leo are always integer literals, so this branch is unreachable in practice.
                    // It is kept as a defensive guard against future changes to the type system.
                    self.emit_err(crate::errors::type_checker::custom(
                        "The outer dimensions of the `Snark::verify_batch` arguments must be statically known integer literals.",
                        arguments[0].1.span()));
                }

                // arg3: [u8; N] — proof (1D byte array)
                let TypeKind::Array(proof_arr) = &arguments[3].0 else {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[3].0,
                        "a [u8; N]",
                        arguments[3].1.span(),
                    ));
                    return TypeKind::Err;
                };
                // Reject nested arrays — proof must be strictly 1D.
                if matches!(proof_arr.element_type(), TypeKind::Array(..)) {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        &arguments[3].0,
                        "a [u8; N]",
                        arguments[3].1.span(),
                    ));
                    return TypeKind::Err;
                }
                self.assert_type(proof_arr.element_type(), &TypeKind::Integer(IntegerType::U8), arguments[3].1.span());

                TypeKind::Boolean
            }
            Intrinsic::MappingGet => {
                let (map_ty, map_expr) = &arguments[0];

                let TypeKind::Mapping(MappingType { value, .. }) = map_ty else {
                    self.assert_mapping_type(map_ty, map_expr.span());
                    return TypeKind::Err;
                };

                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                self.check_access_allowed("Mapping::get", AccessScope::FinalizeRead, function_span);

                if !self.check_path_receiver("Mapping", "get", "mapping", map_expr) {
                    return TypeKind::Err;
                }

                *value.clone()
            }
            Intrinsic::MappingSet => {
                let (map_ty, map_expr) = &arguments[0];

                let TypeKind::Mapping(_) = map_ty else {
                    self.assert_mapping_type(map_ty, map_expr.span());
                    return TypeKind::Err;
                };

                // Check that the operation is invoked in a `finalize` or `async` block.
                self.check_access_allowed("Mapping::set", AccessScope::FinalizeWrite, function_span);

                if !self.check_path_receiver("Mapping", "set", "mapping", map_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external mappings).
                if !is_local_path(map_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "set",
                        "mapping",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                TypeKind::Unit
            }
            Intrinsic::MappingGetOrUse => {
                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                self.check_access_allowed("Mapping::get_or_use", AccessScope::FinalizeRead, function_span);

                let (map_ty, map_expr) = &arguments[0];

                // Check that the first argument is a mapping.
                self.assert_mapping_type(map_ty, map_expr.span());

                let TypeKind::Mapping(MappingType { value, .. }) = map_ty else {
                    // We already handled the error in the assertion.
                    return TypeKind::Err;
                };

                if !self.check_path_receiver("Mapping", "get_or_use", "mapping", map_expr) {
                    return TypeKind::Err;
                }

                value.deref().clone()
            }
            Intrinsic::MappingRemove => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::remove", AccessScope::FinalizeWrite, function_span);

                let (map_ty, map_expr) = &arguments[0];

                // Check that the first argument is a mapping.
                self.assert_mapping_type(map_ty, map_expr.span());

                let TypeKind::Mapping(_) = map_ty else {
                    // We already handled the error in the assertion.
                    return TypeKind::Err;
                };

                if !self.check_path_receiver("Mapping", "remove", "mapping", map_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external mappings).
                if !is_local_path(map_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "remove",
                        "mapping",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                TypeKind::Unit
            }
            Intrinsic::MappingContains => {
                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                self.check_access_allowed("Mapping::contains", AccessScope::FinalizeRead, function_span);

                let (map_ty, map_expr) = &arguments[0];

                // Check that the first argument is a mapping.
                self.assert_mapping_type(map_ty, map_expr.span());

                let TypeKind::Mapping(_) = map_ty else {
                    // We already handled the error in the assertion.
                    return TypeKind::Err;
                };

                if !self.check_path_receiver("Mapping", "contains", "mapping", map_expr) {
                    return TypeKind::Err;
                }

                TypeKind::Boolean
            }
            Intrinsic::OptionalUnwrap => {
                // Check that the first argument is an optional.
                self.assert_optional_type(&arguments[0].0, arguments[0].1.span());

                match &arguments[0].0 {
                    TypeKind::Optional(opt) => opt.inner.deref().clone(),
                    _ => TypeKind::Err,
                }
            }
            Intrinsic::OptionalUnwrapOr => {
                // Check that the first argument is an optional.
                self.assert_optional_type(&arguments[0].0, arguments[0].1.span());

                match &arguments[0].0 {
                    TypeKind::Optional(OptionalType { inner }) => {
                        // Ensure that the wrapped type and the fallback type are the same
                        self.assert_type(&arguments[1].0, inner, arguments[1].1.span());
                        inner.deref().clone()
                    }
                    _ => TypeKind::Err,
                }
            }
            Intrinsic::VectorGet => {
                let (vec_ty, vec_expr) = &arguments[0];

                let TypeKind::Vector(VectorType { element_type }) = vec_ty else {
                    self.assert_vector_or_mapping_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                };

                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                self.check_access_allowed("Vector::get", AccessScope::FinalizeRead, function_span);

                if !self.check_path_receiver("Vector", "get", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                TypeKind::Optional(OptionalType { inner: Box::new(*element_type.clone()) })
            }
            Intrinsic::VectorSet => {
                let (vec_ty, vec_expr) = &arguments[0];

                if !vec_ty.is_vector() {
                    self.assert_vector_or_mapping_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                }

                // Check that the operation is invoked in a `finalize` or `async` block.
                self.check_access_allowed("Vector::set", AccessScope::FinalizeWrite, function_span);

                if !self.check_path_receiver("Vector", "set", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external vectors).
                if !is_local_path(vec_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "set",
                        "vector",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                TypeKind::Unit
            }
            Intrinsic::VectorPush => {
                let (vec_ty, vec_expr) = &arguments[0];
                let (val_ty, val_expr) = &arguments[1];

                // First argument must be a vector.
                let TypeKind::Vector(VectorType { element_type }) = vec_ty else {
                    self.assert_vector_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                };

                // Value being pushed must match element type.
                self.assert_type(val_ty, element_type, val_expr.span());

                // Check that the operation is invoked in a `finalize` or `async` block.
                self.check_access_allowed("Vector::push", AccessScope::FinalizeWrite, function_span);

                if !self.check_path_receiver("Vector", "push", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external vectors).
                if !is_local_path(vec_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "push",
                        "vector",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                TypeKind::Unit
            }
            Intrinsic::VectorLen => {
                let (vec_ty, vec_expr) = &arguments[0];

                // Check that the operation is invoked in a `finalize` / `async` block or a view.
                self.check_access_allowed("Vector::len", AccessScope::FinalizeRead, function_span);

                if !vec_ty.is_vector() {
                    self.assert_vector_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                }

                if !self.check_path_receiver("Vector", "len", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                TypeKind::Integer(IntegerType::U32)
            }
            Intrinsic::VectorPop => {
                let (vec_ty, vec_expr) = &arguments[0];

                // Check that the operation is invoked in a `finalize` or `async` block.
                self.check_access_allowed("Vector::pop", AccessScope::FinalizeWrite, function_span);

                let TypeKind::Vector(VectorType { element_type }) = vec_ty else {
                    self.assert_vector_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                };

                if !self.check_path_receiver("Vector", "pop", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external vectors).
                if !is_local_path(vec_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "pop",
                        "vector",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                TypeKind::Optional(OptionalType { inner: Box::new(*element_type.clone()) })
            }
            Intrinsic::VectorSwapRemove => {
                let (vec_ty, vec_expr) = &arguments[0];

                // Check that the operation is invoked in a `finalize` or `async` block.
                self.check_access_allowed("Vector::swap_remove", AccessScope::FinalizeWrite, function_span);

                let TypeKind::Vector(VectorType { element_type }) = vec_ty else {
                    self.assert_vector_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                };

                if !self.check_path_receiver("Vector", "swap_remove", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external vectors).
                if !is_local_path(vec_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "swap_remove",
                        "vector",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                *element_type.clone()
            }
            Intrinsic::VectorClear => {
                let (vec_ty, vec_expr) = &arguments[0];

                if !vec_ty.is_vector() {
                    self.assert_vector_type(vec_ty, vec_expr.span());
                    return TypeKind::Err;
                }

                self.check_access_allowed("Vector::clear", AccessScope::FinalizeWrite, function_span);

                if !self.check_path_receiver("Vector", "clear", "storage vector", vec_expr) {
                    return TypeKind::Err;
                }

                // Argument 0 must be a local path (cannot modify external vectors).
                if !is_local_path(vec_expr) {
                    self.state.handler.emit_err(crate::errors::type_checker::cannot_modify_external_container(
                        "clear",
                        "vector",
                        function_span,
                    ));
                    return TypeKind::Err;
                }

                TypeKind::Unit
            }
            Intrinsic::GroupToXCoordinate | Intrinsic::GroupToYCoordinate => {
                // Check that the first argument is a group.
                self.assert_type(&arguments[0].0, &TypeKind::Group, arguments[0].1.span());
                TypeKind::Field
            }
            Intrinsic::ChaChaRand(type_) => type_.into(),
            Intrinsic::SignatureVerify => {
                // Check that the third argument is not a mapping nor a tuple. We have to do this
                // before the other checks below to appease the borrow checker
                assert_not_mapping_tuple_unit(&arguments[2].0, arguments[2].1.span());

                // Check that the first argument is a signature.
                self.assert_type(&arguments[0].0, &TypeKind::Signature, arguments[0].1.span());
                // Check that the second argument is an address.
                self.assert_type(&arguments[1].0, &TypeKind::Address, arguments[1].1.span());
                TypeKind::Boolean
            }
            Intrinsic::FinalRun => TypeKind::Unit,
            Intrinsic::GroupGen => TypeKind::Group,
            Intrinsic::AleoGenerator => TypeKind::Group,
            Intrinsic::AleoGeneratorPowers => TypeKind::Array(ArrayType::new(
                TypeKind::Group,
                Expression::Literal(Literal::integer(
                    IntegerType::U32,
                    "251".to_string(),
                    Default::default(),
                    Default::default(),
                )),
            )),
            Intrinsic::ProgramChecksum => {
                // Get the argument type, expression, and span.
                let (type_, expression) = &arguments[0];
                let span = expression.span();
                // Check that the expression is a program ID.
                match expression {
                    Expression::Literal(Literal { variant: LiteralVariant::Address(s), .. })
                        if program_id_regex.is_match(s) => {}
                    _ => {
                        self.emit_err(crate::errors::type_checker::custom(
                            "`Program::checksum` must be called on a program ID, e.g. `foo.aleo`",
                            span,
                        ));
                    }
                }
                self.assert_type(type_, &TypeKind::Address, span);
                TypeKind::Array(ArrayType::new(
                    TypeKind::Integer(IntegerType::U8),
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
                        self.emit_err(crate::errors::type_checker::custom(
                            "`Program::edition` must be called on a program ID, e.g. `foo.aleo`",
                            span,
                        ));
                    }
                }
                self.assert_type(type_, &TypeKind::Address, span);
                TypeKind::Integer(IntegerType::U16)
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
                        self.emit_err(crate::errors::type_checker::custom(
                            "`Program::program_owner` must be called on a program ID, e.g. `foo.aleo`",
                            span,
                        ));
                    }
                }
                self.assert_type(type_, &TypeKind::Address, span);
                TypeKind::Address
            }
            Intrinsic::FunctionChecksum => {
                // The first argument is the program ID, the second the component name.
                let (program_type, program_expr) = &arguments[0];
                let program_span = program_expr.span();
                // Check that the first argument is a program ID.
                let program_id = match program_expr {
                    Expression::Literal(Literal { variant: LiteralVariant::Address(s), .. })
                        if program_id_regex.is_match(s) =>
                    {
                        Some(s.clone())
                    }
                    _ => {
                        self.emit_err(crate::errors::type_checker::custom(
                            "`Program::function_checksum` must be called on a program ID, e.g. `foo.aleo`",
                            program_span,
                        ));
                        None
                    }
                };
                self.assert_type(program_type, &TypeKind::Address, program_span);
                // Check that the second argument is an identifier literal naming the component, e.g. `'foo'`.
                let (component_type, component_expr) = &arguments[1];
                let component_span = component_expr.span();
                let component = match component_expr {
                    Expression::Literal(Literal { variant: LiteralVariant::Identifier(name), .. }) => {
                        Some(name.clone())
                    }
                    _ => {
                        self.emit_err(crate::errors::type_checker::custom(
                            "the function name must be an identifier literal, e.g. `'foo'`",
                            component_span,
                        ));
                        None
                    }
                };
                self.assert_type(component_type, &TypeKind::Identifier, component_span);
                // Checksums exist only for externally-callable components — entry and view functions.
                // Closures and `final fn`s are inlining artifacts with no stable identity, so reject
                // anything that does not resolve to an entry or view function of the named (imported) program.
                if let (Some(program_id), Some(component)) = (program_id, component) {
                    let location = Location::new(Symbol::intern(&program_id), vec![Symbol::intern(&component)]);
                    let current_unit = self.scope_state.unit_name.expect("type checking runs within a program");
                    let is_entry_or_view = self
                        .state
                        .symbol_table
                        .lookup_function(current_unit, &location)
                        .is_some_and(|symbol| symbol.function.variant.is_externally_callable());
                    if !is_entry_or_view {
                        self.emit_err(crate::errors::type_checker::custom(
                            format!("`{component}` must be an entry function or a view function of `{program_id}`"),
                            component_span,
                        ));
                    }
                }
                // Return the type.
                TypeKind::Array(ArrayType::new(
                    TypeKind::Integer(IntegerType::U8),
                    Expression::Literal(Literal::integer(
                        IntegerType::U8,
                        "32".to_string(),
                        Default::default(),
                        Default::default(),
                    )),
                ))
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
                let is_allowed_literal_type = |type_: &TypeKind| -> bool {
                    matches!(
                        type_,
                        TypeKind::Boolean
                            | TypeKind::Field
                            | TypeKind::Group
                            | TypeKind::Scalar
                            | TypeKind::Signature
                            | TypeKind::Address
                            | TypeKind::Integer(_)
                            | TypeKind::String
                            | TypeKind::Numeric
                    )
                };

                // Check that the input type is an allowed literal or a (possibly multi-dimensional) array of literals.
                let is_allowed = match input_type {
                    TypeKind::Array(array_type) => is_allowed_literal_type(array_type.base_element_type()),
                    type_ => is_allowed_literal_type(type_),
                };
                if !is_allowed {
                    self.emit_err(crate::errors::type_checker::type_should_be2(
                        input_type,
                        "a literal type or an (multi-dimensional) array of literal types",
                        arguments[0].1.span(),
                    ));
                    return TypeKind::Err;
                }

                // Get the size in bits.
                let size_in_bits = match self.state.network {
                    NetworkName::TestnetV0 => {
                        input_type.size_in_bits::<TestnetV0, _, _>(is_raw, &struct_not_supported, &struct_not_supported)
                    }
                    NetworkName::MainnetV0 => {
                        input_type.size_in_bits::<MainnetV0, _, _>(is_raw, &struct_not_supported, &struct_not_supported)
                    }
                    NetworkName::CanaryV0 => {
                        input_type.size_in_bits::<CanaryV0, _, _>(is_raw, &struct_not_supported, &struct_not_supported)
                    }
                };

                if let Ok(size_in_bits) = size_in_bits {
                    // Check that the size in bits is valid.
                    let size_in_bits = if size_in_bits > self.limits.max_array_elements {
                        self.emit_err(crate::errors::type_checker::custom(
                        format!("The input type to `Serialize::*` is too large. Found {size_in_bits} bits, but the maximum allowed is {} bits.", self.limits.max_array_elements),
                        arguments[0].1.span()));
                        return TypeKind::Err;
                    } else if size_in_bits == 0 {
                        self.emit_err(crate::errors::type_checker::custom(
                            "The input type to `Serialize::*` is empty.",
                            arguments[0].1.span(),
                        ));
                        return TypeKind::Err;
                    } else {
                        u32::try_from(size_in_bits).expect("`max_array_elements` should fit in a u32")
                    };

                    // Return the array type.
                    return TypeKind::Array(ArrayType::bit_array(size_in_bits));
                }

                // Could not resolve the size in bits at this time.
                TypeKind::Err
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
                        type_.size_in_bits::<TestnetV0, _, _>(is_raw, &struct_not_supported, &struct_not_supported)
                    }
                    NetworkName::MainnetV0 => {
                        type_.size_in_bits::<MainnetV0, _, _>(is_raw, &struct_not_supported, &struct_not_supported)
                    }
                    NetworkName::CanaryV0 => {
                        type_.size_in_bits::<CanaryV0, _, _>(is_raw, &struct_not_supported, &struct_not_supported)
                    }
                };

                if let Ok(size_in_bits) = size_in_bits {
                    // Check that the size in bits is valid.
                    let size_in_bits = if size_in_bits > self.limits.max_array_elements {
                        self.emit_err(crate::errors::type_checker::custom(
                        format!("The output type of `Deserialize::*` is too large. Found {size_in_bits} bits, but the maximum allowed is {} bits.", self.limits.max_array_elements),
                        arguments[0].1.span()));
                        return TypeKind::Err;
                    } else if size_in_bits == 0 {
                        self.emit_err(crate::errors::type_checker::custom(
                            "The output type of `Deserialize::*` is empty.",
                            arguments[0].1.span(),
                        ));
                        return TypeKind::Err;
                    } else {
                        u32::try_from(size_in_bits).expect("`max_array_elements` should fit in a u32")
                    };

                    // Check that the input type is an array of the correct size.
                    let expected_type = TypeKind::Array(ArrayType::bit_array(size_in_bits));
                    if !input_type.types_equivalent(&expected_type) {
                        self.emit_err(crate::errors::type_checker::type_should_be2(
                            input_type,
                            format!("an array of {size_in_bits} bits"),
                            arguments[0].1.span(),
                        ));
                        return TypeKind::Err;
                    }
                };

                type_.clone()
            }
            Intrinsic::SelfAddress => TypeKind::Address,
            Intrinsic::SelfCaller => {
                // Check that the operation is not invoked in a `finalize` block.
                self.check_access_allowed("self.caller", AccessScope::OffchainCaller, function_span);
                TypeKind::Address
            }
            Intrinsic::SelfChecksum => TypeKind::Array(ArrayType::new(
                TypeKind::Integer(IntegerType::U8),
                Expression::Literal(Literal::integer(
                    IntegerType::U8,
                    "32".to_string(),
                    Default::default(),
                    Default::default(),
                )),
            )),
            Intrinsic::SelfEdition => TypeKind::Integer(IntegerType::U16),
            Intrinsic::SelfId => TypeKind::Address,
            Intrinsic::SelfProgramOwner => {
                // Check that the operation is invoked in a `finalize` block or a view.
                // The program owner resolves read-only in the finalize register path, so views can read it
                // (matching the externally-addressed `Program::program_owner` form).
                self.check_access_allowed("program_owner", AccessScope::FinalizeRead, function_span);
                TypeKind::Address
            }
            Intrinsic::SelfSigner => {
                // Check that operation is not invoked in a `finalize` block.
                self.check_access_allowed("self.signer", AccessScope::OffchainCaller, function_span);
                TypeKind::Address
            }
            Intrinsic::BlockHeight => {
                // Check that the operation is invoked in a `finalize` block or a view.
                // Views see the latest block height via FinalizeGlobalState::for_view.
                self.check_access_allowed("block.height", AccessScope::FinalizeRead, function_span);
                TypeKind::Integer(IntegerType::U32)
            }
            Intrinsic::BlockTimestamp => {
                // Check that the operation is invoked in a `finalize` block or a view.
                // Views see the block timestamp via FinalizeGlobalState, exactly like `block.height`.
                self.check_access_allowed("block.timestamp", AccessScope::FinalizeRead, function_span);
                TypeKind::Integer(IntegerType::I64)
            }
            Intrinsic::NetworkId => {
                // Check that the operation is not invoked outside a `finalize` block or a view.
                self.check_access_allowed("network.id", AccessScope::FinalizeRead, function_span);
                TypeKind::Integer(IntegerType::U16)
            }
            // Dynamic dispatch intrinsics are handled in visit_intrinsic before check_intrinsic.
            Intrinsic::DynamicCall
            | Intrinsic::DynamicContains
            | Intrinsic::DynamicGet
            | Intrinsic::DynamicGetOrUse => {
                unreachable!("Dynamic dispatch intrinsics are handled before check_intrinsic")
            }
        }
    }

    /// Validates scope restrictions shared by all dynamic call variants (both intrinsic and Interface@() syntax).
    /// Dynamic calls must be in an entry point, not in a final block, and not in a conditional.
    pub fn validate_dynamic_call_scope(&mut self, span: Span) {
        match self.scope_state.variant.unwrap() {
            Variant::Finalize => {
                self.emit_err(crate::errors::type_checker::dynamic_call_not_allowed_here("a finalize function", span));
            }
            Variant::FinalFn => {
                self.emit_err(crate::errors::type_checker::dynamic_call_not_allowed_here("a final function", span));
            }
            Variant::Fn => {
                self.emit_err(crate::errors::type_checker::dynamic_call_not_allowed_here("a regular function", span));
            }
            Variant::View => {
                self.emit_err(crate::errors::type_checker::dynamic_call_not_allowed_here("a view function", span));
            }
            Variant::EntryPoint => {}
        }
        if self.async_block_id.is_some() {
            self.emit_err(crate::errors::type_checker::dynamic_call_not_allowed_here("a final block", span));
        }
        if self.scope_state.is_conditional {
            self.emit_err(crate::errors::type_checker::dynamic_call_in_conditional(span));
        }
    }

    pub fn check_dynamic_call_intrinsic(
        &mut self,
        input: &IntrinsicExpression,
        expected: &Option<TypeKind>,
    ) -> TypeKind {
        let span = input.span();

        self.validate_dynamic_call_scope(span);

        // Minimum 3 arguments: program, network, function.
        if input.arguments.len() < 3 {
            self.emit_err(crate::errors::type_checker::dynamic_call_min_args(input.arguments.len(), span));
            return TypeKind::Err;
        }

        // First 3 arguments must be field or identifier.
        for arg in input.arguments.iter().take(3) {
            let arg_type = self.visit_expression(arg, &None);
            if !matches!(arg_type, TypeKind::Field | TypeKind::Identifier | TypeKind::Err) {
                self.emit_err(crate::errors::type_checker::type_should_be2(
                    &arg_type,
                    "`field` or `identifier`",
                    arg.span(),
                ));
            }
        }

        // Remaining arguments: if input_types are provided, validate count matches
        // and use them as expected types. Otherwise visit without expectation.
        let call_args = input.arguments.len().saturating_sub(3);
        if !input.input_types.is_empty() && input.input_types.len() != call_args {
            self.emit_err(crate::errors::type_checker::dynamic_call_input_type_count_mismatch(
                input.input_types.len(),
                call_args,
                span,
            ));
        }
        for (i, arg) in input.arguments.iter().skip(3).enumerate() {
            let expected = input.input_types.get(i).map(|(_, t, _)| t.clone());
            self.visit_expression_reject_numeric(arg, &expected);
        }

        // Validate input and return types: reject constant visibility and undefined composite types.
        for (mode, ty, sp) in input.input_types.iter().chain(input.return_types.iter()) {
            if matches!(mode, Mode::Constant) {
                self.emit_err(crate::errors::type_checker::dynamic_call_constant_not_allowed(*sp));
            }
            self.assert_type_is_valid(ty, *sp);
        }

        // Determine return type. Unit `()` is normalized to empty return_types at parse time.
        let return_type = match input.return_types.len() {
            0 => TypeKind::Unit,
            1 => input.return_types[0].1.clone(),
            _ => TypeKind::Tuple(TupleType::new(input.return_types.iter().map(|(_, t, _)| t.clone()).collect())),
        };

        // If return type contains Future, mark as dynamic call location.
        let contains_future = match &return_type {
            TypeKind::Future(..) => true,
            TypeKind::Tuple(tuple) => tuple.elements().iter().any(|t| matches!(t, TypeKind::Future(..))),
            _ => false,
        };
        if contains_future {
            self.scope_state.call_location = Some(Location::dynamic());
        }

        self.assert_and_return_type(return_type, expected, span)
    }

    /// Validates `_dynamic_contains`, `_dynamic_get`, and `_dynamic_get_or_use` intrinsics.
    pub fn check_dynamic_mapping_intrinsic(
        &mut self,
        intrinsic: Intrinsic,
        input: &IntrinsicExpression,
        expected: &Option<TypeKind>,
    ) -> TypeKind {
        let span = input.span();

        // Must be in finalize context.
        if !matches!(self.scope_state.variant, Some(Variant::Finalize | Variant::FinalFn))
            && self.async_block_id.is_none()
        {
            self.emit_err(crate::errors::type_checker::operation_must_be_in_final_block_or_function(span));
        }

        let (expected_args, needs_type_param, name) = match &intrinsic {
            Intrinsic::DynamicContains => (4, false, "_dynamic_contains"),
            Intrinsic::DynamicGet => (4, true, "_dynamic_get"),
            Intrinsic::DynamicGetOrUse => (5, true, "_dynamic_get_or_use"),
            _ => unreachable!(),
        };

        // Check argument count.
        if input.arguments.len() != expected_args {
            self.emit_err(crate::errors::type_checker::dynamic_intrinsic_wrong_arg_count(
                name,
                expected_args,
                input.arguments.len(),
                span,
            ));
            return TypeKind::Err;
        }

        // Check type parameter count.
        if needs_type_param && input.type_parameters.len() != 1 {
            self.emit_err(crate::errors::type_checker::dynamic_intrinsic_missing_type_param(name, span));
            return TypeKind::Err;
        }
        if !needs_type_param && !input.type_parameters.is_empty() {
            self.emit_err(crate::errors::type_checker::custom(
                format!("`{name}` does not accept type parameters."),
                span,
            ));
            return TypeKind::Err;
        }

        // First 3 arguments: program, network, mapping — must be field or identifier.
        for arg in input.arguments.iter().take(3) {
            let arg_type = self.visit_expression(arg, &None);
            if !matches!(arg_type, TypeKind::Field | TypeKind::Identifier | TypeKind::Err) {
                self.emit_err(crate::errors::type_checker::type_should_be2(
                    &arg_type,
                    "`field` or `identifier`",
                    arg.span(),
                ));
            }
        }

        // Arg 4 (key): any type, but must be fully typed (unsuffixed literals are rejected).
        if input.arguments.len() > 3 {
            self.visit_expression_reject_numeric(&input.arguments[3], &None);
        }

        // Determine return type.
        let return_type = match &intrinsic {
            Intrinsic::DynamicContains => TypeKind::Boolean,
            Intrinsic::DynamicGet | Intrinsic::DynamicGetOrUse => {
                let tp = input.type_parameters.first().map(|(t, _)| t.clone()).unwrap_or(TypeKind::Err);
                // For get_or_use, check that default value matches the type param.
                if matches!(intrinsic, Intrinsic::DynamicGetOrUse)
                    && let Some(default_arg) = input.arguments.get(4)
                {
                    self.visit_expression(default_arg, &Some(tp.clone()));
                }
                tp
            }
            _ => unreachable!(),
        };

        // Use `maybe_assert_type` (lenient coercion) rather than `assert_and_return_type` (strict
        // equality) so that e.g. `_dynamic_get_or_use::<T>(..)` satisfies an expected `T?` context.
        self.maybe_assert_type(&return_type, expected, span);
        return_type
    }

    /// Emits an error if the composite member is a record type.
    pub fn assert_member_is_not_record(&mut self, span: Span, parent: Symbol, type_: &TypeKind) {
        match type_ {
            TypeKind::Composite(composite)
                if self
                    .lookup_composite(composite.path.expect_global_location())
                    .is_some_and(|composite| composite.is_record) =>
            {
                self.emit_err(crate::errors::type_checker::struct_or_record_cannot_contain_record(
                    parent,
                    composite.path.clone(),
                    span,
                ))
            }
            TypeKind::DynRecord => {
                self.emit_err(crate::errors::type_checker::struct_or_record_cannot_contain_record(parent, type_, span))
            }
            TypeKind::Tuple(tuple_type) => {
                for type_ in tuple_type.elements().iter() {
                    self.assert_member_is_not_record(span, parent, type_)
                }
            }
            _ => {} // Do nothing.
        }
    }

    /// Emits an error if the type or its constituent types is not valid.
    pub fn assert_type_is_valid(&mut self, type_: &TypeKind, span: Span) {
        match type_ {
            // Unit types may only appear as the return type of a function.
            TypeKind::Unit => {
                self.emit_err(crate::errors::type_checker::unit_type_only_return(span));
            }
            // String types are temporarily disabled.
            TypeKind::String => {
                self.emit_err(crate::errors::type_checker::strings_are_not_supported(span));
            }
            // Check that named composite type has been defined and is accessible.
            TypeKind::Composite(composite) => {
                let loc = composite.path.expect_global_location();
                match self.lookup_composite(loc) {
                    Some(comp) => {
                        self.check_composite_accessible(loc, &comp, span);
                    }
                    None => {
                        self.emit_err(crate::errors::type_checker::undefined_type(composite.path.clone(), span));
                    }
                }
            }
            // Check that the constituent types of the tuple are valid.
            TypeKind::Tuple(tuple_type) => {
                for type_ in tuple_type.elements().iter() {
                    self.assert_type_is_valid(type_, span);
                }
            }
            // Check that the constituent types of mapping are valid.
            TypeKind::Mapping(mapping_type) => {
                self.assert_type_is_valid(&mapping_type.key, span);
                self.assert_type_is_valid(&mapping_type.value, span);
            }
            // Check that the array element types are valid.
            TypeKind::Array(array_type) => {
                // Check that the array length is valid.

                if let Some(length) = array_type.length.as_u32() {
                    if length > self.limits.max_array_elements as u32 {
                        self.emit_err(crate::errors::type_checker::array_too_large(
                            length,
                            self.limits.max_array_elements,
                            span,
                        ));
                    }
                } else if let Expression::Literal(_) = &*array_type.length {
                    // Literal, but not valid u32 (e.g. too big or invalid format)
                    self.emit_err(crate::errors::type_checker::array_too_large_for_u32(span));
                }
                // else: not a literal, so defer for later

                // Check that the array element type is valid.
                match array_type.element_type() {
                    // Array elements cannot be futures.
                    TypeKind::Future(_) => {
                        self.emit_err(crate::errors::type_checker::array_element_cannot_be_final(span))
                    }
                    // Array elements cannot be tuples.
                    TypeKind::Tuple(_) => {
                        self.emit_err(crate::errors::type_checker::array_element_cannot_be_tuple(span))
                    }
                    // Array elements cannot be records.
                    TypeKind::Composite(composite_type) => {
                        // Look up the type.
                        if let Some(composite) = self.lookup_composite(composite_type.path.expect_global_location()) {
                            // Check that the type is not a record.
                            if composite.is_record {
                                self.emit_err(crate::errors::type_checker::array_element_cannot_be_record(span));
                            }
                        }
                    }
                    // Array elements cannot be `dyn record`.
                    TypeKind::DynRecord => {
                        self.emit_err(crate::errors::type_checker::array_element_cannot_be_record(span));
                    }
                    _ => {} // Do nothing.
                }
                self.assert_type_is_valid(array_type.element_type(), span);
            }

            TypeKind::Optional(OptionalType { inner }) => {
                // Some types cannot be wrapped in an optional
                if self.disallowed_inside_optional(inner) {
                    self.emit_err(crate::errors::type_checker::optional_wrapping_unsupported(inner, span));
                }

                // Validate inner type normally
                self.assert_type_is_valid(inner, span);
            }

            // Vector types can only be used in storage declarations.
            TypeKind::Vector(_) => {
                self.emit_err(crate::errors::type_checker::vector_type_only_in_storage(span));
            }

            TypeKind::Address
            | TypeKind::Boolean
            | TypeKind::Field
            | TypeKind::Future(_)
            | TypeKind::Group
            | TypeKind::DynRecord
            | TypeKind::Identifier
            | TypeKind::Ident(_)
            | TypeKind::Integer(_)
            | TypeKind::Scalar
            | TypeKind::Signature
            | TypeKind::Numeric
            | TypeKind::Err => {} // Do nothing.
        }
    }

    /// Can type `ty` be used inside an optional?
    fn disallowed_inside_optional(&mut self, ty: &TypeKind) -> bool {
        let mut visited_paths = IndexSet::<Vec<Symbol>>::new();
        self.disallowed_inside_optional_inner(ty, &mut visited_paths)
    }

    fn disallowed_inside_optional_inner(&mut self, ty: &TypeKind, visited_paths: &mut IndexSet<Vec<Symbol>>) -> bool {
        match ty {
            TypeKind::Unit
            | TypeKind::Err
            | TypeKind::Future(_)
            | TypeKind::Ident(_)
            | TypeKind::Mapping(_)
            | TypeKind::Optional(_)
            | TypeKind::String
            | TypeKind::Identifier
            | TypeKind::DynRecord
            | TypeKind::Tuple(_)
            | TypeKind::Vector(_) => true,

            TypeKind::Composite(composite_type) => {
                let composite_location = composite_type.path.expect_global_location();

                // Prevent revisiting the same type. A composite that recurses through `Option<Self>`
                // (e.g. `struct Node { next: Node? }`) would otherwise drive this walk into an infinite
                // recursion and overflow the stack — the cycle-graph cycle check does not catch it
                // because `TypeKind::Optional(Composite(_))` adds no edge to the composite dependency graph.
                if !visited_paths.insert(composite_location.path.clone()) {
                    return false;
                }

                if let Some(composite) = self.lookup_composite(composite_location) {
                    if composite.is_record {
                        return true;
                    }

                    // recursively check all fields
                    for field in &composite.members {
                        let field_ty = field.type_.kind();
                        // unwrap optional fields for the check
                        let ty_to_check = match field_ty {
                            TypeKind::Optional(OptionalType { inner }) => inner.as_ref(),
                            _ => field_ty,
                        };
                        if self.disallowed_inside_optional_inner(ty_to_check, visited_paths) {
                            return true;
                        }
                    }
                }
                false
            }

            TypeKind::Array(array_type) => {
                let elem_type = match array_type.element_type() {
                    TypeKind::Optional(OptionalType { inner }) => inner,
                    other => other,
                };
                self.disallowed_inside_optional_inner(elem_type, visited_paths)
            }

            TypeKind::Address
            | TypeKind::Boolean
            | TypeKind::Field
            | TypeKind::Group
            | TypeKind::Integer(_)
            | TypeKind::Numeric
            | TypeKind::Scalar
            | TypeKind::Signature => false,
        }
    }

    /// Ensures the given type is valid for use in storage.
    /// Emits an error if the type or any of its inner types are invalid.
    pub fn assert_storage_type_is_valid(&mut self, type_: &TypeKind, span: Span) {
        if type_.is_empty() {
            self.emit_err(crate::errors::type_checker::invalid_storage_type("A zero sized type", span));
        }
        match type_ {
            // Prohibited top-level kinds
            TypeKind::Unit => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("unit", span));
            }
            TypeKind::String => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("string", span));
            }
            TypeKind::Identifier => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("identifier", span));
            }
            TypeKind::DynRecord => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("dyn record", span));
            }
            TypeKind::Future(_) => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("future", span));
            }
            TypeKind::Optional(_) => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("optional", span));
            }
            TypeKind::Mapping(_) => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("mapping", span));
            }
            TypeKind::Tuple(_) => {
                self.emit_err(crate::errors::type_checker::invalid_storage_type("tuple", span));
            }

            // Composites
            TypeKind::Composite(composite_type) => {
                if let Some(composite) = self.lookup_composite(composite_type.path.expect_global_location()) {
                    if composite.is_record {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("record", span));
                        return;
                    }

                    // Recursively check fields.
                    for field in &composite.members {
                        self.assert_storage_type_is_valid(field.type_.kind(), span);
                    }
                } else {
                    self.emit_err(crate::errors::type_checker::invalid_storage_type("undefined composite", span));
                }
            }

            // Arrays
            TypeKind::Array(array_type) => {
                if let Some(length) = array_type.length.as_u32()
                    && (length == 0 || length > self.limits.max_array_elements as u32)
                {
                    self.emit_err(crate::errors::type_checker::invalid_storage_type("array", span));
                }

                let element_ty = array_type.element_type();
                match element_ty {
                    TypeKind::Future(_) => {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("future", span))
                    }
                    TypeKind::Tuple(_) => {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("tuple", span))
                    }
                    TypeKind::Optional(_) => {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("optional", span))
                    }
                    _ => {}
                }

                self.assert_storage_type_is_valid(element_ty, span);
            }

            // Everything else (integers, bool, group, signature, etc.)
            TypeKind::Address
            | TypeKind::Boolean
            | TypeKind::Field
            | TypeKind::Group
            | TypeKind::Ident(_)
            | TypeKind::Integer(_)
            | TypeKind::Scalar
            | TypeKind::Signature
            | TypeKind::Numeric
            | TypeKind::Err => {} // valid
            TypeKind::Vector(vector_type) => {
                let element_ty = vector_type.element_type();
                match element_ty {
                    TypeKind::Future(_) => {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("future", span))
                    }
                    TypeKind::Tuple(_) => {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("tuple", span))
                    }
                    TypeKind::Optional(_) => {
                        self.emit_err(crate::errors::type_checker::invalid_storage_type("optional", span))
                    }
                    _ => {}
                }
                self.assert_storage_type_is_valid(element_ty, span);
            }
        }
    }

    /// Emits an error if the type is not a mapping.
    pub fn assert_mapping_type(&self, type_: &TypeKind, span: Span) {
        if type_ != &TypeKind::Err && !matches!(type_, TypeKind::Mapping(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "a mapping", span));
        }
    }

    /// Emits an error if the type is not an optional.
    pub fn assert_optional_type(&self, type_: &TypeKind, span: Span) {
        if type_ != &TypeKind::Err && !matches!(type_, TypeKind::Optional(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "an optional", span));
        }
    }

    /// Emits an error if the type is not a vector
    pub fn assert_vector_type(&self, type_: &TypeKind, span: Span) {
        if type_ != &TypeKind::Err && !matches!(type_, TypeKind::Vector(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "a vector", span));
        }
    }

    /// Emits an error if the type is not a vector or a mapping.
    pub fn assert_vector_or_mapping_type(&self, type_: &TypeKind, span: Span) {
        if type_ != &TypeKind::Err && !matches!(type_, TypeKind::Vector(_)) && !matches!(type_, TypeKind::Mapping(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "a vector or a mapping", span));
        }
    }

    pub fn contains_optional_type(&mut self, ty: &TypeKind) -> bool {
        let mut visited_paths = IndexSet::<Vec<Symbol>>::new();
        self.contains_optional_type_inner(ty, &mut visited_paths)
    }

    fn contains_optional_type_inner(&mut self, ty: &TypeKind, visited_paths: &mut IndexSet<Vec<Symbol>>) -> bool {
        match ty {
            TypeKind::Optional(_) => true,

            TypeKind::Tuple(tuple) => {
                tuple.elements.iter().any(|e| self.contains_optional_type_inner(e, visited_paths))
            }

            TypeKind::Array(array) => self.contains_optional_type_inner(&array.element_type, visited_paths),

            TypeKind::Composite(composite_type) => {
                let composite_location = composite_type.path.expect_global_location();

                // Prevent revisiting the same type
                // TODO: store locations here not just paths. Pending external structs.
                if !visited_paths.insert(composite_location.path.clone()) {
                    return false;
                }

                if let Some(comp) = self.lookup_composite(composite_location) {
                    comp.members
                        .iter()
                        .any(|Member { type_, .. }| self.contains_optional_type_inner(type_.kind(), visited_paths))
                } else {
                    false
                }
            }

            _ => false,
        }
    }

    pub fn assert_array_type(&self, type_: &TypeKind, span: Span) {
        if type_ != &TypeKind::Err && !matches!(type_, TypeKind::Array(_)) {
            self.emit_err(crate::errors::type_checker::type_should_be2(type_, "an array", span));
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

        let mut inferred_inputs: Vec<TypeKind> = Vec::new();

        if matches!(self.scope_state.variant, Some(Variant::Finalize)) && !self.scope_state.is_stub {
            // Async functions are not allowed to return values.
            if !function.output.is_empty() {
                panic!("Finalize is not allowed to return values");
            }

            // Iterator over the `finalize` member (type Finalizer) of each async transition that calls
            // this async function.
            let mut caller_finalizers = self
                .async_function_callers
                .get(&Location::new(self.scope_state.unit_name.unwrap(), function_path))
                .map(|callers| {
                    callers
                        .iter()
                        .flat_map(|caller| {
                            let caller = Location::new(caller.program, caller.path.clone());
                            self.state.symbol_table.lookup_function(self.scope_state.unit_name.unwrap(), &caller)
                        })
                        .flat_map(|fn_symbol| fn_symbol.finalizer.clone())
                })
                .into_iter()
                .flatten();

            if let Some(first) = caller_finalizers.next() {
                inferred_inputs = first.inferred_inputs.clone();

                // If any input is a future that doesn't have the same member type for all
                // finalizers, set that member to `TypeKind::Err`.
                for finalizer in caller_finalizers {
                    assert_eq!(inferred_inputs.len(), finalizer.inferred_inputs.len());
                    for (t1, t2) in inferred_inputs.iter_mut().zip(finalizer.inferred_inputs.iter()) {
                        self.merge_types(t1, t2);
                    }
                }
            } else {
                panic!("Finalize is never called by entry point.");
            }
        }

        // Const generic parameters can only be monomorphized at inline call sites.  Reject them on
        // any function that will never be inlined or that does not support inlining.
        if !function.const_parameters.is_empty() {
            if function.annotations.iter().any(|a| a.identifier.name == sym::no_inline) {
                self.emit_err(crate::errors::type_checker::cannot_have_const_generics(
                    "functions annotated with `@no_inline`",
                    function.identifier.span(),
                ));
            } else if matches!(self.scope_state.variant, Some(Variant::EntryPoint)) {
                self.emit_err(crate::errors::type_checker::cannot_have_const_generics(
                    "entry point functions",
                    function.identifier.span(),
                ));
            } else if matches!(self.scope_state.variant, Some(Variant::FinalFn)) {
                self.emit_err(crate::errors::type_checker::cannot_have_const_generics(
                    "`final fn` functions",
                    function.identifier.span(),
                ));
            } else if matches!(self.scope_state.variant, Some(Variant::View)) {
                self.emit_err(crate::errors::type_checker::cannot_have_const_generics(
                    "`view fn` functions",
                    function.identifier.span(),
                ));
            }
        }

        // Ensure that `@no_inline` is not used on `final fn` functions.
        if matches!(self.scope_state.variant, Some(Variant::FinalFn))
            && function.annotations.iter().any(|a| a.identifier.name == sym::no_inline)
        {
            self.emit_err(crate::errors::type_checker::no_inline_not_allowed_on_final_fn(function.identifier.span()));
        }

        for const_param in &function.const_parameters {
            self.visit_type(const_param.type_().kind());

            // Restrictions for const parameters
            if !matches!(
                const_param.type_().kind(),
                TypeKind::Boolean
                    | TypeKind::Integer(_)
                    | TypeKind::Address
                    | TypeKind::Scalar
                    | TypeKind::Group
                    | TypeKind::Field
            ) {
                self.emit_err(crate::errors::type_checker::bad_const_generic_type(
                    const_param.type_().kind(),
                    const_param.span(),
                ));
            }

            // Set the type of the input in the symbol table.
            self.state.symbol_table.set_local_type(const_param.identifier.name, const_param.type_().kind().clone());

            // Add the input to the type table.
            let ty = const_param.type_().ty();
            self.state.type_table.insert(const_param.identifier().id(), ty);
        }

        // Ensure there aren't too many inputs
        if (function.variant.is_entry() || function.variant.is_finalize() || function.variant.is_view())
            && function.input.len() > self.limits.max_inputs
        {
            self.state.handler.emit_err(crate::errors::type_checker::function_has_too_many_inputs(
                function.variant,
                function.identifier,
                self.limits.max_inputs,
                function.input.len(),
                function.identifier.span,
            ));
        }

        // The inputs should have access to the const parameters, so handle them after.
        for (i, input) in function.input.iter().enumerate() {
            self.visit_type(input.type_().kind());

            // No need to check compatibility of these types; that's already been done
            let table_type = inferred_inputs.get(i).unwrap_or(input.type_().kind());

            // Check that the type of input parameter is defined.
            self.assert_type_is_valid(table_type, input.span());

            // Check that the type of the input parameter is not a tuple.
            if matches!(table_type, TypeKind::Tuple(_)) {
                self.emit_err(crate::errors::type_checker::function_cannot_take_tuple_as_input(input.span()))
            }

            // Check that the type of the input parameter does not contain an optional.
            if self.contains_optional_type(table_type) && matches!(function.variant, Variant::EntryPoint) {
                self.emit_err(crate::errors::type_checker::function_cannot_take_option_as_input(
                    input.identifier,
                    table_type,
                    input.span(),
                ))
            }

            // Make sure only transitions can take a record as an input.
            if let TypeKind::Composite(composite) = table_type {
                // Throw error for undefined type.
                if !function.variant.is_entry() {
                    if let Some(elem) = self.lookup_composite(composite.path.expect_global_location()) {
                        if elem.is_record {
                            self.emit_err(crate::errors::type_checker::function_cannot_input_or_output_a_record(
                                input.span(),
                            ))
                        }
                    } else {
                        self.emit_err(crate::errors::type_checker::undefined_type(
                            composite.path.clone(),
                            input.span(),
                        ));
                    }
                }
            }

            // This unwrap works since we assign to `variant` above.
            match self.scope_state.variant.unwrap() {
                // If the function is an entry point, then check that the parameter mode is not a constant.
                Variant::EntryPoint if input.mode() == Mode::Constant => {
                    self.emit_err(crate::errors::type_checker::entry_point_fn_inputs_cannot_be_const(input.span()))
                }
                // Helpers, finalize bodies, `final fn`s, and views all lower their input
                // visibility from the variant alone (helpers are inlined; finalize/final fn/view
                // inputs are always `.public`), so an explicit modifier is always redundant or
                // contradictory.
                Variant::Fn if input.mode() != Mode::None => self.emit_err(
                    crate::errors::type_checker::function_inputs_cannot_have_modes("regular `fn`", input.span()),
                ),
                Variant::Finalize | Variant::FinalFn if input.mode() != Mode::None => self.emit_err(
                    crate::errors::type_checker::function_inputs_cannot_have_modes("`final fn`", input.span()),
                ),
                Variant::View if input.mode() != Mode::None => self.emit_err(
                    crate::errors::type_checker::function_inputs_cannot_have_modes("`view fn`", input.span()),
                ),
                _ => {} // Do nothing.
            }

            // Records and `Final`s lower to `.record`/`.future` markers, neither of which carries a
            // visibility, so an explicit mode on such an input is meaningless. (Non-entry variants
            // already reject all input modes above, so this only adds the record/`Final` cases.)
            if function.variant.is_entry() && input.mode() != Mode::None {
                let kind = if self.type_is_record(table_type) {
                    Some("record")
                } else if matches!(table_type, TypeKind::Future(_)) {
                    Some("`Final`")
                } else {
                    None
                };
                if let Some(kind) = kind {
                    self.emit_err(crate::errors::type_checker::cannot_have_mode(kind, input.span()));
                }
            }

            if matches!(table_type, TypeKind::Future(..)) {
                // Future parameters may only appear in onchain functions.
                // TODO: we may want to relax this
                if !matches!(self.scope_state.variant, Some(Variant::Finalize | Variant::FinalFn)) {
                    self.emit_err(crate::errors::type_checker::no_final_parameters(input.span()));
                }
            }

            if !is_stub {
                // Set the type of the input in the symbol table.
                self.state.symbol_table.set_local_type(input.identifier.name, table_type.clone());

                // Add the input to the type table.
                let table_ty = self.state.types.intern(table_type);
                self.state.type_table.insert(input.identifier().id(), table_ty);
            }
        }

        // Ensure there aren't too many outputs (entry points and view fns are
        // externally-callable and bound by snarkVM's MAX_OUTPUTS).
        if function.output.len() > self.limits.max_outputs
            && matches!(function.variant, Variant::EntryPoint | Variant::View)
        {
            self.state.handler.emit_err(crate::errors::type_checker::function_has_too_many_outputs(
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
            self.visit_type(function_output.type_.kind());

            // If the function is not a transition function, then it cannot output a record.
            // Note that an external output must always be a record.
            if let TypeKind::Composite(composite) = function_output.type_.kind().clone()
                && let Some(val) = self.lookup_composite(composite.path.expect_global_location())
                && val.is_record
                && !function.variant.is_entry()
            {
                self.emit_err(crate::errors::type_checker::function_cannot_input_or_output_a_record(
                    function_output.span,
                ));
            }

            // Check that the output type is valid.
            self.assert_type_is_valid(function_output.type_.kind(), function_output.span);

            // Check that the type of the output is not a tuple. This is necessary to forbid nested tuples.
            if matches!(&function_output.type_.kind(), TypeKind::Tuple(_)) {
                self.emit_err(crate::errors::type_checker::nested_tuple_type(function_output.span))
            }

            // Check that the type of the input parameter does not contain an optional.
            if self.contains_optional_type(function_output.type_.kind())
                && matches!(function.variant, Variant::EntryPoint)
            {
                self.emit_err(crate::errors::type_checker::function_cannot_return_option_as_output(
                    function_output.type_.kind(),
                    function_output.span(),
                ))
            }

            // Check that the mode of the output is valid.
            // Records and `Final`s lower to `.record`/`.future` markers, neither of which carries a
            // visibility, so an explicit mode on such an output is meaningless. These types are only
            // valid as outputs on entry points (other variants already error above).
            let record_or_final_output = if self.type_is_record(function_output.type_.kind()) {
                Some("record")
            } else if matches!(function_output.type_.kind(), TypeKind::Future(_)) {
                Some("`Final`")
            } else {
                None
            };
            if let Some(kind) = record_or_final_output {
                if function.variant.is_entry() && function_output.mode != Mode::None {
                    self.emit_err(crate::errors::type_checker::cannot_have_mode(kind, function_output.span));
                }
            } else if function_output.mode == Mode::Constant {
                // For other types, only public and private outputs are allowed.
                self.emit_err(crate::errors::type_checker::cannot_have_constant_output_mode(function_output.span));
            }
            // View outputs lower to `.public` from the variant alone, same as their inputs.
            if matches!(function.variant, Variant::View) && function_output.mode != Mode::None {
                self.emit_err(crate::errors::type_checker::function_outputs_cannot_have_modes(
                    "`view fn`",
                    function_output.span,
                ));
            }
            // `final fn` helpers are inlined into their callsites, so their outputs never lower to
            // AVM outputs and a visibility mode is meaningless.
            if matches!(function.variant, Variant::FinalFn) && function_output.mode != Mode::None {
                self.emit_err(crate::errors::type_checker::function_outputs_cannot_have_modes(
                    "`final fn`",
                    function_output.span,
                ));
            }
            // Async transitions must return exactly one future, and it must be in the last position.
            if function.has_final_output()
                && function.variant.is_entry()
                && ((index < function.output.len() - 1 && matches!(function_output.type_.kind(), TypeKind::Future(_)))
                    || (index == function.output.len() - 1
                        && !matches!(function_output.type_.kind(), TypeKind::Future(_))))
            {
                self.emit_err(crate::errors::type_checker::entry_point_fn_final_invalid_output(function_output.span));
            }
            // If the function is not an async transition, then it cannot have a future as output.
            if !matches!(self.scope_state.variant, Some(Variant::EntryPoint))
                && matches!(function_output.type_.kind(), TypeKind::Future(_))
            {
                self.emit_err(crate::errors::type_checker::only_entry_point_can_return_final(function_output.span));
            }
        });

        self.visit_type(&function.output_type);
    }

    /// Merge inferred types into `lhs`.
    ///
    /// That is, if `lhs` and `rhs` aren't equal, set `lhs` to TypeKind::Err;
    /// or, if they're both futures, set any member of `lhs` that isn't
    /// equal to the equivalent member of `rhs` to `TypeKind::Err`.
    fn merge_types(&self, lhs: &mut TypeKind, rhs: &TypeKind) {
        if let TypeKind::Future(f1) = lhs {
            if let TypeKind::Future(f2) = rhs {
                for (i, type_) in f2.inputs.iter().enumerate() {
                    if let Some(lhs_type) = f1.inputs.get_mut(i) {
                        self.merge_types(lhs_type, type_);
                    } else {
                        f1.inputs.push(TypeKind::Err);
                    }
                }
            } else {
                *lhs = TypeKind::Err;
            }
        } else if !lhs.types_equivalent(rhs) {
            *lhs = TypeKind::Err;
        }
    }

    /// Returns `true` if `type_` resolves to a record, including dynamic interface records.
    pub fn type_is_record(&mut self, type_: &TypeKind) -> bool {
        match type_ {
            TypeKind::DynRecord => true,
            TypeKind::Composite(composite) => {
                self.lookup_composite(composite.path.expect_global_location()).is_some_and(|comp| comp.is_record)
            }
            _ => false,
        }
    }

    /// Wrapper around lookup_struct and lookup_record that additionally records all structs and records that are
    /// used in the program.
    pub fn lookup_composite(&mut self, loc: &Location) -> Option<Composite> {
        let current_program = self.scope_state.unit_name.unwrap();
        let record_comp = self.state.symbol_table.lookup_record(current_program, loc);
        let comp = record_comp.or_else(|| self.state.symbol_table.lookup_struct(current_program, loc));
        if let Some(s) = comp {
            // Record the usage.
            // If it's a struct or internal record, mark it used.
            if !s.is_record || Some(loc.program) == self.scope_state.unit_name {
                self.used_composites.insert(loc.clone());
            }
        }
        comp.cloned()
    }

    /// Emits `inaccessible_item` if `comp` is not visible from the current scope. `span` is the
    /// user's reference site, not the declaration. Returns `true` when accessible.
    pub fn check_composite_accessible(&mut self, loc: &Location, comp: &Composite, span: Span) -> bool {
        if self.scope_state.is_accessible(loc, comp.is_exported) {
            return true;
        }
        let kind = if comp.is_record { "record" } else { "struct" };
        self.emit_err(crate::errors::type_checker::inaccessible_item(kind, comp.identifier.name, span));
        false
    }

    /// Replaces interface record types with `TypeKind::DynRecord`. Only recurses into tuples — records cannot be nested inside structs or arrays.
    pub fn replace_records_with_dyn_record(&mut self, ty: &TypeKind, interface: &Interface) -> TypeKind {
        match ty {
            TypeKind::DynRecord => TypeKind::DynRecord,
            TypeKind::Tuple(tuple) => TypeKind::Tuple(TupleType::new(
                tuple.elements().iter().map(|t| self.replace_records_with_dyn_record(t, interface)).collect(),
            )),
            other => {
                if interface.is_record_type(other) {
                    TypeKind::DynRecord
                } else {
                    other.clone()
                }
            }
        }
    }

    /// Sets the type of a variable in the symbol table.
    pub fn set_local_type(&mut self, inferred_type: Option<TypeKind>, name: &Identifier, type_: TypeKind) {
        self.insert_symbol_conditional_scope(name.name);

        let is_future = match &type_ {
            TypeKind::Future(..) => true,
            TypeKind::Tuple(tuple_type) if matches!(tuple_type.elements().last(), Some(TypeKind::Future(..))) => true,
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

    /// Validates whether an access operation is allowed in the current function or block context.
    /// See [`AccessScope`] for the meaning of each variant.
    pub fn check_access_allowed(&mut self, name: &str, scope: AccessScope, span: Span) {
        let in_view = matches!(self.scope_state.variant, Some(Variant::View));
        let in_finalize_ctx = self.scope_state.variant.is_some_and(|v| v.is_finalize_context());
        let in_async_block = self.async_block_id.is_some();

        // In a view: only finalize-read ops are allowed.
        if in_view {
            if !matches!(scope, AccessScope::FinalizeRead) {
                self.state
                    .handler
                    .emit_err(crate::errors::type_checker::invalid_operation_outside_finalize(name, span));
            }
            return;
        }

        match scope {
            // Finalize-only ops (read or write) must be inside a finalize context or async block.
            AccessScope::FinalizeRead | AccessScope::FinalizeWrite => {
                if !in_finalize_ctx && !in_async_block {
                    self.state
                        .handler
                        .emit_err(crate::errors::type_checker::invalid_operation_outside_finalize(name, span));
                }
            }
            // Caller-context ops (e.g. `self.caller`, `self.signer`) are rejected inside any
            // finalize context.
            AccessScope::OffchainCaller => {
                if in_finalize_ctx {
                    self.state
                        .handler
                        .emit_err(crate::errors::type_checker::invalid_operation_inside_finalize(name, span));
                } else if in_async_block {
                    self.state
                        .handler
                        .emit_err(crate::errors::type_checker::invalid_operation_inside_final_block(name, span));
                }
            }
        }
    }

    pub fn is_external_record(&self, ty: &TypeKind) -> bool {
        if let TypeKind::Composite(typ) = &ty {
            let this_program = self.scope_state.unit_name.unwrap();
            let composite_location = typ.path.expect_global_location();
            composite_location.program != this_program
                && self.state.symbol_table.lookup_record(this_program, composite_location).is_some()
        } else {
            false
        }
    }

    pub fn parse_integer_literal<I: FromStrRadix>(&self, raw_string: &str, span: Span, type_string: &str) {
        let string = raw_string.replace('_', "");
        if I::from_str_by_radix(&string).is_err() {
            self.state.handler.emit_err(crate::errors::type_checker::invalid_int_value(string, type_string, span));
        }
    }

    // Emit an error and update `ty` to be `TypeKind::Err` indicating that the type of the expression could not be inferred.
    // Also update `type_table` accordingly
    pub fn emit_inference_failure_error(&self, ty: &mut TypeKind, expr: &Expression) {
        self.emit_err(crate::errors::type_checker::could_not_determine_type(expr.clone(), expr.span()));
        *ty = TypeKind::Err;
        self.state.type_table.insert(expr.id(), Type::ERR);
    }

    // Given a `Literal` and its type, if the literal is a numeric `Unsuffixed` literal, ensure it's a valid literal
    // given the type. E.g., a `256` is not a valid `u8`.
    pub fn check_numeric_literal(&self, input: &Literal, ty: &TypeKind) -> bool {
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
                TypeKind::Integer(kind) => match kind {
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
                TypeKind::Group => {
                    if has_nondecimal_prefix(s) {
                        // This is not checked in the parser for unsuffixed numerals. So do that here.
                        self.emit_err(crate::errors::type_checker::hexbin_literal_nonintegers(span));
                        return false;
                    } else {
                        let trimmed = s.trim_start_matches('-').trim_start_matches('0');
                        if !trimmed.is_empty()
                            && format!("{trimmed}group")
                                .parse::<snarkvm::prelude::Group<snarkvm::prelude::TestnetV0>>()
                                .is_err()
                        {
                            self.emit_err(crate::errors::type_checker::invalid_int_value(trimmed, "group", span));
                            return false;
                        }
                    }
                }
                // This is not checked in the parser for unsuffixed numerals. So do that here.
                TypeKind::Field | TypeKind::Scalar if has_nondecimal_prefix(s) => {
                    self.emit_err(crate::errors::type_checker::hexbin_literal_nonintegers(span));
                    return false;
                }
                _ => {
                    // Other types aren't expected here
                }
            }
        }
        true
    }

    /// Emits an error if the current scope is not a `final` block or function.
    fn require_final_scope(&mut self, span: Span) {
        if !matches!(self.scope_state.variant, Some(Variant::Finalize | Variant::FinalFn))
            && self.async_block_id.is_none()
        {
            self.emit_err(crate::errors::type_checker::operation_must_be_in_final_block_or_function(span));
        }
    }

    /// Checks that `target_program` is a `field` or `identifier` and that the optional `network` is an `identifier`.
    fn check_dynamic_op_target_and_network(&mut self, input: &DynamicOpExpression) {
        let target_type = self.visit_expression(&input.target_program, &None);
        if !matches!(target_type, TypeKind::Field | TypeKind::Identifier | TypeKind::Err) {
            self.emit_err(crate::errors::type_checker::type_should_be2(
                &target_type,
                "`field` or `identifier`",
                input.target_program.span(),
            ));
        }
        if let Some(ref network) = input.network {
            self.visit_expression(network, &Some(TypeKind::Identifier));
        }
    }

    /// Type-checks `Interface@(target)::member.op(args)`.
    ///
    /// `member` may refer to either a mapping (supporting `get`, `get_or_use`, `contains`)
    /// or a vector storage variable (supporting `get`, `len`).
    pub fn check_dynamic_mapping_op(
        &mut self,
        input: &DynamicOpExpression,
        interface: &Interface,
        expected: &Option<TypeKind>,
    ) -> TypeKind {
        let DynamicOpKind::Op { member, op, arguments } = &input.kind else {
            panic!("check_dynamic_mapping_op expects a DynamicOpKind::Op");
        };
        let op_name = op.name;
        let span = input.span;

        self.require_final_scope(span);
        self.check_dynamic_op_target_and_network(input);

        // Mapping case.
        if let Some(mapping_proto) = interface.mappings.iter().find(|m| m.identifier.name == member.name) {
            let key_type = mapping_proto.key_type.clone();
            let value_type = mapping_proto.value_type.clone();

            let return_type = if op_name == sym::get {
                if arguments.len() != 1 {
                    self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(1, arguments.len(), span));
                    return TypeKind::Err;
                }
                self.visit_expression_reject_numeric(&arguments[0], &Some(key_type));
                value_type
            } else if op_name == sym::contains {
                if arguments.len() != 1 {
                    self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(1, arguments.len(), span));
                    return TypeKind::Err;
                }
                self.visit_expression_reject_numeric(&arguments[0], &Some(key_type));
                TypeKind::Boolean
            } else if op_name == sym::get_or_use {
                if arguments.len() != 2 {
                    self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(2, arguments.len(), span));
                    return TypeKind::Err;
                }
                self.visit_expression_reject_numeric(&arguments[0], &Some(key_type));
                self.visit_expression(&arguments[1], &Some(value_type.clone()));
                value_type
            } else {
                self.emit_err(crate::errors::type_checker::custom(
                    format!("Unknown mapping operation `{op_name}`. Expected `get`, `get_or_use`, or `contains`."),
                    op.span,
                ));
                return TypeKind::Err;
            };

            return self.assert_and_return_type(return_type, expected, span);
        }

        // Storage variable case: only vectors support `.op(args)`; singletons use the bare read form.
        if let Some(storage_proto) = interface.storages.iter().find(|s| s.identifier.name == member.name) {
            let TypeKind::Vector(vector_ty) = &storage_proto.type_.kind() else {
                self.emit_err(crate::errors::type_checker::custom(
                    format!("`{member}` is a singleton storage variable; read it as `Interface@(target)::{member}` without `.` or arguments."),
                    span));
                return TypeKind::Err;
            };
            let element_type = (*vector_ty.element_type).clone();

            let return_type = if op_name == sym::get {
                if arguments.len() != 1 {
                    self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(1, arguments.len(), span));
                    return TypeKind::Err;
                }
                self.visit_expression(&arguments[0], &Some(TypeKind::Integer(IntegerType::U32)));
                // Vector `.get(i)` on external storage yields `Option<element_type>`.
                TypeKind::Optional(OptionalType { inner: Box::new(element_type) })
            } else if op_name == sym::len {
                if !arguments.is_empty() {
                    self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(0, arguments.len(), span));
                    return TypeKind::Err;
                }
                // Vector `.len()` on external storage yields `u32`.
                TypeKind::Integer(IntegerType::U32)
            } else {
                self.emit_err(crate::errors::type_checker::custom(
                    format!("Unknown vector operation `{op_name}`. Expected `get` or `len`."),
                    op.span,
                ));
                return TypeKind::Err;
            };

            return self.assert_and_return_type(return_type, expected, span);
        }

        self.emit_err(crate::errors::type_checker::unknown_sym(
            "mapping or storage variable",
            format!("{}::{}", input.interface, member),
            member.span,
        ));
        TypeKind::Err
    }

    /// Type-checks `Interface@(target)::storage_name` (bare read of a singleton storage variable).
    pub fn check_dynamic_read(
        &mut self,
        input: &DynamicOpExpression,
        interface: &Interface,
        expected: &Option<TypeKind>,
    ) -> TypeKind {
        let DynamicOpKind::Read { storage } = &input.kind else {
            panic!("check_dynamic_read expects a DynamicOpKind::Read");
        };
        let span = input.span;

        self.require_final_scope(span);
        self.check_dynamic_op_target_and_network(input);

        // Look up the storage prototype.
        let Some(storage_proto) = interface.storages.iter().find(|s| s.identifier.name == storage.name) else {
            if interface.mappings.iter().any(|m| m.identifier.name == storage.name) {
                self.emit_err(crate::errors::type_checker::custom(
                    format!(
                        "`{storage}` is a mapping; read a value with `{}::{storage}.get(key)` or `.get_or_use(key, default)`.",
                        input.interface
                    ),
                    span));
            } else {
                self.emit_err(crate::errors::type_checker::unknown_sym(
                    "storage variable",
                    format!("{}::{}", input.interface, storage),
                    storage.span,
                ));
            }
            return TypeKind::Err;
        };

        // Vectors cannot be read with the bare form; they require `.get(i)`.
        if matches!(storage_proto.type_.kind(), TypeKind::Vector(_)) {
            self.emit_err(crate::errors::type_checker::custom(
                format!(
                    "`{}` is a vector storage variable; read an element with `{}::{}.get(index)`.",
                    storage, input.interface, storage
                ),
                span,
            ));
            return TypeKind::Err;
        }

        // Singleton reads yield `Option<declared_type>`.
        let return_type = TypeKind::Optional(OptionalType { inner: Box::new(storage_proto.type_.kind().clone()) });
        self.assert_and_return_type(return_type, expected, span)
    }

    /// Type-checks `Interface@(target)::func(args)`.
    pub fn check_dynamic_function_call(
        &mut self,
        input: &DynamicOpExpression,
        interface: &Interface,
        expected: &Option<TypeKind>,
    ) -> TypeKind {
        let DynamicOpKind::Call { function, arguments } = &input.kind else {
            panic!("check_dynamic_function_call expects a DynamicOpKind::Call");
        };

        // Find the function prototype in the interface.
        let Some((_, func_proto)) = interface.functions.iter().find(|(name, _)| *name == function.name) else {
            self.emit_err(crate::errors::type_checker::unknown_sym(
                "function",
                format!("{}::{}", input.interface, function),
                function.span,
            ));
            return TypeKind::Err;
        };
        let func_proto = func_proto.clone();

        self.validate_dynamic_call_scope(input.span);

        self.check_dynamic_op_target_and_network(input);

        // Check argument count.
        if func_proto.input.len() != arguments.len() {
            self.emit_err(crate::errors::type_checker::incorrect_num_args_to_call(
                func_proto.input.len(),
                arguments.len(),
                input.span(),
            ));
        }

        // Check argument types. Record-typed parameters require `dyn record` at the call site.
        for (expected_input, argument) in func_proto.input.iter().zip(arguments.iter()) {
            let proto_type = expected_input.type_().kind().clone();
            if interface.is_record_type(&proto_type) {
                // Visit without an expected type so only the explicit error below fires.
                let actual_type = self.visit_expression(argument, &None);
                if !matches!(actual_type, TypeKind::DynRecord | TypeKind::Err) {
                    self.emit_err(crate::errors::type_checker::dynamic_call_record_arg_requires_dyn_record(
                        &proto_type,
                        argument.span(),
                    ));
                }
            } else {
                self.visit_expression(argument, &Some(proto_type));
            }
        }

        // Replace interface record types in the output with `dyn record`, then check for futures.
        let output_type = self.replace_records_with_dyn_record(&func_proto.output_type, interface);
        let contains_future = match &output_type {
            TypeKind::Future(..) => true,
            TypeKind::Tuple(tuple) => tuple.elements().iter().any(|t| matches!(t, TypeKind::Future(..))),
            _ => false,
        };
        if contains_future {
            self.scope_state.call_location = Some(Location::dynamic());
        }

        // Return the function prototype's output type.
        self.assert_and_return_type(output_type, expected, input.span())
    }
}
