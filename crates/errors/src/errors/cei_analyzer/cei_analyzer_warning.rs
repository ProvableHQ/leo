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

use std::fmt::Display;

create_messages!(
    /// CeiAnalyzerWarning enum that represents all the warnings for CEI analysis.
    #[derive(Hash, Eq, PartialEq)]
    CeiAnalyzerWarning,
    code_mask: 13000i32,
    code_prefix: "CEI",

    @formatted
    check_after_interaction {
        args: (operation: impl Display),
        msg: format!("Check operation `{operation}` occurs after an interaction (Final.run())."),
        help: Some("Move all checks (reads, asserts) before any calls to `.run()` to prevent reentrancy vulnerabilities.".to_string()),
    }

    @formatted
    effect_after_interaction {
        args: (operation: impl Display),
        msg: format!("Effect operation `{operation}` occurs after an interaction (Final.run())."),
        help: Some("Move all effects (state writes) before any calls to `.run()` to prevent reentrancy vulnerabilities.".to_string()),
    }

    @formatted
    callee_has_effects_after_interaction {
        args: (callee: impl Display),
        msg: format!("Call to `{callee}` (which contains checks or effects) occurs after an interaction (Final.run())."),
        help: Some("Move this call before any calls to `.run()` to prevent reentrancy vulnerabilities.".to_string()),
    }

    @formatted
    cei_violation_in_loop {
        args: (),
        msg: "Loop body contains both interactions (Final.run()) and state operations (checks or effects), which violates the CEI pattern.".to_string(),
        help: Some("Restructure the loop so that interactions do not occur alongside checks or effects within the same iteration.".to_string()),
    }

    @formatted
    tainted_value_in_finalize {
        args: (variable: impl Display, external_call: impl Display),
        msg: format!("Value `{variable}` used in finalize block is derived from external call `{external_call}`, whose finalize may alter the state this value depends on."),
        help: Some("The external program's finalize runs concurrently and may invalidate assumptions based on this value. Consider re-reading the value on-chain inside the finalize block.".to_string()),
    }

    @formatted
    tainted_argument_to_external_call {
        args: (variable: impl Display, callee: impl Display, external_call: impl Display),
        msg: format!("Tainted value `{variable}` (derived from external call `{external_call}`) is passed as an argument to `{callee}`, whose finalize will use the potentially stale value."),
        help: Some("The external program's finalize runs concurrently and may invalidate assumptions based on this value. Consider passing the value on-chain inside the finalize block instead.".to_string()),
    }
);
