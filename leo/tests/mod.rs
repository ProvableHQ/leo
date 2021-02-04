// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{
    cmd::{build::Build, run::Run, setup::Setup, Cmd},
    context::{create_context, Context},
};
use anyhow::Result;
use std::path::PathBuf;

/// Path to the only complex Leo program that we have
const PEDERSEN_HASH_PATH: &'static str = "../../examples/pedersen-hash/";

#[test]
pub fn test_build_example() -> Result<()> {
    let path = PathBuf::from(&PEDERSEN_HASH_PATH);

    println!("{:?}", std::fs::canonicalize(path));

    Ok(())
}
