// Copyright (C) 2019-2024 Aleo Systems Inc.
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

create_messages!(
    /// TestError enum that represents all the errors for the test framework
    TestError,
    code_mask: 8000i32,
    code_prefix: "TST",

    @formatted
    unknown_annotation_key {
        args: (annotation: impl Display, key: impl Display),
        msg: format!("Unknown key `{key}` in test annotation `{annotation}`."),
        help: None,
    }

    @formatted
    missing_annotation_value {
        args: (annotation: impl Display, key: impl Display),
        msg: format!("Missing value for key `{key}` in test annotation `{annotation}`."),
        help: None,
    }

    @formatted
    invalid_annotation_value {
        args: (annotation: impl Display, key: impl Display, value: impl Display, error: impl Display),
        msg: format!("Invalid value `{value}` for key `{key}` in test annotation `{annotation}`. Error: {error}"),
        help: None,
    }

    @formatted
    unexpected_annotation_value {
        args: (annotation: impl Display, key: impl Display, value: impl Display),
        msg: format!("Unexpected value `{value}` for key `{key}` in test annotation `{annotation}`."),
        help: None,
    }

    @formatted
    multiple_test_annotations {
        args: (),
        msg: format!("Multiple test annotations found, only one is allowed."),
        help: None,
    }

    @formatted
    non_transition_test {
        args: (),
        msg: format!("A test annotation is only allowed on transition functions."),
        help: None,
    }
);
