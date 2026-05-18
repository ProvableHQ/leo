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

use leo_errors::{Backtraced, Formatted};
use leo_span::Span;
use std::fmt::Display;

const CODE_PREFIX: &str = "CHI";
const CODE_MASK: i32 = 12000;

pub(crate) fn interface_not_found(name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK, format!("interface `{name}` not found"), span)
        .with_help("Make sure the interface is defined in the current program or an imported program.")
}

pub(crate) fn conflicting_interface_member(
    member_name: impl Display,
    interface_name: impl Display,
    parent_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!(
            "interface `{interface_name}` has a conflicting definition for `{member_name}` inherited from `{parent_name}`"
        ),
        span,
    )
    .with_help("Members with the same name must have identical signatures. Align both definitions, or rename one of them.")
}

pub(crate) fn missing_interface_function(
    func_name: impl Display,
    interface_name: impl Display,
    program_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!(
            "program `{program_name}` implements interface `{interface_name}` but is missing the required function `{func_name}`"
        ),
        span,
    )
    .with_help(format!("Add the missing function `{func_name}` with the exact signature specified in `{interface_name}`."))
}

pub(crate) fn missing_interface_record(
    record_name: impl Display,
    interface_name: impl Display,
    program_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 3,
        format!(
            "program `{program_name}` implements interface `{interface_name}` but is missing the required record `{record_name}`"
        ),
        span,
    )
    .with_help(format!("Add a `record {record_name}` definition matching the shape declared by `{interface_name}`."))
}

pub(crate) fn signature_mismatch(
    func_name: impl Display,
    interface_name: impl Display,
    expected: impl Display,
    found: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!(
            "function `{func_name}` does not match the signature required by interface `{interface_name}`\n\
             Expected:\n\
             {expected}\n\
             Found:\n\
             {found}"
        ),
        span,
    )
    .with_help("Function signatures must match exactly: same parameter types, modes, order, and return type.")
}

pub(crate) fn not_an_interface(name: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, format!("`{name}` is not an interface"), span)
        .with_help("Only interface declarations can be inherited from. Check the name for typos.")
}

pub(crate) fn missing_interface_mapping(
    mapping_name: impl Display,
    interface_name: impl Display,
    program_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 6,
        format!(
            "program `{program_name}` implements interface `{interface_name}` but is missing the required mapping `{mapping_name}`"
        ),
        span,
    )
    .with_help(format!("Add a `mapping {mapping_name}` definition with the key/value types declared by `{interface_name}`."))
}

pub(crate) fn missing_interface_storage(
    storage_name: impl Display,
    interface_name: impl Display,
    program_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 7,
        format!(
            "program `{program_name}` implements interface `{interface_name}` but is missing the required storage variable `{storage_name}`"
        ),
        span,
    )
    .with_help(format!("Add a `storage {storage_name}` definition with the type declared by `{interface_name}`."))
}

pub(crate) fn mapping_type_mismatch(
    mapping_name: impl Display,
    interface_name: impl Display,
    expected_key: impl Display,
    expected_value: impl Display,
    found_key: impl Display,
    found_value: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 8,
        format!(
            "mapping `{mapping_name}` does not match the type required by interface `{interface_name}`\n\
             Expected: {expected_key} => {expected_value}\n\
             Found: {found_key} => {found_value}"
        ),
        span,
    )
    .with_help("Mapping key and value types must match exactly.")
}

pub(crate) fn storage_type_mismatch(
    storage_name: impl Display,
    interface_name: impl Display,
    expected: impl Display,
    found: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 9,
        format!(
            "storage variable `{storage_name}` does not match the type required by interface `{interface_name}`\n\
             Expected: {expected}\n\
             Found: {found}"
        ),
        span,
    )
    .with_help("Storage variable types must match exactly.")
}

pub(crate) fn cyclic_interface_inheritance(path: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 10, format!("cyclic interface inheritance detected: {path}")).with_help(
        "Interface inheritance must be acyclic. Break the cycle by removing one of the parent relationships.",
    )
}

pub(crate) fn conflicting_record_field(
    field_name: impl Display,
    record_name: impl Display,
    interface_name: impl Display,
    parent_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 11,
        format!(
            "record `{record_name}` has a conflicting definition for field `{field_name}` in interface `{interface_name}` inherited from `{parent_name}`"
        ),
        span,
    )
    .with_help("Fields with the same name must have identical types and modes. Align both definitions, or rename one of them.")
}

pub(crate) fn record_field_missing(
    field_name: impl Display,
    record_name: impl Display,
    interface_name: impl Display,
    program_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 12,
        format!(
            "record `{record_name}` in program `{program_name}` is missing field `{field_name}` required by interface `{interface_name}`"
        ),
        span,
    )
    .with_help(format!("Add the missing field `{field_name}` with the exact type and mode specified in `{interface_name}`."))
}

pub(crate) fn record_field_type_mismatch(
    field_name: impl Display,
    record_name: impl Display,
    interface_name: impl Display,
    expected: impl Display,
    found: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 13,
        format!(
            "field `{field_name}` in record `{record_name}` does not match the type required by interface `{interface_name}`\n\
             Expected: {expected}\n\
             Found: {found}"
        ),
        span,
    )
    .with_help("Field types and modes must match exactly.")
}

pub(crate) fn record_prototype_owner_wrong_type(
    record_name: impl Display,
    found_type: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 14,
        format!("field `owner` in record prototype `{record_name}` must have type `address`, found `{found_type}`"),
        span,
    )
    .with_help("Change the field's type to `address`. The `owner` field of every record is always `address`.")
}
