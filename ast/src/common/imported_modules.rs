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

use crate::Program;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use indexmap::IndexMap;

#[allow(clippy::ptr_arg)]
pub fn serialize<S: Serializer>(
    imported_modules: &IndexMap<Vec<String>, Program>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let joined: IndexMap<String, Program> = imported_modules
        .into_iter()
        .map(|(package, program)| (package.join("."), program.clone()))
        .collect();

    joined.serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<IndexMap<Vec<String>, Program>, D::Error> {
    Ok(IndexMap::<String, Program>::deserialize(deserializer)?
        .into_iter()
        .map(|(package, program)| {
            (
                package
                    .split('.')
                    .map(|segment| segment.to_string())
                    .collect::<Vec<String>>(),
                program,
            )
        })
        .collect())
}
