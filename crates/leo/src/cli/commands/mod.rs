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

mod abi;
pub use abi::LeoAbi;

mod add;
pub use add::{DependencySource, LeoAdd};

mod account;
pub use account::Account;

mod build;
pub use build::LeoBuild;

mod clean;
pub use clean::LeoClean;

mod common;
pub use common::*;

mod deploy;
pub use deploy::LeoDeploy;
use deploy::{Task, compute_deployment_stats, print_deployment_plan, print_deployment_summary};

mod devnet;
pub use devnet::LeoDevnet;

mod devnode;
pub use devnode::LeoDevnode;

mod execute;
pub use execute::LeoExecute;

mod format;
pub use format::LeoFormat;

pub mod query;
pub use query::LeoQuery;

mod new;
pub use new::LeoNew;

mod remove;
pub use remove::LeoRemove;

mod run;
pub use run::LeoRun;

mod synthesize;
pub use synthesize::LeoSynthesize;

mod test;
pub use test::LeoTest;

mod update;
pub use update::LeoUpdate;

pub mod upgrade;
pub use upgrade::LeoUpgrade;

use super::*;
use crate::cli::{helpers::context::*, query::QueryCommands};

use leo_errors::{CliError, Handler, PackageError, Result};
use snarkvm::{
    console::network::Network,
    prelude::{Address, Ciphertext, Plaintext, PrivateKey, Record, Value, ViewKey, block::Transaction},
};

use clap::{Args, Parser};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::{iter, str::FromStr};
use tracing::span::Span;
use ureq::http::Uri;

/// Base trait for the Leo CLI, see methods and their documentation for details.
pub trait Command {
    /// If the current command requires running another command beforehand
    /// and needs its output result, this is where the result type is defined.
    /// Example: type Input: <CommandA as Command>::Out
    type Input;

    /// Defines the output of this command, which may be used as `Input` for another
    /// command. If this command is not used as a prelude for another command,
    /// this field may be left empty.
    type Output;

    /// Adds a span to the logger via `tracing::span`.
    /// Because of the specifics of the macro implementation, it is not possible
    /// to set the span name with a non-literal i.e. a dynamic variable even if this
    /// variable is a &'static str.
    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    /// Runs the prelude and returns the Input of the current command.
    fn prelude(&self, context: Context) -> Result<Self::Input>
    where
        Self: std::marker::Sized;

    /// Runs the main operation of this command. This function is run within
    /// context of 'execute' function, which sets logging and timers.
    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output>
    where
        Self: std::marker::Sized;

    /// A wrapper around the `apply` method.
    /// This function sets up tracing, timing, and the context.
    fn execute(self, context: Context) -> Result<Self::Output>
    where
        Self: std::marker::Sized,
    {
        let input = self.prelude(context.clone())?;

        // Create the span for this command.
        let span = self.log_span();
        let span = span.enter();

        // Calculate the execution time for this command.
        let out = self.apply(context, input);

        drop(span);

        out
    }

    /// Executes command but empty the result. Comes in handy where there's a
    /// need to make match arms compatible while keeping implementation-specific
    /// output possible. Errors however are all of the type Error
    fn try_execute(self, context: Context) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        self.execute(context).map(|_| Ok(()))?
    }
}

/// A helper function to parse an input string into a `Value`, handling record ciphertexts as well.
pub fn parse_input<N: Network>(input: &str, private_key: &PrivateKey<N>) -> Result<Value<N>> {
    // Trim whitespace from the input.
    let input = input.trim();
    // Check if the input is a record ciphertext.
    if input.starts_with("record1") {
        // Get the view key from the private key.
        let view_key = ViewKey::<N>::try_from(private_key)
            .map_err(|e| CliError::custom(format!("Failed to view key from the private key: {e}")))?;
        // Parse the input as a record.
        Record::<N, Ciphertext<N>>::from_str(input)
            .and_then(|ciphertext| ciphertext.decrypt(&view_key))
            .map(Value::Record)
            .map_err(|e| CliError::custom(format!("Failed to parse input as record: {e}")).into())
    } else {
        // Pre-validate numeric literals to reject malformed inputs that snarkvm would silently coerce to zero.
        validate_cli_literal(input)?;
        Value::from_str(input).map_err(|e| CliError::custom(format!("Failed to parse input: {e}")).into())
    }
}

/// Pre-validates a CLI input literal to reject malformed numeric literals
/// that snarkvm would silently coerce to zero (e.g. `truefield`, `""field`).
fn validate_cli_literal(input: &str) -> Result<()> {
    // Suffixes ordered longest-first to avoid ambiguous matches (e.g. u128 before u8).
    const UNSIGNED_SUFFIXES: &[&str] = &["u128", "u64", "u32", "u16", "u8"];
    const SIGNED_SUFFIXES: &[&str] = &["i128", "i64", "i32", "i16", "i8"];
    const FIELD_LIKE_SUFFIXES: &[&str] = &["field", "scalar", "group"];

    for suffix in UNSIGNED_SUFFIXES {
        if let Some(prefix) = input.strip_suffix(suffix) {
            return validate_numeric_prefix(prefix, suffix, false, false);
        }
    }
    for suffix in SIGNED_SUFFIXES {
        if let Some(prefix) = input.strip_suffix(suffix) {
            return validate_numeric_prefix(prefix, suffix, true, false);
        }
    }
    for suffix in FIELD_LIKE_SUFFIXES {
        if let Some(prefix) = input.strip_suffix(suffix) {
            // Group supports coordinate pair syntax like (x, y)group — defer to snarkvm.
            if *suffix == "group" && prefix.starts_with('(') {
                return Ok(());
            }
            return validate_numeric_prefix(prefix, suffix, true, true);
        }
    }

    // For all other inputs (bool, address, struct, record, etc.), skip pre-validation.
    Ok(())
}

/// Validates that `prefix` is a well-formed numeric string for the given type `suffix`.
fn validate_numeric_prefix(prefix: &str, suffix: &str, allow_negative: bool, decimal_only: bool) -> Result<()> {
    if prefix.is_empty() {
        return Err(
            CliError::custom(format!("Invalid {suffix} literal: missing numeric value before '{suffix}'")).into()
        );
    }
    let valid = if decimal_only {
        is_valid_decimal(prefix, allow_negative)
    } else {
        is_valid_decimal(prefix, allow_negative) || is_valid_radix_prefixed(prefix, allow_negative)
    };
    if !valid {
        return Err(
            CliError::custom(format!("Invalid {suffix} literal: '{prefix}' is not a valid numeric value")).into()
        );
    }
    Ok(())
}

/// Checks if `s` is a valid decimal integer string (e.g. `123`, `-42`, `1_000`).
fn is_valid_decimal(s: &str, allow_negative: bool) -> bool {
    let s = if allow_negative { s.strip_prefix('-').unwrap_or(s) } else { s };
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    // Must start with a digit.
    if !chars.next().unwrap().is_ascii_digit() {
        return false;
    }
    chars.all(|c| c.is_ascii_digit() || c == '_')
}

/// Checks if `s` is a valid radix-prefixed integer string (e.g. `0xFF`, `0b1010`, `0o77`).
fn is_valid_radix_prefixed(s: &str, allow_negative: bool) -> bool {
    let s = if allow_negative { s.strip_prefix('-').unwrap_or(s) } else { s };
    if s.len() < 3 || !s.starts_with('0') {
        return false;
    }
    let radix_char = s.as_bytes()[1];
    let rest = &s[2..];
    if rest.is_empty() || rest.starts_with('_') {
        return false;
    }
    match radix_char {
        b'x' | b'X' => rest.chars().all(|c| c.is_ascii_hexdigit() || c == '_'),
        b'o' => rest.chars().all(|c| matches!(c, '0'..='7' | '_')),
        b'b' => rest.chars().all(|c| matches!(c, '0' | '1' | '_')),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_literals() {
        let valid = [
            "42field",
            "-7field",
            "0field",
            "1_000_000field",
            "100u64",
            "0x1Fu8",
            "0b1010u32",
            "0o77u16",
            "-128i8",
            "-0x80i16",
            "0scalar",
            "42group",
            "(1, 2)group",
            // Non-numeric types: no suffix match, so validation is skipped.
            "true",
            "false",
            "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9",
        ];
        for input in &valid {
            assert!(validate_cli_literal(input).is_ok(), "expected '{input}' to be valid");
        }
    }

    #[test]
    fn test_invalid_literals() {
        let invalid = [
            "truefield",
            "falsefield",
            "field",
            "scalar",
            "u8",
            "abcu64",
            "-u8",
            "hello_worldscalar",
            "truegroup",
            "xxxi128",
        ];
        for input in &invalid {
            assert!(validate_cli_literal(input).is_err(), "expected '{input}' to be invalid");
        }
    }
}
