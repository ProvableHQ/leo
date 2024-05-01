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

/// Migrates the Leo program to a new version.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Aleo Team <hello@aleo.org>", version)]
pub struct Migrate {
    #[clap(name = "OLD", help = "The old version")]
    pub(crate) old: String,

    #[clap(name = "NEW", help = "The new version")]
    pub(crate) new: String,
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
        let path = MigrationPath::new(&self.old, &self.new)?;
        // Check if the migration path is supported.
        if !path.is_supported() {
            return Err(CliError::unsupported_migration_path(&path.old, &path.new).into());
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
    const SUPPORTED: [Self; 1] = [Self { old: Version::new(0, 1, 0), new: Version::new(0, 2, 0) }];

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
