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

use crate::create_messages;
use std::fmt::{Debug, Display};

// TODO: Consolidate errors.

create_messages!(
    /// InputError enum that represents all the errors for the inputs part of `leo-ast` crate.
    TypeCheckerError,
    code_mask: 2000i32,
    code_prefix: "TYC",

    /// For when the parser encountered an invalid assignment target.
    @formatted
    invalid_assignment_target {
        args: (),
        msg: "invalid assignment target",
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    cannot_assign_to_const_input {
        args: (input: impl Display),
        msg: format!(
            "Cannot assign to const input `{input}`",
        ),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    cannot_assign_to_const_var {
        args: (var: impl Display),
        msg: format!(
            "Cannot assign to const variable `{var}`",
        ),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    type_should_be {
        args: (type_: impl Display, expected: impl Display),
        msg: format!(
            "Expected type `{expected}` but type `{type_}` was found",
        ),
        help: None,
    }

    /// For when the type checker cannot determine the type of an expression.
    @formatted
    could_not_determine_type {
        args: (expr: impl Display),
        msg: format!(
            "Could not determine the type of `{expr}`",
        ),
        help: None,
    }

    /// For when the user tries to return a unknown variable.
    @formatted
    unknown_sym {
        args: (kind: impl Display, sym: impl Display),
        msg: format!(
            "Unknown {kind} `{sym}`",
        ),
        help: None,
    }

    /// For when the user tries calls a function with the incorrect number of args.
    @formatted
    incorrect_num_args_to_call {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "Call expected `{expected}` args, but got `{received}`",
        ),
        help: None,
    }

    /// For when one of the following types was expected.
    @formatted
    expected_one_type_of {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "Expected one type from `{expected}`, but got `{received}`",
        ),
        help: None,
    }

    /// For when an integer is not in a valid range.
    @formatted
    invalid_int_value {
        args: (value: impl Display, type_: impl Display),
        msg: format!(
            "The value {value} is not a valid `{type_}`",
        ),
        help: None,
    }

    /// For when an invalid core function is used.
    @formatted
    invalid_core_function {
        args: (struct_: impl Display, function: impl Display),
        msg: format!(
            "{struct_}::{function} is not a valid core function.",
        ),
        help: None,
    }

    /// For when a struct is created with the same name as a core type.
    @formatted
    core_type_name_conflict {
        args: (type_: impl Display),
        msg: format!(
            "The type {type_} is a reserved core type name.",
        ),
        help: None,
    }

    /// For when a function doesn't have a return statement.
    @formatted
    function_has_no_return {
        args: (func: impl Display),
        msg: format!(
            "The function {func} has no return statement.",
        ),
        help: None,
    }

    /// For when the user tries initialize a struct with the incorrect number of args.
    @formatted
    incorrect_num_struct_members {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "Struct expected `{expected}` members, but got `{received}`",
        ),
        help: None,
    }

    /// For when the user is missing a struct member during initialization.
    @formatted
    missing_struct_member {
        args: (struct_: impl Display, member: impl Display),
        msg: format!(
            "Struct initialization expression for `{struct_}` is missing member `{member}`.",
        ),
        help: None,
    }

    /// An invalid access call is made e.g., `SHA256::hash()
    @formatted
    invalid_core_function_call {
        args: (expr: impl Display),
        msg: format!(
            "{expr} is not a valid core function call."
        ),
        help: None,
    }

    /// Attempted to define more that one struct member with the same name.
    @formatted
    duplicate_struct_member {
        args: (struct_: impl Display),
        msg: format!(
            "Struct {struct_} defined with more than one member with the same name."
        ),
        help: None,
    }

    /// Attempted to define more that one record variable with the same name.
    @formatted
    duplicate_record_variable {
        args: (record: impl Display),
        msg: format!(
            "Record {record} defined with more than one variable with the same name."
        ),
        help: None,
    }

    /// Attempted to access an invalid struct.
    @formatted
    undefined_type {
        args: (type_: impl Display),
        msg: format!(
            "The type `{type_}` is not found in the current scope."
        ),
        help: None,
    }

    /// Attempted to access an invalid struct variable.
    @formatted
    invalid_struct_variable {
        args: (variable: impl Display, struct_: impl Display),
        msg: format!(
            "Variable {variable} is not a member of struct {struct_}."
        ),
        help: None,
    }

    @formatted
    required_record_variable {
        args: (name: impl Display, type_: impl Display),
        msg: format!("The `record` type requires the variable `{name}: {type_}`."),
        help: None,
    }

    @formatted
    record_var_wrong_type {
        args: (name: impl Display, type_: impl Display),
        msg: format!("The field `{name}` in a `record` must have type `{type_}`."),
        help: None,
    }

    @formatted
    compare_address {
        args: (operator: impl Display),
        msg: format!("Comparison `{operator}` is not supported for the address type."),
        help: None,
    }

    @formatted
    incorrect_tuple_length {
        args: (expected: impl Display, actual: impl Display),
        msg: format!("Expected a tuple of length `{expected}` found length `{actual}`"),
        help: None,
    }

    @formatted
    invalid_tuple {
        args: (),
        msg: "Tuples must be explicitly typed in Leo".to_string(),
        help: Some("The function definition must match the function return statement".to_string()),
    }

    @formatted
    tuple_out_of_range {
        args: (index: impl Display, length: impl Display),
        msg: format!("Tuple index `{index}` out of range for a tuple with length `{length}`"),
        help: None,
    }

    @formatted
    unreachable_code_after_return {
        args: (),
        msg: format!("Cannot reach the following statement."),
        help: Some("Remove the unreachable code.".to_string()),
    }

    @formatted
    loop_body_contains_return {
        args: (),
        msg: format!("Loop body contains a return statement or always returns."),
        help: Some("Remove the code in the loop body that always returns.".to_string()),
    }

    // TODO: Consider emitting a warning instead of an error.
    @formatted
    unknown_annotation {
        args: (annotation: impl Display),
        msg: format!("Unknown annotation: `{annotation}`."),
        help: None,
    }

    @formatted
    regular_function_inputs_cannot_have_modes {
        args: (),
        msg: format!("Standard functions cannot have modes associated with their inputs."),
        help: Some("Consider removing the mode or using the keyword `transition` instead of `function`.".to_string()),
    }

    @formatted
    struct_or_record_cannot_contain_record {
        args: (parent: impl Display, child: impl Display),
        msg: format!("A struct or record cannot contain another record."),
        help: Some(format!("Remove the record `{child}` from `{parent}`.")),
    }

    @formatted
    invalid_mapping_type {
        args: (component: impl Display, type_: impl Display),
        msg: format!("A mapping's {component} cannot be a {type_}"),
        help: None,
    }

    @formatted
    only_transition_functions_can_have_finalize {
        args: (),
        msg: format!("Only transition functions can have a `finalize` block."),
        help: Some("Remove the `finalize` block or use the keyword `transition` instead of `function`.".to_string()),
    }

    @formatted
    finalize_input_mode_must_be_public {
        args: (),
        msg: format!("An input to a finalize block must be public."),
        help: Some("Use a `public` modifier to the input variable declaration or remove the visibility modifier entirely.".to_string()),
    }

    @formatted
    finalize_output_mode_must_be_public {
        args: (),
        msg: format!("An output from a finalize block must be public."),
        help: Some("Use a `public` modifier to the output type declaration or remove the visibility modifier entirely.".to_string()),
    }

    @formatted
    finalize_in_finalize {
        args: (),
        msg: format!("A finalize block cannot contain a finalize statement."),
        help: None,
    }

    @formatted
    invalid_operation_outside_finalize {
        args: (operation: impl Display),
        msg: format!("`{operation}` must be inside a finalize block."),
        help: None,
    }

    @formatted
    finalize_without_finalize_block {
        args: (),
        msg: format!("Cannot use a `finalize` statement without a `finalize` block."),
        help: None,
    }

    @formatted
    loop_body_contains_finalize {
        args: (),
        msg: format!("Loop body contains a finalize statement."),
        help: Some("Remove the finalize statement.".to_string()),
    }

    @formatted
    missing_return {
        args: (),
        msg: format!("Function must return a value."),
        help: None,
    }

    @formatted
    finalize_block_must_not_be_empty {
        args: (),
        msg: format!("A finalize block cannot be empty."),
        help: None,
    }

    @formatted
    cannot_have_constant_output_mode {
        args: (),
        msg: format!("A returned value cannot be a constant."),
        help: None,
    }

    @formatted
    transition_function_inputs_cannot_be_const {
        args: (),
        msg: format!("Transition functions cannot have constant inputs."),
        help: None,
    }

    @formatted
    incorrect_num_args_to_finalize {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "`finalize` expected `{expected}` args, but got `{received}`",
        ),
        help: None,
    }

    @formatted
    invalid_self_access {
        args: (),
        msg: format!("The allowed accesses to `self` are `self.caller` and `self.signer`."),
        help: None,
    }

    @formatted
    missing_finalize {
        args: (),
        msg: format!("Function must contain a `finalize` statement on all execution paths."),
        help: None,
    }

    @formatted
    finalize_name_mismatch {
        args: (finalize_name: impl Display, function_name: impl Display),
        msg: format!("`finalize` name `{finalize_name}` does not match function name `{function_name}`"),
        help: None,
    }

    @formatted
    invalid_type {
        args: (type_: impl Display),
        msg: format!("Invalid type `{type_}`"),
        help: None,
    }

    @formatted
    can_only_call_inline_function {
        args: (),
        msg: format!("Only `inline` can be called from a `function` or `inline`."),
        help: None,
    }

    @formatted
    cannot_invoke_call_to_local_transition_function {
        args: (),
        msg: format!("Cannot call a local transition function from a transition function."),
        help: None,
    }

    @formatted
    loop_bound_must_be_a_literal {
        args: (),
        msg: format!("Loop bound must be a literal."),
        help: None,
    }

    @formatted
    strings_are_not_supported {
        args: (),
        msg: format!("Strings are not yet supported."),
        help: None,
    }

    @formatted
    imported_program_cannot_import_program {
        args: (),
        msg: format!("An imported program cannot import another program."),
        help: None,
    }

    @formatted
    too_many_transitions {
        args: (max: impl Display),
        msg: format!("The number of transitions exceeds the maximum. snarkVM allows up to {max} transitions within a single program."),
        help: None,
    }

    // TODO: Consider chainging this to a warning.

    @formatted
    assign_unit_expression_to_variable {
        args: (),
        msg: format!("Cannot assign a unit expression to a variable."),
        help: None,
    }

    @formatted
    nested_tuple_type {
        args: (),
        msg: format!("A tuple type cannot contain a tuple."),
        help: None,
    }

    @formatted
    composite_data_type_cannot_contain_tuple {
        args: (data_type: impl Display),
        msg: format!("A {data_type} cannot contain a tuple."),
        help: None,
    }

    @formatted
    function_cannot_take_tuple_as_input {
        args: (),
        msg: format!("A function cannot take in a tuple as input."),
        help: None,
    }

    @formatted
    finalize_cannot_take_tuple_as_input {
        args: (),
        msg: format!("A finalize block cannot take in a tuple as input."),
        help: None,
    }

    @formatted
    nested_tuple_expression {
        args: (),
        msg: format!("A tuple expression cannot contain another tuple expression."),
        help: None,
    }

    @formatted
    finalize_statement_cannot_contain_tuples {
        args: (),
        msg: format!("A finalize statement cannot contain tuple expressions."),
        help: None,
    }

    @formatted
    expression_statement_must_be_function_call {
        args: (),
        msg: format!("An expression statement must be a function call."),
        help: None,
    }

    @formatted
    lhs_tuple_element_must_be_an_identifier {
        args: (),
        msg: format!("Tuples on the left-hand side of a `DefinitionStatement` can only contain identifiers."),
        help: None,
    }

    @formatted
    lhs_must_be_identifier_or_tuple {
        args: (),
        msg: format!("The left-hand side of a `DefinitionStatement` can only be an identifier or tuple. Note that a tuple must contain at least two elements."),
        help: None,
    }

    @formatted
    unit_expression_only_in_return_statements {
        args: (),
        msg: format!("Unit expressions can only be used in return statements."),
        help: None,
    }

    @formatted
    function_cannot_output_record {
        args: (),
        msg: format!("A `function` cannot output a record."),
        help: None,
    }

    @backtraced
    cyclic_struct_dependency {
        args: (path: Vec<impl Display>),
        msg: {
            let path_string = path.into_iter().map(|name| format!("`{name}`")).collect::<Vec<String>>().join(" --> ");
            format!("Cyclic dependency between structs: {path_string}")
        },
        help: None,
    }

    @backtraced
    cyclic_function_dependency {
        args: (path: Vec<impl Display>),
        msg: {
            let path_string = path.into_iter().map(|name| format!("`{name}`")).collect::<Vec<String>>().join(" --> ");
            format!("Cyclic dependency between functions: {path_string}")
        },
        help: None,
    }

    @formatted
    struct_cannot_have_member_mode {
        args: (),
        msg: format!("A struct cannot have a member with mode `constant`, `private`, or `public`."),
        help: None,
    }

    @formatted
    cannot_call_external_inline_function {
        args: (),
        msg: format!("Cannot call an external `inline` function."),
        help: None,
    }

    @formatted
    finalize_cannot_take_record_as_input {
        args: (),
        msg: format!("A finalize block cannot take in a record as input."),
        help: None,
    }

    @formatted
    finalize_cannot_output_record {
        args: (),
        msg: format!("A finalize block cannot return a record."),
        help: None,
    }

    @formatted
    finalize_cannot_return_value {
        args: (),
        msg: format!("A finalize block cannot return a value."),
        help: None,
    }
    @formatted
    too_many_mappings {
        args: (max: impl Display),
        msg: format!("The number of mappings exceeds the maximum. snarkVM allows up to {max} mappings within a single program."),
        help: None,
    }

    /// A call to an invalid associated constant is made e.g., `bool::MAX`
    @formatted
    invalid_associated_constant {
        args: (expr: impl Display),
        msg: format!(
            "{expr} is not a valid associated constant."
        ),
        help: None,
    }

    /// For when an invalid core constant is called.
    @formatted
    invalid_core_constant {
        args: (type_: impl Display, constant: impl Display),
        msg: format! (
            "{type_}::{constant} is not a valid core constant.",
        ),
        help: None,
    }

    /// For when an invalid field of block is called.
    @formatted
    invalid_block_access {
        args: (),
        msg: format!("The allowed accesses to `block` are `block.height`."),
        help: None,
    }

    @formatted
    invalid_operation_inside_finalize {
        args: (operation: impl Display),
        msg: format!("`{operation}` is not a valid operand in a finalize context."),
        help: None,
    }

    @formatted
    operation_must_be_in_finalize_block {
        args: (),
        msg: format!("This operation can only be used in a `finalize` block."),
        help: None,
    }

    @formatted
    loop_range_decreasing {
        args: (),
        msg: format!("The loop range must be increasing."),
        help: None,
    }

    @formatted
    loop_bound_type_mismatch {
        args: (),
        msg: format!("The loop bounds must be same type"),
        help: None,
    }

    @formatted
    const_declaration_must_be_literal_or_tuple_of_literals {
        args: (),
        msg: format!("The value of a const declaration must be a literal"),
        help: None,
    }

    @formatted
    loop_bound_must_be_literal_or_const {
        args: (),
        msg: format!("The loop bound must be a literal or a const"),
        help: None,
    }

    @formatted
    incorrect_num_tuple_elements {
        args: (identifiers: impl Display, types: impl Display),
        msg: format!("Expected a tuple with {types} elements, found one with {identifiers} elements"),
        help: None,
    }

    @formatted
    const_declaration_can_only_have_one_binding {
        args: (),
        msg: format!("A constant declaration statement can only bind a single value"),
        help: None,
    }

    @formatted
    array_empty {
        args: (),
        msg: format!("An array cannot be empty"),
        help: None,
    }

    @formatted
    array_too_large {
        args: (size: impl Display, max: impl Display),
        msg: format!("An array cannot have more than {max} elements, found one with {size} elements"),
        help: None,
    }

    @formatted
    array_element_cannot_be_tuple {
        args: (),
        msg: format!("An array cannot have a tuple as an element type"),
        help: None,
    }

    @formatted
    array_element_cannot_be_record {
        args: (),
        msg: format!("An array cannot have a record as an element type"),
        help: None,
    }
);
