// Copyright (C) 2019-2025 Provable Inc.
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

// use serde::Serialize;
// use std::sync::OnceLock;

// // Include the generated build information
// mod built_info {
//     include!(concat!(env!("OUT_DIR"), "/built.rs"));
// }

// // Cache for version info to avoid repeated string allocations
// static VERSION_INFO: OnceLock<VersionInfo> = OnceLock::new();

// #[derive(Clone, Debug, Serialize)]
// pub struct VersionInfo {
//     /// The version from Cargo.toml
//     pub version: String,
//     /// Git commit hash
//     pub git_commit: String,
//     /// Git branch name
//     pub git_branch: String,
// }

// impl VersionInfo {
//     /// Get the cached version information
//     pub fn get() -> &'static VersionInfo {
//         VERSION_INFO.get_or_init(|| VersionInfo {
//             version: built_info::PKG_VERSION.to_string(),
//             git_commit: built_info::GIT_COMMIT_HASH.unwrap_or("unknown").to_string(),
//             git_branch: built_info::GIT_HEAD_REF.unwrap_or("unknown").to_string(),
//         })
//     }
// }
