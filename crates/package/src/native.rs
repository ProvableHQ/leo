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

//! Native-only helpers: name validation (via snarkVM keyword tables) plus
//! HTTP fetchers for the on-chain program registry. All of these pull in
//! `snarkvm` or `ureq`, so the module is mod-gated to `not(wasm32)` in
//! `lib.rs` and the file itself stays `#[cfg]`-free.

use crate::{Edition, max_program_size};

use leo_ast::NetworkName;
use leo_errors::Backtraced;

/// Checks whether a string is a valid Aleo program name.
///
/// A valid program name must end with `.aleo` and the base name (without the
/// suffix) must satisfy Aleo package naming rules.
pub fn is_valid_program_name(name: &str) -> bool {
    let Some(rest) = name.strip_suffix(".aleo") else {
        tracing::error!("Program names must end with `.aleo`.");
        return false;
    };
    is_valid_package_name(rest)
}

/// Checks whether a string is a valid Aleo library name.
///
/// Library names must satisfy Aleo package naming rules but do not require
/// a `.aleo` suffix.
pub fn is_valid_library_name(name: &str) -> bool {
    is_valid_package_name(name)
}

/// Checks whether a string satisfies general Aleo package naming rules.
///
/// Names must be nonempty, start with a letter, contain only ASCII alphanumeric
/// characters or underscores, avoid reserved keywords, and not contain "aleo".
fn is_valid_package_name(name: &str) -> bool {
    if name.is_empty() {
        tracing::error!("Aleo names must be nonempty");
        return false;
    }
    let first = name.chars().next().unwrap();
    if first == '_' {
        tracing::error!("Aleo names cannot begin with an underscore");
        return false;
    }
    if first.is_numeric() {
        tracing::error!("Aleo names cannot begin with a number");
        return false;
    }
    if name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        tracing::error!("Aleo names can only contain ASCII alphanumeric characters and underscores.");
        return false;
    }
    if reserved_keywords().any(|kw| kw == name) {
        tracing::error!(
            "Aleo names cannot be a SnarkVM reserved keyword. Reserved keywords are: {}.",
            reserved_keywords().collect::<Vec<_>>().join(", ")
        );
        return false;
    }
    if name.contains("aleo") {
        tracing::error!("Aleo names cannot contain the keyword `aleo`.");
        return false;
    }
    true
}

/// Get the list of all reserved and restricted keywords from snarkVM.
/// These keywords cannot be used as program names.
/// See: https://github.com/ProvableHQ/snarkVM/blob/046a2964f75576b2c4afbab9aa9eabc43ceb6dc3/synthesizer/program/src/lib.rs#L192
pub fn reserved_keywords() -> impl Iterator<Item = &'static str> {
    use snarkvm::prelude::{Program, TestnetV0};
    // Flatten RESTRICTED_KEYWORDS by ignoring ConsensusVersion
    let restricted = Program::<TestnetV0>::RESTRICTED_KEYWORDS.iter().flat_map(|(_, kws)| kws.iter().copied());
    Program::<TestnetV0>::KEYWORDS.iter().copied().chain(restricted)
}

/// Creates a configured ureq agent for Leo network requests.
///
/// Disables `http_status_as_error` so 4xx/5xx responses return `Ok(Response)`
/// instead of `Err(StatusCode)`. This preserves response bodies which often
/// contain useful error details from the server.
pub fn create_http_agent() -> ureq::Agent {
    ureq::Agent::config_builder().max_redirects(0).http_status_as_error(false).build().new_agent()
}

/// Retries a fallible network operation with exponential backoff.
///
/// Attempts the operation `retries + 1` times. Delays between attempts are
/// 1 s, 2 s, 4 s, …, capped at 64 s. Returns the result of the last attempt.
///
/// Only use this for idempotent, read-only network calls (GET requests);
/// never use it for state-mutating calls such as transaction broadcasts.
pub fn retry_network_call<T, E: std::fmt::Display>(
    network_retries: u32,
    mut f: impl FnMut() -> std::result::Result<T, E>,
) -> std::result::Result<T, E> {
    let mut result = f();
    for attempt in 1..=network_retries {
        if result.is_ok() {
            break;
        }
        let delay_secs = 2u64.pow(attempt - 1).min(64);
        eprintln!("⚠️  Network request failed, retrying in {delay_secs}s (attempt {attempt}/{network_retries})...");
        std::thread::sleep(std::time::Duration::from_secs(delay_secs));
        result = f();
    }
    result
}

/// Fetch the given endpoint url and return the sanitized response.
pub fn fetch_from_network(url: &str, network_retries: u32) -> Result<String, Backtraced> {
    fetch_from_network_plain(url, network_retries).map(|s| s.replace("\\n", "\n").replace('\"', ""))
}

pub fn fetch_from_network_plain(url: &str, network_retries: u32) -> Result<String, Backtraced> {
    // Retry only on transport-level failures (connection errors, timeouts, etc.).
    // HTTP 3xx/4xx/5xx responses are not retried since they reflect persistent conditions.
    let agent = create_http_agent();
    let mut response = retry_network_call(network_retries, || {
        agent
            .get(url)
            .header("X-Leo-Version", env!("CARGO_PKG_VERSION"))
            .call()
            .map_err(|e| crate::errors::failed_to_retrieve_from_endpoint(url, e))
    })?;
    match response.status().as_u16() {
        200..=299 => Ok(response.body_mut().read_to_string().unwrap()),
        301 => Err(crate::errors::endpoint_moved_error(url)),
        _ => Err(crate::errors::network_error(url, response.status())),
    }
}

/// Fetch the given program from the network and return the program as a string.
// TODO (@d0cd) Unify with `leo_package::CompilationUnit::fetch`.
pub fn fetch_program_from_network(
    name: &str,
    endpoint: &str,
    network: NetworkName,
    network_retries: u32,
) -> Result<String, Backtraced> {
    let url = format!("{endpoint}/{network}/program/{name}");
    let program = fetch_from_network(&url, network_retries)?;
    Ok(program)
}

/// Fetch the latest edition of a program from the network.
///
/// Returns the actual latest edition number for the given program.
/// This should be used instead of defaulting to arbitrary edition numbers.
pub fn fetch_latest_edition(
    name: &str,
    endpoint: &str,
    network: NetworkName,
    network_retries: u32,
) -> Result<Edition, Backtraced> {
    let name_without_suffix = name.strip_suffix(".aleo").unwrap_or(name);
    let url = format!("{endpoint}/{network}/program/{name_without_suffix}.aleo/latest_edition");
    let contents = fetch_from_network(&url, network_retries)?;
    contents.parse::<u16>().map_err(|e| {
        crate::errors::failed_to_retrieve_from_endpoint(url, format!("Failed to parse edition as u16: {e}"))
    })
}

/// Verify that a fetched program is valid aleo instructions for `network`.
pub fn verify_valid_program(name: &str, program: &str, network: NetworkName) -> Result<(), Backtraced> {
    use snarkvm::prelude::{Program, TestnetV0};
    use std::str::FromStr as _;

    let max_size = max_program_size(network);
    if program.len() > max_size {
        return Err(crate::errors::program_size_limit_exceeded(name, program.len(), max_size));
    }
    match Program::<TestnetV0>::from_str(program) {
        Ok(_) => Ok(()),
        Err(_) => Err(crate::errors::snarkvm_parsing_error(name)),
    }
}
