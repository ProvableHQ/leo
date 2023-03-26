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

use crate::{CallGraph, StructGraph, SymbolTable, VariableSymbol, VariableType};

use leo_ast::{Expression, ExpressionVisitor, Identifier, IntegerType, Node, Type, Variant};
use leo_core::*;
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
    /// Whether or not we are the top level of a function body.
    /// This is used to check that assembly blocks are not nested.
    pub(crate) is_top_level: bool,
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

const MAGNITUDE_TYPES: [Type; 3] = [
    Type::Integer(IntegerType::U8),
    Type::Integer(IntegerType::U16),
    Type::Integer(IntegerType::U32),
];

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
            is_top_level: true,
        }
    }

    /// Enters a child scope.
    pub(crate) fn enter_scope(&mut self, index: usize) {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(previous_symbol_table.borrow().lookup_scope_by_index(index).unwrap());
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
        self.symbol_table
            .swap(previous_symbol_table.lookup_scope_by_index(index).unwrap());
        self.symbol_table = RefCell::new(previous_symbol_table);
    }

    /// Emits a type checker error.
    pub(crate) fn emit_err(&self, err: TypeCheckerError) {
        self.handler.emit_err(err);
    }

    /// Helper to insert the variables into the symbol table.
    pub(crate) fn insert_variable(
        &mut self,
        symbol: Symbol,
        type_: Type,
        span: Span,
        declaration: VariableType,
    ) -> bool {
        if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
            symbol,
            VariableSymbol {
                type_,
                span,
                declaration,
            },
        ) {
            self.handler.emit_err(err);
            false
        } else {
            true
        }
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
    /// Returns `true` if the types are equal, and `false` otherwise.
    pub(crate) fn check_eq_types(&self, t1: &Option<Type>, t2: &Option<Type>, span: Span) -> bool {
        match (t1, t2) {
            (Some(t1), Some(t2)) if !Type::eq_flat(t1, t2) => {
                self.emit_err(TypeCheckerError::type_should_be(t1, t2, span));
                false
            }
            (Some(type_), None) | (None, Some(type_)) => {
                self.emit_err(TypeCheckerError::type_should_be("no type", type_, span));
                false
            }
            _ => true,
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
        self.check_type(
            |actual: &Type| actual.eq_flat(expected),
            expected.to_string(),
            actual,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not a boolean.
    pub(crate) fn assert_bool_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| BOOLEAN_TYPE.eq(type_),
            BOOLEAN_TYPE.to_string(),
            type_,
            span,
        )
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
        self.check_type(
            |type_: &Type| SCALAR_TYPE.eq(type_),
            SCALAR_TYPE.to_string(),
            type_,
            span,
        )
    }

    /// Emits an error to the handler if the given type is not an integer.
    pub(crate) fn assert_int_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_: &Type| INT_TYPES.contains(type_),
            types_to_string(&INT_TYPES),
            type_,
            span,
        )
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
        self.check_type(
            |type_: &Type| MAGNITUDE_TYPES.contains(type_),
            types_to_string(&MAGNITUDE_TYPES),
            type_,
            span,
        )
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
            format!(
                "{}, {}, {}, {}",
                FIELD_TYPE,
                GROUP_TYPE,
                SCALAR_TYPE,
                types_to_string(&INT_TYPES),
            ),
            type_,
            span,
        )
    }

    /// Emits an error if the `struct` is not a core library struct.
    /// Emits an error if the `function` is not supported by the struct.
    pub(crate) fn check_core_function_call(&self, struct_: &Type, function: &Identifier) -> Option<CoreInstruction> {
        if let Type::Identifier(ident) = struct_ {
            // Lookup core struct
            match CoreInstruction::from_symbols(ident.name, function.name) {
                None => {
                    // Not a core library struct.
                    self.emit_err(TypeCheckerError::invalid_core_function(
                        ident.name,
                        function.name,
                        ident.span(),
                    ));
                }
                Some(core_instruction) => return Some(core_instruction),
            }
        }
        None
    }

    /// Returns the `struct` type and emits an error if the `expected` type does not match.
    pub(crate) fn check_expected_struct(&mut self, struct_: Identifier, expected: &Option<Type>, span: Span) -> Type {
        if let Some(Type::Identifier(expected)) = expected {
            if !struct_.matches(expected) {
                self.emit_err(TypeCheckerError::type_should_be(struct_.name, expected.name, span));
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
                self.emit_err(TypeCheckerError::struct_or_record_cannot_contain_record(
                    parent,
                    identifier.name,
                    span,
                ))
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
    pub(crate) fn assert_type_is_defined(&self, type_: &Type, span: Span) {
        match type_ {
            // String types are temporarily disabled.
            Type::String => {
                self.emit_err(TypeCheckerError::strings_are_not_supported(span));
            }
            // Check that the named composite type has been defined.
            Type::Identifier(identifier) if self.symbol_table.borrow().lookup_struct(identifier.name).is_none() => {
                self.emit_err(TypeCheckerError::undefined_type(identifier.name, span));
            }
            // Check that the constituent types of the tuple are valid.
            Type::Tuple(tuple_type) => {
                for type_ in tuple_type.iter() {
                    self.assert_type_is_defined(type_, span)
                }
            }
            // Check that the constituent types of mapping are valid.
            Type::Mapping(mapping_type) => {
                self.assert_type_is_defined(&mapping_type.key, span);
                self.assert_type_is_defined(&mapping_type.value, span);
            }
            _ => {} // Do nothing.
        }
    }

    /// Emits an error if the type is not a mapping.
    pub(crate) fn assert_mapping_type(&self, type_: &Option<Type>, span: Span) {
        self.check_type(
            |type_| matches!(type_, Type::Mapping(_)),
            "mapping".to_string(),
            type_,
            span,
        )
    }

    // A helper to type check accesses to a mapping.
    pub(crate) fn check_mapping_access(
        &mut self,
        mapping: &'a Identifier,
        key: &'a Expression,
        value: &'a Expression,
        span: Span,
    ) {
        if !self.is_finalize {
            self.emit_err(TypeCheckerError::increment_or_decrement_outside_finalize(span));
        }

        // Assert that the first operand is a mapping.
        let mapping_type = self.visit_identifier(&mapping, &None);
        self.assert_mapping_type(&mapping_type, span);

        match mapping_type {
            None => self.emit_err(TypeCheckerError::could_not_determine_type(mapping, mapping.span)),
            Some(Type::Mapping(mapping_type)) => {
                // Check that the key matches the key type of the mapping.
                let key_type = self.visit_expression(key, &None);
                self.assert_type(&key_type, &mapping_type.key, key.span());

                // Check that the value matches the value type of the mapping.
                let value_type = self.visit_expression(value, &None);
                self.assert_type(&value_type, &mapping_type.value, value.span());

                // Check that the amount type is incrementable.
                self.assert_field_group_scalar_int_type(&value_type, value.span());
            }
            Some(mapping_type) => self.emit_err(TypeCheckerError::expected_one_type_of(
                "mapping",
                mapping_type,
                mapping.span,
            )),
        }
    }

    // A helper to check function calls.
    pub(crate) fn check_function_call(
        &mut self,
        function: &'a Expression,
        arguments: &'a [Expression],
        is_external: bool,
        expected: &Option<Type>,
        span: Span,
    ) -> Option<Type> {
        match function {
            // Note that the parser guarantees that `input.function` is always an identifier.
            Expression::Identifier(ident) => {
                // Note: The function symbol lookup is performed outside of the `if let Some(func) ...` block to avoid a RefCell lifetime bug in Rust.
                // Do not move it into the `if let Some(func) ...` block or it will keep `self.symbol_table_creation` alive for the entire block and will be very memory inefficient!
                let func = self.symbol_table.borrow().lookup_fn_symbol(ident.name).cloned();

                if let Some(func) = func {
                    // Check that the call is valid.
                    // Note that this unwrap is safe since we always set the variant before traversing the body of the function.
                    match self.variant.unwrap() {
                        // If the function is not a transition function, it can only call "inline" functions.
                        Variant::Inline | Variant::Standard => {
                            if !matches!(func.variant, Variant::Inline) {
                                self.emit_err(TypeCheckerError::can_only_call_inline_function(span));
                            }
                        }
                        // If the function is a transition function, then check that the call is not to another local transition function.
                        Variant::Transition => {
                            if matches!(func.variant, Variant::Transition) && !is_external {
                                self.emit_err(TypeCheckerError::cannot_invoke_call_to_local_transition_function(span));
                            }
                        }
                    }

                    // Check that the call is not to an external `inline` function.
                    if func.variant == Variant::Inline && is_external {
                        self.emit_err(TypeCheckerError::cannot_call_external_inline_function(span));
                    }

                    let ret = self.assert_and_return_type(func.output_type, expected, span);

                    // Check number of function arguments.
                    if func.input.len() != arguments.len() {
                        self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                            func.input.len(),
                            arguments.len(),
                            span,
                        ));
                    }

                    // Check function argument types.
                    func.input
                        .iter()
                        .zip(arguments.iter())
                        .for_each(|(expected, argument)| {
                            self.visit_expression(argument, &Some(expected.type_()));
                        });

                    // Add the call to the call graph.
                    let caller_name = match self.function {
                        None => unreachable!("`self.function` is set every time a function is visited."),
                        Some(func) => func,
                    };
                    self.call_graph.add_edge(caller_name, ident.name);

                    Some(ret)
                } else {
                    self.emit_err(TypeCheckerError::unknown_sym("function", ident.name, ident.span()));
                    None
                }
            }
            _ => unreachable!("Parser guarantees that `input.function` is always an identifier."),
        }
    }
}

fn types_to_string(types: &[Type]) -> String {
    types.iter().map(|type_| type_.to_string()).join(", ")
}
