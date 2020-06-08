//! The `.gitignore` file.

use crate::errors::GitignoreError;

use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub static GITIGNORE_FILE_NAME: &str = ".gitignore";

#[derive(Deserialize)]
pub struct Gitignore;

impl Gitignore {
    pub fn new() -> Self {
        Self
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(GITIGNORE_FILE_NAME));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), GitignoreError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(GITIGNORE_FILE_NAME));
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"/output
/.leo
"#,
        )
    }
}
