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
    Formatted::error(CODE_PREFIX, CODE_MASK, format!("invalid assignment target: `{target}`"), span)
        .with_help("Valid assignment targets are identifiers, tuple accesses, array accesses, and struct accesses.")
}

pub(crate) fn cannot_assign_to_const_input(input: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 1, format!("cannot assign to const input `{input}`"), span).with_help(
        format!("Function inputs are immutable. Bind `{input}` to a new local with `let` and assign to that instead."),
    )
}

pub(crate) fn cannot_assign_to_const_var(var: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 2, format!("cannot assign to const variable `{var}`"), span)
        .with_help(format!("Declare `{var}` with `let` instead of `const` to make it mutable."))
}

pub(crate) fn type_should_be(type_: impl Display, expected: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 3, format!("expected type `{expected}`, found type `{type_}`"), span)
        .with_help(format!(
            "Change the expression to produce a `{expected}`, or cast it with `as {expected}` if a conversion exists."
        ))
}

pub(crate) fn could_not_determine_type(expr: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 4, format!("could not determine the type of `{expr}`"), span)
        .with_help("Add an explicit type annotation, e.g. `let x: u32 = …;`, so the type can be inferred.")
}

pub(crate) fn unknown_sym(kind: impl Display, sym: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, format!("unknown {kind} `{sym}`"), span).with_help(format!(
        "Check `{sym}` for typos and confirm it is declared in this scope. If it lives in another program, import it with the program-qualified name (e.g. `credits.aleo::credits`)."
    ))
}

pub(crate) fn incorrect_num_args_to_call(expected: impl Display, received: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 6,
        format!("this call expected {expected} argument(s), but {received} were supplied"),
        span,
    )
    .with_help("Pass exactly the number of arguments the function's signature declares.")
}

pub(crate) fn invalid_int_value(value: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 8, format!("the value `{value}` is not a valid `{type_}`"), span)
        .with_help(format!("Use a literal that fits within the range of `{type_}`."))
}

pub(crate) fn incorrect_num_composite_members(expected: impl Display, received: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 12,
        format!("expected {expected} member(s), but {received} were provided"),
        span,
    )
    .with_help("Initialize every member declared on the struct or record, no more and no less.")
}

pub(crate) fn missing_composite_member(composite: impl Display, member: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 13,
        format!("initializer for `{composite}` is missing member `{member}`"),
        span,
    )
    .with_help(format!("Add `{member}: <value>` to the initializer."))
}

pub(crate) fn duplicate_struct_member(member_name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 15, format!("struct field `{member_name}` is already declared"), span)
        .with_help(format!("Remove or rename the duplicate `{member_name}` field."))
}

pub(crate) fn duplicate_record_variable(variable_name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 16, format!("record field `{variable_name}` is already declared"), span)
        .with_help(format!("Remove or rename the duplicate `{variable_name}` field."))
}

pub(crate) fn undefined_type(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 17, format!("the type `{type_}` is not found in the current scope"), span)
        .with_help("Check the type name for typos. If it comes from another program, qualify it with the program name, e.g. `credits.aleo::credits` instead of `credits`.")
}

pub(crate) fn invalid_composite_variable(variable: impl Display, composite: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 18, format!("`{variable}` is not a member of `{composite}`"), span)
        .with_help(format!("Check the field name for typos and confirm `{variable}` is declared on `{composite}`."))
}

pub(crate) fn required_record_variable(name: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 19,
        format!("the `record` type requires the field `{name}: {type_}`"),
        span,
    )
    .with_help(format!("Add `{name}: {type_}` to the record definition."))
}

pub(crate) fn record_var_wrong_type(name: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 20,
        format!("the field `{name}` in a `record` must have type `{type_}`"),
        span,
    )
    .with_help(format!("Change the field's type to `{type_}`."))
}

pub(crate) fn incorrect_tuple_length(expected: impl Display, actual: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 22,
        format!("expected a tuple of length {expected}, found length {actual}"),
        span,
    )
    .with_help("Adjust the tuple expression so its arity matches the expected type.")
}

pub(crate) fn tuple_out_of_range(index: impl Display, length: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 24,
        format!("tuple index `{index}` is out of range for a tuple of length {length}"),
        span,
    )
    .with_help(format!("Tuple indices are zero-based, so the valid range is `0` to `{length} - 1`."))
}

pub(crate) fn unreachable_code_after_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 25, "cannot reach the following statement", span)
        .with_help("All paths above this point already return. Remove the unreachable code or rework the control flow.")
}

pub(crate) fn loop_body_contains_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 26, "loop body contains a return statement or always returns", span)
        .with_help("Move the `return` out of the loop, or add a branch that does not return so the loop can iterate.")
}

pub(crate) fn unknown_annotation(annotation: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 27, format!("unknown annotation `{annotation}`"), span).with_help(
        "Check the annotation name for typos. See the Leo documentation for the list of supported annotations.",
    )
}

pub(crate) fn function_inputs_cannot_have_modes(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 28, format!("{kind} inputs cannot have visibility modes"), span)
        .with_help("Remove the `public`, `private`, or `constant` modifier.")
}

pub(crate) fn struct_or_record_cannot_contain_record(
    parent: impl Display,
    child: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 30, "a struct or record cannot contain another record", span).with_help(
        format!("Remove the record field of type `{child}` from `{parent}`, or store its individual fields instead."),
    )
}

pub(crate) fn invalid_mapping_type(component: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 31,
        format!("a mapping's {component} cannot be a `{type_}`"),
        span,
    )
    .with_help("Mapping keys and values must be primitive or simple composite types. Replace this type with a supported one.")
}

pub(crate) fn function_outputs_cannot_have_modes(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 32, format!("{kind} outputs cannot have visibility modes"), span)
        .with_help("Remove the `public`, `private`, or `constant` modifier.")
}

pub(crate) fn invalid_operation_outside_finalize(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 34,
        format!("`{operation}` must be inside a `final fn` or a `final` block"),
        span,
    )
    .with_help(format!("Move the `{operation}` call into a `final fn` or wrap it in a `final {{ … }}` block."))
}

pub(crate) fn invalid_operation_outside_onchain(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 34,
        format!("`{operation}` must be inside a `final fn`, a `final` block, or a `view fn`"),
        span,
    )
    .with_help(format!(
        "Move the `{operation}` call into a `final fn`, a `view fn`, or wrap it in a `final {{ … }}` block."
    ))
}

pub(crate) fn loop_body_contains_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 35, "loop body contains a `final` context", span)
        .with_help("Move the `final` block outside of the loop.")
}

pub(crate) fn missing_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 36, "function must return a value", span).with_help(
        "Ensure every path through the function ends in a `return` expression matching the declared output type.",
    )
}

pub(crate) fn cannot_have_constant_output_mode(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 38, "a returned value cannot have `constant` visibility", span)
        .with_help("Remove the `constant` modifier from the output, or return a non-`constant` value.")
}

pub(crate) fn entry_point_fn_inputs_cannot_be_const(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 39, "entry point functions cannot have constant inputs", span)
        .with_help("Remove the `constant` modifier from the input, or make this function a regular `fn` instead of an entry point.")
}

pub(crate) fn can_only_call_inline_function(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 42, format!("only regular `fn`s can be called from {kind}"), span)
        .with_help("Move the callee out of the `program` block, or call a regular `fn` from this site instead.")
}

pub(crate) fn call_final_fn_outside_finalize_context(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 42, "a `final fn` can only be called from a finalize context", span)
        .with_help(
            "Wrap the call in a `final { … }` block inside an entry point, or move the calling code into a `final fn`.",
        )
}

pub(crate) fn call_reaches_offchain_from_onchain_scope(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 42, "this call reads off-chain-only values from on-chain code", span)
        .with_note("The caller and signer only exist while a transition is being proved off-chain")
        .with_help(
            "Read the value in an off-chain scope (an entry point body outside its `final` block) and pass it in.",
        )
}

pub(crate) fn cannot_invoke_call_to_local_entry_point_fn(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 43, "cannot call a local entry point fn from an entry point fn", span)
        .with_help("Refactor the shared logic into a regular `fn` and call that from both entry points.")
}

pub(crate) fn strings_are_not_supported(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 45, "strings are not yet supported", span).with_help(
        "Represent text as a fixed-size array of integers (e.g. `[u8; 32]`) until first-class strings are added.",
    )
}

pub(crate) fn too_many_entry_points(max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 47,
        format!("the number of entry point fns exceeds the maximum of {max} per program"),
        span,
    )
    .with_help(format!("Reduce the program to at most {max} entry point fns, or split it across multiple programs."))
}

pub(crate) fn nested_tuple_type(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 49, "a tuple type cannot contain a tuple", span)
        .with_help("Flatten the nested tuple into a single tuple, or wrap the inner tuple in a `struct`.")
}

pub(crate) fn composite_data_type_cannot_contain_tuple(data_type: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 50, format!("a {data_type} cannot contain a tuple"), span).with_help(
        format!(
            "Flatten the tuple into separate fields, or wrap it in a `struct` before storing it in the {data_type}."
        ),
    )
}

pub(crate) fn function_cannot_take_tuple_as_input(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 51, "a function cannot take a tuple as input", span)
        .with_help("Pass the tuple's elements as separate parameters, or wrap them in a `struct`.")
}

pub(crate) fn nested_tuple_expression(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 52, "a tuple expression cannot contain another tuple expression", span)
        .with_help("Flatten the expression into a single tuple, or assign the inner tuple to a `let` binding first.")
}

pub(crate) fn expression_statement_must_be_function_call(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 53, "an expression statement must be a function call", span).with_help(
        "Bind the expression's result to a variable with `let`, or remove the statement if it has no effect.",
    )
}

pub(crate) fn lhs_must_be_identifier_or_tuple(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 55,
        "the left-hand side of a definition must be an identifier or a tuple",
        span,
    )
    .with_help("Use `let x = …;` for a single binding, or `let (a, b) = …;` for a destructuring tuple of at least two elements.")
}

pub(crate) fn function_cannot_input_or_output_a_record(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 57, "only entry point fns can have a record as input or output", span)
        .with_help("Move the function inside the `program` block to make it an entry point fn, or change its signature to avoid records.")
}

pub(crate) fn cyclic_composite_dependency(path: Vec<impl Display>, span: Span) -> Formatted {
    let path_string = path.into_iter().map(|name| format!("`{name}`")).collect::<Vec<String>>().join(" --> ");
    Formatted::error(CODE_PREFIX, CODE_MASK + 58, format!("cyclic dependency between composites: {path_string}"), span)
        .with_help("Break the cycle by removing one of the references. Composite types cannot contain themselves transitively.")
}

pub(crate) fn cyclic_function_dependency(path: Vec<impl Display>) -> LeoError {
    let path_string = path.into_iter().map(|name| format!("`{name}`")).collect::<Vec<String>>().join(" --> ");
    LeoError::Backtraced(
        Backtraced::error(CODE_PREFIX, CODE_MASK + 59, format!("cyclic dependency between functions: {path_string}"))
            .with_help("Break the cycle by removing one of the calls. Recursion is not supported in Leo."),
    )
}

pub(crate) fn struct_cannot_have_member_mode(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 60,
        "a struct cannot have a member with mode `constant`, `private`, or `public`",
        span,
    )
    .with_help("Remove the visibility modifier from the struct field. Visibility is only meaningful on function inputs and outputs.")
}

pub(crate) fn too_many_mappings(max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 62,
        format!("the number of mappings exceeds the maximum of {max} per program"),
        span,
    )
    .with_help(format!("Reduce the program to at most {max} mappings, or split storage across multiple programs."))
}

pub(crate) fn invalid_operation_inside_finalize(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 66, format!("`{operation}` is not valid in a finalization context"), span)
        .with_help(format!("Move `{operation}` out of the `final` block. On-chain code cannot perform this operation."))
}

pub(crate) fn operation_must_be_in_final_block_or_function(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 67,
        "this operation can only be used in a `final fn`, a `final` block, or a script",
        span,
    )
    .with_help(
        "Move the operation into a `final fn`, a `final { … }` block inside an entry point, or a script function.",
    )
}

pub(crate) fn incorrect_num_tuple_elements(identifiers: impl Display, types: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 72,
        format!("expected a tuple with {types} elements, found one with {identifiers} elements"),
        span,
    )
    .with_help("Match the destructuring pattern to the tuple's arity exactly.")
}

pub(crate) fn array_too_large(size: impl Display, max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 77,
        format!("an array cannot have more than {max} elements, found one with {size} elements"),
        span,
    )
    .with_help(format!("Reduce the array length to at most {max}, or split the data across multiple arrays."))
}

pub(crate) fn array_element_cannot_be_tuple(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 78, "an array cannot have a tuple as its element type", span)
        .with_help("Wrap the tuple in a `struct` and use that as the element type, or use multiple parallel arrays.")
}

pub(crate) fn array_element_cannot_be_record(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 79, "an array cannot have a record as its element type", span)
        .with_help("Store the records' field values in parallel arrays, or reference them by id stored in a mapping.")
}

pub(crate) fn stubs_cannot_have_const_declarations(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 81,
        "stubs can only declare records, entry point fns, regular fns, mappings, and imports",
        span,
    )
    .with_help("Remove the `const` declaration. Stubs describe an external program's surface, not its constants.")
}

pub(crate) fn stub_name_mismatch(stub_name: impl Display, program_name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 82,
        format!("`stub` name `{stub_name}` does not match program name `{program_name}`"),
        span,
    )
    .with_help("Make sure the name you used as a dependency in `program.json` matches the name you imported the program with in the Leo source.")
}

pub(crate) fn no_entry_points(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 83, "a program must have at least one entry point fn", span)
        .with_help("Define at least one function inside the `program` block.")
}

pub(crate) fn missing_constructor(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 84, "a program must declare a constructor", span)
        .with_primary_span_underline()
        .with_help("Add a constructor such as `@noupgrade constructor() {}` inside the `program` block.")
}

pub(crate) fn entry_point_fn_final_invalid_output(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 86,
        "an entry point fn returning `Final` must return it as the last output and nowhere else",
        span,
    )
    .with_help("Example: `fn foo() -> (u8, bool, Final) { … }`. `Final` must be the last element of the return type.")
}

pub(crate) fn cannot_reassign_final_variable(var: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 94,
        format!("cannot reassign variable `{var}` because it has type `Final`"),
        span,
    )
    .with_help("`Final`s can only be defined as the result of a `final fn` call or a `final` block, they cannot be overwritten.")
}

pub(crate) fn can_only_run_one_final_at_a_time(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 96, "must run exactly one `Final` at a time", span)
        .with_help("For a `Final` value `f`, call `f.run()` to execute it. Run one per entry point fn.")
}

pub(crate) fn external_call_after_final(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 102,
        format!("external calls must appear before the local final {kind}"),
        span,
    )
    .with_help(format!("Reorder the code so the external entry point fn call happens before the local final {kind}."))
}

pub(crate) fn unknown_final_consumed(fin: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 103, format!("unknown `Final` consumed: `{fin}`"), span)
        .with_help("Make sure the `Final` is defined in this scope and consumed exactly once.")
}

pub(crate) fn not_all_finals_consumed(unconsumed: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 104, format!("not all `Final`s were consumed: {unconsumed}"), span)
        .with_help(
            "Each `Final` must be consumed exactly once. Pass it to a `final fn` call or use it inside a `final` block.",
        )
}

pub(crate) fn entry_point_missing_final_to_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 105, "an entry point fn returning `Final` must return a `Final`", span)
        .with_help("Add a `final { … }` block inside the entry point fn body and return its result.")
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
        format!("cannot use operation `{operation}` on external {kind}s"),
        span,
    )
    .with_help(format!("The only valid operations on external {kind}s are {allowed}."))
}

pub(crate) fn final_cannot_assign_outside_conditional(
    variable: impl Display,
    kind: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 109,
        format!("cannot reassign `{variable}` from a conditional scope to an outer scope in a final {kind}"),
        span,
    )
    .with_help("Use a ternary `?` or redeclare the variable in the current scope. Future ARC work will lift this restriction (see https://github.com/ProvableHQ/ARCs).")
}

pub(crate) fn only_entry_point_can_return_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 110, "only entry point fns can return a `Final`", span).with_help(
        "Move this function inside the `program` block to make it an entry point fn, or change its return type.",
    )
}

pub(crate) fn empty_struct(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 112, "a struct must have at least one member", span)
        .with_help("Add at least one field to the struct, or remove the declaration if it is unused.")
}

pub(crate) fn composite_data_type_cannot_contain_final(data_type: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 114, format!("a {data_type} cannot contain a `Final`"), span)
        .with_help("`Final`s exist only as ephemeral execution receipts and cannot be stored inside composite types.")
}

pub(crate) fn array_element_cannot_be_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 115, "an array cannot have a `Final` as its element type", span)
        .with_help("`Final`s exist only as ephemeral execution receipts and cannot be stored in arrays.")
}

pub(crate) fn no_final_parameters(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 116, "`Final`s may only appear as parameters to a `final fn`", span)
        .with_help("Remove the `Final` parameter, or change the function to a `final fn` if it is consuming a `Final`.")
}

pub(crate) fn type_should_be2(type_: impl Display, expected: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 117, format!("expected {expected}, but type `{type_}` was found"), span)
        .with_help(format!("Change the expression to produce {expected}."))
}

pub(crate) fn ternary_branch_mismatch(type1: impl Display, type2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 118,
        format!("ternary branches have different types: `{type1}` and `{type2}`"),
        span,
    )
    .with_help("Make both branches produce the same type, e.g. by casting one of them.")
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
        format!("operands of `{operation}` have different types: `{type1}` and `{type2}`"),
        span,
    )
    .with_help("Make both operands the same type, e.g. by casting one of them with `as`.")
}

pub(crate) fn mul_types_mismatch(type1: impl Display, type2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 120,
        format!("operands of `*` have unsupported types: `{type1}` and `{type2}`"),
        span,
    )
    .with_help("Valid operands for `*` are two integers of the same type, two `field`s, or a `scalar` and a `group`.")
}

pub(crate) fn pow_types_mismatch(type1: impl Display, type2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 121,
        format!("operands of `pow` have unsupported types: `{type1}` and `{type2}`"),
        span,
    )
    .with_help("Valid operands for `pow` are two `field`s, or an integer base with a `u8`, `u16`, or `u32` exponent.")
}

pub(crate) fn shift_type_magnitude(operation: impl Display, rhs_type: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 122,
        format!("the right operand of `{operation}` has unsupported type `{rhs_type}`"),
        span,
    )
    .with_help("Valid right operands for shift operations are `u8`, `u16`, or `u32`.")
}

pub(crate) fn unit_type_only_return(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 123,
        "the unit type `()` may appear only as the return type of a function",
        span,
    )
    .with_help("Replace `()` with a concrete type, or remove the annotation where it is not allowed.")
}

pub(crate) fn cannot_reassign_mapping(var: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 125, format!("cannot assign to the mapping `{var}`"), span).with_help(
        format!("Modify the mapping with `{var}.set(key, value)` or related operations instead of assigning to it."),
    )
}

pub(crate) fn records_not_allowed_inside_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 126, "records cannot be instantiated in a final context", span).with_help(
        "Records exist only off-chain. Construct them inside an entry point fn body before any `final` block.",
    )
}

pub(crate) fn annotation_error(message: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 129, format!("invalid annotation: {message}"), span)
        .with_help("Check the annotation syntax against the Leo documentation.")
}

pub(crate) fn ternary_over_external_records(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 130, format!("cannot apply a ternary conditional to type `{ty}`"), span)
        .with_help("Ternary conditionals cannot operate on external record types. Branch on individual fields instead.")
}

pub(crate) fn record_prefixed_by_other_record(r1: impl Display, r2: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 133,
        format!("record name `{r1}` is prefixed by the record name `{r2}`"),
        span,
    )
    .with_help(format!("Rename `{r1}` so it does not start with the name of another record. Record names must not be prefixes of each other."))
}

pub(crate) fn range_bounds_type_mismatch(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 134, "mismatched types in loop iterator range bounds", span)
        .with_help("Use bounds of the same integer type, e.g. `0u32..10u32`.")
}

pub(crate) fn assignment_to_external_record_member(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 135,
        format!("cannot assign to a member of the external record `{ty}`"),
        span,
    )
    .with_help("External records are immutable from the consumer's side. Produce a new record value with the desired field instead.")
}

pub(crate) fn assignment_to_external_record_cond(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 136,
        format!("cannot assign to the external record `{ty}` in this conditional scope"),
        span,
    )
    .with_help(
        "External record variables cannot be assigned in conditional scopes narrower than where they were defined. Move the assignment to the same scope as the declaration.",
    )
}

pub(crate) fn assignment_to_external_record_tuple_cond(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 137,
        format!("cannot assign to the tuple `{ty}` containing an external record in this conditional scope"),
        span,
    )
    .with_help("Tuples containing external records cannot be assigned in conditional scopes narrower than where they were defined. Move the assignment to the same scope as the declaration.")
}

pub(crate) fn hexbin_literal_nonintegers(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 138,
        "hex, octal, and binary literals may only be used for integer types",
        span,
    )
    .with_help("Use a decimal literal for non-integer types, e.g. `1field` or `1group`.")
}

pub(crate) fn unexpected_unsuffixed_numeral(expected: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 139,
        format!("expected {expected}, but an unsuffixed numeral was found"),
        span,
    )
    .with_help("Add a type suffix to the literal, e.g. `1u32` or `1field`.")
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
        format!("{kind} expected {expected} const argument(s), but got {received}"),
        span,
    )
    .with_help("Match the const argument count to the declared generic const parameters.")
}

pub(crate) fn bad_const_generic_type(found: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 141,
        format!("a generic const parameter must be a primitive type, but `{found}` was found"),
        span,
    )
    .with_help("Use one of: `bool`, an integer type, `scalar`, `group`, `field`, or `address`.")
}

pub(crate) fn cannot_assign_to_generic_const_function_parameter(param: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 142, format!("cannot assign to const parameter `{param}`"), span)
        .with_help(format!("Bind `{param}` to a new local with `let` and assign to that instead."))
}

pub(crate) fn cannot_have_const_generics(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 143, format!("{kind} cannot have generic const parameters"), span)
        .with_help(format!("Remove the const generic parameters from this {kind}."))
}

pub(crate) fn array_too_large_for_u32(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 144, "an array length must fit into a `u32`", span)
        .with_help("Reduce the array length below 2^32, or split the data across multiple arrays.")
}

pub(crate) fn unexpected_record_const_parameters(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 145, "records cannot be declared with generic const parameters", span)
        .with_help("Remove the const generic parameters from the record definition.")
}

pub(crate) fn unexpected_const_args(item: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 146, format!("unexpected generic const argument for {item}"), span)
        .with_help("If this is an external struct, use a resolved non-generic version instead. External structs cannot be instantiated with const arguments.")
}

pub(crate) fn invalid_operation_inside_final_block(operation: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 147,
        format!("`{operation}` cannot be used directly inside a `final` block"),
        span,
    )
    .with_help(format!("Bind `{operation}` to a variable before the `final` block: `let val = {operation};`."))
}

pub(crate) fn illegal_final_block_location(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 148,
        "`final` blocks are only allowed inside an entry point fn returning `Final` or a script function",
        span,
    )
    .with_help("Move this `final` block into an entry point fn or a script function.")
}

pub(crate) fn multiple_final_blocks_not_allowed(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 150, "an entry point fn cannot contain more than one `final` block", span)
        .with_help("Combine the logic into a single `final` block, or restructure the code so only one runs.")
}

pub(crate) fn final_block_in_conditional(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 151, "`final` blocks are not allowed inside conditional blocks", span)
        .with_help("Move the `final` block outside of the surrounding `if`/`else`.")
}

pub(crate) fn final_block_cannot_return(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 153, "a `final` block cannot contain a `return` statement", span)
        .with_help("Move the `return` to the outer function. The `Final` value is the implicit result of the block.")
}

pub(crate) fn cannot_assign_to_vars_outside_final_block(input: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 155,
        format!("cannot assign to `{input}` inside a `final` block because it was declared outside the block"),
        span,
    )
    .with_help(format!("Declare `{input}` inside the `final` block, or move the assignment outside of it."))
}

pub(crate) fn custom(msg: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 156, msg.to_string(), span)
}

pub(crate) fn constructor_can_only_return_unit(expression: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 157,
        format!("constructors can only return unit `()`, but found `{expression}`"),
        span,
    )
    .with_help("Remove the return value, or omit the `return` statement entirely.")
}

pub(crate) fn none_found_non_optional(expected: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 158,
        format!("found `none`, but the expected type `{expected}` is not optional"),
        span,
    )
    .with_help(format!("Provide a value of type `{expected}`, or change the expected type to `{expected}?`."))
}

pub(crate) fn optional_wrapping_unsupported(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 160, format!("the type `{ty}` cannot be wrapped in an optional"), span)
        .with_help("Optionals cannot wrap signatures, finals, mappings, tuples, vectors, records, arrays whose elements are optional-unsafe, or structures containing any such types.")
}

pub(crate) fn optional_type_not_allowed_in_mapping(ty: impl Display, kind: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 161,
        format!("the type `{ty}` contains an optional and cannot be used as the {kind} in a mapping"),
        span,
    )
    .with_help(format!("Use a non-optional type for the mapping's {kind}, or represent absence with a sentinel value."))
}

pub(crate) fn record_field_cannot_be_optional(name: impl Display, ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 162,
        format!("the field `{name}` has type `{ty}`, which is or contains an optional"),
        span,
    )
    .with_help("Records cannot have optional fields. Move the optionality outside the record, or use a sentinel value.")
}

pub(crate) fn const_cannot_be_optional(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 163,
        "constants cannot have an optional type or a type that contains an optional",
        span,
    )
    .with_help("Provide a concrete value at the declaration site, or move the optionality to a runtime variable.")
}

pub(crate) fn function_cannot_take_option_as_input(name: impl Display, ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 164,
        format!("the input `{name}` has type `{ty}`, which is or contains an optional"),
        span,
    )
    .with_help("Inputs to entry point fns cannot be optional. Move the optionality outside the call site.")
}

pub(crate) fn function_cannot_return_option_as_output(ty: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 165,
        format!("this function has output type `{ty}`, which is or contains an optional"),
        span,
    )
    .with_help("Outputs of entry point fns cannot be optional. Move the optionality outside the call site.")
}

pub(crate) fn invalid_storage_type(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 166, format!("`{type_}` is not a valid storage type"), span)
        .with_help("Use a primitive type, a struct of primitives, or a supported container as the storage type.")
}

pub(crate) fn storage_vectors_cannot_be_moved_or_assigned(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 167, "storage vectors cannot be moved or assigned", span)
        .with_help("Access or modify storage vectors only through methods like `get`, `push`, or `pop`.")
}

pub(crate) fn function_has_too_many_inputs(
    variant: impl Display,
    name: impl Display,
    limit: usize,
    actual: usize,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 168,
        format!("the {variant} `{name}` has {actual} input parameters, which exceeds the allowed limit of {limit}"),
        span,
    )
    .with_help(
        "Reduce the number of inputs by grouping related parameters into a `struct`, or splitting the function in two.",
    )
}

pub(crate) fn function_has_too_many_outputs(
    variant: impl Display,
    name: impl Display,
    limit: usize,
    actual: usize,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 169,
        format!("the {variant} `{name}` has {actual} output parameters, which exceeds the allowed limit of {limit}"),
        span,
    )
    .with_help(
        "Reduce the number of outputs by grouping related values into a `struct`, or splitting the function in two.",
    )
}

pub(crate) fn zero_size_struct(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 171, "a struct must have at least one member of non-zero size", span)
        .with_help("Add a field of non-zero size, or remove the struct if it is unused.")
}

pub(crate) fn invalid_intrinsic(intr: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 172, format!("`{intr}` is not a valid intrinsic"), span).with_help(
        "Check the intrinsic name for typos. See the Leo documentation for the list of supported intrinsics.",
    )
}

pub(crate) fn cannot_instantiate_external_record(loc: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 173,
        format!("cannot create external record `{loc}`"),
        span,
    )
    .with_help("Records can only be created in the program that defines them. Call an entry point fn of that program to obtain one.")
}

pub(crate) fn cannot_modify_external_storage_variable(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 174,
        "cannot assign to a storage variable of another program",
        span,
    )
    .with_help("Storage can only be modified by the program that declares it. Call an entry point fn of that program instead.")
}

pub(crate) fn no_inline_not_allowed_on_final_fn(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 175, "`@no_inline` is not allowed on `final fn` functions", span)
        .with_help("Remove the `@no_inline` annotation. `final fn`s must always be inlined.")
}

pub(crate) fn record_captured_by_final_block(var_name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 176,
        format!("a `final` block cannot capture the record variable `{var_name}`"),
        span,
    )
    .with_help(format!(
        "Records cannot be used in on-chain code. Extract the needed fields before the block, e.g. `let val = {var_name}.field_name;`."
    ))
}

pub(crate) fn dynamic_call_not_allowed_here(context: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 177,
        format!("dynamic calls can only be made from an entry point fn, but found one in {context}"),
        span,
    )
    .with_help("Move the dynamic call into an entry point fn body.")
}

pub(crate) fn dyn_record_field_requires_type(field: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 178,
        format!("accessing field `{field}` on a `dyn record` requires a type annotation"),
        span,
    )
    .with_help(format!("Use `let x: <type> = r.{field};` or `r.{field} as <type>`."))
}

pub(crate) fn cannot_cast_to_dyn_record(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 179, format!("cannot cast `{type_}` to `dyn record`"), span).with_help(
        "Only concrete record types can be cast to `dyn record`. Construct or obtain a concrete record first.",
    )
}

pub(crate) fn dynamic_call_min_args(found: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 180,
        format!(
            "`_dynamic_call` requires at least 3 arguments (program, network, function), but {found} were supplied"
        ),
        span,
    )
    .with_help("Pass the program id, network id, and function name as the first three arguments.")
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
        format!("`{name}` requires {expected} argument(s), but {found} were supplied"),
        span,
    )
    .with_help(format!("Pass exactly {expected} argument(s) to `{name}`."))
}

pub(crate) fn dynamic_intrinsic_missing_type_param(name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 182, format!("`{name}` requires exactly one type parameter"), span)
        .with_help(format!("Provide the type parameter explicitly, e.g. `{name}::[u64](…)`."))
}

pub(crate) fn dynamic_call_input_type_count_mismatch(
    annotated: impl Display,
    actual: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 183,
        format!(
            "`_dynamic_call` has {annotated} input type annotation(s) but {actual} call argument(s) were provided (excluding the 3 target arguments)"
        ),
        span,
    )
    .with_help("The number of type annotations before the return type must match the number of call arguments after program/network/function.")
}

pub(crate) fn dynamic_call_in_conditional(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 184, "dynamic calls cannot be used inside a conditional branch", span)
        .with_help("Move the dynamic call outside the surrounding `if`/`else` block.")
}

pub(crate) fn dynamic_call_constant_not_allowed(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 185,
        "`constant` visibility is not allowed in `_dynamic_call` type annotations",
        span,
    )
    .with_help("Use `public` or `private` instead.")
}

pub(crate) fn dynamic_call_record_arg_requires_dyn_record(record_type: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 187,
        format!("dynamic call argument has record type `{record_type}`, but dynamic calls require `dyn record`"),
        span,
    )
    .with_help("Cast the argument with `<value> as dyn record`.")
}

pub(crate) fn vector_type_only_in_storage(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 189, "vector types can only be used in storage declarations", span)
        .with_help("Replace the vector with a fixed-size array, or move the value into a `storage` declaration.")
}

pub(crate) fn multi_identifier_definition_requires_tuple(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 190,
        format!(
            "a definition with multiple identifiers requires a tuple on the right-hand side, but found type `{type_}`"
        ),
        span,
    )
    .with_help("Use a tuple expression, e.g. `let (a, b) = (x, y);`.")
}

pub(crate) fn storage_op_requires_path_receiver(
    module: impl Display,
    operation: impl Display,
    kind: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 191,
        format!("the receiver of `{module}::{operation}` must be a {kind}"),
        span,
    )
    .with_help(format!("Call `{operation}` directly on a declared {kind}, not on a temporary expression."))
}

pub(crate) fn cannot_have_mode(kind: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 192, format!("a {kind} cannot have a visibility mode"), span)
        .with_help("Remove the `public` or `private` modifier.")
        .with_note(
            "Records lower to a `.record` marker and `Final`s to a `.future`, neither of which carries a visibility.",
        )
}

pub(crate) fn inaccessible_item(kind: impl Display, item: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 193, format!("{kind} `{item}` is not accessible from this module"), span)
        .with_help(format!(
            "Add `export` to the declaration of `{item}` in its defining module to make it accessible from other modules."
        ))
}

// TypeCheckerWarning builder functions

pub(crate) fn caller_as_record_owner(record_name: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("`std::ctx::caller()` used as the owner of record `{record_name}`"),
        span,
    )
    .with_help("`std::ctx::caller()` may return a program address, which cannot spend records. Use `std::ctx::signer()` if you want the user that initiated the transaction.")
}

pub(crate) fn no_inline_ignored(name: impl Display, reason: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 5,
        format!("`@no_inline` on `{name}` will be ignored because {reason}"),
        span,
    )
    .with_help("Remove the `@no_inline` annotation to silence this warning.")
}

pub(crate) fn comparison_of_unit_operands_is_constant(op: impl Display, value: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 6,
        format!("comparison `{op}` between two operands of type `()` always evaluates to `{value}` at compile time"),
        span,
    )
    .with_help(
        "Both operands are unit values (e.g. the return of `Mapping::set` or other side-effecting calls), so the comparison has no runtime effect. If you meant to compare values, change the operands to expressions that produce non-unit values.",
    )
}
