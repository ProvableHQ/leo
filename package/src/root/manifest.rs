use crate::errors::ManifestError;

use serde::Deserialize;
use std::{
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub const MANIFEST_FILE_NAME: &str = "Leo.toml";

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
            path.push(PathBuf::from(MANIFEST_FILE_NAME));
        }
        path.exists()
    }

    pub fn get_package_name(&self) -> String {
        self.package.name.clone()
    }

    pub fn get_package_version(&self) -> String {
        self.package.version.clone()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), ManifestError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(MANIFEST_FILE_NAME));
        }

        let mut file = File::create(&path).map_err(|error| ManifestError::Creating(MANIFEST_FILE_NAME, error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| ManifestError::Writing(MANIFEST_FILE_NAME, error))
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
            path.push(PathBuf::from(MANIFEST_FILE_NAME));
        }

        let mut file = File::open(path).map_err(|error| ManifestError::Opening(MANIFEST_FILE_NAME, error))?;
        let size = file
            .metadata()
            .map_err(|error| ManifestError::Metadata(MANIFEST_FILE_NAME, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| ManifestError::Reading(MANIFEST_FILE_NAME, error))?;

        Ok(toml::from_str(&buffer).map_err(|error| ManifestError::Parsing(MANIFEST_FILE_NAME, error))?)
    }
}
