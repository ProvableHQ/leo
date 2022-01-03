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

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::SerializedCircuit;

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct SummarizedCircuit {
    pub num_public_variables: usize,
    pub num_private_variables: usize,
    pub num_constraints: usize,

    // pub public_variables: String,
    // pub private_variables: String,
    pub at: String,
    pub bt: String,
    pub ct: String,
}

fn hash_field(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let output = hasher.finalize();
    hex::encode(&output[..])
}

impl From<SerializedCircuit> for SummarizedCircuit {
    fn from(other: SerializedCircuit) -> Self {
        Self {
            num_public_variables: other.num_public_variables,
            num_private_variables: other.num_private_variables,
            num_constraints: other.num_constraints,
            // public_variables: hash_field(&serde_json::to_string(&other.public_variables)
            //     .expect("failed to serialize public_variables")),
            // private_variables: hash_field(&serde_json::to_string(&other.private_variables)
            //     .expect("failed to serialize private_variables")),
            at: hash_field(&serde_json::to_string(&other.at).expect("failed to serialize at")),
            bt: hash_field(&serde_json::to_string(&other.bt).expect("failed to serialize bt")),
            ct: hash_field(&serde_json::to_string(&other.ct).expect("failed to serialize ct")),
        }
    }
}
