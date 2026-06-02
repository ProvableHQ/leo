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

//! Shared per-command helpers: program input parsing, edition checks,
//! `--with`-style program loading, etc. Moved out of
//! `crates/leo/src/cli/commands/common/util.rs` so the shared
//! `handle_run` / `handle_test` cores can reach them.

#![cfg(not(target_arch = "wasm32"))]

use crate::errors;

use leo_errors::{Backtraced, Result};
use leo_package::Edition;

use snarkvm::prelude::{
    Ciphertext,
    ConsensusVersion,
    Network,
    PrivateKey,
    Program,
    Record,
    Value,
    ViewKey,
    store::helpers::memory::ConsensusMemory,
};

use std::str::FromStr as _;

/// Prints a program's ID and source (local or network edition).
pub fn print_program_source(id: &str, edition: Option<Edition>) {
    match (id, edition) {
        ("credits.aleo", _) => println!("  - {id} (already included)"),
        (_, Some(e)) => println!("  - {id} (edition: {e})"),
        (_, None) => println!("  - {id} (local)"),
    }
}

/// Checks if any programs violate edition/constructor requirements.
///
/// Programs at edition 0 without a constructor cannot be executed after
/// `ConsensusVersion::V8`. Call before attempting execution.
pub fn check_edition_constructor_requirements<N: Network>(
    programs: &[(Program<N>, Edition)],
    consensus_version: ConsensusVersion,
    action: &str,
) -> Result<(), Backtraced> {
    if consensus_version < ConsensusVersion::V8 {
        return Ok(());
    }

    for (program, edition) in programs {
        if *edition == 0 && !program.contains_constructor() {
            let id = program.id();
            if id.to_string() != "credits.aleo" {
                return Err(errors::custom(format!(
                    "Cannot {action} with dependency '{id}' (edition 0)\n\n\
                    Programs at edition 0 without a constructor cannot be executed under \
                    consensus version V8 or later (current: V{}).\n\n\
                    The program '{id}' must be upgraded on-chain before it can be used.",
                    consensus_version as u8
                )));
            }
        }
    }

    Ok(())
}

/// Load additional programs specified by `--with` and add them to the VM.
///
/// Each entry is either a local `.aleo` file path (if it exists on disk)
/// or a remote program name fetched from the network endpoint with
/// transitive dependencies.
#[allow(clippy::too_many_arguments)]
pub fn load_extra_programs_into_vm<N: Network>(
    entries: &[String],
    vm: &snarkvm::prelude::VM<N, ConsensusMemory<N>>,
    home_path: &std::path::Path,
    network: leo_ast::NetworkName,
    endpoint: Option<&str>,
    network_retries: u32,
) -> Result<()> {
    use crate::commands::{LOCAL_PROGRAM_DEFAULT_EDITION, query::load_latest_programs_from_network};
    use snarkvm::prelude::ProgramID;
    use std::path::Path;

    let mut extras: Vec<(Program<N>, Edition)> = Vec::new();

    for entry in entries {
        let path = Path::new(entry);
        if path.is_file() {
            println!("📂 Loading local program from {entry}...");
            let bytecode = std::fs::read_to_string(path)
                .map_err(|e| errors::custom(format!("Failed to read program file '{entry}': {e}")))?;
            let program = Program::<N>::from_str(&bytecode)
                .map_err(|e| errors::custom(format!("Failed to parse program from '{entry}': {e}")))?;
            extras.push((program, LOCAL_PROGRAM_DEFAULT_EDITION));
        } else if path.exists() {
            return Err(errors::custom(format!("'{entry}' exists but is not a file.")).into());
        } else {
            let endpoint = endpoint.ok_or_else(|| {
                errors::custom(format!(
                    "'{entry}' is not a local file; fetching from the network requires --endpoint to be set."
                ))
            })?;
            let name = if entry.ends_with(".aleo") { entry.clone() } else { format!("{entry}.aleo") };
            println!("⬇️  Fetching remote program {name} and its dependencies from {endpoint}...");
            let program_id = ProgramID::<N>::from_str(&name)
                .map_err(|e| errors::custom(format!("Failed to parse program ID '{name}': {e}")))?;
            let fetched = load_latest_programs_from_network(home_path, program_id, network, endpoint, network_retries)?;
            extras.extend(fetched.into_iter().map(|(p, ed)| (p, ed.unwrap_or(LOCAL_PROGRAM_DEFAULT_EDITION))));
        }
    }

    vm.process().lock().add_programs_with_editions(&extras)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// CLI input parsing
// ---------------------------------------------------------------------------

/// Parses a CLI input string into a `Value`, handling record ciphertexts.
pub fn parse_input<N: Network>(input: &str, private_key: &PrivateKey<N>) -> Result<Value<N>> {
    let input = input.trim();
    if input.starts_with("record1") {
        let view_key = ViewKey::<N>::try_from(private_key)
            .map_err(|e| errors::custom(format!("Failed to view key from the private key: {e}")))?;
        Record::<N, Ciphertext<N>>::from_str(input)
            .and_then(|ciphertext| ciphertext.decrypt(&view_key))
            .map(Value::Record)
            .map_err(|e| errors::custom(format!("Failed to parse input as record: {e}")).into())
    } else {
        validate_cli_literal(input)?;
        Value::from_str(input).map_err(|e| errors::custom(format!("Failed to parse input: {e}")).into())
    }
}

fn validate_cli_literal(input: &str) -> Result<()> {
    const ALEO_BECH32_PREFIXES: &[&str] = &["aleo1", "sign1", "APrivateKey1", "AViewKey1"];
    if ALEO_BECH32_PREFIXES.iter().any(|prefix| input.starts_with(prefix)) {
        return Ok(());
    }

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
            if *suffix == "group" && prefix.starts_with('(') {
                return Ok(());
            }
            return validate_numeric_prefix(prefix, suffix, true, true);
        }
    }

    Ok(())
}

fn validate_numeric_prefix(prefix: &str, suffix: &str, allow_negative: bool, decimal_only: bool) -> Result<()> {
    if prefix.is_empty() {
        return Err(errors::custom(format!("Invalid {suffix} literal: missing numeric value before '{suffix}'")).into());
    }
    let valid = if decimal_only {
        is_valid_decimal(prefix, allow_negative)
    } else {
        is_valid_decimal(prefix, allow_negative) || is_valid_radix_prefixed(prefix, allow_negative)
    };
    if !valid {
        return Err(errors::custom(format!("Invalid {suffix} literal: '{prefix}' is not a valid numeric value")).into());
    }
    Ok(())
}

fn is_valid_decimal(s: &str, allow_negative: bool) -> bool {
    let s = if allow_negative { s.strip_prefix('-').unwrap_or(s) } else { s };
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    if !chars.next().unwrap().is_ascii_digit() {
        return false;
    }
    chars.all(|c| c.is_ascii_digit() || c == '_')
}

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
