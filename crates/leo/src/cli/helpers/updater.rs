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
use self_update::{Download, Extract, Status, backends::github, get_target, update::Release, version::bump_is_greater};
use std::{
    env::consts::EXE_SUFFIX,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct Updater;

impl Updater {
    /// Bundled plugin binaries and their release tag prefixes.
    const BUNDLED_PLUGINS: &'static [(&'static str, &'static str)] =
        &[("leo-fmt", "leo-fmt-v"), ("leo-lsp", "leo-lsp-v")];
    const LEO_BIN_NAME: &'static str = "leo";
    const LEO_CACHE_LAST_CHECK_FILE: &'static str = "leo_cache_last_update_check";
    const LEO_CACHE_VERSION_FILE: &'static str = "leo_cache_latest_version";
    /// Releases are tagged per crate (e.g. `leo-lang-v4.3.2`); the `leo` binary ships in `leo-lang` releases.
    const LEO_LANG_TAG_PREFIX: &'static str = "leo-lang-v";
    const LEO_REPO_NAME: &'static str = "leo";
    const LEO_REPO_OWNER: &'static str = "ProvableHQ";
    // 24 hours
    const LEO_UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

    /// Read a `LEO_UPDATE_*_BASE_URL` override, used by tests to point the updater at a
    /// local mock server. Only loopback URLs are honored: accepting arbitrary hosts would
    /// let a poisoned environment silently redirect where binaries are downloaded from.
    fn base_url_override(var: &str) -> Option<String> {
        let url = std::env::var(var).ok()?;
        if url.starts_with("http://localhost:") || url.starts_with("http://127.0.0.1:") {
            Some(url)
        } else {
            tracing::warn!("Ignoring `{var}`: only loopback URLs may override the update endpoints");
            None
        }
    }

    /// Base URL for release archive downloads. Overridable via `LEO_UPDATE_DOWNLOAD_BASE_URL`
    /// so tests can point the updater at a local mock server.
    fn download_base_url() -> String {
        Self::base_url_override("LEO_UPDATE_DOWNLOAD_BASE_URL").unwrap_or_else(|| "https://github.com".to_string())
    }

    /// Fetch all releases that provide binaries for the current target.
    pub fn fetch_releases() -> Result<Vec<Release>> {
        let mut list = github::ReleaseList::configure();
        list.repo_owner(Self::LEO_REPO_OWNER).repo_name(Self::LEO_REPO_NAME).with_target(get_target());
        if let Some(url) = Self::base_url_override("LEO_UPDATE_API_BASE_URL") {
            list.with_url(&url);
        }
        let releases = list
            .build()
            .map_err(crate::errors::self_update_error)?
            .fetch()
            .map_err(crate::errors::could_not_fetch_versions)?;
        Ok(releases)
    }

    /// Find the newest version among releases whose tag starts with `tag_prefix`,
    /// returned without the prefix (e.g. `4.3.2` for tag `leo-lang-v4.3.2`).
    fn latest_version(releases: &[Release], tag_prefix: &str) -> Option<String> {
        releases
            .iter()
            // `Release::version` is the tag with any leading `v` trimmed, so crate tags keep their prefix.
            .filter_map(|release| release.version.strip_prefix(tag_prefix))
            // Exclude prereleases (e.g. `5.0.0-rc.1`) so they are never installed implicitly, and
            // versions that do not parse so one malformed tag cannot poison the result.
            // A `bump_is_greater` self-comparison doubles as a semver parse check.
            .filter(|version| !version.contains('-') && bump_is_greater(version, version).is_ok())
            // On a version parse failure, keep the current best.
            .reduce(|best, version| if bump_is_greater(best, version).unwrap_or(false) { version } else { best })
            .map(str::to_string)
    }

    /// Extract the bare version (e.g. `4.3.2`) from a user-provided release name,
    /// accepting `4.3.2`, `v4.3.2`, and full tags like `leo-lang-v4.3.2`.
    fn normalize_version(name: &str) -> &str {
        let name = name.strip_prefix(Self::LEO_LANG_TAG_PREFIX).unwrap_or(name);
        name.strip_prefix('v').unwrap_or(name)
    }

    /// Resolve a user-provided release name to `(tag, version)` for a release that ships
    /// the `leo` binary, accepting `4.3.2`, `v4.3.2`, and the full `leo-lang-v4.3.2` tag.
    fn resolve_release_tag(releases: &[Release], name: &str) -> Result<(String, String)> {
        // A plugin tag names a release without the `leo` binary; reject it rather than
        // silently reinstalling `leo` itself at that version.
        if let Some((plugin, _)) = Self::BUNDLED_PLUGINS.iter().find(|(_, prefix)| name.starts_with(prefix)) {
            return Err(crate::errors::custom(format!(
                "`{name}` is a `{plugin}` release and does not contain the `leo` binary"
            ))
            .into());
        }

        let version = Self::normalize_version(name);
        let tag = format!("{}{version}", Self::LEO_LANG_TAG_PREFIX);
        // `Release::version` is the tag with any leading `v` trimmed, so crate tags keep their prefix.
        if releases.iter().any(|release| release.version == tag) {
            return Ok((tag, version.to_string()));
        }

        // Releases predating per-crate tags are matched by their original tag: `v<version>`
        // for a bare or `v`-prefixed version (e.g. `v4.0.2`), the name verbatim otherwise
        // (e.g. `canary-v3.5.0`).
        if releases.iter().any(|release| release.version == version) {
            let tag = if version.starts_with(|c: char| c.is_ascii_digit()) {
                format!("v{version}")
            } else {
                name.to_string()
            };
            return Ok((tag, version.to_string()));
        }

        Err(crate::errors::custom(format!(
            "could not find a `leo` release matching `{name}`; run `leo update --list` to see available versions"
        ))
        .into())
    }

    /// Show all available releases for `leo`.
    pub fn show_available_releases() -> Result<String> {
        let releases = Self::fetch_releases()?;

        let mut output = format!(
            "\nList of available `leo` versions for: {}.\nUse `leo update --name <version>` to install a specific version.\n\n",
            get_target()
        );
        for release in &releases {
            if let Some(version) = release.version.strip_prefix(Self::LEO_LANG_TAG_PREFIX) {
                let _ = writeln!(output, "  * v{version}");
            }
        }

        Ok(output)
    }

    /// Update `leo`. If a version is provided, then `leo` is updated to the specific version
    /// otherwise the update defaults to the latest version.
    ///
    /// The release tag is resolved against `releases` instead of relying on the GitHub
    /// "latest release", which may belong to another crate such as `leo-fmt` or `leo-lsp`.
    pub fn update(show_output: bool, version: Option<String>, releases: &[Release]) -> Result<Status> {
        let current_version = env!("CARGO_PKG_VERSION");

        // Resolve the tag of the release to install.
        let (tag, version) = match version {
            Some(name) => Self::resolve_release_tag(releases, &name)?,
            None => {
                let latest = Self::latest_version(releases, Self::LEO_LANG_TAG_PREFIX)
                    .ok_or_else(|| crate::errors::custom("could not find a `leo` release for this platform"))?;
                // `target_version_tag` installs unconditionally, so check for a newer version first.
                if !bump_is_greater(current_version, &latest).map_err(crate::errors::self_update_error)? {
                    return Ok(Status::UpToDate(latest));
                }
                (format!("{}{latest}", Self::LEO_LANG_TAG_PREFIX), latest)
            }
        };

        let mut update = github::Update::configure();
        update
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(current_version)
            .target_version_tag(&tag)
            .show_download_progress(show_output)
            .no_confirm(true)
            .show_output(show_output);
        if let Some(url) = Self::base_url_override("LEO_UPDATE_API_BASE_URL") {
            update.with_url(&url);
        }
        let status = update
            .build()
            .map_err(crate::errors::self_update_build_error)?
            .update()
            .map_err(crate::errors::self_update_error)?;

        // The crate tag does not parse as a bare version, so report the resolved version instead.
        Ok(match status {
            Status::UpToDate(_) => Status::UpToDate(version),
            Status::Updated(_) => Status::Updated(version),
        })
    }

    /// Best-effort update of the bundled plugin binaries (`leo-fmt` and `leo-lsp`).
    ///
    /// Each plugin ships in its own release (e.g. `leo-fmt-v4.3.2`), so its version is
    /// resolved independently against `releases`. Prints a warning on failure rather
    /// than aborting.
    pub fn update_bundled_plugins(show_output: bool, version: Option<&str>, releases: &[Release]) {
        let install_dir = match std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)) {
            Some(dir) => dir,
            None => {
                tracing::warn!("Could not determine leo install directory; skipping plugin update");
                return;
            }
        };

        for (plugin, tag_prefix) in Self::BUNDLED_PLUGINS {
            if let Err(e) = Self::update_plugin(show_output, plugin, tag_prefix, version, releases, &install_dir) {
                tracing::warn!("Failed to update bundled plugin '{plugin}': {e}");
            }
        }
    }

    /// Best-effort version of an installed plugin binary, read by running `<plugin> --version`
    /// and extracting the first version-shaped token (e.g. `leo-fmt 4.3.2` -> `4.3.2`).
    fn installed_plugin_version(plugin_bin: &Path) -> Option<String> {
        let output = Command::new(plugin_bin).arg("--version").output().ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = String::from_utf8(output.stdout).ok()?;
        stdout
            .split_whitespace()
            .map(|token| token.strip_prefix('v').unwrap_or(token))
            // A `bump_is_greater` self-comparison doubles as a semver parse check.
            .find(|token| bump_is_greater(token, token).is_ok())
            .map(str::to_string)
    }

    /// Download and install a single plugin binary into `install_dir`.
    ///
    /// The archive is downloaded and extracted manually because `self_update` can only
    /// replace the currently running executable reliably: for other binaries it renames
    /// across filesystems and loses the executable bit set in the archive.
    fn update_plugin(
        show_output: bool,
        plugin: &str,
        tag_prefix: &str,
        version: Option<&str>,
        releases: &[Release],
        install_dir: &Path,
    ) -> Result<()> {
        let plugin_version = match version {
            Some(version) => {
                let plugin_version = Self::normalize_version(version).to_string();
                // Plugin releases are not cut for every `leo` version; leave the installed
                // plugin unchanged rather than attempting a download that cannot succeed.
                let plugin_tag = format!("{tag_prefix}{plugin_version}");
                if !releases.iter().any(|release| release.version == plugin_tag) {
                    tracing::warn!("No '{plugin}' release found for version {plugin_version}; '{plugin}' is unchanged");
                    return Ok(());
                }
                plugin_version
            }
            None => Self::latest_version(releases, tag_prefix).ok_or_else(|| {
                crate::errors::custom(format!("could not find a `{plugin}` release for this platform"))
            })?,
        };

        let bin_name = format!("{plugin}{EXE_SUFFIX}");
        let dest = install_dir.join(&bin_name);

        // Skip when the installed plugin already reports the newest released version. On a
        // probe or version parse failure, attempt the install and let it surface any real error.
        if version.is_none()
            && let Some(installed) = Self::installed_plugin_version(&dest)
            && !bump_is_greater(&installed, &plugin_version).unwrap_or(true)
        {
            if show_output {
                tracing::info!("'{plugin}' is already on the latest version");
            }
            return Ok(());
        }

        if show_output {
            tracing::info!("Updating bundled plugin '{plugin}' to v{plugin_version}...");
        }

        // Download the release archive into a temporary directory and extract the plugin binary.
        let tag = format!("{tag_prefix}{plugin_version}");
        let archive_name = format!("{tag}-{}.zip", get_target());
        let url = format!(
            "{}/{}/{}/releases/download/{tag}/{archive_name}",
            Self::download_base_url(),
            Self::LEO_REPO_OWNER,
            Self::LEO_REPO_NAME
        );
        let tmp_dir = tempfile::tempdir().map_err(crate::errors::cli_io_error)?;
        let archive_path = tmp_dir.path().join(&archive_name);
        let mut archive = fs::File::create(&archive_path).map_err(crate::errors::cli_io_error)?;
        let mut download = Download::from_url(&url);
        download.show_progress(show_output);
        download.download_to(&mut archive).map_err(crate::errors::self_update_error)?;
        Extract::from_source(&archive_path)
            .extract_file(tmp_dir.path(), &bin_name)
            .map_err(crate::errors::self_update_error)?;

        // Stage next to the destination so the final rename is atomic and never crosses filesystems.
        let staged = install_dir.join(format!(".{bin_name}.tmp"));
        fs::copy(tmp_dir.path().join(&bin_name), &staged).map_err(crate::errors::cli_io_error)?;
        // Zip extraction does not preserve the executable bit.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&staged, fs::Permissions::from_mode(0o755)).map_err(crate::errors::cli_io_error)?;
        }
        if let Err(rename_error) = fs::rename(&staged, &dest) {
            // A running plugin (e.g. an editor's `leo-lsp`) locks its executable against
            // replacement on Windows, but a locked executable can still be renamed; move
            // the old binary aside and retry.
            let displaced = install_dir.join(format!(".{bin_name}.old"));
            if fs::rename(&dest, &displaced).and_then(|()| fs::rename(&staged, &dest)).is_err() {
                // Best-effort cleanup of the staged binary; the original rename error is the
                // one worth reporting.
                let _ = fs::remove_file(&staged);
                return Err(crate::errors::cli_io_error(rename_error).into());
            }
            // Deleting the displaced binary fails on Windows while it is still running; it is
            // then left behind as a hidden `.<bin>.old` file and replaced on the next update.
            let _ = fs::remove_file(&displaced);
        }

        if show_output {
            tracing::info!("Successfully updated '{plugin}' to v{plugin_version}");
        }
        Ok(())
    }

    /// Check if there is an available update for `leo` and return the newest release.
    pub fn update_available() -> Result<String> {
        let current_version = env!("CARGO_PKG_VERSION");
        let releases = Self::fetch_releases()?;
        let latest_version = Self::latest_version(&releases, Self::LEO_LANG_TAG_PREFIX)
            .ok_or_else(|| crate::errors::custom("could not find a `leo` release for this platform"))?;

        if bump_is_greater(current_version, &latest_version).map_err(crate::errors::self_update_error)? {
            Ok(latest_version)
        } else {
            Err(crate::errors::old_release_version(current_version, latest_version).into())
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
                "A new version is available! Run".bold().green(),
                "`leo update`".bold().white(),
                format!("to update to v{latest_version}.").bold().green()
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
