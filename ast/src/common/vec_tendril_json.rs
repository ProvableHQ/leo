// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tendril::StrTendril;

#[allow(clippy::ptr_arg)]
pub fn serialize<S: Serializer>(tendril: &Vec<StrTendril>, serializer: S) -> Result<S::Ok, S::Error> {
    tendril
        .iter()
        .map(|x| x.as_ref())
        .collect::<Vec<_>>()
        .serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<StrTendril>, D::Error> {
    Ok(Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|x| x.into())
        .collect())
}
