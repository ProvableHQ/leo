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

use leo_errors::Result;
// use std::path::PathBuf;

/* use crate::{
    commands::{
        // package::{Login, Logout},
        Build,
        Command,
        // Prove, Run, Setup, Test,
    },
    context::{create_context, Context},
}; */

/// Path to the only complex Leo program that we have
/// - relative to source dir - where Cargo.toml is located
// const PEDERSEN_HASH_PATH: &str = "./examples/pedersen-hash/";

#[test]
pub fn init_logger() -> Result<()> {
    crate::cli::helpers::logger::init_logger("test_init_logger", 1)?;
    Ok(())
}

#[test]
pub fn format_event() -> Result<()> {
    crate::cli::helpers::logger::init_logger("test_format_event", 1)?;
    tracing::info!("test");
    Ok(())
}

// todo (collin): uncomment after refactor
// #[test]
// pub fn build_pedersen_hash() -> Result<()> {
//     (Build {
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, ())?;
//     Ok(())
// }
// #[test]
// pub fn setup_pedersen_hash() -> Result<()> {
//     let build = (Build {
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, ())?;
//     (Setup {
//         skip_key_check: false,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, build.clone())?;
//     (Setup {
//         skip_key_check: true,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, build)?;
//     Ok(())
// }
//
// #[test]
// pub fn prove_pedersen_hash() -> Result<()> {
//     let build = (Build {
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, ())?;
//     let setup = (Setup {
//         skip_key_check: false,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, build)?;
//     (Prove {
//         skip_key_check: false,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, setup.clone())?;
//     (Prove {
//         skip_key_check: true,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, setup)?;
//     Ok(())
// }
//
// #[test]
// pub fn run_pedersen_hash() -> Result<()> {
//     let build = (Build {
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, ())?;
//     let setup = (Setup {
//         skip_key_check: false,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, build)?;
//     let prove = (Prove {
//         skip_key_check: false,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, setup)?;
//     (Run {
//         skip_key_check: false,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, prove.clone())?;
//     (Run {
//         skip_key_check: true,
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, prove)?;
//     Ok(())
// }
//
// #[test]
// pub fn test_pedersen_hash() -> Result<()> {
//     let mut main_file = PathBuf::from(PEDERSEN_HASH_PATH);
//     main_file.push("src/main.leo");
//
//     (Test {
//         files: vec![],
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, ())?;
//     (Test {
//         files: vec![main_file],
//         compiler_options: Default::default(),
//     })
//     .apply(context()?, ())?;
//     Ok(())
// }
//
// #[test]
// pub fn test_logout() -> Result<()> {
//     let logout = (Logout {}).apply(context()?, ());
//     assert!(logout.is_err());
//     Ok(())
// }
//
// // Decided to not go all-in on error messages since they might change in the future
// // So this test only tells that error cases are errors
// #[test]
// pub fn login_incorrect_credentials_or_token() -> Result<()> {
//     test_logout()?;
//
//     // no credentials passed
//     let login = Login::new(None, None, None).apply(context()?, ());
//     assert!(login.is_err());
//
//     // incorrect token
//     let login = Login::new(Some("none".to_string()), None, None).apply(context()?, ());
//     assert!(login.is_err());
//
//     // only user, no pass
//     let login = Login::new(None, Some("user".to_string()), None).apply(context()?, ());
//     assert!(login.is_err());
//
//     // no user, only pass
//     let login = Login::new(None, None, Some("pass".to_string())).apply(context()?, ());
//     assert!(login.is_err());
//
//     Ok(())
// }
//
// #[cfg(not(target_os = "macos"))]
// #[test]
// pub fn leo_update_and_update_automatic() -> Result<()> {
//     use crate::commands::{Update, UpdateAutomatic};
//
//     let update = Update {
//         list: true,
//         studio: true,
//         automatic: None,
//     };
//     update.apply(context()?, ())?;
//
//     let update = Update {
//         list: false,
//         studio: true,
//         automatic: None,
//     };
//     update.apply(context()?, ())?;
//
//     let update = Update {
//         list: false,
//         studio: false,
//         automatic: None,
//     };
//     update.apply(context()?, ())?;
//
//     let update = Update {
//         list: false,
//         studio: false,
//         automatic: Some(UpdateAutomatic::Automatic { value: true }),
//     };
//     update.apply(context()?, ())?;
//
//     let update = Update {
//         list: false,
//         studio: false,
//         automatic: Some(UpdateAutomatic::Automatic { value: false }),
//     };
//     update.apply(context()?, ())?;
//
//     Ok(())
// }

// /// Create context for Pedersen Hash example
/* fn context() -> Result<Context> {
    let path = PathBuf::from(&PEDERSEN_HASH_PATH);
    let context = create_context(path, None)?;

    Ok(context)
} */
