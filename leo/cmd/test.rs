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

use crate::{cmd::Cmd, context::Context};

use anyhow::Error;
use structopt::StructOpt;

/// Add package from Aleo Package Manager
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Test {}

impl Test {
    pub fn new() -> Test {
        Test {}
    }
}

impl Cmd for Test {
    type Output = ();

    fn apply(self, _ctx: Context) -> Result<Self::Output, Error> {
        Ok(())
    }
}
