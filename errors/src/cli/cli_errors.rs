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
    /// CliError enum that represents all the errors for the `leo-lang` crate.
    CliError,
    exit_code_mask: 7000i32,
    error_code_prefix: "CLI",

    /// Not actually ever returned anywhere outside a test.
    @backtraced
    opt_args_error {
        args: (error: impl ErrorArg),
        msg: format!("opt arg error {}", error),
        help: None,
    }

    /// For when APM returns an unidentifed API error.
    @backtraced
    unidentified_api {
        args: (),
        msg: "Unidentified API error",
        help: None,
    }

    /// For when the CLI is unable to connect to a Leo Package Manager.
    @backtraced
    unable_to_connect_aleo_pm {
        args: (),
        msg: "Unable to connect to Aleo PM. If you specified custom API endpoint, then check the URL for errors",
        help: None,
    }

    /// For when APM is unable to find to a package.
    @backtraced
    package_not_found {
        args: (),
        msg: "Package is not found - check author and/or package name",
        help: None,
    }

    /// For when APM returns an unknown API error.
    @backtraced
    unkown_api_error {
        args: (status: impl Display),
        msg: format!("Unknown API error: {}", status),
        help: None,
    }

    /// For when the APM account username is not registered, or password is incorrect.
    @backtraced
    account_not_found {
        args: (),
        msg: "This username is not yet registered or the password is incorrect",
        help: None,
    }

    /// For when APM account password is incorrect.
    @backtraced
    incorrect_password {
        args: (),
        msg: "Incorrect password",
        help: None,
    }

    /// For when a request to APM fails witha bad request status.
    @backtraced
    bad_request {
        args: (msg: impl Display),
        msg: msg,
        help: None,
    }

    /// For when the user is performing some APM action that
    /// requires the user to be logged in first.
    @backtraced
    not_logged_in {
        args: (),
        msg: "You are not logged in. Please use `leo login` to login",
        help: None,
    }

    /// For when a package with the same name and author name is already published.
    @backtraced
    already_published {
        args: (),
        msg: "This package version is already published",
        help: None,
    }

    /// For when APM is experiencing a HTTP Status of 500.
    @backtraced
    internal_server_error {
        args: (),
        msg: "Server error, please contact us at https://github.com/AleoHQ/leo/issues",
        help: None,
    }

    /// For when the reqwest library fails to get the request JSON.
    @backtraced
    reqwest_json_error {
        args: (error: impl ErrorArg),
        msg: format!("request JSON failed {}", error),
        help: None,
    }

    /// For when the user provides an incorrect command argument.
    @backtraced
    incorrect_command_argument {
        args: (),
        msg: "Incorrect argument, please use --help for information on command use",
        help: None,
    }

    /// For when the CLI experiences an IO error.
    @backtraced
    cli_io_error {
        args: (error: impl ErrorArg),
        msg: format!("cli io error {}", error),
        help: None,
    }

    /// For when the CLI experiences a bytes conversion error.
    @backtraced
    cli_bytes_conversion_error {
        args: (error: impl ErrorArg),
        msg: format!("cli bytes conversion error {}", error),
        help: None,
    }

    /// For when the CLI experiences a zip error.
    @backtraced
    cli_zip_error {
        args: (error: impl ErrorArg),
        msg: format!("cli zip error {}", error),
        help: None,
    }

    /// For when the CLI cannot find the manifest file.
    @backtraced
    manifest_file_not_found {
        args: (),
        msg: "Package manifest not found, try running `leo init`",
        help: None,
    }

    /// For when the CLI was unable to get the user token.
    @backtraced
    unable_to_get_user_token {
        args: (),
        msg: "Unable to get token",
        help: None,
    }

    /// For when the CLI was supplied an incorrect user token.
    @backtraced
    supplied_token_is_incorrect {
        args: (),
        msg: "Supplied token is incorrect",
        help: None,
    }

    /// For when the CLI user's stored credentials expire.
    @backtraced
    stored_credentials_expired {
        args: (),
        msg: "Stored credentials are incorrect or expired, please login again",
        help: None,
    }

    /// For when the user does not provide credentials to the CLI.
    @backtraced
    no_credentials_provided {
        args: (),
        msg: "No credentials provided",
        help: None,
    }

    /// For when the CLI does not have persmission to modify the .leo folder
    /// and cannot logout the user.
    @backtraced
    logout_permision_denied {
        args: (),
        msg: "permission denied - check file permission in .leo folder",
        help: None,
    }

    /// For when the CLI cannot access the logout file.
    @backtraced
    cannot_access_logout_file {
        args: (),
        msg: "something went wrong, can't access the file",
        help: None,
    }

    /// For when the user tries to name a package after a Leo keyword.
    @backtraced
    package_cannot_be_named_after_a_keyword {
        args: (),
        msg: "Cannot be named a package after a keyword",
        help: None,
    }

    /// For when the user has not provided a package description.
    @backtraced
    no_package_description {
        args: (),
        msg: "No package description",
        help: None,
    }

    /// For when the user has not provided a package license.
    @backtraced
    missing_package_license {
        args: (),
        msg: "Missing package license",
        help: None,
    }

    /// For when the package is missing its remote section in the Leo.toml file.
    @backtraced
    missing_package_remote {
        args: (),
        msg: "Missing package remote",
        help: None,
    }

    /// For when the user has not provided the package author field in the Leo.toml file.
    @backtraced
    package_author_is_not_set {
        args: (),
        msg: "Package author is not set. Specify package author in [remote] section of Leo.toml",
        help: None,
    }

    /// For when the CLI fails to convert an object to TOML.
    @backtraced
    failed_to_convert_to_toml {
        args: (error: impl ErrorArg),
        msg: format!("failed to covnert to TOML {}", error),
        help: None,
    }

    /// For when the CLI fails to TOML an object.
    @backtraced
    failed_to_convert_from_toml {
        args: (error: impl ErrorArg),
        msg: format!("failed to covnert from TOML {}", error),
        help: None,
    }

    /// For when the current package directory doesn't exist.
    @backtraced
    package_directory_does_not_exist {
        args: (),
        msg: "Directory does not exist",
        help: None,
    }

    /// For when the current project has an invalid name.
    @backtraced
    invalid_project_name {
        args: (),
        msg: "Project name invalid",
        help: None,
    }

    /// For when the current package has an invalid name.
    @backtraced
    invalid_package_name {
        args: (name: impl Display),
        msg: format!("Invalid Leo package name: {}", name),
        help: None,
    }

    /// For when the package main.leo file is not found.
    @backtraced
    package_main_file_not_found {
        args: (),
        msg: "File main.leo not found in src/ directory",
        help: None,
    }

    /// For when the package directory already exists.
    @backtraced
    package_directory_already_exists {
        args: (path: impl Debug),
        msg: format!("Directory already exists {:?}", path),
        help: None,
    }

    /// For when the CLI could not a directory.
    @backtraced
    package_could_not_create_directory {
        args: (error: impl ErrorArg),
        msg: format!("Could not create directory {}", error),
        help: None,
    }

    /// For when the CLI could not setup a Leo command.
    @backtraced
    unable_to_setup {
        args: (),
        msg: "Unable to setup, see command output for more details",
        help: None,
    }

    /// For when the program file does not exist.
    @backtraced
    program_file_does_not_exist {
        args: (path: impl Display),
        msg: format!("Program file does not exist {}", path),
        help: None,
    }

    /// For when the CLI could not fetch the versions.
    @backtraced
    could_not_fetch_versions {
        args: (error: impl ErrorArg),
        msg: format!("Could not fetch versions: {}", error),
        help: None,
    }

    /// For when the CLI failed to watch the Leo package.
    @backtraced
    unable_to_watch {
        args: (error: impl ErrorArg),
        msg: format!("Unable to watch, check that directory contains Leo.toml file. Error: {}", error),
        help: None,
    }

    /// For when the CLI fails to enable ansi support.
    @backtraced
    failed_to_enable_ansi_support {
        args: (),
        msg: "failed to enable ansi_support",
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate Error: {}", error),
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_build_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate failed to build Error: {}", error),
        help: None,
    }

    /// For when the CLI has an old release version.
    @backtraced
    old_release_version {
        args: (current: impl Display, latest: impl Display),
        msg: format!("Old release version {} {}", current, latest),
        help: None,
    }

    @backtraced
    dependencies_are_not_installed {
        args: (),
        msg: "dependencies are not installed, please run `leo fetch` first",
        help: None,
    }

    @backtraced
    recursive_dependency_found {
        args: (message: impl Display),
        msg: format!("recursive dependency found \n{}", message),
        help: None,
    }

    @backtraced
    unable_to_read_imported_dependency_manifest {
        args: (),
        msg: "unable to parse imported dependency's manifest",
        help: None,
    }
);

impl CliError {
    /// Possible errors that can be thrown when the CLI is removing the token and username.
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
