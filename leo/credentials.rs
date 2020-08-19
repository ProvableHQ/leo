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

use dirs::home_dir;
use lazy_static::lazy_static;
use std::{
    fs::{create_dir_all, File},
    io,
    io::prelude::*,
    path::{Path, PathBuf},
};

pub const PACKAGE_MANAGER_URL: &str = "https://apm-backend-dev.herokuapp.com/";

pub const LEO_CREDENTIALS_FILE: &str = "credentials";

lazy_static! {
    pub static ref LEO_CREDENTIALS_DIR: PathBuf = {
        let mut path = home_dir().expect("Invalid home directory");
        path.push(".leo");
        path
    };
    pub static ref LEO_CREDENTIALS_PATH: PathBuf = {
        let mut path = LEO_CREDENTIALS_DIR.to_path_buf();
        path.push(LEO_CREDENTIALS_FILE);
        path
    };
}

pub fn write_token(token: &str) -> Result<(), io::Error> {
    // Create Leo credentials directory if it not exists
    if !Path::new(&LEO_CREDENTIALS_DIR.to_path_buf()).exists() {
        create_dir_all(&LEO_CREDENTIALS_DIR.to_path_buf())?;
    }

    let mut credentials = File::create(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    credentials.write_all(&token.as_bytes())?;
    Ok(())
}

pub fn read_token() -> Result<String, io::Error> {
    let mut credentials = File::open(&LEO_CREDENTIALS_PATH.to_path_buf())?;
    let mut buf = String::new();
    credentials.read_to_string(&mut buf)?;
    Ok(buf)
}
