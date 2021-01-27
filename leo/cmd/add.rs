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

use crate::cmd::Cmd;
use crate::context::{create_context, Context};
use anyhow::Error;
use structopt::StructOpt;

/// Add package from Aleo Package Manager
#[derive(StructOpt, Debug)]
pub struct Add {}

impl Add {
    pub fn new() -> Add {
        Add {}
    }
}

impl Cmd for Add {
    fn context(&self) -> Result<Context, Error> {
        create_context()
    }

    /// TODO: add package fetching logic here
    fn apply(self, ctx: Context) -> Result<(), Error> {
        match ctx.api.fetch("ray".to_string(), "hello-world".to_string(), None, None) {
            Ok(res) => println!("{:?}", res),
            Err(err) => println!("{:?}", err),
        };

        Ok(())
    }
}
