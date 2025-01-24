// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use std::{fmt, fmt::Write};

/// Implements `Display` by putting 4 spaces in front of each line
/// of `T`'s output.
pub struct Indent<T>(pub T);

impl<T: fmt::Display> fmt::Display for Indent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(IndentWriter { f, new_line: true }, "{}", self.0)
    }
}

const SPACES: &str = "    ";

struct IndentWriter<'a, 'b> {
    new_line: bool,
    f: &'b mut fmt::Formatter<'a>,
}

impl Write for IndentWriter<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut iter = s.lines().peekable();

        while let Some(line) = iter.next() {
            if self.new_line {
                self.f.write_str(SPACES)?;
            }
            self.f.write_str(line)?;
            if iter.peek().is_some() || s.ends_with('\n') {
                self.f.write_str("\n")?;
                self.new_line = true;
            } else {
                self.new_line = false;
            }
        }

        Ok(())
    }
}
