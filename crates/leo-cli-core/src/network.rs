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
//! Re-exports the implementations from `leo-package` for now. The
//! `crates/package` crate still hosts the bodies behind a
//! `#[cfg(not(target_arch = "wasm32"))]` gate; this module is the
//! single import surface every CLI command should reach for. A follow-up
//! PR migrates the bodies here and lets `crates/package` drop the gates.

#![cfg(not(target_arch = "wasm32"))]

pub use leo_package::{
    create_http_agent,
    fetch_from_network,
    fetch_from_network_plain,
    fetch_latest_edition,
    fetch_program_from_network,
    retry_network_call,
};
