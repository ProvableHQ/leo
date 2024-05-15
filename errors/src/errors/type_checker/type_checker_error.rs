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
        help: Some("If you are using an external type, make sure to preface with the program name. Ex: `credits.aleo/credits` instead of `credits`".to_string()),
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
    async_function_input_cannot_be_private {
        args: (),
        msg: format!("Async functions cannot have private inputs."),
        help: Some("Use a `public` modifier to the input variable declaration or remove the visibility modifier entirely.".to_string()),
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
    async_function_input_must_be_public {
        args: (),
        msg: format!("An input to an async function must be public."),
        help: Some("Use a `public` modifier to the input variable declaration or remove the visibility modifier entirely.".to_string()),
    }

    @formatted
    finalize_output_mode_must_be_public {
        args: (),
        msg: format!("An output from an async function block must be public."),
        help: Some("Use a `public` modifier to the output type declaration or remove the visibility modifier entirely.".to_string()),
    }

    @formatted
    invalid_operation_outside_finalize {
        args: (operation: impl Display),
        msg: format!("`{operation}` must be inside an async function block."),
        help: None,
    }

    @formatted
    loop_body_contains_finalize {
        args: (),
        msg: format!("Loop body contains an async function call."),
        help: Some("Remove the async function call.".to_string()),
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
        msg: format!("An async function call block cannot be empty."),
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
    invalid_self_access {
        args: (),
        msg: format!("The allowed accesses to `self` are `self.caller` and `self.signer`."),
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

    // TODO: Consider changing this to a warning.

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
    nested_tuple_expression {
        args: (),
        msg: format!("A tuple expression cannot contain another tuple expression."),
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
    function_cannot_input_or_output_a_record {
        args: (),
        msg: format!("Only `transition` functions can have a record as input or output."),
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
        msg: format!("`{operation}` is not a valid operand in an async function call context."),
        help: None,
    }

    @formatted
    operation_must_be_in_finalize_block {
        args: (),
        msg: format!("This operation can only be used in an async function block."),
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
    stub_functions_must_not_be_inlines {
        args: (),
        msg: format!("Function stubs must be transitions or functions not inlines"),
        help: None,
    }

    @formatted
    stub_functions_must_be_empty {
        args: (),
        msg: format!("Functions stubs must be empty"),
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

    @formatted
    stubs_cannot_have_non_record_structs {
        args: (),
        msg: format!("Stubs can only have records, transitions, functions, mappings and imports -- found non-record struct"),
        help: None,
    }

    @formatted
    stubs_cannot_have_const_declarations {
        args: (),
        msg: format!("Stubs can only have records, transitions, functions, mappings and imports -- found const declaration"),
        help: None,
    }

    @formatted
    stub_name_mismatch {
        args: (stub_name: impl Display, program_name: impl Display),
        msg: format!("`stub` name `{stub_name}` does not match program name `{program_name}`"),
        help: Some("Check that the name you used as a dependency in program.json matches the name you used to import the program in the main leo file.".to_string()),
    }

    @formatted
    no_transitions {
        args: (),
        msg: "A program must have at least one transition function.".to_string(),
        help: None,
    }

    @formatted
    cannot_define_external_struct {
        args: (struct_: impl Display),
        msg: format!("Cannot define external struct `{struct_}`"),
        help: Some("Copy the external definition of the struct into the current program, and then define without the `.aleo` extension.".to_string()),
    }

    @formatted
    struct_definitions_dont_match {
        args: (struct_: impl Display, program_1: impl Display, program_2: impl Display),
        msg: format!("The definition for `{struct_}` in program `{program_1}.aleo` does not match the definition in program `{program_2}.aleo`"),
        help: Some("Check that the struct definition in the current program matches the definition in the imported program.".to_string()),
    }

    @formatted
    async_transition_invalid_output {
        args: (),
        msg: "An async transition must return a future as the final output, and in no other position return a future.".to_string(),
        help: Some("Example: `async transition foo() -> (u8, bool, Future) {...}`".to_string()),
    }

    @formatted
    must_propagate_all_futures {
        args: (never_propagated: impl Display),
        msg: format!("All futures generated from external transition calls must be inserted into an async function call in the order they were called. The following were never were: {never_propagated}"),
        help: Some("Example: `async transition foo() -> Future { let a: Future = b.aleo/bar(); return await_futures(a); }`".to_string()),
    }

    @formatted
    async_transition_must_call_async_function {
        args: (),
        msg: "An async transition must call an async function.".to_string(),
        help: Some("Example: `async transition foo() -> Future { let a: Future = bar(); return await_futures(a); }`".to_string()),
    }
    @formatted
    async_function_input_length_mismatch {
        args: (expected: impl Display, received: impl Display),
        msg: format!("Expected `{expected}` inputs, but got `{received}`"),
        help: Some("Check that the number of arguments passed in are the same as the number in the function signature. Ex: `async function foo(a: u8, b: u8)` has two input arguments.".to_string()),
    }

    @formatted
    invalid_future_access {
        args: (num: impl Display, len: impl Display),
        msg: format!(
            "Cannot access argument `{num}` from future. The future only has `{len}` arguments."
        ),
        help: None,
    }

    @formatted
    future_access_must_be_number {
        args: (name: impl Display),
        msg: format!("Future access must be a number not `{name}`."),
        help: Some(" Future arguments must be addressed by their index. Ex: `f.1.3`.".to_string()),
    }

    @formatted
    no_path_awaits_all_futures_exactly_once {
        args: (num_total_paths: impl Display),
        msg: format!("Futures must be awaited exactly once. Out of `{num_total_paths}`, there does not exist a single path in which all futures are awaited exactly once."),
        help: Some("Ex: for `f: Future` call `f.await()` to await a future. Remove duplicate future await redundancies, and add future awaits for un-awaited futures.".to_string()),
    }

    @formatted
    future_awaits_missing {
        args: (unawaited: impl Display),
        msg: format!("The following futures were never awaited: {unawaited}"),
        help: Some("Ex: for `f: Future` call `f.await()` to await a future.".to_string()),
    }

    @formatted
    cannot_reassign_future_variable {
        args: (var: impl Display),
        msg: format!("Cannot reassign variable `{var}` since it has type Future."),
        help: Some("Futures can only be defined as the result of async calls.".to_string()),
    }

    @formatted
    invalid_await_call {
        args: (),
        msg: "Not a valid await call.".to_string(),
        help: Some("Ex: for `f: Future` call `f.await()` or `Future::await(f)` to await a future.".to_string()),
    }

    @formatted
    can_only_await_one_future_at_a_time {
        args: (),
        msg: "Must await exactly one future at a time".to_string(),
        help: Some("Ex: for `f: Future` call `f.await()` or `Future::await(f)` to await a future.".to_string()),
    }

    @formatted
    expected_future {
        args: (type_: impl Display),
        msg: format!("Expected a future, but found `{type_}`"),
        help: Some("Only futures can be awaited.".to_string()),
    }

    @formatted
    invalid_method_call {
        args: (),
        msg: "Not a valid method call.".to_string(),
        help: Some("For a `f: Future`, call the associated method `f.await()`.".to_string()),
    }

    @formatted
    async_call_in_conditional {
        args: (),
        msg: "Cannot call an async function in a conditional block.".to_string(),
        help: Some("Move the async call outside of the conditional block.".to_string()),
    }

    @formatted
    must_call_async_function_once {
        args: (),
        msg: "Must call exactly one local async function per transition function.".to_string(),
        help: Some("Move the async call outside of the transition block.".to_string()),
    }

    @formatted
    async_call_can_only_be_done_from_async_transition {
        args: (),
        msg: "Can only make an async call from an async transition.".to_string(),
        help: Some("Move the async call inside of the async transition block.".to_string()),
    }

    @formatted
    external_transition_call_must_be_before_finalize {
        args: (),
        msg: "External async transition calls cannot be made after local async function call".to_string(),
        help: Some("Move the async call before the function call.".to_string()),
    }

    @formatted
    unknown_future_consumed {
        args: (future: impl Display),
        msg: format!("Unknown future consumed: `{future}`"),
        help: Some("Make sure the future is defined and consumed exactly once.".to_string()),
    }

    @formatted
    not_all_futures_consumed {
        args: (unconsumed: impl Display),
        msg: format!("Not all futures were consumed: {unconsumed}"),
        help: Some("Make sure all futures are consumed exactly once. Consume by passing to an async function call.".to_string()),
    }

    @formatted
    async_transition_missing_future_to_return {
        args: (),
        msg: "An async transition must return a future.".to_string(),
        help: Some("Call an async function inside of the async transition body so that there is a future to return.".to_string()),
    }

    @formatted
    async_function_cannot_return_value {
        args: (),
        msg: "An async function is not allowed to return a value.".to_string(),
        help: Some("Remove an output type in the function signature, and remove the return statement from the function. Note that the future returned by async functions is automatically inferred, and must not be explicitly written.".to_string()),
    }

    @formatted
    return_type_of_finalize_function_is_future {
        args: (),
        msg: "The output of an async function must be assigned to a `Future` type..".to_string(),
        help: None,
    }
    @formatted
    cannot_modify_external_mapping {
        args: (operation: impl Display),
        msg: format!("Cannot use operation `{operation}` on external mapping."),
        help: Some("The only valid operations on external mappings are contains, get, and get_or_use.".to_string()),
    }

    @formatted
    async_cannot_assign_outside_conditional {
        args: (variable: impl Display),
        msg: format!("Cannot re-assign to `{variable}` from a conditional scope to an outer scope in an async function."),
        help: Some("This is a fundamental restriction that can often be avoided by using a ternary operator `?` or re-declaring the variable in the current scope. In the future, ARC XXXX (https://github.com/AleoHQ/ARCs) will support more complex assignments in async functions.".to_string()),
    }

    @formatted
    only_async_transition_can_return_future {
        args: (),
        msg: "A `transition` cannot return a future.".to_string(),
        help: Some("Use an `async transition` instead.".to_string()),
    }

    @formatted
    async_function_not_found {
        args: (name: impl Display),
        msg: format!("The async function `{name}` does not exist."),
        help: Some(format!("Ensure that `{name}` is defined as an async function in the current program.")),
    }
);
