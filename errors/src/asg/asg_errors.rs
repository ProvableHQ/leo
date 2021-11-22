// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::create_errors;

use std::fmt::{Debug, Display};

create_errors!(
    /// AsgError enum that represents all the errors for the `leo-asg` crate.
    AsgError,
    exit_code_mask: 3000i32,
    error_code_prefix: "ASG",

    /// For when a struct of the specified type is unresolved.
    /// Note that the type for a struct is represented by a name.
    @formatted
    unresolved_struct {
        args: (name: impl Display),
        msg: format!("failed to resolve struct: '{}'", name),
        help: None,
    }

     /// For when a struct member of the specified name is unresolved.
    @formatted
    unresolved_struct_member {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "illegal reference to non-existant member '{}' of struct '{}'",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user is initializing a struct, and it's missing struct member.
    @formatted
    missing_struct_member {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "missing struct member '{}' for initialization of struct '{}'",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user is initializing a struct, and they declare a cirucit member twice.
    @formatted
    overridden_struct_member {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "cannot declare struct member '{}' more than once for initialization of struct '{}'",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user is defining a struct, and they define a struct member multiple times.
    @formatted
    redefined_struct_member {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "cannot declare struct member '{}' multiple times in struct '{}'",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user is initializing a struct, and they add an extra struct member.
    @formatted
    extra_struct_member {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "extra struct member '{}' for initialization of struct '{}' is not allowed",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user attempts to assign to a function.
    @formatted
    illegal_function_assign {
        args: (name: impl Display),
        msg: format!("attempt to assign to function '{}'", name),
        help: None,
    }

    /// For when a user tries to call a struct variable as a function.
    @formatted
    struct_variable_call {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!("cannot call variable member '{}' of struct '{}'", name, struct_name),
        help: None,
    }

    /// For when a user tries to call an invalid struct static function.
    @formatted
    struct_static_call_invalid {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "cannot call static function '{}' of struct '{}' from target",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user tries to call a mutable struct member function from immutable context.
    @formatted
    struct_member_mut_call_invalid {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "cannot call mutable member function '{}' of struct '{}' from immutable context",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user tries to call a struct member function from static context.
    @formatted
    struct_member_call_invalid {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!(
            "cannot call member function '{}' of struct '{}' from static context",
            name, struct_name
        ),
        help: None,
    }

    /// For when a user tries to index into a non-array type.
    @formatted
    index_into_non_array {
        args: (name: impl Display),
        msg: format!("failed to index into non-array '{}'", name),
        help: None,
    }

    /// For when a user tries index with an invalid integer.
    @formatted
    invalid_assign_index {
        args: (name: impl Display, num: impl Display),
        msg: format!("failed to index array with invalid integer '{}'[{}]", name, num),
        help: None,
    }

    /// For when a user tries to index an array range, with a left value greater than right value.
    @formatted
    invalid_backwards_assignment {
        args: (name: impl Display, left: impl Display, right: impl Display),
        msg: format!(
            "failed to index array range for assignment with left > right '{}'[{}..{}]",
            name, left, right
        ),
        help: None,
    }

    /// For when a user tries to create a constant varaible from non constant values.
    @formatted
    invalid_const_assign {
        args: (name: impl Display),
        msg: format!(
            "failed to create const variable(s) '{}' with non constant values.",
            name
        ),
        help: None,
    }

    /// For when a user defines a function with the same name twice.
    @formatted
    duplicate_function_definition {
        args: (name: impl Display),
        msg: format!("a function named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user defines variable with the same name twice.
    @formatted
    duplicate_variable_definition {
        args: (name: impl Display),
        msg: format!("a variable named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user tries to index into a non-tuple type.
    @formatted
    index_into_non_tuple {
        args: (name: impl Display),
        msg: format!("failed to index into non-tuple '{}'", name),
        help: None,
    }

    /// For when a user tries access a tuple index out of bounds.
    @formatted
    tuple_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("tuple index out of bounds: '{}'", index),
        help: None,
    }

    /// For when a user tries access a array index out of bounds.
    @formatted
    array_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("array index out of bounds: '{}'", index),
        help: None,
    }

    /// For when a user tries have either side of a ternary return different variable types.
    @formatted
    ternary_different_types {
        args: (left: impl Display, right: impl Display),
        msg: format!("ternary sides had different types: left {}, right {}", left, right),
        help: None,
    }

    /// For when an array size cannot be inferred.
    @formatted
    unknown_array_size {
        args: (),
        msg: "array size cannot be inferred, add explicit types",
        help: None,
    }

    /// For when a user passes more arguements to a function than expected.
    @formatted
    unexpected_call_argument_count {
        args: (expected: impl Display, got: impl Display),
        msg: format!("function call expected {} arguments, got {}", expected, got),
        help: None,
    }

    /// For whan a function is unresolved.
    @formatted
    unresolved_function {
        args: (name: impl Display),
        msg: format!("failed to resolve function: '{}'", name),
        help: None,
    }

    /// For when a type cannot be resolved.
    @formatted
    unresolved_type {
        args: (name: impl Display),
        msg: format!("failed to resolve type for variable definition '{}'", name),
        help: None,
    }

    /// For when a user passes a type, but another was expected.
    @formatted
    unexpected_type {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "unexpected type, expected: '{}', received: '{}'",
            expected,
            received,
        ),
        help: None,
    }

    /// For when a constant value was expected, but a non-constant one was received.
    @formatted
    unexpected_nonconst {
        args: (),
        msg: "expected const, found non-const value",
        help: None,
    }

    /// For whan a variable is unresolved.
    @formatted
    unresolved_reference {
        args: (name: impl Display),
        msg: format!("failed to resolve variable reference '{}'", name),
        help: None,
    }

    /// For when a boolean value cannot be parsed.
    @formatted
    invalid_boolean {
        args: (value: impl Display),
        msg: format!("failed to parse boolean value '{}'", value),
        help: None,
    }

    /// For when a char value cannot be parsed.
    @formatted
    invalid_char {
        args: (value: impl Display),
        msg: format!("failed to parse char value '{}'", value),
        help: None,
    }

    /// For when an int value cannot be parsed.
    @formatted
    invalid_int {
        args: (value: impl Display),
        msg: format!("failed to parse int value '{}'", value),
        help: None,
    }

    /// For when a user tries to negate an unsigned integer.
    @formatted
    unsigned_negation {
        args: (),
        msg: "cannot negate unsigned integer",
        help: None,
    }

    /// For when a user tries to assign to an immutable variable.
    @formatted
    immutable_assignment {
        args: (name: impl Display),
        msg: format!("illegal assignment to immutable variable '{}'", name),
        help: None,
    }

    /// For when a function is missing a return statement.
    @formatted
    function_missing_return {
        args: (name: impl Display),
        msg: format!("function '{}' missing return for all paths", name),
        help: None,
    }

    /// For when a function fails to resolve the correct return.
    @formatted
    function_return_validation {
        args: (name: impl Display, description: impl Display),
        msg: format!("function '{}' failed to validate return path: '{}'", name, description),
        help: None,
    }

    /// For when the type for an input variable could not be infered.
    @formatted
    input_ref_needs_type {
        args: (category: impl Display, name: impl Display),
        msg: format!("could not infer type for input in '{}': '{}'", category, name),
        help: None,
    }

    /// For when a user tries to call a test function.
    @formatted
    call_test_function {
        args: (),
        msg: "cannot call test function",
        help: None,
    }

    /// For when a user tries to define a struct function as a test function.
    @formatted
    struct_test_function {
        args: (),
        msg: "cannot have test function as member of struct",
        help: None,
    }

    /// Failed to parse index.
    @formatted
    parse_index_error {
        args: (),
        msg: "failed to parse index",
        help: None,
    }

    /// Failed to parse array dimensions.
    @formatted
    parse_dimension_error {
        args: (),
        msg: "failed to parse dimension",
        help: None,
    }

    /// For when there is an illegal ast structure.
    @formatted
    illegal_ast_structure {
        args: (details: impl Display),
        msg: format!("illegal ast structure: {}", details),
        help: None,
    }

    /// For when a user tries to reference an input varaible but none is in scope.
    @formatted
    illegal_input_variable_reference {
        args: (),
        msg:  "attempted to reference input when none is in scope",
        help: None,
    }

    /// For the ASG receives an big Self, which should never happen
    /// as they should be resolved in an earlier compiler phase.
    @formatted
    unexpected_big_self {
        args: (),
        msg:  "received a Self statement, which should never happen.",
        help: Some("Something went wrong during canonicalization, or you ran the ASG on an uncanonicalized AST.".to_string()),
    }

    /// For when a import of the specified name is unresolved.
    @formatted
    unresolved_import {
        args: (name: impl Display),
        msg: format!("failed to resolve import: '{}'", name),
        help: None,
    }

    /// For when a user defines an alias with the same name twice.
    @formatted
    duplicate_alias_definition {
        args: (name: impl Display),
        msg: format!("an alias named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user defines an struct with the same name twice.
    @formatted
    duplicate_struct_definition {
        args: (name: impl Display),
        msg: format!("a struct named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user defines a function input with the same name twice.
    @formatted
    duplicate_function_input_definition {
        args: (name: impl Display),
        msg: format!("a function input named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user defines a global const with the same name twice.
    @formatted
    duplicate_global_const_definition {
        args: (name: impl Display),
        msg: format!("a global const named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a function input shadows a global const.
    @formatted
    function_input_cannot_shadow_global_const {
        args: (name: impl Display),
        msg: format!("a function input cannot be named `{}` as a global const with that name already exists in this scope", name),
        help: None,
    }

    /// For when a variable definition shadows a global const.
    @formatted
    function_variable_cannot_shadow_global_const {
        args: (name: impl Display),
        msg: format!("a variable cannot be named `{}` as a global const with that name already exists in this scope", name),
        help: None,
    }

    /// For when a variable definition shadows a function input.
    @formatted
    function_variable_cannot_shadow_other_function_variable {
        args: (name: impl Display),
        msg: format!("a variable cannot be named `{}` as a function input or variable with that name already exists in this scope", name),
        help: None,
    }

    /// For when operator is used on an unsupported type.
    @formatted
    operator_allowed_only_for_type {
        args: (operator: impl Display, type_: impl Display, received: impl Display),
        msg: format!("operator '{}' is only allowed for type '{}', received: '{}'", operator, type_, received),
        help: None,
    }

    /// For when a user tries to call a struct variable as a function.
    @formatted
    struct_const_call {
        args: (struct_name: impl Display, name: impl Display),
        msg: format!("cannot call const member '{}' of struct '{}'", name, struct_name),
        help: None,
    }

    /// For when `input` variable is accessed inside a const function.
    @formatted
    illegal_input_variable_reference_in_const_function {
        args: (),
        msg: "input access is illegal in const functions",
        help: None,
    }

    /// For when non-const function is called from a const context.
    @formatted
    calling_non_const_in_const_context {
        args: (),
        msg: "non-const functions cannot be called from const context",
        help: None,
    }

    /// For when const function modifier is added to the main function.
    @formatted
    main_cannot_be_const {
        args: (),
        msg: "main function cannot be const",
        help: None,
    }

    /// For when const function has non-const inputs or self.
    @formatted
    const_function_cannot_have_inputs {
        args: (),
        msg: "const function cannot have non-const input",
        help: None,
    }

    /// For when `main` is annotated.
    @formatted
    main_cannot_have_annotations {
        args: (),
        msg: "main function cannot have annotations",
        help: None,
    }

    /// For when unsupported annotation is added.
    @formatted
    unsupported_annotation {
        args: (name: impl Display),
        msg: format!("annotation `{}` does not exist", name),
        help: None,
    }
);
