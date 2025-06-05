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

use super::*;
use cursor::Cursor;

use leo_ast::interpreter_value::{CoreFunctionHelper, LeoValue};

use rand_chacha::ChaCha20Rng;

impl CoreFunctionHelper for Cursor<'_> {
    fn pop_value_impl(&mut self) -> Option<LeoValue> {
        let function_call = self.function_call_stack.last_mut()?;
        let cursor::FunctionCall::Leo(leo_function_call) = function_call else {
            return None;
        };

        let statement_frame = leo_function_call.statement_frames.last_mut()?;

        statement_frame.values.pop()
    }

    fn set_block_height(&mut self, height: u32) {
        self.block_height = height;
    }

    fn rng(&mut self) -> Option<&mut ChaCha20Rng> {
        Some(&mut self.rng)
    }
}
