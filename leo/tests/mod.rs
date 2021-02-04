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
    cmd::{package::Login, Build, Cmd, Prove, Run, Setup},
    context::{create_context, Context},
};
use anyhow::{anyhow, Result};
use std::{any, path::PathBuf};

/// Path to the only complex Leo program that we have
/// - relative to source dir - where Cargo.toml is located
const PEDERSEN_HASH_PATH: &'static str = "./examples/pedersen-hash/";

#[test]
pub fn build_pedersen_hash() -> Result<()> {
    Build::new().apply(ctx()?, ())?;
    Ok(())
}

#[test]
pub fn setup_pedersen_hash() -> Result<()> {
    let build = Build::new().apply(ctx()?, ())?;
    Setup::new(false).apply(ctx()?, build.clone())?;
    Setup::new(true).apply(ctx()?, build)?;
    Ok(())
}

#[test]
pub fn prove_pedersen_hash() -> Result<()> {
    let build = Build::new().apply(ctx()?, ())?;
    let setup = Setup::new(false).apply(ctx()?, build)?;
    Prove::new(false).apply(ctx()?, setup)?;
    Ok(())
}

#[test]
pub fn run_pedersen_hash() -> Result<()> {
    let build = Build::new().apply(ctx()?, ())?;
    let setup = Setup::new(false).apply(ctx()?, build)?;
    let prove = Prove::new(false).apply(ctx()?, setup)?;
    Run::new(false).apply(ctx()?, prove)?;
    Ok(())
}

// Decided to not go all-in on error messages since they might change in the future
// So this test only tells that error cases are errors
#[test]
pub fn login_incorrect_credentials_or_token() -> Result<()> {
    // no credentials passed
    let login = Login::new(None, None, None).apply(ctx()?, ());
    assert!(login.is_err());

    // incorrect token
    let login = Login::new(Some("none".to_string()), None, None).apply(ctx()?, ());
    assert!(login.is_err());

    // only user, no pass
    let login = Login::new(None, Some("user".to_string()), None).apply(ctx()?, ());
    assert!(login.is_err());

    // no user, only pass
    let login = Login::new(None, None, Some("pass".to_string())).apply(ctx()?, ());
    assert!(login.is_err());

    Ok(())
}

/// Create context for Pedersen Hash example
fn ctx() -> Result<Context> {
    let path = PathBuf::from(&PEDERSEN_HASH_PATH);
    let ctx = create_context(path)?;

    Ok(ctx)
}
