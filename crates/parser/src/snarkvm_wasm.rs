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

//! Minimal `snarkvm::...` facade for WASM targets.
//!
//! Replaces the full `snarkvm` crate (which pulls in native-only deps like
//! `ring` via `ureq`) with the console and program-parsing surface only.

pub use snarkvm_console as console;

pub mod prelude {
    pub use snarkvm_console::{network::prelude::*, program::*, types::prelude::*};
    pub use snarkvm_synthesizer_program::{Mapping, Program};
}

pub mod synthesizer {
    pub mod program {
        pub use snarkvm_synthesizer_program::*;
    }
}
