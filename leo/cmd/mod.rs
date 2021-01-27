// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::context::{get_context, Context};
use anyhow::Result;

pub mod add;
pub mod clean;
pub mod init;
pub mod lint;
pub mod new;

/// Leo command
pub trait Cmd {
    /// Returns project context.
    fn context(&self) -> Result<Context> {
        get_context()
    }

    /// Apply command with given context.
    fn apply(self, ctx: Context) -> Result<()>
    where
        Self: std::marker::Sized;

    /// Functions create execution context and apply command with it.
    fn execute(self) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        let context = self.context()?;
        self.apply(context)
    }
}
