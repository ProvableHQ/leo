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
use crate::config::Config;
use anyhow::{anyhow, Error};

use colored::Colorize;
use self_update::{backends::github, version::bump_is_greater, Status};

pub struct Updater;

// TODO Add logic for users to easily select release versions.
impl Updater {
    const LEO_BIN_NAME: &'static str = "leo";
    const LEO_REPO_NAME: &'static str = "leo";
    const LEO_REPO_OWNER: &'static str = "AleoHQ";

    /// Show all available releases for `leo`.
    pub fn show_available_releases() -> Result<(), Error> {
        let releases = github::ReleaseList::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .build()?
            .fetch()?;

        let mut output = "\nList of available versions\n".to_string();
        for release in releases {
            output += &format!("  * {}\n", release.version);
        }

        // Forgo using tracing to list the available versions without a log status.
        println!("{}", output);

        Ok(())
    }

    /// Update `leo` to the latest release.
    pub fn update_to_latest_release(show_output: bool) -> Result<Status, Error> {
        let status = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(&env!("CARGO_PKG_VERSION"))
            .show_download_progress(show_output)
            .no_confirm(true)
            .show_output(show_output)
            .build()?
            .update()?;

        Ok(status)
    }

    /// Check if there is an available update for `leo` and return the newest release.
    pub fn update_available() -> Result<String, Error> {
        let updater = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(&env!("CARGO_PKG_VERSION"))
            .build()?;

        let current_version = updater.current_version();
        let latest_release = updater.get_latest_release()?;

        if bump_is_greater(&current_version, &latest_release.version)? {
            Ok(latest_release.version)
        } else {
            Err(anyhow!(
                "Old release version {} {}",
                current_version,
                latest_release.version
            ))
        }
    }

    /// Display the CLI message, if the Leo configuration allows.
    pub fn print_cli() {
        let config = Config::read_config().unwrap();

        if config.update.automatic {
            // If the auto update configuration is on, attempt to update the version.
            if let Ok(status) = Self::update_to_latest_release(false) {
                if status.updated() {
                    tracing::info!("Successfully updated to {}", status.version());
                }
            }
        } else {
            // If the auto update configuration is off, notify the user to update leo.
            if let Ok(latest_version) = Self::update_available() {
                let mut message = "🟢 A new version is available! Run".bold().green().to_string();
                message += &" `leo update` ".bold().white();
                message += &format!("to update to v{}.", latest_version).bold().green();

                tracing::info!("\n{}\n", message);
            }
        }
    }
}
