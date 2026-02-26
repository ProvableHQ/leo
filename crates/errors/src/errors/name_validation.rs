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
    NameValidationError,
    code_mask: 11000i32,
    code_prefix: "NV",

    @formatted
    illegal_name {
        args: (item_name: impl Display, item_type: impl Display, keyword: impl Display),
        msg: format!("`{item_name}` is an invalid {item_type} name. A {item_type} cannot be called \"{keyword}\"."),
        help: None,
    }

    @formatted
    illegal_name_content {
        args: (item_name: impl Display, item_type: impl Display, keyword: impl Display),
        msg: format!("`{item_name}` is an invalid {item_type} name. A {item_type} cannot have \"{keyword}\" in its name."),
        help: None,
    }
);
