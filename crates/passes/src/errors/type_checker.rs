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

use leo_errors::{Backtraced, Formatted, LeoError};
use leo_span::Span;
use std::fmt::Display;

const CODE_PREFIX: &str = "TYC";
const CODE_MASK: i32 = 2000;

// TypeCheckerError builder functions

pub(crate) fn invalid_assignment_target(target: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK, format!("Invalid assignment target: {target}."), span)
        .with_help("Valid assignment targets are identifiers, tuple accesses, array accesses, and struct accesses.")
}

pub(crate) fn cannot_assign_to_const_input(input: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 1, format!("Cannot assign to const input `{input}`"), span)
}

pub(crate) fn cannot_assign_to_const_var(var: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 2, format!("Cannot assign to const variable `{var}`"), span)
}

pub(crate) fn type_should_be(type_: impl Display, expected: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 3,
        format!("Expected type `{expected}` but type `{type_}` was found"),
        span,
    )
}

pub(crate) fn could_not_determine_type(expr: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 4, format!("Could not determine the type of `{expr}`"), span)
        .with_help("Consider using explicit type annotations.")
}

pub(crate) fn unknown_sym(kind: impl Display, sym: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, format!("Unknown {kind} `{sym}`"), span)
}

pub(crate) fn incorrect_num_args_to_call(expected: impl Display, received: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 6, format!("Call expected `{expected}` args, but got `{received}`"), span)
}

pub(crate) fn invalid_int_value(value: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 8, format!("The value {value} is not a valid `{type_}`"), span)
}

pub(crate) fn incorrect_num_composite_members(expected: impl Display, received: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 12,
        format!("Composite expected `{expected}` members, but got `{received}`"),
        span,
    )
}

pub(crate) fn missing_composite_member(composite: impl Display, member: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 13,
        format!("Composite initialization expression for `{composite}` is missing member `{member}`."),
        span,
    )
}

pub(crate) fn duplicate_struct_member(member_name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 15, format!("Struct field `{member_name}` is already declared."), span)
}

pub(crate) fn duplicate_record_variable(variable_name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 16,
        format!("Record variable `{variable_name}` is already declared."),
        span,
    )
}

pub(crate) fn undefined_type(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 17, format!("The type `{type_}` is not found in the current scope."), span)
        .with_help("If you are using an external type, make sure to preface with the program name. Ex: `credits.aleo::credits` instead of `credits`")
}

pub(crate) fn invalid_composite_variable(variable: impl Display, composite: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 18, format!("Variable {variable} is not a member of {composite}."), span)
}

pub(crate) fn required_record_variable(name: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 19,
        format!("The `record` type requires the variable `{name}: {type_}`."),
        span,
    )
}

pub(crate) fn record_var_wrong_type(name: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 20,
        format!("The field `{name}` in a `record` must have type `{type_}`."),
        span,
    )
}

pub(crate) fn incorrect_tuple_length(expected: impl Display, actual: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 22,
        format!("Expected a tuple of length `{expected}` found length `{actual}`"),
        span,
    )
}

pub(crate) fn tuple_out_of_range(index: impl Display, length: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 24,
        format!("Tuple index `{index}` out of range for a tuple with length `{length}`"),
        span,
    )
}

pub(crate) fn unreachable_code_after_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 25, "Cannot reach the following statement.", span)
        .with_help("Remove the unreachable code.")
}

pub(crate) fn loop_body_contains_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 26, "Loop body contains a return statement or always returns.", span)
        .with_help("Remove the code in the loop body that always returns.")
}

pub(crate) fn unknown_annotation(annotation: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 27, format!("Unknown annotation: `{annotation}`."), span)
}

pub(crate) fn regular_function_inputs_cannot_have_modes(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 28, "Regular fns cannot have modes associated with their inputs.", span)
        .with_help("Consider removing the mode or moving this function inside the `program` block to make it an entry point fn.")
}

pub(crate) fn struct_or_record_cannot_contain_record(
    parent: impl Display,
    child: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 30, "A struct or record cannot contain another record.", span)
        .with_help(format!("Remove the record `{child}` from `{parent}`."))
}

pub(crate) fn invalid_mapping_type(component: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 31, format!("A mapping's {component} cannot be a {type_}"), span)
}

pub(crate) fn final_fn_input_must_be_public(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 32, "An input to an final fn must be public.", span).with_help(
        "Use a `public` modifier to the input variable declaration or remove the visibility modifier entirely.",
    )
}

pub(crate) fn invalid_operation_outside_finalize(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 34,
        format!("`{operation}` must be inside a `final fn` or a `final` block."),
        span,
    )
}

pub(crate) fn loop_body_contains_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 35, "Loop body contains a final context.", span)
        .with_help("Remove the final context.")
}

pub(crate) fn missing_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 36, "Function must return a value.", span)
}

pub(crate) fn cannot_have_constant_output_mode(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 38, "A returned value cannot be a constant.", span)
}

pub(crate) fn entry_point_fn_inputs_cannot_be_const(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 39, "Entry point functions cannot have constant inputs.", span)
}

pub(crate) fn can_only_call_inline_function(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 42, format!("Only regular fns can be called from {kind}."), span)
}

pub(crate) fn cannot_invoke_call_to_local_entry_point_fn(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 43, "Cannot call a local entry point fn from an entry point fn.", span)
}

pub(crate) fn strings_are_not_supported(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 45, "Strings are not yet supported.", span)
}

pub(crate) fn too_many_entry_points(max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 47,
        format!(
            "The number of entry point fns exceeds the maximum. snarkVM allows up to {max} entry point fns within a single program."
        ),
        span,
    )
}

pub(crate) fn nested_tuple_type(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 49, "A tuple type cannot contain a tuple.", span)
}

pub(crate) fn composite_data_type_cannot_contain_tuple(data_type: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 50, format!("A {data_type} cannot contain a tuple."), span)
}

pub(crate) fn function_cannot_take_tuple_as_input(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 51, "A function cannot take in a tuple as input.", span)
}

pub(crate) fn nested_tuple_expression(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 52, "A tuple expression cannot contain another tuple expression.", span)
}

pub(crate) fn expression_statement_must_be_function_call(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 53, "An expression statement must be a function call.", span)
}

pub(crate) fn lhs_must_be_identifier_or_tuple(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 55,
        "The left-hand side of a `DefinitionStatement` can only be an identifier or tuple. Note that a tuple must contain at least two elements.",
        span,
    )
}

pub(crate) fn function_cannot_input_or_output_a_record(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 57, "Only entry point fns can have a record as input or output.", span)
}

pub(crate) fn cyclic_composite_dependency(path: Vec<impl Display>, span: Span) -> Formatted {
    let path_string = path.into_iter().map(|name| format!("`{name}`")).collect::<Vec<String>>().join(" --> ");
    Formatted::error(CODE_PREFIX, CODE_MASK + 58, format!("Cyclic dependency between composites: {path_string}"), span)
}

pub(crate) fn cyclic_function_dependency(path: Vec<impl Display>) -> LeoError {
    let path_string = path.into_iter().map(|name| format!("`{name}`")).collect::<Vec<String>>().join(" --> ");
    LeoError::Backtraced(Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 59,
        format!("Cyclic dependency between functions: {path_string}"),
    ))
}

pub(crate) fn struct_cannot_have_member_mode(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 60,
        "A struct cannot have a member with mode `constant`, `private`, or `public`.",
        span,
    )
}

pub(crate) fn too_many_mappings(max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 62,
        format!(
            "The number of mappings exceeds the maximum. snarkVM allows up to {max} mappings within a single program."
        ),
        span,
    )
}

pub(crate) fn invalid_operation_inside_finalize(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 66,
        format!("`{operation}` is not a valid operand in a finalization context."),
        span,
    )
}

pub(crate) fn operation_must_be_in_final_block_or_function(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 67,
        "This operation can only be used in a final fn, a final block, or script.",
        span,
    )
}

pub(crate) fn incorrect_num_tuple_elements(identifiers: impl Display, types: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 72,
        format!("Expected a tuple with {types} elements, found one with {identifiers} elements"),
        span,
    )
}

pub(crate) fn array_too_large(size: impl Display, max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 77,
        format!("An array cannot have more than {max} elements, found one with {size} elements"),
        span,
    )
}

pub(crate) fn array_element_cannot_be_tuple(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 78, "An array cannot have a tuple as an element type", span)
}

pub(crate) fn array_element_cannot_be_record(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 79, "An array cannot have a record as an element type", span)
}

pub(crate) fn stubs_cannot_have_const_declarations(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 81,
        "Stubs can only have records, entry point fns, regular fns, mappings and imports -- found const declaration",
        span,
    )
}

pub(crate) fn stub_name_mismatch(stub_name: impl Display, program_name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 82, format!("`stub` name `{stub_name}` does not match program name `{program_name}`"), span)
        .with_help("Check that the name you used as a dependency in program.json matches the name you used to import the program in the main leo file.")
}

pub(crate) fn no_entry_points(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 83, "A program must have at least one entry point fn.", span)
}

pub(crate) fn entry_point_fn_final_invalid_output(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 86, "An entry point fn returning Final must return a Final as the final output, and in no other position return a Final.", span)
        .with_help("Example: `fn foo() -> (u8, bool, Final) {...}`")
}

pub(crate) fn cannot_reassign_final_variable(var: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 94,
        format!("Cannot reassign variable `{var}` since it has type Final."),
        span,
    )
    .with_help("Finals can only be defined as the result of final fn calls or final blocks.")
}

pub(crate) fn can_only_run_one_final_at_a_time(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 96, "Must run exactly one Final at a time", span)
        .with_help("Ex: for `f: Final` call `f.run()` to run a Final.")
}

pub(crate) fn external_call_after_final(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 102,
        format!("External calls must appear before the local final {kind}."),
        span,
    )
    .with_help(format!("Reorder your code so the external entry point fn call happens before the local final {kind}."))
}

pub(crate) fn unknown_final_consumed(fin: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 103, format!("Unknown Final consumed: `{fin}`"), span)
        .with_help("Make sure the Final is defined and consumed exactly once.")
}

pub(crate) fn not_all_finals_consumed(unconsumed: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 104, format!("Not all Finals were consumed: {unconsumed}"), span)
        .with_help(
            "Make sure all Finals are consumed exactly once. Consume by passing to a final fn call or final block.",
        )
}

pub(crate) fn entry_point_missing_final_to_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 105, "An entry point fn returning Final must return a Final.", span)
        .with_help("Instantiate a final block inside the entry point fn body so that there is a Final to return.")
}

pub(crate) fn final_fn_cannot_return_value(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 106, "A final fn is not allowed to return a value.", span)
        .with_help("Remove an output type in the function signature, and remove the return statement from the function. Note that the Final returned by final fn is automatically inferred, and must not be explicitly written.")
}

pub(crate) fn cannot_modify_external_container(operation: impl Display, kind: impl Display, span: Span) -> Formatted {
    let allowed = if kind.to_string() == "vector" {
        "`get` and `len`"
    } else if kind.to_string() == "mapping" {
        "`contains`, `get`, and `get_or_use`"
    } else {
        panic!("no other kinds expected here")
    };
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 108,
        format!("Cannot use operation `{operation}` on external {kind}s."),
        span,
    )
    .with_help(format!("The only valid operations on external {kind}s are {allowed}."))
}

pub(crate) fn final_cannot_assign_outside_conditional(
    variable: impl Display,
    kind: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 109, format!("Cannot re-assign to `{variable}` from a conditional scope to an outer scope in a final {kind}."), span)
        .with_help("This is a fundamental restriction that can often be avoided by using a ternary operator `?` or re-declaring the variable in the current scope. In the future, ARC XXXX (https://github.com/ProvableHQ/ARCs) will support more complex assignments in final fns.")
}

pub(crate) fn only_entry_point_can_return_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 110, "Only entry point fns can return a Final.", span)
        .with_help("Move this function inside the `program` block to make it an entry point fn.")
}

pub(crate) fn empty_struct(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 112, "A struct must have at least one member.", span)
}

pub(crate) fn composite_data_type_cannot_contain_final(data_type: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 114, format!("A {data_type} cannot contain a Final."), span)
}

pub(crate) fn array_element_cannot_be_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 115, "An array cannot have a Final as an element type.", span)
}

pub(crate) fn no_final_parameters(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 116, "Finals may only appear as parameters to final fn.", span)
}

pub(crate) fn type_should_be2(type_: impl Display, expected: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 117, format!("Expected {expected} but type `{type_}` was found."), span)
}

pub(crate) fn ternary_branch_mismatch(type1: impl Display, type2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 118,
        format!("Received different types `{type1}` and `{type2}` for the arms of a ternary conditional."),
        span,
    )
    .with_help("Make both branches the same type.")
}

pub(crate) fn operation_types_mismatch(
    operation: impl Display,
    type1: impl Display,
    type2: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 119,
        format!("Received different types `{type1}` and `{type2}` for the operation `{operation}`."),
        span,
    )
    .with_help("Make both operands the same type.")
}

pub(crate) fn mul_types_mismatch(type1: impl Display, type2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 120,
        format!("Received types `{type1}` and `{type2}` for the operation `*`."),
        span,
    )
    .with_help("Valid operands are two integers of the same type, two fields, or a scalar and a group.")
}

pub(crate) fn pow_types_mismatch(type1: impl Display, type2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 121,
        format!("Received types `{type1}` and `{type2}` for the operation `pow`."),
        span,
    )
    .with_help("Valid operands are two fields, or an integer base and a `u8`, `u16`, or `u32` exponent.")
}

pub(crate) fn shift_type_magnitude(operation: impl Display, rhs_type: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 122,
        format!("Received type `{rhs_type}` for the second operand of the operation `{operation}`."),
        span,
    )
    .with_help("Valid second operands are `u8`, `u16`, or `u32`")
}

pub(crate) fn unit_type_only_return(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 123,
        "The unit type () may appear only as the return type of a function.",
        span,
    )
}

pub(crate) fn cannot_reassign_mapping(var: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 125, format!("Cannot assign to the mapping `{var}`."), span)
}

pub(crate) fn records_not_allowed_inside_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 126, "records cannot be instantiated in a final context.", span)
}

pub(crate) fn annotation_error(message: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 129, format!("Invalid annotation: {message}."), span)
}

pub(crate) fn ternary_over_external_records(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 130, format!("Cannot apply ternary conditional to type `{ty}`."), span)
        .with_help("Ternary conditionals may not contain an external record type.")
}

pub(crate) fn record_prefixed_by_other_record(r1: impl Display, r2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 133,
        format!(
            "Record name `{r1}` is prefixed by the record name `{r2}`. Record names must not be prefixes of other record names."
        ),
        span,
    )
}

pub(crate) fn range_bounds_type_mismatch(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 134, "mismatched types in loop iterator range bounds", span)
}

pub(crate) fn assignment_to_external_record_member(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 135,
        format!("Cannot assign to a member of the external record `{ty}`."),
        span,
    )
}

pub(crate) fn assignment_to_external_record_cond(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 136,
        format!("Cannot assign to the external record type `{ty}` in this location."),
        span,
    )
    .with_help(
        "External record variables may not be assigned to in narrower conditional scopes than they were defined.",
    )
}

pub(crate) fn assignment_to_external_record_tuple_cond(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 137, format!("Cannot assign to the tuple type `{ty}` containing an external record in this location."), span)
        .with_help("Tuples containing external records may not be assigned to in narrower conditional scopes than they were defined.")
}

pub(crate) fn hexbin_literal_nonintegers(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 138,
        "Hex, octal, and binary literals may only be used for integer types.",
        span,
    )
}

pub(crate) fn unexpected_unsuffixed_numeral(expected: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 139,
        format!("Expected {expected} but an unsuffixed numeral was found."),
        span,
    )
}

pub(crate) fn incorrect_num_const_args(
    kind: impl Display,
    expected: impl Display,
    received: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 140,
        format!("{kind} expected `{expected}` const args, but got `{received}`"),
        span,
    )
}

pub(crate) fn bad_const_generic_type(found: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 141,
        format!(
            "A generic const parameter must be a `bool`, an integer, a `scalar`, a `group`, a `field`, or an `address`, but {found} was found"
        ),
        span,
    )
}

pub(crate) fn cannot_assign_to_generic_const_function_parameter(param: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 142, format!("Cannot assign to const parameter `{param}`"), span)
}

pub(crate) fn cannot_have_const_generics(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 143, format!("{kind} cannot have generic const parameters."), span)
}

pub(crate) fn array_too_large_for_u32(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 144, "An array length must be small enough to fit in a `u32`", span)
}

pub(crate) fn unexpected_record_const_parameters(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 145, "Records cannot be declared with generic const parameters.", span)
}

pub(crate) fn unexpected_const_args(item: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 146, format!("unexpected generic const argment for {item}."), span)
        .with_help("If this is an external struct, consider using a resolved non-generic version of it instead. External structs can't be instantiated with const arguments")
}

pub(crate) fn invalid_operation_inside_final_block(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 147,
        format!("Invalid expression in a final block. `{operation}` cannot be used directly here."),
        span,
    )
    .with_help(format!("Bind `{operation}` to a variable before the `final` block: `let val = {operation};`"))
}

pub(crate) fn illegal_final_block_location(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 148,
        "`final` blocks are only allowed inside an entry point fn returning `Final` or a script function.",
        span,
    )
    .with_help("Try moving this `final` block into an entry point fn or a script function.")
}

pub(crate) fn multiple_final_blocks_not_allowed(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 150, "An entry point fn cannot contain more than one `final` block.", span)
        .with_help("Combine the logic into a single `final` block, or restructure your code to avoid multiple final blocks within the same entry point fn.")
}

pub(crate) fn final_block_in_conditional(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 151, "`final` blocks are not allowed inside conditional blocks.", span)
        .with_help("Refactor your code to move the `final` block outside of the conditional block.")
}

pub(crate) fn final_block_cannot_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 153, "A `final` block cannot contain a `return` statement.", span)
}

pub(crate) fn cannot_assign_to_vars_outside_final_block(input: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 155,
        format!("Cannot assign to `{input}` inside a `final` block because it was declared outside the block."),
        span,
    )
}

pub(crate) fn custom(msg: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 156, msg.to_string(), span)
}

pub(crate) fn constructor_can_only_return_unit(expression: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 157,
        format!("Constructors can only return unit, but found `{expression}`."),
        span,
    )
}

pub(crate) fn none_found_non_optional(expected: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 158,
        format!("Found `none`, but the expected type `{expected}` is not an optional type."),
        span,
    )
}

pub(crate) fn optional_wrapping_unsupported(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 160, format!("The type `{ty}` cannot be wrapped in an optional."), span)
        .with_help("Optionals cannot wrap signatures, finals, mappings, tuples, vectors, records, arrays whose elements are optional-unsafe, or structures containing any such types.")
}

pub(crate) fn optional_type_not_allowed_in_mapping(ty: impl Display, kind: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 161,
        format!("The type `{ty}` is or contains an optional type which cannot be used as the {kind} in a mapping"),
        span,
    )
}

pub(crate) fn record_field_cannot_be_optional(name: impl Display, ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 162, format!("The field `{name}` in this record has type `{ty}`, which is or contains an optional type."), span)
        .with_help("Records cannot have fields that are optional or contain optionals. Consider moving the optionality outside the record.")
}

pub(crate) fn const_cannot_be_optional(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 163,
        "Constants cannot have an optional type or a type that contains an optional",
        span,
    )
}

pub(crate) fn function_cannot_take_option_as_input(name: impl Display, ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 164, format!("The input `{name}` has type `{ty}`, which is or contains an optional type and is not allowed as an input to an entry point fn."), span)
        .with_help("Inputs to entry point fn definitions cannot be optional or contain optionals. Consider moving the optionality outside the call site.")
}

pub(crate) fn function_cannot_return_option_as_output(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 165, format!("This function has an output of type `{ty}`, which is or contains an optional type and is not allowed as an output of an entry point fn."), span)
        .with_help("Outputs of entry point fns cannot be optional or contain optionals. Consider moving the optionality outside the function call.")
}

pub(crate) fn invalid_storage_type(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 166, format!("{type_} is an invalid storage type"), span)
}

pub(crate) fn storage_vectors_cannot_be_moved_or_assigned(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 167,
        "Storage vectors cannot be moved or assigned. You can only access or modify them using methods like `get`, `push`, or `pop`.",
        span,
    )
}

pub(crate) fn function_has_too_many_inputs(
    variant: impl Display,
    name: impl Display,
    limit: usize,
    actual: usize,
    span: Span,
) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 168, format!("The {variant} `{name}` has {actual} input parameters, which exceeds the allowed limit of {limit}."), span)
        .with_help("Consider reducing the number of input parameters. You might combine some parameters into a struct or refactor the function to simplify its signature.")
}

pub(crate) fn function_has_too_many_outputs(
    variant: impl Display,
    name: impl Display,
    limit: usize,
    actual: usize,
    span: Span,
) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 169, format!("The {variant} `{name}` has {actual} output parameters, which exceeds the allowed limit of {limit}."), span)
        .with_help("Consider reducing the number of output parameters. You might combine some parameters into a struct or refactor the function to simplify its signature.")
}

pub(crate) fn zero_size_struct(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 171, "A struct must have at least one member of non-zero size.", span)
}

pub(crate) fn invalid_intrinsic(intr: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 172, format!("{intr} is not a valid intrinsic."), span)
}

pub(crate) fn cannot_instantiate_external_record(loc: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 173,
        format!(
            "Cannot create external record `{loc}`. Records can only be created in the program that they are defined in"
        ),
        span,
    )
}

pub(crate) fn cannot_modify_external_storage_variable(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 174,
        "Assignment to storage variables of another program is not allowed. You can only modify storage declared in the current program.",
        span,
    )
}

pub(crate) fn no_inline_not_allowed_on_final_fn(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 175,
        "`@no_inline` is not allowed on `final fn` functions because they must always be inlined.",
        span,
    )
}

pub(crate) fn record_captured_by_final_block(var_name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 176,
        format!(
            "A `final` block cannot capture the record variable `{var_name}`. Records cannot be used in on-chain code."
        ),
        span,
    )
    .with_help(format!(
        "Extract the needed fields before the `final` block. For example: `let val = {var_name}.field_name;`"
    ))
}

pub(crate) fn dynamic_call_not_allowed_here(context: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 177,
        format!("Dynamic calls can only be made from an entry point, but found one in {context}."),
        span,
    )
}

pub(crate) fn dyn_record_field_requires_type(field: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 178,
        format!("Accessing field `{field}` on a `dyn record` requires a type annotation."),
        span,
    )
    .with_help(format!("Use `let x: <type> = r.{field};` or `r.{field} as <type>`."))
}

pub(crate) fn cannot_cast_to_dyn_record(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 179,
        format!("Cannot cast `{type_}` to `dyn record`: only concrete record types can be cast to `dyn record`."),
        span,
    )
}

pub(crate) fn dynamic_call_min_args(found: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 180,
        format!("`_dynamic_call` requires at least 3 arguments (program, network, function), but found {found}."),
        span,
    )
}

pub(crate) fn dynamic_intrinsic_wrong_arg_count(
    name: impl Display,
    expected: impl Display,
    found: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 181,
        format!("`{name}` requires {expected} arguments, but found {found}."),
        span,
    )
}

pub(crate) fn dynamic_intrinsic_missing_type_param(name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 182,
        format!("`{name}` requires exactly one type parameter, e.g. `{name}::[u64](...)`"),
        span,
    )
}

pub(crate) fn dynamic_call_input_type_count_mismatch(
    annotated: impl Display,
    actual: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 183, format!("`_dynamic_call` has {annotated} input type annotation(s) but {actual} call argument(s) were provided (excluding the 3 target arguments)."), span)
        .with_help("The number of type annotations before the return type must match the number of call arguments after program/network/function.")
}

pub(crate) fn dynamic_call_in_conditional(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 184, "Dynamic calls cannot be used inside a conditional branch.", span)
        .with_help("Move the dynamic call outside the `if`/`else` block.")
}

pub(crate) fn dynamic_call_constant_not_allowed(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 185,
        "`constant` visibility is not allowed in `_dynamic_call` type annotations.",
        span,
    )
    .with_help("Use `public` or `private` instead.")
}

pub(crate) fn dynamic_call_record_arg_requires_dyn_record(record_type: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 187,
        format!("Dynamic call argument has record type `{record_type}`, but dynamic calls require `dyn record`."),
        span,
    )
    .with_help("Cast your record value with `my_arg as dyn record`.")
}

pub(crate) fn vector_type_only_in_storage(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 189, "Vector types can only be used in storage declarations.", span)
}

pub(crate) fn multi_identifier_definition_requires_tuple(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 190,
        format!(
            "A definition with multiple identifiers requires a tuple on the right-hand side, but found type `{type_}`."
        ),
        span,
    )
    .with_help("Use a tuple expression, e.g. `let (a, b) = (x, y);`.")
}

// TypeCheckerWarning builder functions

pub(crate) fn caller_as_record_owner(record_name: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("`self.caller` used as the owner of record `{record_name}`"),
        span,
    )
    .with_help("`self.caller` may refer to a program address, which cannot spend records.")
}

pub(crate) fn no_inline_ignored(name: impl Display, reason: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 5,
        format!("`@no_inline` on `{name}` will be ignored because {reason}."),
        span,
    )
    .with_help("Remove the `@no_inline` annotation to silence this warning.")
}
pub(crate) fn comparison_of_unit_operands_is_constant(op: impl Display, value: impl Display, span: Span) -> Formatted {
    Formatted::warning(CODE_PREFIX, CODE_MASK+6, format!("Comparison `{op}` between two operands of type `()` always evaluates to `{value}` at compile time."), span).with_help(
                    "Both operands have type `()` (e.g. the return type of `Mapping::set` or other side-effecting calls), so the comparison has no runtime effect — the branch decision is baked in at compile time. If you intended to compare actual values, the operands likely need to be expressions that produce a value, not unit-returning calls.")
}
