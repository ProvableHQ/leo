// Copyright (C) 2019-2026 Provable Inc.
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

use leo_errors::{Backtraced, Result};

use aleo_std;

use colored::Colorize;
use self_update::{backends::github, version::bump_is_greater};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct Updater;

impl Updater {
    const LEO_BIN_NAME: &'static str = "leo";
    const LEO_CACHE_LAST_CHECK_FILE: &'static str = "leo_cache_last_update_check";
    const LEO_CACHE_VERSION_FILE: &'static str = "leo_cache_latest_version";
    const LEO_REPO_NAME: &'static str = "leo";
    const LEO_REPO_OWNER: &'static str = "ProvableHQ";
    // 24 hours
    const LEO_UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

    /// Check if there is an available update for `leo` and return the newest release.
    pub fn update_available() -> Result<String> {
        let updater = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(env!("CARGO_PKG_VERSION"))
            .build()
            .map_err(crate::errors::self_update_error)?;

        let current_version = updater.current_version();
        let latest_release = updater.get_latest_release().map_err(crate::errors::self_update_error)?;

        if bump_is_greater(&current_version, &latest_release.version).map_err(crate::errors::self_update_error)? {
            Ok(latest_release.version)
        } else {
            Err(crate::errors::old_release_version(current_version, latest_release.version).into())
        }
    }

    /// Read the latest version from the version file.
    pub fn read_latest_version() -> Result<Option<String>, Backtraced> {
        let version_file_path = Self::get_version_file_path();
        match fs::read_to_string(version_file_path) {
            Ok(version) => Ok(Some(version.trim().to_string())),
            Err(_) => Ok(None),
        }
    }

    /// Generate the CLI message if a new version is available.
    pub fn get_cli_string() -> Result<Option<String>, Backtraced> {
        if let Some(latest_version) = Self::read_latest_version()? {
            let colorized_message = format!(
                "\n🟢 {} {} {}",
                format!("Leo v{latest_version} is available.").bold().green(),
                "For installation instructions, see".bold().green(),
                "https://github.com/ProvableHQ/leo#-build-guide".bold().white()
            );
            Ok(Some(colorized_message))
        } else {
            Ok(None)
        }
    }

    /// Display the CLI message if a new version is available.
    pub fn print_cli() -> Result<(), Backtraced> {
        if let Some(message) = Self::get_cli_string()? {
            println!("{message}");
        }
        Ok(())
    }

    /// Check for updates, respecting the update interval. (Currently once per day.)
    /// If a new version is found, write it to a cache file and alert in every call.
    pub fn check_for_updates(force: bool) -> Result<bool, Backtraced> {
        // Get the cache directory and relevant file paths.
        let cache_dir = Self::get_cache_dir();
        let last_check_file = cache_dir.join(Self::LEO_CACHE_LAST_CHECK_FILE);
        let version_file = Self::get_version_file_path();

        // Determine if we should check for updates.
        let should_check = force || Self::should_check_for_updates(&last_check_file)?;

        if should_check {
            match Self::update_available() {
                Ok(latest_version) => {
                    // A new version is available
                    Self::update_check_files(&cache_dir, &last_check_file, &version_file, &latest_version)?;
                    Ok(true)
                }
                Err(_) => {
                    // No new version available or error occurred
                    // We'll treat both cases as "no update" for simplicity
                    Self::update_check_files(&cache_dir, &last_check_file, &version_file, env!("CARGO_PKG_VERSION"))?;
                    Ok(false)
                }
            }
        } else if version_file.exists() {
            if let Ok(stored_version) = fs::read_to_string(&version_file) {
                let current_version = env!("CARGO_PKG_VERSION");
                Ok(bump_is_greater(current_version, stored_version.trim()).map_err(crate::errors::self_update_error)?)
            } else {
                // If we can't read the file, assume no update is available
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Updates the check files with the latest version information and timestamp.
    ///
    /// This function creates the cache directory if it doesn't exist, writes the current time
    /// to the last check file, and writes the latest version to the version file.
    fn update_check_files(
        cache_dir: &Path,
        last_check_file: &Path,
        version_file: &Path,
        latest_version: &str,
    ) -> Result<(), Backtraced> {
        // Recursively create the cache directory and all of its parent components if they are missing.
        fs::create_dir_all(cache_dir).map_err(crate::errors::cli_io_error)?;

        // Get the current time.
        let current_time = Self::get_current_time()?;

        // Write the current time to the last check file.
        fs::write(last_check_file, current_time.to_string()).map_err(crate::errors::cli_io_error)?;

        // Write the latest version to the version file.
        fs::write(version_file, latest_version).map_err(crate::errors::cli_io_error)?;

        Ok(())
    }

    /// Determines if an update check should be performed based on the last check time.
    ///
    /// This function reads the last check timestamp from a file and compares it with
    /// the current time to decide if enough time has passed for a new check.
    fn should_check_for_updates(last_check_file: &Path) -> Result<bool, Backtraced> {
        match fs::read_to_string(last_check_file) {
            Ok(contents) => {
                // Parse the last check timestamp from the file.
                let last_check = contents
                    .parse::<u64>()
                    .map_err(|e| crate::errors::cli_runtime_error(format!("Failed to parse last check time: {e}")))?;

                // Get the current time.
                let current_time = Self::get_current_time()?;

                // Check if enough time has passed since the last check.
                Ok(current_time.saturating_sub(last_check) > Self::LEO_UPDATE_CHECK_INTERVAL.as_secs())
            }
            // If we can't read the file, assume we should check
            Err(_) => Ok(true),
        }
    }

    /// Gets the current system time as seconds since the Unix epoch.
    fn get_current_time() -> Result<u64, Backtraced> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| crate::errors::cli_runtime_error(format!("System time error: {e}")))
            .map(|duration| duration.as_secs())
    }

    /// Get the path to the file storing the latest version information.
    fn get_version_file_path() -> PathBuf {
        Self::get_cache_dir().join(Self::LEO_CACHE_VERSION_FILE)
    }

    /// Get the cache directory for Leo.
    fn get_cache_dir() -> PathBuf {
        aleo_std::aleo_dir().join("leo")
    }
}
