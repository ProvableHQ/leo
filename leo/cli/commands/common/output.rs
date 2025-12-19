// Copyright (C) 2019-2025 Provable Inc.
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

//! JSON-serializable output types and helpers for CLI commands.

use serde::Serialize;
use snarkvm::prelude::{Network, ProgramID, block::Transaction};

/// Output for `leo deploy` and `leo upgrade`.
#[derive(Serialize, Default)]
pub struct DeployOutput {
    pub deployments: Vec<DeployedProgram>,
}

/// A single deployed program.
#[derive(Serialize)]
pub struct DeployedProgram {
    pub program_id: String,
    pub transaction_id: String,
}

/// Output for `leo run`.
#[derive(Serialize)]
pub struct RunOutput {
    pub program: String,
    pub function: String,
    pub outputs: Vec<String>,
}

/// Output for `leo execute`.
#[derive(Serialize, Default)]
pub struct ExecuteOutput {
    pub program: String,
    pub function: String,
    pub outputs: Vec<String>,
    pub transaction_id: String,
}

/// Output for `leo test`.
#[derive(Serialize, Default)]
pub struct TestOutput {
    pub passed: usize,
    pub failed: usize,
    pub tests: Vec<TestResult>,
}

/// A single test result.
#[derive(Serialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Output for `leo query`. Wraps raw JSON from the network.
#[derive(Serialize)]
pub struct QueryOutput {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Output for `leo synthesize`.
#[derive(Serialize, Default)]
pub struct SynthesizeOutput {
    pub program: String,
    pub functions: Vec<SynthesizedFunction>,
}

/// Prover/verifier key metadata.
#[derive(Serialize, Clone, Default)]
pub struct Metadata {
    pub prover_checksum: String,
    pub prover_size: usize,
    pub verifier_checksum: String,
    pub verifier_size: usize,
}

/// A single synthesized function's key metadata.
#[derive(Serialize)]
pub struct SynthesizedFunction {
    pub name: String,
    #[serde(flatten)]
    pub metadata: Metadata,
}

/// Convert an iterator of displayable items to a `Vec<String>`.
pub fn stringify_outputs<T: std::fmt::Display>(outputs: impl IntoIterator<Item = T>) -> Vec<String> {
    outputs.into_iter().map(|o| o.to_string()).collect()
}

/// Build a `DeployOutput` from a list of transactions.
pub fn build_deploy_output<N: Network>(transactions: &[(ProgramID<N>, Transaction<N>)]) -> DeployOutput {
    DeployOutput {
        deployments: transactions
            .iter()
            .map(|(program_id, transaction)| DeployedProgram {
                program_id: program_id.to_string(),
                transaction_id: transaction.id().to_string(),
            })
            .collect(),
    }
}
