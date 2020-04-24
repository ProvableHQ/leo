use crate::errors::ManifestError;

use failure::Fail;
use serde::Deserialize;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub static FILE_NAME_DEFAULT: &str = "Leo.toml";

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
}

impl Manifest {
    pub fn new(package_name: &str) -> Self {
        Self {
            package: Package {
                name: package_name.to_owned(),
                version: "0.1.0".to_owned(),
            },
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), ManifestError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }

        let mut file =
            File::create(&path).map_err(|error| ManifestError::Creating(FILE_NAME_DEFAULT, error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| ManifestError::Writing(FILE_NAME_DEFAULT, error))
    }

    fn template(&self) -> String {
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
"#,
            self.package.name
        )
    }
}

impl TryFrom<&PathBuf> for Manifest {
    type Error = ManifestError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(FILE_NAME_DEFAULT));
        }

        let mut file =
            File::open(path).map_err(|error| ManifestError::Opening(FILE_NAME_DEFAULT, error))?;
        let size = file
            .metadata()
            .map_err(|error| ManifestError::Metadata(FILE_NAME_DEFAULT, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| ManifestError::Reading(FILE_NAME_DEFAULT, error))?;

        Ok(toml::from_str(&buffer).map_err(|error| ManifestError::Parsing(FILE_NAME_DEFAULT, error))?)
    }
}
