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

use std::fmt::{Debug, Display};

create_messages!(
    CheckInterfacesError,
    code_mask: 12000i32,
    code_prefix: "CHI",

    @formatted
    interface_not_found {
        args: (name: impl Display),
        msg: format!("Interface `{name}` not found."),
        help: Some("Make sure the interface is defined in the current program or an imported program.".to_string()),
    }

    @formatted
    conflicting_interface_member {
        args: (member_name: impl Display, interface_name: impl Display, parent_name: impl Display),
        msg: format!(
            "Interface `{interface_name}` has a conflicting definition for `{member_name}` inherited from `{parent_name}`. \
             Members with the same name must have identical signatures."
        ),
        help: Some("Ensure both interfaces define the same signature for this member, or rename one of them.".to_string()),
    }

    @formatted
    missing_interface_function {
        args: (func_name: impl Display, interface_name: impl Display, program_name: impl Display),
        msg: format!(
            "Program `{program_name}` implements interface `{interface_name}` but is missing the required function `{func_name}`."
        ),
        help: Some("Add the missing function with the exact signature specified in the interface.".to_string()),
    }

    @formatted
    missing_interface_record {
        args: (record_name: impl Display, interface_name: impl Display, program_name: impl Display),
        msg: format!(
            "Program `{program_name}` implements interface `{interface_name}` but is missing the required record `{record_name}`."
        ),
        help: Some("Add a record definition with the specified name.".to_string()),
    }

    @formatted
    signature_mismatch {
        args: (func_name: impl Display, interface_name: impl Display, expected: impl Display, found: impl Display),
        msg: format!(
            "Function `{func_name}` does not match the signature required by interface `{interface_name}`.\n\
             Expected:\n\
             {expected}\n\
             Found:\n\
             {found}"
        ),
        help: Some("Function signatures must match exactly: same parameter names, types, modes, order, and return type.".to_string()),
    }

    @formatted
    not_an_interface {
        args: (name: impl Display),
        msg: format!("`{name}` is not an interface."),
        help: Some("Only interface declarations can be inherited from.".to_string()),
    }
);
