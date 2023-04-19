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

use crate::{CallGraph, StructGraph, SymbolTable};

use leo_ast::{CoreFunction, Identifier, IntegerType, MappingType, Node, Type, Variant};
use leo_errors::{emitter::Handler, TypeCheckerError};
use leo_span::{Span, Symbol};

use itertools::Itertools;
use std::cell::RefCell;

pub struct TypeChecker<'a> {
    /// The symbol table for the program.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// A dependency graph of the structs in program.
    pub(crate) struct_graph: StructGraph,
    /// The call graph for the program.
    pub(crate) call_graph: CallGraph,
    /// The error handler.
    pub(crate) handler: &'a Handler,
    /// The name of the function that we are currently traversing.
    pub(crate) function: Option<Symbol>,
    /// The variant of the function that we are currently traversing.
    pub(crate) variant: Option<Variant>,
    /// Whether or not the function that we are currently traversing has a return statement.
    pub(crate) has_return: bool,
    /// Whether or not the function that we are currently traversing invokes the finalize block.
    pub(crate) has_finalize: bool,

    /// Whether or not we are currently traversing a finalize block.
    pub(crate) is_finalize: bool,
    /// Whether or not we are currently traversing an imported program.
    pub(crate) is_imported: bool,
    /// Whether or not we are currently traversing a return statement.
    pub(crate) is_return: bool,
}

const BOOLEAN_TYPE: Type = Type::Boolean;

const FIELD_TYPE: Type = Type::Field;

const GROUP_TYPE: Type = Type::Group;

const SCALAR_TYPE: Type = Type::Scalar;

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

impl<'a> TypeChecker<'a> {
    /// Returns a new type checker given a symbol table and error handler.
    pub fn new(symbol_table: SymbolTable, handler: &'a Handler) -> Self {
        let struct_names = symbol_table.structs.keys().cloned().collect();

        let function_names = symbol_table.functions.keys().cloned().collect();

        // Note that the `struct_graph` and `call_graph` are initialized with their full node sets.
        Self {
            symbol_table: RefCell::new(symbol_table),
            struct_graph: StructGraph::new(struct_names),
            call_graph: CallGraph::new(function_names),
            handler,
            function: None,
            variant: None,
            has_return: false,
            has_finalize: false,
            is_finalize: false,
            is_imported: false,
            is_return: false,
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
            (Some(t1), Some(t2)) if !Type::eq_flat(t1, t2) => {
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
    pub(crate) fn assert_and_return_type(&self, actual: Type, expected: &Option<Type>, span: Span) -> Type {
        if let Some(expected) = expected {
            if !actual.eq_flat(expected) {
                self.emit_err(TypeCheckerError::type_should_be(actual.clone(), expected, span));
            }
        }

        actual
    }

    /// Emits an error to the error handler if the `actual` type is not equal to the `expected` type.
    pub(crate) fn assert_type(&self, actual: &Option<Type>, expected: &Type, span: Span) {
        self.check_type(|actual: &Type| actual.eq_flat(expected), expected.to_string(), actual, span)
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

    /// Emits an error to the handler if the given type is not a field, group, scalar or integer.
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

    /// Emits an error if the `struct` is not a core library struct.
    /// Emits an error if the `function` is not supported by the struct.
    pub(crate) fn get_core_function_call(&self, struct_: &Type, function: &Identifier) -> Option<CoreFunction> {
        if let Type::Identifier(ident) = struct_ {
            // Lookup core struct
            match CoreFunction::from_symbols(ident.name, function.name) {
                None => {
                    // Not a core library struct.
                    self.emit_err(TypeCheckerError::invalid_core_function(ident.name, function.name, ident.span()));
                }
                Some(core_instruction) => return Some(core_instruction),
            }
        }
        None
    }

    /// Type checks the inputs to a core function call and returns the expected output type.
    /// Emits an error if the correct number of arguments are not provided.
    /// Emits an error if the arguments are not of the correct type.
    pub(crate) fn check_core_function_call(
        &self,
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
                "address, boolean, field, group, struct, integer, scalar, scalar, string".to_string(),
                type_,
                *span,
            );
        };

        // Helper to check that the type of the argument is a valid input to a Pedersen hash/commit with 64-bit inputs.
        let check_pedersen_64_bit_input = |type_: &Option<Type>, span: &Span| {
            self.check_type(
                |type_: &Type| {
                    matches!(
                        type_,
                        Type::Boolean
                            | Type::Integer(IntegerType::I8)
                            | Type::Integer(IntegerType::I16)
                            | Type::Integer(IntegerType::I32)
                            | Type::Integer(IntegerType::I64)
                            | Type::Integer(IntegerType::U8)
                            | Type::Integer(IntegerType::U16)
                            | Type::Integer(IntegerType::U32)
                            | Type::Integer(IntegerType::U64)
                            | Type::String
                    )
                },
                "boolean, integer (up to 64 bits), string".to_string(),
                type_,
                *span,
            );
        };

        // Helper to check that the type of the argument is a valid input to a Pedersen hash/commit with 128-bit inputs.
        let check_pedersen_128_bit_input = |type_: &Option<Type>, span: &Span| {
            self.check_type(
                |type_: &Type| matches!(type_, Type::Boolean | Type::Integer(_) | Type::String),
                "boolean, integer, string".to_string(),
                type_,
                *span,
            );
        };

        // Check that the arguments are of the correct type.
        match core_function {
            CoreFunction::BHP256Commit
            | CoreFunction::BHP512Commit
            | CoreFunction::BHP768Commit
            | CoreFunction::BHP1024Commit => {
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);
                Some(Type::Field)
            }
            CoreFunction::BHP256Hash
            | CoreFunction::BHP512Hash
            | CoreFunction::BHP768Hash
            | CoreFunction::BHP1024Hash => {
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::Pedersen64Commit => {
                // Check that the first argument is either a boolean, integer up to 64 bits, or field element.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Group)
            }
            CoreFunction::Pedersen64Hash => {
                // Check that the first argument is either a boolean, integer up to 64 bits, or field element.
                check_pedersen_64_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::Pedersen128Commit => {
                // Check that the first argument is either a boolean, integer, or field element.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                // Check that the second argument is a scalar.
                self.assert_scalar_type(&arguments[1].0, arguments[1].1);

                Some(Type::Group)
            }
            CoreFunction::Pedersen128Hash => {
                // Check that the first argument is either a boolean, integer, or field element.
                check_pedersen_128_bit_input(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::Poseidon2Hash | CoreFunction::Poseidon4Hash | CoreFunction::Poseidon8Hash => {
                // Check that the first argument is not a mapping, tuple, err, or unit type.
                check_not_mapping_tuple_err_unit(&arguments[0].0, &arguments[0].1);
                Some(Type::Field)
            }
            CoreFunction::MappingGet => {
                // Check that the operation is invoked in a `finalize` block.
                if !self.is_finalize {
                    self.handler
                        .emit_err(TypeCheckerError::invalid_operation_outside_finalize("Mapping::get", function_span))
                }
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
            CoreFunction::MappingGetOrInit => {
                // Check that the operation is invoked in a `finalize` block.
                if !self.is_finalize {
                    self.handler.emit_err(TypeCheckerError::invalid_operation_outside_finalize(
                        "Mapping::get_or",
                        function_span,
                    ))
                }
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
                if !self.is_finalize {
                    self.handler
                        .emit_err(TypeCheckerError::invalid_operation_outside_finalize("Mapping::set", function_span))
                }
                // Check that the first argument is a mapping.
                if let Some(mapping_type) = self.assert_mapping_type(&arguments[0].0, arguments[0].1) {
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
        }
    }

    /// Returns the `struct` type and emits an error if the `expected` type does not match.
    pub(crate) fn check_expected_struct(&mut self, struct_: Identifier, expected: &Option<Type>, span: Span) -> Type {
        if let Some(expected) = expected {
            if !Type::Identifier(struct_).eq_flat(expected) {
                self.emit_err(TypeCheckerError::type_should_be(struct_.name, expected, span));
            }
        }
        Type::Identifier(struct_)
    }

    /// Emits an error if the struct member is a record type.
    pub(crate) fn assert_member_is_not_record(&self, span: Span, parent: Symbol, type_: &Type) {
        match type_ {
            Type::Identifier(identifier)
                if self
                    .symbol_table
                    .borrow()
                    .lookup_struct(identifier.name)
                    .map_or(false, |struct_| struct_.is_record) =>
            {
                self.emit_err(TypeCheckerError::struct_or_record_cannot_contain_record(parent, identifier.name, span))
            }
            Type::Tuple(tuple_type) => {
                for type_ in tuple_type.iter() {
                    self.assert_member_is_not_record(span, parent, type_)
                }
            }
            _ => {} // Do nothing.
        }
    }

    /// Emits an error if the type or its constituent types are not defined.
    pub(crate) fn assert_type_is_defined(&self, type_: &Type, span: Span) -> bool {
        let mut is_defined = true;
        match type_ {
            // String types are temporarily disabled.
            Type::String => {
                is_defined = false;
                self.emit_err(TypeCheckerError::strings_are_not_supported(span));
            }
            // Check that the named composite type has been defined.
            Type::Identifier(identifier) if self.symbol_table.borrow().lookup_struct(identifier.name).is_none() => {
                is_defined = false;
                self.emit_err(TypeCheckerError::undefined_type(identifier.name, span));
            }
            // Check that the constituent types of the tuple are valid.
            Type::Tuple(tuple_type) => {
                for type_ in tuple_type.iter() {
                    is_defined &= self.assert_type_is_defined(type_, span)
                }
            }
            // Check that the constituent types of mapping are valid.
            Type::Mapping(mapping_type) => {
                is_defined &= self.assert_type_is_defined(&mapping_type.key, span);
                is_defined &= self.assert_type_is_defined(&mapping_type.value, span);
            }
            _ => {} // Do nothing.
        }
        is_defined
    }

    /// Emits an error if the type is not a mapping.
    pub(crate) fn assert_mapping_type(&self, type_: &Option<Type>, span: Span) -> Option<MappingType> {
        self.check_type(|type_| matches!(type_, Type::Mapping(_)), "mapping".to_string(), type_, span);
        match type_ {
            Some(Type::Mapping(mapping_type)) => Some(mapping_type.clone()),
            _ => None,
        }
    }
}

fn types_to_string(types: &[Type]) -> String {
    types.iter().map(|type_| type_.to_string()).join(", ")
}
