// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{
    type_checking::{await_checker::AwaitChecker, scope_state::ScopeState},
    CallGraph,
    StructGraph,
    SymbolTable,
    TypeTable,
    VariableSymbol,
    VariableType,
};

use leo_ast::*;
use leo_errors::{emitter::Handler, TypeCheckerError, TypeCheckerWarning};
use leo_span::{Span, Symbol};

use snarkvm::console::network::Network;

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::{cell::RefCell, marker::PhantomData};

pub struct TypeChecker<'a, N: Network> {
    /// The symbol table for the program.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// A mapping from node IDs to their types.
    pub(crate) type_table: &'a TypeTable,
    /// A dependency graph of the structs in program.
    pub(crate) struct_graph: StructGraph,
    /// The call graph for the program.
    pub(crate) call_graph: CallGraph,
    /// The error handler.
    pub(crate) handler: &'a Handler,
    /// The state of the current scope being traversed.
    pub(crate) scope_state: ScopeState,
    /// Struct to store the state relevant to checking all futures are awaited.
    pub(crate) await_checker: AwaitChecker,
    /// Mapping from async function name to the inferred input types.
    pub(crate) async_function_input_types: IndexMap<Location, Vec<Type>>,
    /// The set of used composites.
    pub(crate) used_structs: IndexSet<Symbol>,
    // Allows the type checker to be generic over the network.
    phantom: PhantomData<N>,
}

const ADDRESS_TYPE: Type = Type::Address;

const BOOLEAN_TYPE: Type = Type::Boolean;

const FIELD_TYPE: Type = Type::Field;

const GROUP_TYPE: Type = Type::Group;

const SCALAR_TYPE: Type = Type::Scalar;

const SIGNATURE_TYPE: Type = Type::Signature;

const INT_TYPES: [Type; 10] = [
    Type::Integer(IntegerType::I8),
    Type::Integer(IntegerType::I16),
    Type::Integer(IntegerType::I32),
    Type::Integer(IntegerType::I64),
    Type::Integer(IntegerType::I128),
    Type::Integer(IntegerType::U8),
    Type::Integer(IntegerType::U16),
    Type::Integer(IntegerType::U32),
    Type::Integer(IntegerType::U64),
    Type::Integer(IntegerType::U128),
];

const SIGNED_INT_TYPES: [Type; 5] = [
    Type::Integer(IntegerType::I8),
    Type::Integer(IntegerType::I16),
    Type::Integer(IntegerType::I32),
    Type::Integer(IntegerType::I64),
    Type::Integer(IntegerType::I128),
];

const UNSIGNED_INT_TYPES: [Type; 5] = [
    Type::Integer(IntegerType::U8),
    Type::Integer(IntegerType::U16),
    Type::Integer(IntegerType::U32),
    Type::Integer(IntegerType::U64),
    Type::Integer(IntegerType::U128),
];

const MAGNITUDE_TYPES: [Type; 3] =
    [Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U32)];

impl<'a, N: Network> TypeChecker<'a, N> {
    /// Returns a new type checker given a symbol table and error handler.
    pub fn new(
        symbol_table: SymbolTable,
        type_table: &'a TypeTable,
        handler: &'a Handler,
        max_depth: usize,
        disabled: bool,
    ) -> Self {
        let struct_names = symbol_table.structs.keys().map(|loc| loc.name).collect();
        let function_names = symbol_table.functions.keys().map(|loc| loc.name).collect();

        // Note that the `struct_graph` and `call_graph` are initialized with their full node sets.
        Self {
            symbol_table: RefCell::new(symbol_table),
            type_table,
            struct_graph: StructGraph::new(struct_names),
            call_graph: CallGraph::new(function_names),
            handler,
            scope_state: ScopeState::new(),
            await_checker: AwaitChecker::new(max_depth, !disabled),
            async_function_input_types: IndexMap::new(),
            used_structs: IndexSet::new(),
            phantom: Default::default(),
        }
    }

    /// Enters a child scope.
    pub(crate) fn enter_scope(&mut self, index: usize) {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(previous_symbol_table.borrow().lookup_scope_by_index(index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));
    }

    /// Creates a new child scope.
    pub(crate) fn create_child_scope(&mut self) -> usize {
        // Creates a new child scope.
        let scope_index = self.symbol_table.borrow_mut().insert_block();
        // Enter the new scope.
        self.enter_scope(scope_index);
        // Return the index of the new scope.
        scope_index
    }

    /// Exits the current scope.
    pub(crate) fn exit_scope(&mut self, index: usize) {
        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(previous_symbol_table.lookup_scope_by_index(index).unwrap());
        self.symbol_table = RefCell::new(previous_symbol_table);
    }

    /// Emits a type checker error.
    pub(crate) fn emit_err(&self, err: TypeCheckerError) {
        self.handler.emit_err(err);
    }

    /// Emits a type checker warning
    pub fn emit_warning(&self, warning: TypeCheckerWarning) {
        self.handler.emit_warning(warning.into());
    }

    /// Emits an error to the handler if the given type is invalid.
    fn check_type(&self, is_valid: impl Fn(&Type) -> bool, error_string: String, type_: &Option<Type>, span: Span) {
        if let Some(type_) = type_ {
            if !is_valid(type_) {
                self.emit_err(TypeCheckerError::expected_one_type_of(error_string, type_, span));
            }
        }
    }

    /// Emits an error if the two given types are not equal.
    pub(crate) fn check_eq_types(&self, t1: &Option<Type>, t2: &Option<Type>, span: Span) {
        match (t1, t2) {
            (Some(t1), Some(t2)) if !Type::eq_flat_relax_composite(t1, t2) => {
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
    pub(crate) fn assert_and_return_type(&mut self, actual: Type, expected: &Option<Type>, span: Span) -> Type {
        if expected.is_some() {
            self.check_eq_types(&Some(actual.clone()), expected, span);
        }
        actual
    }

    /// Emits an error to the error handler if the `actual` type is not equal to the `expected` type.
    pub(crate) fn assert_type(&mut self, actual: &Option<Type>, expected: &Type, span: Span) {
        if let Some(actual) = actual {
            self.check_eq_types(&Some(actual.clone()), &Some(expected.clone()), span);
        }
    }

    /// Emits an error to the error handler if the given type is not an address.
    pub(crate) fn assert_address_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| ADDRESS_TYPE.eq(type_), ADDRESS_TYPE.to_string(), type_, span)
    }

    /// Emits an error to the handler if the given type is not a boolean.
    pub(crate) fn assert_bool_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| BOOLEAN_TYPE.eq(type_), BOOLEAN_TYPE.to_string(), type_, span)
    }

    /// Emits an error to the handler if the given type is not a field.
    pub(crate) fn assert_field_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| FIELD_TYPE.eq(type_), FIELD_TYPE.to_string(), type_, span)
    }

    /// Emits an error to the handler if the given type is not a group.
    pub(crate) fn assert_group_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| GROUP_TYPE.eq(type_), GROUP_TYPE.to_string(), type_, span)
    }

    /// Emits an error to the handler if the given type is not a scalar.
    pub(crate) fn assert_scalar_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| SCALAR_TYPE.eq(type_), SCALAR_TYPE.to_string(), type_, span)
    }

    /// Emits an error to the handler if the given type is not a signature.
    pub(crate) fn assert_signature_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| SIGNATURE_TYPE.eq(type_), SIGNATURE_TYPE.to_string(), type_, span)
    }

    /// Emits an error to the handler if the given type is not an integer.
    pub(crate) fn assert_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| INT_TYPES.contains(type_), types_to_string(&INT_TYPES), type_, span)
    }

    /// Emits an error to the handler if the given type is not a signed integer.
    pub(crate) fn assert_signed_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| SIGNED_INT_TYPES.contains(type_),
            types_to_string(&SIGNED_INT_TYPES),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not an unsigned integer.
    pub(crate) fn assert_unsigned_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| UNSIGNED_INT_TYPES.contains(type_),
            types_to_string(&UNSIGNED_INT_TYPES),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a magnitude (u8, u16, u32).
    pub(crate) fn assert_magnitude_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_: &Type| MAGNITUDE_TYPES.contains(type_), types_to_string(&MAGNITUDE_TYPES), type_, span)
    }

    /// Emits an error to the handler if the given type is not a boolean or an integer.
    pub(crate) fn assert_bool_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| BOOLEAN_TYPE.eq(type_) | INT_TYPES.contains(type_),
            format!("{BOOLEAN_TYPE}, {}", types_to_string(&INT_TYPES)),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field or integer.
    pub(crate) fn assert_field_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| FIELD_TYPE.eq(type_) | INT_TYPES.contains(type_),
            format!("{FIELD_TYPE}, {}", types_to_string(&INT_TYPES)),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field or group.
    pub(crate) fn assert_field_group_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| FIELD_TYPE.eq(type_) | GROUP_TYPE.eq(type_),
            format!("{FIELD_TYPE}, {GROUP_TYPE}"),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field, group, or integer.
    pub(crate) fn assert_field_group_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| FIELD_TYPE.eq(type_) | GROUP_TYPE.eq(type_) | INT_TYPES.contains(type_),
            format!("{FIELD_TYPE}, {GROUP_TYPE}, {}", types_to_string(&INT_TYPES),),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field, group, or signed integer.
    pub(crate) fn assert_field_group_signed_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| FIELD_TYPE.eq(type_) | GROUP_TYPE.eq(type_) | SIGNED_INT_TYPES.contains(type_),
            format!("{FIELD_TYPE}, {GROUP_TYPE}, {}", types_to_string(&SIGNED_INT_TYPES),),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field, scalar, or integer.
    pub(crate) fn assert_field_scalar_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| FIELD_TYPE.eq(type_) | SCALAR_TYPE.eq(type_) | INT_TYPES.contains(type_),
            format!("{FIELD_TYPE}, {SCALAR_TYPE}, {}", types_to_string(&INT_TYPES),),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field, group, scalar, integer, or boolean.
    pub(crate) fn assert_field_group_scalar_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| {
                FIELD_TYPE.eq(type_) | GROUP_TYPE.eq(type_) | SCALAR_TYPE.eq(type_) | INT_TYPES.contains(type_)
            },
            format!("{}, {}, {}, {}", FIELD_TYPE, GROUP_TYPE, SCALAR_TYPE, types_to_string(&INT_TYPES),),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a field, group, scalar, integer, boolean, or address.
    pub(crate) fn assert_castable_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| {
                FIELD_TYPE.eq(type_)
                    | GROUP_TYPE.eq(type_)
                    | SCALAR_TYPE.eq(type_)
                    | INT_TYPES.contains(type_)
                    | BOOLEAN_TYPE.eq(type_)
                    | ADDRESS_TYPE.eq(type_)
            },
            format!(
                "{}, {}, {}, {}, {}, {}",
                FIELD_TYPE,
                GROUP_TYPE,
                SCALAR_TYPE,
                types_to_string(&INT_TYPES),
                BOOLEAN_TYPE,
                ADDRESS_TYPE
            ),
            type_,
            span,
        )
    }

    /// Type checks the inputs to an associated constant and returns the expected output type.
    pub(crate) fn get_core_constant(&self, type_: &Type, constant: &Identifier) -> Option<CoreConstant> {
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
    pub(crate) fn get_core_function_call(&self, struct_: &Identifier, function: &Identifier) -> Option<CoreFunction> {
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
    pub(crate) fn check_core_function_call(
        &mut self,
        core_function: CoreFunction,
        arguments: &[(Option<Type>, Span)],
        function_span: Span,
    ) -> Option<Type> {
        // Check that the number of arguments is correct.
        if arguments.len() != core_function.num_args() {
            self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                core_function.num_args(),
                arguments.len(),
                function_span,
            ));
            return None;
        }

        // Helper to check that the type of argument is not a mapping, tuple, err, or unit type.
        let check_not_mapping_tuple_err_unit = |type_: &Option<Type>, span: &Span| {
            self.check_type(
                |type_: &Type| !matches!(type_, Type::Mapping(_) | Type::Tuple(_) | Type::Err | Type::Unit),
                "address, bool, field, group, struct, integer, scalar, struct".to_string(),
                type_,
                *span,
            );
        };

        // Helper to check that the type of the argument is a valid input to a Pedersen hash/commit with 64-bit inputs.
        // The console types in snarkVM have some overhead in their bitwise representation. Consequently, Pedersen64 cannot accept a u64.
        let check_pedersen_64_bit_input = |type_: &Option<Type>, span: &Span| {
            self.check_type(
                |type_: &Type| {
                    !matches!(
                        type_,
                        Type::Integer(IntegerType::U64)
                            | Type::Integer(IntegerType::I64)
                            | Type::Integer(IntegerType::U128)
                            | Type::Integer(IntegerType::I128)
                            | Type::Mapping(_)
                            | Type::Tuple(_)
                            | Type::Err
                            | Type::Unit
                    )
                },
                "address, bool, field, group, struct, integer, scalar, struct".to_string(),
                type_,
                *span,
            );
        };

        // Helper to check that the type of the argument is a valid input to a Pedersen hash/commit with 128-bit inputs.
        // The console types in snarkVM have some overhead in their bitwise representation. Consequently, Pedersen128 cannot accept a u128.
        let check_pedersen_128_bit_input = |type_: &Option<Type>, span: &Span| {
            self.check_type(
                |type_: &Type| {
                    !matches!(
                        type_,
                        Type::Integer(IntegerType::U128)
                            | Type::Integer(IntegerType::I128)
                            | Type::Mapping(_)
                            | Type::Tuple(_)
                            | Type::Err
                            | Type::Unit
                    )
                },
                "address, bool, field, group, struct, integer, scalar, struct".to_string(),
                type_,
                *span,
            );
        };

        // Check that the arguments are of the correct type.
        match core_function {
            CoreFunction::BHP256CommitToAddress
            | CoreFunction::BHP512CommitToAddress
            | CoreFunction::BHP768CommitToAddress
            | CoreFunction::BHP1024CommitToAddress => {
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);
                Some(Type::Address)
            }
            CoreFunction::BHP256CommitToField
            | CoreFunction::BHP512CommitToField
            | CoreFunction::BHP768CommitToField
            | CoreFunction::BHP1024CommitToField => {
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);
                Some(Type::Field)
            }
            CoreFunction::BHP256CommitToGroup
            | CoreFunction::BHP512CommitToGroup
            | CoreFunction::BHP768CommitToGroup
            | CoreFunction::BHP1024CommitToGroup => {
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);
                Some(Type::Group)
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Address)
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Group)
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I8))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I16))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I32))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I64))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I128))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U8))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U16))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U32))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U64))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U128))
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
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Scalar)
            }
            CoreFunction::Pedersen64CommitToAddress => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Address)
            }
            CoreFunction::Pedersen64CommitToField => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Field)
            }
            CoreFunction::Pedersen64CommitToGroup => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Group)
            }
            CoreFunction::Pedersen64HashToAddress => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Address)
            }
            CoreFunction::Pedersen64HashToField => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::Pedersen64HashToGroup => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Group)
            }
            CoreFunction::Pedersen64HashToI8 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I8))
            }
            CoreFunction::Pedersen64HashToI16 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I16))
            }
            CoreFunction::Pedersen64HashToI32 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I32))
            }
            CoreFunction::Pedersen64HashToI64 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I64))
            }
            CoreFunction::Pedersen64HashToI128 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I128))
            }
            CoreFunction::Pedersen64HashToU8 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U8))
            }
            CoreFunction::Pedersen64HashToU16 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U16))
            }
            CoreFunction::Pedersen64HashToU32 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U32))
            }
            CoreFunction::Pedersen64HashToU64 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U64))
            }
            CoreFunction::Pedersen64HashToU128 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U128))
            }
            CoreFunction::Pedersen64HashToScalar => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 32 bits.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Scalar)
            }
            CoreFunction::Pedersen128CommitToAddress => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Address)
            }
            CoreFunction::Pedersen128CommitToField => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Field)
            }
            CoreFunction::Pedersen128CommitToGroup => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Group)
            }
            CoreFunction::Pedersen128HashToAddress => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Address)
            }
            CoreFunction::Pedersen128HashToField => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::Pedersen128HashToGroup => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Group)
            }
            CoreFunction::Pedersen128HashToI8 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I8))
            }
            CoreFunction::Pedersen128HashToI16 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I16))
            }
            CoreFunction::Pedersen128HashToI32 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I32))
            }
            CoreFunction::Pedersen128HashToI64 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I64))
            }
            CoreFunction::Pedersen128HashToI128 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::I128))
            }
            CoreFunction::Pedersen128HashToU8 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U8))
            }
            CoreFunction::Pedersen128HashToU16 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U16))
            }
            CoreFunction::Pedersen128HashToU32 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U32))
            }
            CoreFunction::Pedersen128HashToU64 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U64))
            }
            CoreFunction::Pedersen128HashToU128 => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Integer(IntegerType::U128))
            }
            CoreFunction::Pedersen128HashToScalar => {
                // Check that the first argument is not a mapping, tuple, err, unit type, or integer over 64 bits.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Scalar)
            }
            CoreFunction::MappingGet => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::get", true, function_span);
                // Check that the first argument is a mapping.
                if let Some(mapping_type) = self.assert_mapping_type(&arguments[0].0, arguments[0].1) {
                    // Check that the second argument matches the key type of the mapping.
                    self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                    // Return the value type of the mapping.
                    Some(*mapping_type.value)
                } else {
                    None
                }
            }
            CoreFunction::MappingGetOrUse => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::get_or", true, function_span);
                // Check that the first argument is a mapping.
                if let Some(mapping_type) = self.assert_mapping_type(&arguments[0].0, arguments[0].1) {
                    // Check that the second argument matches the key type of the mapping.
                    self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                    // Check that the third argument matches the value type of the mapping.
                    self.assert_type(&arguments[2].0, &mapping_type.value, arguments[2].1);
                    // Return the value type of the mapping.
                    Some(*mapping_type.value)
                } else {
                    None
                }
            }
            CoreFunction::MappingSet => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::set", true, function_span);
                // Check that the first argument is a mapping.
                if let Some(mapping_type) = self.assert_mapping_type(&arguments[0].0, arguments[0].1) {
                    // Cannot modify external mappings.
                    if mapping_type.program != self.scope_state.program_name.unwrap() {
                        self.handler.emit_err(TypeCheckerError::cannot_modify_external_mapping("set", function_span));
                    }
                    // Check that the second argument matches the key type of the mapping.
                    self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                    // Check that the third argument matches the value type of the mapping.
                    self.assert_type(&arguments[2].0, &mapping_type.value, arguments[2].1);
                    // Return the mapping type.
                    Some(Type::Unit)
                } else {
                    None
                }
            }
            CoreFunction::MappingRemove => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::remove", true, function_span);
                // Check that the first argument is a mapping.
                if let Some(mapping_type) = self.assert_mapping_type(&arguments[0].0, arguments[0].1) {
                    // Cannot modify external mappings.
                    if mapping_type.program != self.scope_state.program_name.unwrap() {
                        self.handler
                            .emit_err(TypeCheckerError::cannot_modify_external_mapping("remove", function_span));
                    }
                    // Check that the second argument matches the key type of the mapping.
                    self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                    // Return nothing.
                    Some(Type::Unit)
                } else {
                    None
                }
            }
            CoreFunction::MappingContains => {
                // Check that the operation is invoked in a `finalize` block.
                self.check_access_allowed("Mapping::contains", true, function_span);
                // Check that the first argument is a mapping.
                if let Some(mapping_type) = self.assert_mapping_type(&arguments[0].0, arguments[0].1) {
                    // Check that the second argument matches the key type of the mapping.
                    self.assert_type(&arguments[1].0, &mapping_type.key, arguments[1].1);
                    // Return a boolean.
                    Some(Type::Boolean)
                } else {
                    None
                }
            }
            CoreFunction::GroupToXCoordinate | CoreFunction::GroupToYCoordinate => {
                // Check that the first argument is a group.
                self.assert_group_type(&arguments[0].0, arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::ChaChaRandAddress => Some(Type::Address),
            CoreFunction::ChaChaRandBool => Some(Type::Boolean),
            CoreFunction::ChaChaRandField => Some(Type::Field),
            CoreFunction::ChaChaRandGroup => Some(Type::Group),
            CoreFunction::ChaChaRandI8 => Some(Type::Integer(IntegerType::I8)),
            CoreFunction::ChaChaRandI16 => Some(Type::Integer(IntegerType::I16)),
            CoreFunction::ChaChaRandI32 => Some(Type::Integer(IntegerType::I32)),
            CoreFunction::ChaChaRandI64 => Some(Type::Integer(IntegerType::I64)),
            CoreFunction::ChaChaRandI128 => Some(Type::Integer(IntegerType::I128)),
            CoreFunction::ChaChaRandScalar => Some(Type::Scalar),
            CoreFunction::ChaChaRandU8 => Some(Type::Integer(IntegerType::U8)),
            CoreFunction::ChaChaRandU16 => Some(Type::Integer(IntegerType::U16)),
            CoreFunction::ChaChaRandU32 => Some(Type::Integer(IntegerType::U32)),
            CoreFunction::ChaChaRandU64 => Some(Type::Integer(IntegerType::U64)),
            CoreFunction::ChaChaRandU128 => Some(Type::Integer(IntegerType::U128)),
            CoreFunction::SignatureVerify => {
                // Check that the first argument is a signature.
                self.assert_signature_type(&arguments[0].0, arguments[0].1);
                // Check that the second argument is an address.
                self.assert_address_type(&arguments[1].0, arguments[1].1);
                // Return a boolean.
                Some(Type::Boolean)
            }
            CoreFunction::FutureAwait => Some(Type::Unit),
        }
    }

    /// Returns the `struct` type and emits an error if the `expected` type does not match.
    pub(crate) fn check_expected_struct(&mut self, struct_: &Composite, expected: &Option<Type>, span: Span) -> Type {
        let current_struct = CompositeType { id: struct_.identifier, program: struct_.external };
        if expected.is_some() {
            self.check_eq_types(&Some(Type::Composite(current_struct)), expected, span);
        }
        Type::Composite(current_struct)
    }

    /// Emits an error if the struct member is a record type.
    pub(crate) fn assert_member_is_not_record(&mut self, span: Span, parent: Symbol, type_: &Type) {
        match type_ {
            Type::Composite(struct_)
                if self.lookup_struct(struct_.program, struct_.id.name).map_or(false, |struct_| struct_.is_record) =>
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
    pub(crate) fn assert_type_is_valid(&mut self, type_: &Type, span: Span) -> bool {
        let mut is_valid = true;
        match type_ {
            // String types are temporarily disabled.
            Type::String => {
                is_valid = false;
                self.emit_err(TypeCheckerError::strings_are_not_supported(span));
            }
            // Check that the named composite type has been defined.
            Type::Composite(struct_) if self.lookup_struct(struct_.program, struct_.id.name).is_none() => {
                is_valid = false;
                self.emit_err(TypeCheckerError::undefined_type(struct_.id.name, span));
            }
            // Check that the constituent types of the tuple are valid.
            Type::Tuple(tuple_type) => {
                for type_ in tuple_type.elements().iter() {
                    is_valid &= self.assert_type_is_valid(type_, span)
                }
            }
            // Check that the constituent types of mapping are valid.
            Type::Mapping(mapping_type) => {
                is_valid &= self.assert_type_is_valid(&mapping_type.key, span);
                is_valid &= self.assert_type_is_valid(&mapping_type.value, span);
            }
            // Check that the array element types are valid.
            Type::Array(array_type) => {
                // Check that the array length is valid.
                match array_type.length() {
                    0 => self.emit_err(TypeCheckerError::array_empty(span)),
                    length => {
                        if length > N::MAX_ARRAY_ELEMENTS {
                            self.emit_err(TypeCheckerError::array_too_large(length, N::MAX_ARRAY_ELEMENTS, span))
                        }
                    }
                }
                // Check that the array element type is valid.
                match array_type.element_type() {
                    // Array elements cannot be tuples.
                    Type::Tuple(_) => self.emit_err(TypeCheckerError::array_element_cannot_be_tuple(span)),
                    // Array elements cannot be records.
                    Type::Composite(struct_type) => {
                        // Look up the type.
                        if let Some(struct_) = self.lookup_struct(struct_type.program, struct_type.id.name) {
                            // Check that the type is not a record.
                            if struct_.is_record {
                                self.emit_err(TypeCheckerError::array_element_cannot_be_record(span));
                            }
                        }
                    }
                    _ => {} // Do nothing.
                }
                is_valid &= self.assert_type_is_valid(array_type.element_type(), span)
            }
            _ => {} // Do nothing.
        }
        is_valid
    }

    /// Emits an error if the type is not a mapping.
    pub(crate) fn assert_mapping_type(&self, type_: &Option<Type>, span: Span) -> Option<MappingType> {
        self.check_type(|type_| matches!(type_, Type::Mapping(_)), "mapping".to_string(), type_, span);
        match type_ {
            Some(Type::Mapping(mapping_type)) => Some(mapping_type.clone()),
            _ => None,
        }
    }

    /// Emits an error if the type is not an array.
    pub(crate) fn assert_array_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(|type_| matches!(type_, Type::Array(_)), "array".to_string(), type_, span);
    }

    /// Helper function to check that the input and output of function are valid
    pub(crate) fn check_function_signature(&mut self, function: &Function) {
        self.scope_state.variant = Some(function.variant);

        // Special type checking for finalize blocks. Can skip for stubs.
        if self.scope_state.variant == Some(Variant::AsyncFunction) && !self.scope_state.is_stub {
            // Finalize functions are not allowed to return values.
            if !function.output.is_empty() {
                self.emit_err(TypeCheckerError::async_function_cannot_return_value(function.span()));
            }

            // Check that the input types are consistent with when the function is invoked.
            if let Some(inferred_input_types) = self.async_function_input_types.get(&self.scope_state.location()) {
                // Check same number of inputs as expected.
                if inferred_input_types.len() != function.input.len() {
                    return self.emit_err(TypeCheckerError::async_function_input_length_mismatch(
                        inferred_input_types.len(),
                        function.input.len(),
                        function.span(),
                    ));
                }
                // Check that the input parameters match the inferred types from when the async function is invoked.
                for (t1, t2) in function.input.iter().zip_eq(inferred_input_types.iter()) {
                    // Allow partial type matching of futures since inferred are fully typed, whereas AST has default futures.
                    if matches!(t2, Type::Future(_)) && matches!(t1.type_(), Type::Future(_)) {
                        // Insert to symbol table
                        if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                            Location::new(None, t1.identifier().name),
                            VariableSymbol {
                                type_: t2.clone(),
                                span: t1.identifier.span(),
                                declaration: VariableType::Input(Mode::Public),
                            },
                        ) {
                            self.handler.emit_err(err);
                        }
                    }
                    self.check_eq_types(&Some(t1.type_().clone()), &Some(t2.clone()), t1.span())
                }
            } else {
                self.emit_warning(TypeCheckerWarning::async_function_is_never_called_by_transition_function(
                    function.identifier.name,
                    function.span(),
                ));
            }
        }

        // Type check the function's parameters.
        function.input.iter().enumerate().for_each(|(_index, input_var)| {
            // Check that the type of input parameter is defined.
            self.assert_type_is_valid(input_var.type_(), input_var.span());
            // Check that the type of the input parameter is not a tuple.
            if matches!(input_var.type_(), Type::Tuple(_)) {
                self.emit_err(TypeCheckerError::function_cannot_take_tuple_as_input(input_var.span()))
            }
            // Make sure only transitions can take a record as an input.
            else if let Type::Composite(struct_) = input_var.type_() {
                // Throw error for undefined type.
                if !function.variant.is_transition() {
                    if let Some(elem) = self.lookup_struct(struct_.program, struct_.id.name) {
                        if elem.is_record {
                            self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(input_var.span()))
                        }
                    } else {
                        self.emit_err(TypeCheckerError::undefined_type(struct_.id, input_var.span()));
                    }
                }
            }

            // Note that this unwrap is safe since we assign to `self.variant` above.
            match self.scope_state.variant.unwrap() {
                // If the function is a transition function, then check that the parameter mode is not a constant.
                Variant::Transition | Variant::AsyncTransition if input_var.mode() == Mode::Constant => {
                    self.emit_err(TypeCheckerError::transition_function_inputs_cannot_be_const(input_var.span()))
                }
                // If the function is standard function or inline, then check that the parameters do not have an associated mode.
                Variant::Function | Variant::Inline if input_var.mode() != Mode::None => {
                    self.emit_err(TypeCheckerError::regular_function_inputs_cannot_have_modes(input_var.span()))
                }
                // If the function is an async function, then check that the input parameter is not constant or private.
                Variant::AsyncFunction if input_var.mode() == Mode::Constant || input_var.mode() == Mode::Private => {
                    self.emit_err(TypeCheckerError::async_function_input_must_be_public(input_var.span()));
                }
                _ => {} // Do nothing.
            }

            // If the function is not a transition function, then it cannot have a record as input
            if let Type::Composite(struct_) = input_var.type_() {
                if let Some(val) = self.lookup_struct(struct_.program, struct_.id.name) {
                    if val.is_record && !function.variant.is_transition() {
                        self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(input_var.span()));
                    }
                }
            }

            // Add function inputs to the symbol table. Futures have already been added.
            if !matches!(&input_var.type_(), &Type::Future(_)) {
                if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                    Location::new(None, input_var.identifier().name),
                    VariableSymbol {
                        type_: input_var.type_().clone(),
                        span: input_var.identifier().span(),
                        declaration: VariableType::Input(input_var.mode()),
                    },
                ) {
                    self.handler.emit_err(err);
                }
            }
        });

        // Type check the function's return type.
        // Note that checking that each of the component types are defined is sufficient to check that `output_type` is defined.
        function.output.iter().enumerate().for_each(|(index, function_output)| {
            // If the function is not a transition function, then it cannot output a record.
            // Note that an external output must always be a record.
            if let Type::Composite(struct_) = function_output.type_.clone() {
                if let Some(val) = self.lookup_struct(struct_.program, struct_.id.name) {
                    if val.is_record && !function.variant.is_transition() {
                        self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(function_output.span));
                    }
                }
            }
            // Check that the type of output is defined.
            if self.assert_type_is_valid(&function_output.type_, function_output.span) {
                // If the function is not a transition function, then it cannot output a record.
                if let Type::Composite(struct_) = function_output.type_.clone() {
                    if !function.variant.is_transition()
                        && self.lookup_struct(struct_.program, struct_.id.name).unwrap().is_record
                    {
                        self.emit_err(TypeCheckerError::function_cannot_input_or_output_a_record(function_output.span));
                    }
                }
            }
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
            if self.scope_state.variant != Some(Variant::AsyncTransition)
                && matches!(function_output.type_, Type::Future(_))
            {
                self.emit_err(TypeCheckerError::only_async_transition_can_return_future(function_output.span));
            }
        });
    }

    /// Wrapper around lookup_struct that additionally records all structs that are used in the program.
    pub(crate) fn lookup_struct(&mut self, program: Option<Symbol>, name: Symbol) -> Option<Composite> {
        let struct_ = self
            .symbol_table
            .borrow()
            .lookup_struct(Location::new(program, name), self.scope_state.program_name)
            .cloned();
        // Record the usage.
        if let Some(s) = &struct_ {
            self.used_structs.insert(s.identifier.name);
        }
        struct_
    }

    /// Type checks the awaiting of a future.
    pub(crate) fn assert_future_await(&mut self, future: &Option<&Expression>, span: Span) {
        // Make sure that it is an identifier expression.
        let future_variable = match future {
            Some(Expression::Identifier(name)) => name,
            _ => {
                return self.emit_err(TypeCheckerError::invalid_await_call(span));
            }
        };

        // Make sure that the future is defined.
        match self.symbol_table.borrow().lookup_variable(Location::new(None, future_variable.name)) {
            Some(var) => {
                if !matches!(&var.type_, &Type::Future(_)) {
                    self.emit_err(TypeCheckerError::expected_future(future_variable.name, future_variable.span()));
                }
                // Mark the future as consumed.
                self.await_checker.remove(future_variable);
            }
            None => {
                self.emit_err(TypeCheckerError::expected_future(future_variable.name, future_variable.span()));
            }
        }
    }

    /// Inserts variable to symbol table.
    pub(crate) fn insert_variable(
        &mut self,
        inferred_type: Option<Type>,
        name: &Identifier,
        type_: Type,
        index: usize,
        span: Span,
    ) {
        let ty: Type = if let Type::Future(_) = type_ {
            // Need to insert the fully inferred future type, or else will just be default future type.
            let ret = match inferred_type.unwrap() {
                Type::Future(future) => Type::Future(future),
                Type::Tuple(tuple) => match tuple.elements().get(index) {
                    Some(Type::Future(future)) => Type::Future(future.clone()),
                    _ => unreachable!("Parsing guarantees that the inferred type is a future."),
                },
                _ => {
                    unreachable!("TYC guarantees that the inferred type is a future, or tuple containing futures.")
                }
            };
            // Insert future into list of futures for the function.
            self.scope_state.futures.insert(name.name, self.scope_state.call_location.clone().unwrap());
            ret
        } else {
            type_
        };
        // Insert the variable into the symbol table.
        if let Err(err) =
            self.symbol_table.borrow_mut().insert_variable(Location::new(None, name.name), VariableSymbol {
                type_: ty,
                span,
                declaration: VariableType::Mut,
            })
        {
            self.handler.emit_err(err);
        }
    }

    // Checks if the access operation is valid inside the current function variant.
    pub(crate) fn check_access_allowed(&mut self, name: &str, finalize_op: bool, span: Span) {
        // Check that the function context matches.
        if self.scope_state.variant == Some(Variant::AsyncFunction) && !finalize_op {
            self.handler.emit_err(TypeCheckerError::invalid_operation_inside_finalize(name, span))
        } else if self.scope_state.variant != Some(Variant::AsyncFunction) && finalize_op {
            self.handler.emit_err(TypeCheckerError::invalid_operation_outside_finalize(name, span))
        }
    }
}

fn types_to_string(types: &[Type]) -> String {
    types.iter().map(|type_| type_.to_string()).join(", ")
}
