// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use leo_errors::{CliError, Result};

use std::fmt::Write as _;

use colored::Colorize;
use dirs;
use self_update::{backends::github, version::bump_is_greater, Status};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct Updater;

// TODO Add logic for users to easily select release versions.
impl Updater {
    const LEO_BIN_NAME: &'static str = "leo";
    const LEO_LAST_CHECK_FILE: &'static str = "leo_last_update_check";
    const LEO_REPO_NAME: &'static str = "leo";
    const LEO_REPO_OWNER: &'static str = "AleoHQ";
    //  const LEO_UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours
    const LEO_UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(5);
    // 24 hours
    const LEO_VERSION_FILE: &'static str = "leo_latest_version";

    /// Show all available releases for `leo`.
    pub fn show_available_releases() -> Result<String> {
        let releases = github::ReleaseList::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .build()
            .map_err(CliError::self_update_error)?
            .fetch()
            .map_err(CliError::could_not_fetch_versions)?;

        let mut output = "\nList of available versions\n".to_string();
        for release in releases {
            let _ = writeln!(output, "  * {}", release.version);
        }

        Ok(output)
    }

    /// Update `leo` to the latest release.
    pub fn update_to_latest_release(show_output: bool) -> Result<Status> {
        let status = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(env!("CARGO_PKG_VERSION"))
            .show_download_progress(show_output)
            .no_confirm(true)
            .show_output(show_output)
            .build()
            .map_err(CliError::self_update_build_error)?
            .update()
            .map_err(CliError::self_update_error)?;

        Ok(status)
    }

    /// Check if there is an available update for `leo` and return the newest release.
    pub fn update_available() -> Result<String> {
        let updater = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(env!("CARGO_PKG_VERSION"))
            .build()
            .map_err(CliError::self_update_error)?;

        let current_version = updater.current_version();
        let latest_release = updater.get_latest_release().map_err(CliError::self_update_error)?;

        if bump_is_greater(&current_version, &latest_release.version).map_err(CliError::self_update_error)? {
            Ok(latest_release.version)
        } else {
            Err(CliError::old_release_version(current_version, latest_release.version).into())
        }
    }

    /// Read the latest version from the version file.
    pub fn read_latest_version() -> Result<Option<String>, CliError> {
        let version_file_path = Self::get_version_file_path()?;
        match fs::read_to_string(version_file_path) {
            Ok(version) => Ok(Some(version.trim().to_string())),
            Err(_) => Ok(None),
        }
    }

    /// Generate the CLI message if a new version is available.
    pub fn get_cli_string() -> Result<Option<String>, CliError> {
        if let Some(latest_version) = Self::read_latest_version()? {
            let colorized_message = format!(
                "\n🟢 {} {} {}",
                "A new version is available! Run".bold().green(),
                "`leo update`".bold().white(),
                format!("to update to v{}.", latest_version).bold().green()
            );
            Ok(Some(colorized_message))
        } else {
            Ok(None)
        }
    }

    /// Display the CLI message if a new version is available.
    pub fn print_cli() -> Result<(), CliError> {
        if let Some(message) = Self::get_cli_string()? {
            println!("{}", message);
        }
        Ok(())
    }

    /// Check for updates, respecting the update interval. (Currently once per day.)
    /// If a new version is found, write it to a cache file and alert in every call.
    pub fn check_for_updates(force: bool) -> Result<bool, CliError> {
        // Get the cache directory and relevant file paths.
        let cache_dir = Self::get_cache_dir()?;
        let last_check_file = cache_dir.join(Self::LEO_LAST_CHECK_FILE);
        let version_file = Self::get_version_file_path()?;

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
        } else {
            // We're not checking for updates, so just return whether we have a stored version
            Ok(version_file.exists())
        }
    }

    fn update_check_files(
        cache_dir: &Path,
        last_check_file: &Path,
        version_file: &Path,
        latest_version: &str,
    ) -> Result<(), CliError> {
        fs::create_dir_all(cache_dir).map_err(CliError::cli_io_error)?;

        let current_time = Self::get_current_time()?;

        fs::write(last_check_file, &current_time.to_string()).map_err(CliError::cli_io_error)?;
        fs::write(version_file, latest_version).map_err(CliError::cli_io_error)?;

        Ok(())
    }

    fn should_check_for_updates(last_check_file: &Path) -> Result<bool, CliError> {
        match fs::read_to_string(last_check_file) {
            Ok(contents) => {
                let last_check = contents
                    .parse::<u64>()
                    .map_err(|e| CliError::cli_runtime_error(format!("Failed to parse last check time: {}", e)))?;
                let current_time = Self::get_current_time()?;

                Ok(current_time.saturating_sub(last_check) > Self::LEO_UPDATE_CHECK_INTERVAL.as_secs())
            }
            Err(_) => Ok(true),
        }
    }

    fn get_current_time() -> Result<u64, CliError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| CliError::cli_runtime_error(format!("System time error: {}", e)))
            .map(|duration| duration.as_secs())
    }

    /// Get the path to the file storing the latest version information.
    fn get_version_file_path() -> Result<PathBuf, CliError> {
        Self::get_cache_dir().map(|dir| dir.join(Self::LEO_VERSION_FILE))
    }

    /// Get the cache directory for Leo.
    fn get_cache_dir() -> Result<PathBuf, CliError> {
        dirs::cache_dir()
            .ok_or_else(|| CliError::cli_runtime_error("Failed to get cache directory".to_string()))
            .map(|dir| dir.join("leo"))
    }
}
