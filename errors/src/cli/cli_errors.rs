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

use crate::create_errors;
use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_errors!(
    CliError,
    exit_code_mask: 7000i32,
    error_code_prefix: "CLI",

    @backtraced
    unidentified_api {
        args: (),
        msg: "Unidentified API error",
        help: None,
    }

    @backtraced
    unable_to_connect_aleo_pm {
        args: (),
        msg: "Unable to connect to Aleo PM. If you specified custom API endpoint, then check the URL for errors",
        help: None,
    }

    @backtraced
    package_not_found {
        args: (),
        msg: "Package is not found - check author and/or package name",
        help: None,
    }

    @backtraced
    unkown_api_error {
        args: (status: impl Display),
        msg: format!("Unknown API error: {}", status),
        help: None,
    }

    @backtraced
    account_not_found {
        args: (),
        msg: "This username is not yet registered or the password is incorrect",
        help: None,
    }

    @backtraced
    incorrect_password {
        args: (),
        msg: "",
        help: None,
    }

    @backtraced
    bad_request {
        args: (msg: impl Display),
        msg: msg,
        help: None,
    }

    @backtraced
    not_logged_in {
        args: (),
        msg: "You are not logged in. Please use `leo login` to login",
        help: None,
    }

    @backtraced
    already_published {
        args: (),
        msg: "This package version is already published",
        help: None,
    }

    @backtraced
    internal_server_error {
        args: (),
        msg: "Server error, please contact us at https://github.com/AleoHQ/leo/issues",
        help: None,
    }

    @backtraced
    reqwest_json_error {
        args: (error: impl ErrorArg),
        msg: format!("request json failed {}", error),
        help: None,
    }

    @backtraced
    incorrect_command_argument {
        args: (),
        msg: "Incorrect argument, please use --help for information on command use",
        help: None,
    }

    @backtraced
    cli_io_error {
        args: (error: impl ErrorArg),
        msg: format!("cli io error {}", error),
        help: None,
    }

    @backtraced
    cli_bytes_conversion_error {
        args: (error: impl ErrorArg),
        msg: format!("cli bytes conversion error {}", error),
        help: None,
    }

    @backtraced
    cli_zip_error {
        args: (error: impl ErrorArg),
        msg: format!("cli zip error {}", error),
        help: None,
    }

    @backtraced
    mainifest_file_not_found {
        args: (),
        msg: "Package manifest not found, try running `leo init`",
        help: None,
    }

    @backtraced
    unable_to_get_token {
        args: (),
        msg: "Unable to get token",
        help: None,
    }

    @backtraced
    supplied_token_is_incorrect {
        args: (),
        msg: "Supplied token is incorrect",
        help: None,
    }

    @backtraced
    stored_credentials_expired {
        args: (),
        msg: "Stored credentials are incorrect or expired, please login again",
        help: None,
    }

    @backtraced
    no_credentials_provided {
        args: (),
        msg: "No credentials provided",
        help: None,
    }

    @backtraced
    logout_permision_denied {
        args: (),
        msg: "permission denied - check file permission in .leo folder",
        help: None,
    }

    @backtraced
    cannot_access_logout_file {
        args: (),
        msg: "something went wrong, can't access the file",
        help: None,
    }

    @backtraced
    package_cannot_be_named_after_a_keyword {
        args: (),
        msg: "Cannot be named a package after a keyword",
        help: None,
    }

    @backtraced
    no_package_description {
        args: (),
        msg: "No package description",
        help: None,
    }

    @backtraced
    missing_package_license {
        args: (),
        msg: "Missing package license",
        help: None,
    }

    @backtraced
    missing_package_remote {
        args: (),
        msg: "Missing package remote",
        help: None,
    }

    @backtraced
    package_author_is_not_set {
        args: (),
        msg: "Package author is not set. Specify package author in [remote] section of Leo.toml",
        help: None,
    }

    @backtraced
    failed_to_convert_to_toml {
        args: (error: impl ErrorArg),
        msg: format!("failed to covnert to toml {}", error),
        help: None,
    }

    @backtraced
    failed_to_convert_from_toml {
        args: (error: impl ErrorArg),
        msg: format!("failed to covnert from toml {}", error),
        help: None,
    }

    @backtraced
    package_directory_does_not_exist {
        args: (),
        msg: "Directory does not exist",
        help: None,
    }

    @backtraced
    invalid_project_name {
        args: (),
        msg: "Project name invalid",
        help: None,
    }

    @backtraced
    invalid_package_name {
        args: (name: impl Display),
        msg: format!("Invalid Leo project name: {}", name),
        help: None,
    }

    @backtraced
    package_main_file_not_found {
        args: (),
        msg: "File main.leo not found in src/ directory",
        help: None,
    }

    @backtraced
    package_directory_already_exists {
        args: (path: impl Debug),
        msg: format!("Directory already exists {:?}", path),
        help: None,
    }

    @backtraced
    package_could_not_create_directory {
        args: (error: impl ErrorArg),
        msg: format!("Could not create directory {}", error),
        help: None,
    }

    @backtraced
    unable_to_setup {
        args: (),
        msg: "Unable to setup, see command output for more details",
        help: None,
    }

    @backtraced
    program_file_does_not_exist {
        args: (path: impl Display),
        msg: format!("Program file does not exist {}", path),
        help: None,
    }

    @backtraced
    could_not_fetch_versions {
        args: (error: impl ErrorArg),
        msg: format!("Could not fetch versions: {}", error),
        help: None,
    }

    @backtraced
    unable_to_watch {
        args: (error: impl ErrorArg),
        msg: format!("Unable to watch, check that directory contains Leo.toml file. Error: {}", error),
        help: None,
    }

    @backtraced
    failed_to_enable_ansi_support {
        args: (),
        msg: "failed to enable ansi_support",
        help: None,
    }

    @backtraced
    self_update_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate Error: {}", error),
        help: None,
    }

    @backtraced
    old_release_version {
        args: (current: impl Display, latest: impl Display),
        msg: format!("Old release version {} {}", current, latest),
        help: None,
    }
);

impl CliError {
    pub fn remove_token_and_username(error: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match error.kind() {
            ErrorKind::NotFound => {
                // tracing::info!("you are not logged in");
                Self::not_logged_in()
            }
            ErrorKind::PermissionDenied => {
                // tracing::error!("permission denied - check file permission in .leo folder");
                Self::logout_permision_denied()
            }
            _ => {
                // tracing::error!("something went wrong, can't access the file");
                Self::cannot_access_logout_file()
            }
        }
    }
}
