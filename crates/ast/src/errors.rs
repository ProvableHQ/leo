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

use std::fmt::Display;

use leo_errors::Backtraced;

const CODE_PREFIX: &str = "AST";
const CODE_MASK: i32 = 2000;

pub(crate) fn invalid_network_name(network: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 18, format!("invalid network name: `{network}`"))
        .with_help("Valid network names are `testnet`, `mainnet`, and `canary`.")
}
