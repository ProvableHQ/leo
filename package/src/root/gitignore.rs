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

//! The `.gitignore` file.

use crate::errors::GitignoreError;

use serde::Deserialize;
use std::{fs::File, io::Write, path::Path};

pub static GITIGNORE_FILENAME: &str = ".gitignore";

#[derive(Deserialize, Default)]
pub struct Gitignore;

impl Gitignore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(GITIGNORE_FILENAME);
        }
        path.exists()
    }

    pub fn write_to(self, path: &Path) -> Result<(), GitignoreError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(GITIGNORE_FILENAME);
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        "outputs/\n".to_string()
    }
}
