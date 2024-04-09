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

use std::fmt::Display;

create_messages!(
    /// ParserWarning enum that represents all the warnings for the `leo-parser` crate.
    TypeCheckerWarning,
    code_mask: 2000i32,
    code_prefix: "TYC",

    @formatted
    some_paths_do_not_await_all_futures {
        args: (num_total_paths: impl Display, num_unawaited_paths: impl Display),
        msg: format!("Not all paths through the function await all futures. {num_unawaited_paths}/{num_total_paths} paths contain at least one future that is never awaited."),
        help: Some("Ex: `f.await()` to await a future. Remove this warning by including the `--disable-conditional-branch-type-checking` flag.".to_string()),
    }

    @formatted
    some_paths_contain_duplicate_future_awaits {
        args: (num_total_paths: impl Display, num_duplicate_await_paths: impl Display),
        msg: format!("Some paths through the function contain duplicate future awaits. {num_duplicate_await_paths}/{num_total_paths} paths contain at least one future that is awaited more than once."),
        help: Some("Look at the times `.await()` is called, and try to reduce redundancies. Remove this warning by including the `--disable-conditional-branch-type-checking` flag.".to_string()),
    }

    @formatted
    async_function_is_never_called_by_transition_function {
        args: (name: impl Display),
        msg: format!("The async function `{name}` is never called by an async transition."),
        help: None,
    }

    @formatted
    max_conditional_block_depth_exceeded {
        args: (max: impl Display),
        msg: format!("The type checker has exceeded the max depth of nested conditional blocks: {max}."),
        help: Some("Re-run with a larger maximum depth using the `--conditional_block_max_depth` build option. Ex: `leo run main --conditional_block_max_depth 25`.".to_string()),
    }
);
