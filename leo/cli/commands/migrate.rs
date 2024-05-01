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

use super::*;
use leo_errors::Result;

use semver::Version;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Migrates the Leo program to a new version.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Aleo Team <hello@aleo.org>", version)]
pub struct Migrate {
    #[clap(name = "OLD", help = "The old version")]
    pub(crate) old: String,
    #[clap(name = "NEW", help = "The new version")]
    pub(crate) new: String,
    #[clap(short = 'p', long, help = "The path to the program directory.", default_value = "./src")]
    pub(crate) path: PathBuf,
    #[clap(short = 'r', long, help = "Recursively migrate all subdirectories.", default_value = "false")]
    pub(crate) recursive: bool,
}

impl Command for Migrate {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _: Context, _: Self::Input) -> Result<Self::Output> {
        // Parse the version strings into a migration path.
        let migration_path = MigrationPath::new(&self.old, &self.new)?;
        // Check if the migration path is supported.
        if !migration_path.is_supported() {
            return Err(CliError::unsupported_migration_path(&migration_path.old, &migration_path.new).into());
        }
        // Get all files in the path. If recursive, get all files in subdirectories.
        let walker = match self.recursive {
            true => WalkDir::new(&self.path).into_iter(),
            false => WalkDir::new(&self.path).max_depth(1).into_iter(),
        };
        // Filter all files that are not `.leo` files.
        let files = walker
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "leo"));

        // Print the files that were found and ask the user if they want to migrate them.
        println!("Found the following Leo files:");
        for file in files {
            println!("  {}", file.path().display());
        }
        println!("Do you want to migrate **all** of these files? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).map_err(CliError::failed_to_read_stdin)?;
        if input.trim().to_lowercase() != "y" {
            return Ok(());
        }

        Ok(())
    }
}

/// A migration path.
#[derive(Clone, Eq, PartialEq)]
pub struct MigrationPath {
    /// The old version.
    pub old: Version,
    /// The new version.
    pub new: Version,
}

impl MigrationPath {
    /// The supported migration paths.
    const SUPPORTED: [Self; 2] = [
        /// 1.11.0 -> 1.13.0
        Self { old: Version::new(1, 11, 0), new: Version::new(1, 13, 0) },
        /// 1.12.0 -> 1.13.0
        Self { old: Version::new(1, 12, 0), new: Version::new(1, 13, 0) },
    ];

    /// Constructs a new migration path from two version strings.
    fn new(old: &str, new: &str) -> Result<Self> {
        // Parse the old version.
        let old = Version::parse(old).map_err(|_| CliError::invalid_version(old))?;
        // Parse the new version.
        let new = Version::parse(new).map_err(|_| CliError::invalid_version(new))?;
        // Return the migration path.
        Ok(Self { old, new })
    }

    /// Returns whether or not the migration path is supported.
    fn is_supported(&self) -> bool {
        Self::SUPPORTED.iter().any(|path| path == self)
    }
}
