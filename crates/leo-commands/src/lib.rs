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

//! Wasm-buildable `handle_build` core, shared between the Leo CLI
//! ([`crates/leo`]) and the `leo-wasm` bindings ([`crates/leo-wasm`]).
//! Both surfaces become thin wrappers around
//! [`commands::build::handle_build`] — the CLI passes a
//! [`commands::build::DiskSink`] + `DiskFileSource`, the wasm side passes
//! a [`commands::build::MemorySink`] + `InMemoryFileSource`.
//!
//! [`crates/leo`]: https://github.com/ProvableHQ/leo/tree/master/crates/leo
//! [`crates/leo-wasm`]: https://github.com/ProvableHQ/leo/tree/master/crates/leo-wasm

#![forbid(unsafe_code)]

pub mod commands;
pub mod errors;
pub mod options;
