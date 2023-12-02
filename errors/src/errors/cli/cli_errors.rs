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

use crate::create_messages;
use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_messages!(
    /// CliError enum that represents all the errors for the `leo-lang` crate.
    CliError,
    code_mask: 7000i32,
    code_prefix: "CLI",

    /// For when the CLI experiences an IO error.
    @backtraced
    cli_io_error {
        args: (error: impl ErrorArg),
        msg: format!("cli io error {error}"),
        help: None,
    }

    /// For when the CLI could not fetch the versions.
    @backtraced
    could_not_fetch_versions {
        args: (error: impl ErrorArg),
        msg: format!("Could not fetch versions: {error}"),
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
        msg: format!("self update crate Error: {error}"),
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_build_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate failed to build Error: {error}"),
        help: None,
    }

    /// For when the CLI has an old release version.
    @backtraced
    old_release_version {
        args: (current: impl Display, latest: impl Display),
        msg: format!("Old release version {current} {latest}"),
        help: None,
    }

    @backtraced
    failed_to_load_instructions {
        args: (error: impl Display),
        msg: format!("Failed to load compiled Aleo instructions into an Aleo file.\nSnarkVM Error: {error}"),
        help: Some("Generated Aleo instructions have been left in `main.aleo`".to_string()),
    }

    @backtraced
    needs_leo_build {
        args: (),
        msg: "You must run leo build before deploying a program.".to_string(),
        help: None,
    }

    @backtraced
    failed_to_execute_build {
        args: (error: impl Display),
        msg: format!("Failed to execute the `build` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_new {
        args: (error: impl Display),
        msg: format!("Failed to execute the `new` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_run {
        args: (error: impl Display),
        msg: format!("Failed to execute the `run` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_node {
        args: (error: impl Display),
        msg: format!("Failed to execute the `node` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_deploy {
        args: (error: impl Display),
        msg: format!("Failed to execute the `deploy` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_new {
        args: (error: impl Display),
        msg: format!("Failed to parse the `new` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_run {
        args: (error: impl Display),
        msg: format!("Failed to parse the `run` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_node {
        args: (error: impl Display),
        msg: format!("Failed to parse the `node` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_deploy {
        args: (error: impl Display),
        msg: format!("Failed to parse the `deploy` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_execute {
        args: (error: impl Display),
        msg: format!("Failed to parse the `execute` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_execute {
        args: (error: impl Display),
        msg: format!("Failed to execute the `execute` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_seed {
        args: (error: impl Display),
        msg: format!("Failed to parse the seed string for account.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_write_file {
        args: (error: impl Display),
        msg: format!("Failed to write file.\nIO Error: {error}"),
        help: None,
    }
);
