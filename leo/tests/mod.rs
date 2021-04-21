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

mod cmd;

use anyhow::Result;
use std::path::PathBuf;

use crate::{
    commands::{
        package::{Login, Logout},
        Build,
        Command,
        Prove,
        Run,
        Setup,
        Test,
        Update,
        UpdateAutomatic,
    },
    context::{create_context, Context},
};

/// Path to the only complex Leo program that we have
/// - relative to source dir - where Cargo.toml is located
const PEDERSEN_HASH_PATH: &str = "./examples/pedersen-hash/";

#[test]
pub fn build_pedersen_hash() -> Result<()> {
    (Build {}).apply(context()?, ())?;
    Ok(())
}

#[test]
pub fn setup_pedersen_hash() -> Result<()> {
    let build = (Build {}).apply(context()?, ())?;
    (Setup { skip_key_check: false }).apply(context()?, build.clone())?;
    (Setup { skip_key_check: true }).apply(context()?, build)?;
    Ok(())
}

#[test]
pub fn prove_pedersen_hash() -> Result<()> {
    let build = (Build {}).apply(context()?, ())?;
    let setup = (Setup { skip_key_check: false }).apply(context()?, build)?;
    (Prove { skip_key_check: false }).apply(context()?, setup.clone())?;
    (Prove { skip_key_check: true }).apply(context()?, setup)?;
    Ok(())
}

#[test]
pub fn run_pedersen_hash() -> Result<()> {
    let build = (Build {}).apply(context()?, ())?;
    let setup = (Setup { skip_key_check: false }).apply(context()?, build)?;
    let prove = (Prove { skip_key_check: false }).apply(context()?, setup)?;
    (Run { skip_key_check: false }).apply(context()?, prove.clone())?;
    (Run { skip_key_check: true }).apply(context()?, prove)?;
    Ok(())
}

#[test]
pub fn test_pedersen_hash() -> Result<()> {
    let mut main_file = PathBuf::from(PEDERSEN_HASH_PATH);
    main_file.push("src/main.leo");

    (Test { files: vec![] }).apply(context()?, ())?;
    (Test { files: vec![main_file] }).apply(context()?, ())?;
    Ok(())
}

#[test]
pub fn test_logout() -> Result<()> {
    (Logout {}).apply(context()?, ())?;
    Ok(())
}

// Decided to not go all-in on error messages since they might change in the future
// So this test only tells that error cases are errors
#[test]
pub fn login_incorrect_credentials_or_token() -> Result<()> {
    // no credentials passed
    let login = Login::new(None, None, None).apply(context()?, ());
    assert!(login.is_err());

    // incorrect token
    let login = Login::new(Some("none".to_string()), None, None).apply(context()?, ());
    assert!(login.is_err());

    // only user, no pass
    let login = Login::new(None, Some("user".to_string()), None).apply(context()?, ());
    assert!(login.is_err());

    // no user, only pass
    let login = Login::new(None, None, Some("pass".to_string())).apply(context()?, ());
    assert!(login.is_err());

    Ok(())
}

#[test]
pub fn leo_update_and_update_automatic() -> Result<()> {
    let update = Update {
        list: true,
        studio: true,
        automatic: None,
    };
    update.apply(context()?, ())?;

    let update = Update {
        list: false,
        studio: true,
        automatic: None,
    };
    update.apply(context()?, ())?;

    let update = Update {
        list: false,
        studio: false,
        automatic: None,
    };
    update.apply(context()?, ())?;

    let update = Update {
        list: false,
        studio: false,
        automatic: Some(UpdateAutomatic::Automatic { value: true }),
    };
    update.apply(context()?, ())?;

    let update = Update {
        list: false,
        studio: false,
        automatic: Some(UpdateAutomatic::Automatic { value: false }),
    };
    update.apply(context()?, ())?;

    Ok(())
}

/// Create context for Pedersen Hash example
fn context() -> Result<Context> {
    let path = PathBuf::from(&PEDERSEN_HASH_PATH);
    let context = create_context(path)?;

    Ok(context)
}
