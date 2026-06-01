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

//! Native-only HTTP helpers for talking to the Aleo network endpoint.
//!
//! Lives in `leo-cli-core` (rather than `crates/leo-package`) so the
//! package crate stays purely wasm-buildable. Every function is
//! `#[cfg(not(target_arch = "wasm32"))]`-gated.

#![cfg(not(target_arch = "wasm32"))]

use leo_ast::NetworkName;
use leo_errors::Backtraced;
use leo_package::{Edition, endpoint_moved_error, failed_to_retrieve_from_endpoint, network_error};

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

/// Like [`fetch_from_network`] but returns the response body verbatim
/// (no quote / escape stripping).
pub fn fetch_from_network_plain(url: &str, network_retries: u32) -> Result<String, Backtraced> {
    // Retry only on transport-level failures (connection errors, timeouts, etc.).
    // HTTP 3xx/4xx/5xx responses are not retried since they reflect persistent conditions.
    let agent = create_http_agent();
    let mut response = retry_network_call(network_retries, || {
        agent
            .get(url)
            .header("X-Leo-Version", env!("CARGO_PKG_VERSION"))
            .call()
            .map_err(|e| failed_to_retrieve_from_endpoint(url, e))
    })?;
    match response.status().as_u16() {
        200..=299 => Ok(response.body_mut().read_to_string().unwrap()),
        301 => Err(endpoint_moved_error(url)),
        _ => Err(network_error(url, response.status())),
    }
}

/// Fetch the given program from the network and return the program as a string.
pub fn fetch_program_from_network(
    name: &str,
    endpoint: &str,
    network: NetworkName,
    network_retries: u32,
) -> Result<String, Backtraced> {
    let url = format!("{endpoint}/{network}/program/{name}");
    fetch_from_network(&url, network_retries)
}

/// Fetch the latest edition of a program from the network.
pub fn fetch_latest_edition(
    name: &str,
    endpoint: &str,
    network: NetworkName,
    network_retries: u32,
) -> Result<Edition, Backtraced> {
    let name_without_suffix = name.strip_suffix(".aleo").unwrap_or(name);
    let url = format!("{endpoint}/{network}/program/{name_without_suffix}.aleo/latest_edition");
    let contents = fetch_from_network(&url, network_retries)?;
    contents
        .parse::<u16>()
        .map_err(|e| failed_to_retrieve_from_endpoint(url, format!("Failed to parse edition as u16: {e}")))
}
