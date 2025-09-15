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

use crate::create_messages;
use std::fmt::{Debug, Display};

// TODO: Consolidate errors.

create_messages!(
    TypeCheckerError,
    code_mask: 2000i32,
    code_prefix: "TYC",

    /// For when the type checker encountered an invalid assignment target.
    @formatted
    invalid_assignment_target {
        args: (target: impl Display),
        msg: format!("Invalid assignment target: {target}."),
        help: Some("Valid assignment targets are identifiers, tuple accesses, array accesses, and struct accesses.".to_string()),
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
        help: Some("Consider using explicit type annotations.".into()),
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
    /// Also repurposing for when a group value is not valid.
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
            "Variable {variable} is not a member of {struct_}."
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

    // Not currently used
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

    // Not currently used
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
    loop_body_contains_async {
        args: (kind: impl Display),
        msg: format!("Loop body contains an async {kind}."),
        help: Some(format!("Remove the async {kind}.")),
    }

    @formatted
    missing_return {
        args: (),
        msg: format!("Function must return a value."),
        help: None,
    }

    // TODO This error is unused. Remove it in a future version.
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
        msg: format!("The allowed accesses to `self` are `self.{{caller, checksum, edition, program_owner, signer}}`."),
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
        args: (kind: impl Display),
        msg: format!("Only `inline` can be called from {kind}."),
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

    // TODO This error is unused. Remove it in a future version.
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

    // TODO This error is unused. Remove it in a future version.
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
    operation_must_be_in_async_block_or_function {
        args: (),
        msg: "This operation can only be used in an async function, an async block, or script.".to_string(),
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

    // TODO This error is unused. Remove it in a future version.
    @formatted
    const_declaration_must_be_literal_or_tuple_of_literals {
        args: (),
        msg: format!("The value of a const declaration must be a literal"),
        help: None,
    }

    // TODO This error is unused. Remove it in a future version.
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
    missing_async_operation_in_async_transition {
        args: (),
        msg: "An `async` transition must contain at least one async operation — either a call to an `async` function or an `async` block.".to_string(),
        help: Some("Example: `async transition foo() -> Future { let a: Future = bar(); return await_futures(a); }`".to_string()),
    }

    // TODO This error is unused. Remove it in a future version.
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

    // TODO: This error is deprecated. Remove.
    @formatted
    no_path_awaits_all_futures_exactly_once {
        args: (num_total_paths: impl Display),
        msg: format!("Futures must be awaited exactly once. Out of `{num_total_paths}`, there does not exist a single path in which all futures are awaited exactly once."),
        help: Some("Ex: for `f: Future` call `f.await()` to await a future. Remove duplicate future await redundancies, and add future awaits for un-awaited futures.".to_string()),
    }

    // TODO: This error is deprecated. Remove.
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

    // TODO: This error is deprecated. Remove.
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

    // TODO: This error is deprecated. Remove.
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
    external_call_after_async {
        args: (kind: impl Display),
        msg: format!("External transition calls must appear before the local async {kind}."),
        help: Some(format!("Reorder your code so the external transition call happens before the local async {kind}.")),
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
        help: Some("Make sure all futures are consumed exactly once. Consume by passing to an async function call or async block.".to_string()),
    }

    @formatted
    async_transition_missing_future_to_return {
        args: (),
        msg: "An async transition must return a future.".to_string(),
        help: Some("Call an async function or instantiate an async block inside of the async transition body so that there is a future to return.".to_string()),
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
        args: (variable: impl Display, kind: impl Display),
        msg: format!("Cannot re-assign to `{variable}` from a conditional scope to an outer scope in an async {kind}."),
        help: Some("This is a fundamental restriction that can often be avoided by using a ternary operator `?` or re-declaring the variable in the current scope. In the future, ARC XXXX (https://github.com/ProvableHQ/ARCs) will support more complex assignments in async functions.".to_string()),
    }

    @formatted
    only_async_transition_can_return_future {
        args: (),
        msg: "Only `async transition` can return a future.".to_string(),
        help: Some("Use an `async transition` instead.".to_string()),
    }

    @formatted
    async_function_not_found {
        args: (name: impl Display),
        msg: format!("The async function `{name}` does not exist."),
        help: Some(format!("Ensure that `{name}` is defined as an async function in the current program.")),
    }

    @formatted
    empty_struct {
        args: (),
        msg: "A struct must have at least one member.".to_string(),
        help: None,
    }

    @formatted
    empty_function_arglist {
        args: (),
        msg: format!("Cannot define a function with no parameters."),
        help: None,
    }

    @formatted
    composite_data_type_cannot_contain_future {
        args: (data_type: impl Display),
        msg: format!("A {data_type} cannot contain a future."),
        help: None,
    }

    @formatted
    array_element_cannot_be_future {
        args: (),
        msg: format!("An array cannot have a future as an element type."),
        help: None,
    }

    @formatted
    no_future_parameters {
        args: (),
        msg: format!("Futures may only appear as parameters to async functions."),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    ///
    /// This is a replacement for `type_should_be` with a slightly better message.
    @formatted
    type_should_be2 {
        args: (type_: impl Display, expected: impl Display),
        msg: format!(
            "Expected {expected} but type `{type_}` was found.",
        ),
        help: None,
    }

    @formatted
    ternary_branch_mismatch {
        args: (type1: impl Display, type2: impl Display),
        msg: format!(
            "Received different types `{type1}` and `{type2}` for the arms of a ternary conditional."
        ),
        help: Some("Make both branches the same type.".into()),
    }

    @formatted
    operation_types_mismatch {
        args: (operation: impl Display, type1: impl Display, type2: impl Display),
        msg: format!(
            "Received different types `{type1}` and `{type2}` for the operation `{operation}`."
        ),
        help: Some("Make both operands the same type.".into()),
    }

    @formatted
    mul_types_mismatch {
        args: (type1: impl Display, type2: impl Display),
        msg: format!(
            "Received types `{type1}` and `{type2}` for the operation `*`."
        ),
        help: Some("Valid operands are two integers of the same type, two fields, or a scalar and a group.".into()),
    }

    @formatted
    pow_types_mismatch {
        args: (type1: impl Display, type2: impl Display),
        msg: format!(
            "Received types `{type1}` and `{type2}` for the operation `pow`."
        ),
        help: Some("Valid operands are two fields, or an integer base and a `u8`, `u16`, or `u32` exponent.".into()),
    }

    @formatted
    shift_type_magnitude {
        args: (operation: impl Display, rhs_type: impl Display),
        msg: format!(
            "Received type `{rhs_type}` for the second operand of the operation `{operation}`."
        ),
        help: Some("Valid second operands are `u8`, `u16`, or `u32`".into()),
    }

    @formatted
    unit_type_only_return {
        args: (),
        msg: "The unit type () may appear only as the return type of a function.".to_string(),
        help: None,
    }

    @formatted
    future_error_member {
        args: (num: impl Display),
        msg: format!("Cannot access argument `{num}` from future."),
        help: Some(
            "Ensure that the async function is not called with multiple times with incompatible types.".to_string()
        ),
    }

    @formatted
    cannot_reassign_mapping {
        args: (var: impl Display),
        msg: format!("Cannot assign to the mapping `{var}`."),
        help: None,
    }

    @formatted
    records_not_allowed_inside_async {
        args: (kind: impl Display),
        msg: format!("records cannot be instantiated in an async {kind} context."),
        help: None,
    }

    @formatted
    script_in_non_test {
        args: (func: impl Display),
        msg: format!("`script` {func} appears in a non-test program."),
        help: Some("Move this to a test program, or replace it with a function or transition".to_string()),
    }

    @formatted
    non_script_calls_script {
        args: (call: impl Display),
        msg: format!("`script` {call} is called by a non-`script`."),
        help: None,
    }

    @formatted
    annotation_error {
        args: (message: impl Display),
        msg: format!("Invalid annotation: {message}."),
        help: None,
    }

    @formatted
    ternary_over_external_records {
        args: (ty: impl Display),
        msg: format!("Cannot apply ternary conditional to type `{ty}`."),
        help: Some("Ternary conditionals may not contain an external record type.".to_string()),
    }

    // TODO: unused.
    @formatted
    assignment_to_external_record {
        args: (ty: impl Display),
        msg: format!("Cannot assign to type `{ty}` or a member thereof."),
        help: Some("External record types and tuples containing them may not be assigned to.".to_string()),
    }

    @formatted
    illegal_name {
        args: (item_name: impl Display, item_type: impl Display, keyword: impl Display),
        msg: format!("`{item_name}` is an invalid {item_type} name. A {item_type} cannot have \"{keyword}\" in its name."),
        help: None,
    }

    @formatted
    record_prefixed_by_other_record {
        args: (r1: impl Display, r2: impl Display),
        msg: format!("Record name `{r1}` is prefixed by the record name `{r2}`. Record names must not be prefixes of other record names."),
        help: None,
    }

    @formatted
    range_bounds_type_mismatch {
        args: (),
        msg: format!("mismatched types in loop iterator range bounds"),
        help: None,
    }

    @formatted
    assignment_to_external_record_member {
        args: (ty: impl Display),
        msg: format!("Cannot assign to a member of the external record `{ty}`."),
        help: None,
    }

    @formatted
    assignment_to_external_record_cond {
        args: (ty: impl Display),
        msg: format!("Cannot assign to the external record type `{ty}` in this location."),
        help: Some("External record variables may not be assigned to in narrower conditional scopes than they were defined.".into()),
    }

    @formatted
    assignment_to_external_record_tuple_cond {
        args: (ty: impl Display),
        msg: format!("Cannot assign to the tuple type `{ty}` containing an external record in this location."),
        help: Some("Tuples containing external records may not be assigned to in narrower conditional scopes than they were defined.".into()),
    }

    @formatted
    hexbin_literal_nonintegers {
        args: (),
        msg: format!("Hex, octal, and binary literals may only be used for integer types."),
        help: None,
    }

    @formatted
    unexpected_unsuffixed_numeral {
        args: (expected: impl Display),
        msg: format!(
            "Expected {expected} but an unsuffixed numeral was found.",
        ),
        help: None,
    }

    @formatted
    incorrect_num_const_args {
        args: (kind: impl Display, expected: impl Display, received: impl Display),
        msg: format!(
            "{kind} expected `{expected}` const args, but got `{received}`",
        ),
        help: None,
    }

    @formatted
    bad_const_generic_type {
        args: (found: impl Display),
        msg: format!("A generic const parameter must be a `bool`, an integer, a `scalar`, a `group`, a `field`, or an `address`, but {found} was found"),
        help: None,
    }

    /// For when the user tries to assign to a generic const function parameter.
    @formatted
    cannot_assign_to_generic_const_function_parameter {
        args: (param: impl Display),
        msg: format!(
            "Cannot assign to const parameter `{param}`",
        ),
        help: None,
    }

    @formatted
    only_inline_can_have_const_generics {
        args: (),
        msg: format!("Only `inline` functions can have generic const parameters."),
        help: None,
    }

    @formatted
    array_too_large_for_u32 {
        args: (),
        msg: format!("An array length must be small enough to fit in a `u32`"),
        help: None,
    }

    @formatted
    unexpected_record_const_parameters {
        args: (),
        msg: format!("Records cannot be declared with generic const parameters."),
        help: None,
    }

    @formatted
    unexpected_const_args {
        args: (item: impl Display),
        msg: format!("unexpected generic const argment for {item}."),
        help: Some("If this is an external struct, consider using a resolved non-generic version of it instead. External structs can't be instantiated with const arguments".to_string()),
    }

    @formatted
    invalid_operation_inside_async_block {
        args: (operation: impl Display),
        msg: format!("Invalid expression in an async block. `{operation}` cannot be used directly here"),
        help: None,
    }

    @formatted
    illegal_async_block_location {
        args: (),
        msg: "`async` blocks are only allowed inside an `async` transition or a script function.".to_string(),
        help: Some("Try moving this `async` block into an `async` transition or a script function.".to_string()),
    }

    @formatted
    conflicting_async_call_and_block {
        args: (),
        msg: "A transition function cannot contain both an `async` function call and an `async` block at the same time.".to_string(),
        help: Some("Refactor the transition to use either an `async` call or an `async` block, but not both.".to_string()),
    }

    @formatted
    multiple_async_blocks_not_allowed {
        args: (),
        msg: "A transition function cannot contain more than one `async` block.".to_string(),
        help: Some("Combine the logic into a single `async` block, or restructure your code to avoid multiple async blocks within the same transition.".to_string()),
    }

    @formatted
    async_block_in_conditional {
        args: (),
        msg: "`async` blocks are not allowed inside conditional blocks.".to_string(),
        help: Some("Refactor your code to move the `async` block outside of the conditional block.".to_string()),
    }

    @formatted
    cannot_use_private_inpt_in_async_block {
        args: (),
        msg: format!("`private` inputs cannot be used inside async blocks."),
        help: None,
    }

    @formatted
    async_block_cannot_return {
        args: (),
        msg: "An `async` block cannot contain a `return` statement.".to_string(),
        help: None,
    }

    @formatted
    invalid_async_block_future_access {
        args: (),
        msg: format!(
            "Cannot access argument from future produced by an `async` block."
        ),
        help: None,
    }

    @formatted
    cannot_assign_to_vars_outside_async_block {
        args: (input: impl Display),
        msg: format!(
            "Cannot assign to `{input}` inside an `async` block because it was declared outside the block."
        ),
        help: None,
    }

    @formatted
    custom {
        args: (msg: impl Display),
        msg: msg.to_string(),
        help: None,
    }

    @formatted
    constructor_can_only_return_unit {
        args: (expression: impl Display),
        msg: format!("Constructors can only return unit, but found `{expression}`."),
        help: None,
    }

    @formatted
    none_found_non_optional {
        args: (expected: impl Display),
        msg: format!(
            "Found `none`, but the expected type `{expected}` is not an optional type.",
        ),
        help: None,
    }

    @formatted
    optional_wrapping_of_records_unsupported {
        args: (ty: impl Display),
        msg: format!(
            "The type `{ty}` cannot be wrapped in an optional because it is a record.",
        ),
        help: None,
    }

    @formatted
    optional_wrapping_unsupported {
        args: (ty: impl Display),
        msg: format!(
            "The type `{ty}` cannot be wrapped in an optional.",
        ),
        help: None,
    }

    @formatted
    optional_type_not_allowed_in_mapping {
        args: (ty: impl Display, kind: impl Display),
        msg: format!(
            "The type `{ty}` is or contains an optional type which cannot be used as the {kind} in a mapping",
        ),
        help: None,
    }

    @formatted
    record_field_cannot_be_optional {
        args: (name: impl Display, ty: impl Display),
        msg: format!(
            "The field `{name}` in this record has type `{ty}`, which is or contains an optional type.",
        ),
        help: Some(
            "Records cannot have fields that are optional or contain optionals. Consider moving the optionality outside the record.".to_string()
        ),
    }

    @formatted
    const_cannot_be_optional {
        args: (),
        msg: format!(
            "Constants cannot have an optional type or a type that contains an optional",
        ),
        help: None,
    }

    @formatted
    function_cannot_take_option_as_input {
        args: (name: impl Display, ty: impl Display),
        msg: format!(
            "The input `{name}` has type `{ty}`, which is or contains an optional type and is not allowed as an input to a `transition`, `async transition`, or `function`.",
        ),
        help: Some(
            "Inputs to `transition`, `async transition`, and `function` definitions cannot be optional or contain optionals. Consider moving the optionality outside the call site.".to_string()
        ),
    }

    @formatted
    function_cannot_return_option_as_output {
        args: (ty: impl Display),
        msg: format!(
            "This function has an output of type `{ty}`, which is or contains an optional type and is not allowed as an output of a `transition`, `async transition`, or `function`.",
        ),
        help: Some(
            "Outputs of `transition`, `async transition`, and `function` definitions cannot be optional or contain optionals. Consider moving the optionality outside the function call.".to_string()
        ),
    }

    @formatted
    invalid_storage_type {
        args: (type_: impl Display),
        msg: format!("{type_} is an invalid storage type"),
        help: None,
    }

    @formatted
    storage_vectors_cannot_be_moved_or_assigned {
        args: (),
        msg: format!(
            "Storage vectors cannot be moved or assigned. You can only access or modify them using methods like `get`, `push`, or `pop`."
        ),
        help: None,
    }
);
