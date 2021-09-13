// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{DefinitionStatement, Identifier};

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[allow(clippy::ptr_arg)]
pub fn serialize<S: Serializer>(
    global_consts: &IndexMap<Vec<Identifier>, DefinitionStatement>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let joined: IndexMap<String, DefinitionStatement> = global_consts
        .into_iter()
        .map(|(idents, program)| {
            (
                idents.iter().map(|i| i.name.to_string()).collect::<Vec<_>>().join(","),
                program.clone(),
            )
        })
        .collect();

    joined.serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<IndexMap<Vec<Identifier>, DefinitionStatement>, D::Error> {
    Ok(IndexMap::<String, DefinitionStatement>::deserialize(deserializer)?
        .into_iter()
        .map(|(name, program)| {
            (
                name.split(',')
                    .map(|ident_name| Identifier {
                        name: ident_name.into(),
                        span: Default::default(),
                    })
                    .collect::<Vec<Identifier>>(),
                program,
            )
        })
        .collect())
}
