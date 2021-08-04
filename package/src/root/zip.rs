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

//! The program package zip file.

use crate::{
    imports::IMPORTS_DIRECTORY_NAME,
    inputs::{INPUTS_DIRECTORY_NAME, INPUT_FILE_EXTENSION, STATE_FILE_EXTENSION},
    outputs::{
        CHECKSUM_FILE_EXTENSION,
        CIRCUIT_FILE_EXTENSION,
        OUTPUTS_DIRECTORY_NAME,
        PROOF_FILE_EXTENSION,
        PROVING_KEY_FILE_EXTENSION,
        VERIFICATION_KEY_FILE_EXTENSION,
    },
    root::{MANIFEST_FILENAME, README_FILENAME},
    source::{SOURCE_DIRECTORY_NAME, SOURCE_FILE_EXTENSION},
};
use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{
    borrow::Cow,
    fs::{
        File,
        {self},
    },
    io::{Read, Write},
    path::Path,
};
use walkdir::WalkDir;
use zip::write::{FileOptions, ZipWriter};

pub static ZIP_FILE_EXTENSION: &str = ".zip";

#[derive(Deserialize)]
pub struct ZipFile {
    pub package_name: String,
}

impl ZipFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    pub fn get_file_path<'a>(&self, current_dir: &'a Path) -> Cow<'a, Path> {
        self.setup_file_path(current_dir)
    }

    // /// Reads the program bytes from the given file path if it exists.
    // pub fn read_from(&self, path: &Path) -> Result<Vec<u8>> {
    //     let path = self.setup_file_path(path);
    //
    //     Ok(fs::read(&path).map_err(|_| PackageError::FileReadError(path.clone()))?)
    // }

    /// Writes the current package contents to a zip file.
    pub fn write(&self, src_dir: &Path) -> Result<()> {
        // Build walkdir iterator from current package
        let walkdir = WalkDir::new(src_dir);

        // Create zip file
        let path = self.setup_file_path(src_dir);

        let file = File::create(&path).map_err(|e| PackageError::failed_to_create_zip_file(e))?;
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        // Walk through files in directory and write desired ones to the zip file
        let mut buffer = Vec::new();
        for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(src_dir).unwrap();

            // Add file/directory exclusion

            let included = is_included(name);
            tracing::debug!("Checking if {:?} is included - {}", name, included);
            if !included {
                continue;
            }

            // Write file or directory
            if path.is_file() {
                tracing::info!("Adding file {:?} as {:?}", path, name);
                #[allow(deprecated)]
                zip.start_file_from_path(name, options)
                    .map_err(|e| PackageError::io_error_zip_file(e))?;

                let mut f = File::open(path).map_err(|e| PackageError::failed_to_open_zip_file(e))?;
                f.read_to_end(&mut buffer)
                    .map_err(|e| PackageError::failed_to_read_zip_file(e))?;
                zip.write_all(&*buffer)
                    .map_err(|e| PackageError::failed_to_write_zip_file(e))?;

                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                // Only if not root Avoids path spec / warning
                // and mapname conversion failed error on unzip
                tracing::info!("Adding directory {:?} as {:?}", path, name);
                #[allow(deprecated)]
                zip.add_directory_from_path(name, options)
                    .map_err(|e| PackageError::io_error_zip_file(e))?;
            }
        }

        zip.finish().map_err(|e| PackageError::io_error_zip_file(e))?;

        tracing::info!("Package zip file created successfully {:?}", path);

        Ok(())
    }

    /// Removes the zip file at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| PackageError::failed_to_remove_zip_file(path))?;
        Ok(true)
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(format!("{}{}", self.package_name, ZIP_FILE_EXTENSION));
        }
        path
    }
}

/// Check if the file path should be included in the package zip file.
fn is_included(path: &Path) -> bool {
    // excluded directories: `output`, `imports`
    if path.ends_with(OUTPUTS_DIRECTORY_NAME.trim_end_matches('/'))
        | path.ends_with(IMPORTS_DIRECTORY_NAME.trim_end_matches('/'))
    {
        return false;
    }

    // excluded extensions: `.in`, `.bytes`, `lpk`, `lvk`, `.proof`, `.sum`, `.zip`, `.bytes`
    if let Some(true) = path.extension().map(|ext| {
        ext.eq(ZIP_FILE_EXTENSION.trim_start_matches('.'))
            | ext.eq(PROVING_KEY_FILE_EXTENSION.trim_start_matches('.'))
            | ext.eq(VERIFICATION_KEY_FILE_EXTENSION.trim_start_matches('.'))
            | ext.eq(PROOF_FILE_EXTENSION.trim_start_matches('.'))
            | ext.eq(CHECKSUM_FILE_EXTENSION.trim_start_matches('.'))
            | ext.eq(ZIP_FILE_EXTENSION.trim_start_matches('.'))
            | ext.eq(CIRCUIT_FILE_EXTENSION.trim_start_matches('.'))
    }) {
        return false;
    }

    // Allow `inputs` folder
    if path.ends_with(INPUTS_DIRECTORY_NAME.trim_end_matches('/')) {
        return true;
    }

    // Allow `.state` and `.in` files
    if let Some(true) = path.extension().map(|ext| {
        ext.eq(INPUT_FILE_EXTENSION.trim_start_matches('.')) | ext.eq(STATE_FILE_EXTENSION.trim_start_matches('.'))
    }) {
        return true;
    }

    // Allow the README.md and Leo.toml files in the root directory
    if (path.ends_with(README_FILENAME) | path.ends_with(MANIFEST_FILENAME)) & (path.parent() == Some(Path::new(""))) {
        return true;
    }

    // Only allow additional files in the `src/` directory
    if !path.starts_with(SOURCE_DIRECTORY_NAME.trim_end_matches('/')) {
        return false;
    }

    // Allow the `.leo` files in the `src/` directory
    path.extension()
        .map(|ext| ext.eq(SOURCE_FILE_EXTENSION.trim_start_matches('.')))
        .unwrap_or(false)
}
