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

#![forbid(unsafe_code)]
#![cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]

//! WASM bindings for ABI generation from Aleo bytecode.
//!
//! This crate intentionally exposes only the ABI-from-compiled-`.aleo` path.

use leo_ast::NetworkName;

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use super::generate_abi_from_aleo_json;

    use wasm_bindgen::prelude::*;

    /// Generates pretty-printed ABI JSON from compiled Aleo bytecode for the requested network.
    ///
    /// This binding accepts an existing `.aleo` artifact. It does not parse or
    /// compile `.leo` source.
    #[wasm_bindgen]
    pub fn generate_abi_from_aleo(bytecode: &str, network: &str) -> Result<String, JsValue> {
        generate_abi_from_aleo_json(bytecode, network).map_err(|error| JsValue::from_str(&error))
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::generate_abi_from_aleo;

// Keep the implementation target-neutral so native unit tests exercise the
// same parsing, ABI generation, and JSON serialization path as the WASM export.
fn generate_abi_from_aleo_json(bytecode: &str, network: &str) -> Result<String, String> {
    let network = network.parse::<NetworkName>().map_err(|error| error.to_string())?;
    let abi =
        leo_abi::aleo::generate_from_bytecode("program.aleo", bytecode, network).map_err(|error| error.to_string())?;
    serde_json::to_string_pretty(&abi).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::generate_abi_from_aleo_json;

    #[test]
    fn generate_abi_from_aleo_rejects_unknown_network() {
        let error = generate_abi_from_aleo_json("", "unknown").expect_err("expected unknown network to fail");
        assert!(error.contains("Invalid network name"), "unexpected error: {error}");
    }
}
