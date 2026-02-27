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

//! JSON-serializable output types and helpers for CLI commands.

use serde::Serialize;
use snarkvm::prelude::{Network, ProgramID, block::Transaction};
use std::fmt;

/// Convert microcredits to credits for display.
pub(crate) fn microcredits_to_credits(microcredits: u64) -> f64 {
    microcredits as f64 / 1_000_000.0
}

/// Per-function cost information for a deployed program.
#[derive(Serialize, Clone, Default)]
pub struct FunctionCostStats {
    pub name: String,
    pub finalize_cost: u64,
    /// The cost of proving the execution transition (execution_cost - finalize_cost).
    /// `None` when authorization sampling fails (e.g. functions requiring specific record types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_cost: Option<u64>,
    /// The total execution cost (finalize + proof).
    /// `None` when authorization sampling fails (e.g. functions requiring specific record types).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_cost: Option<u64>,
}

/// Statistics for a deployed program.
#[derive(Serialize, Clone, Default)]
pub struct DeploymentStats {
    pub program_size_bytes: usize,
    pub max_program_size_bytes: usize,
    /// Total circuit variables across all functions.
    /// `None` when `--skip-deploy-certificate` is used (placeholder verifying keys have no circuit metadata).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_variables: Option<u64>,
    /// Total circuit constraints across all functions.
    /// `None` when `--skip-deploy-certificate` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_constraints: Option<u64>,
    /// Maximum allowed circuit variables for a deployment.
    /// `None` when `--skip-deploy-certificate` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_variables: Option<u64>,
    /// Maximum allowed circuit constraints for a deployment.
    /// `None` when `--skip-deploy-certificate` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_constraints: Option<u64>,
    pub storage_cost: u64,
    pub namespace_cost: u64,
    pub synthesis_cost: u64,
    pub constructor_cost: u64,
    pub priority_fee: u64,
    pub total_cost: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function_costs: Vec<FunctionCostStats>,
}

impl fmt::Display for DeploymentStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::*;
        use num_format::{Locale, ToFormattedString};

        writeln!(
            f,
            "  {:22}{:.2} KB / {:.2} KB",
            "Program Size:".cyan(),
            self.program_size_bytes as f64 / 1024.0,
            self.max_program_size_bytes as f64 / 1024.0
        )?;
        if let Some(total_variables) = self.total_variables {
            writeln!(
                f,
                "  {:22}{}",
                "Total Variables:".cyan(),
                total_variables.to_formatted_string(&Locale::en).yellow()
            )?;
        }
        if let Some(total_constraints) = self.total_constraints {
            writeln!(
                f,
                "  {:22}{}",
                "Total Constraints:".cyan(),
                total_constraints.to_formatted_string(&Locale::en).yellow()
            )?;
        }
        if let Some(max_variables) = self.max_variables {
            writeln!(f, "  {:22}{}", "Max Variables:".cyan(), max_variables.to_formatted_string(&Locale::en).green())?;
        }
        if let Some(max_constraints) = self.max_constraints {
            writeln!(
                f,
                "  {:22}{}",
                "Max Constraints:".cyan(),
                max_constraints.to_formatted_string(&Locale::en).green()
            )?;
        }
        if self.total_variables.is_some() || self.total_constraints.is_some() {
            writeln!(f)?;
        }
        writeln!(f, "{}", "💰 Cost Breakdown (credits)".bold())?;
        writeln!(f, "  {:22}{:.6}", "Transaction Storage:".cyan(), microcredits_to_credits(self.storage_cost))?;
        writeln!(f, "  {:22}{:.6}", "Program Synthesis:".cyan(), microcredits_to_credits(self.synthesis_cost))?;
        writeln!(f, "  {:22}{:.6}", "Namespace:".cyan(), microcredits_to_credits(self.namespace_cost))?;
        writeln!(f, "  {:22}{:.6}", "Constructor:".cyan(), microcredits_to_credits(self.constructor_cost))?;
        writeln!(f, "  {:22}{:.6}", "Priority Fee:".cyan(), microcredits_to_credits(self.priority_fee))?;
        writeln!(f, "  {:22}{:.6}", "Total Fee:".cyan(), microcredits_to_credits(self.total_cost))?;

        for fc in &self.function_costs {
            writeln!(f, "{}", format!("  Function '{}'", fc.name).bold())?;
            if let Some(execution_cost) = fc.execution_cost {
                writeln!(f, "    {:24}{:.6}", "Total Execution Cost:".cyan(), microcredits_to_credits(execution_cost))?;
                writeln!(f, "    {:24}{:.6}", "|- Finalize Cost:".cyan(), microcredits_to_credits(fc.finalize_cost))?;
                if let Some(proof_cost) = fc.proof_cost {
                    writeln!(f, "    {:24}{:.6}", "|- Proof Cost:".cyan(), microcredits_to_credits(proof_cost))?;
                }
            } else {
                writeln!(f, "    {:24}{}", "Total Execution Cost:".cyan(), "Undetermined".dimmed())?;
                writeln!(f, "    {:24}{:.6}", "|- Finalize Cost:".cyan(), microcredits_to_credits(fc.finalize_cost))?;
            }
        }
        Ok(())
    }
}

/// Statistics for an execution.
#[derive(Serialize, Clone, Default)]
pub struct ExecutionStats {
    pub storage_cost: u64,
    pub execution_cost: u64,
    pub priority_fee: u64,
    pub total_cost: u64,
}

impl fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::*;
        writeln!(f, "{}", "💰 Cost Breakdown (credits)".bold())?;
        writeln!(f, "  {:22}{:.6}", "Transaction Storage:".cyan(), microcredits_to_credits(self.storage_cost))?;
        writeln!(f, "  {:22}{:.6}", "On-chain Execution:".cyan(), microcredits_to_credits(self.execution_cost))?;
        writeln!(f, "  {:22}{:.6}", "Priority Fee:".cyan(), microcredits_to_credits(self.priority_fee))?;
        writeln!(f, "  {:22}{:.6}", "Total Fee:".cyan(), microcredits_to_credits(self.total_cost))
    }
}

/// Broadcast result when a transaction is sent to the network.
#[derive(Serialize, Clone, Default)]
pub struct BroadcastStats {
    pub fee_id: String,
    pub fee_transaction_id: String,
    pub confirmed: bool,
}

/// Configuration used for the command.
#[derive(Serialize, Clone, Default)]
pub struct Config {
    pub address: String,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consensus_version: Option<u8>,
}

/// Output for `leo deploy` and `leo upgrade`.
#[derive(Serialize, Default)]
pub struct DeployOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    pub deployments: Vec<DeployedProgram>,
}

/// A single deployed program.
#[derive(Serialize)]
pub struct DeployedProgram {
    pub program_id: String,
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<DeploymentStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<BroadcastStats>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    pub program: String,
    pub function: String,
    pub outputs: Vec<String>,
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<ExecutionStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<BroadcastStats>,
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

/// Circuit information from key synthesis.
#[derive(Serialize, Clone, Default)]
pub struct CircuitInfo {
    pub num_public_inputs: u64,
    pub num_variables: u64,
    pub num_constraints: u64,
    pub num_non_zero_a: u64,
    pub num_non_zero_b: u64,
    pub num_non_zero_c: u64,
    pub circuit_id: String,
}

/// A single synthesized function's key metadata.
#[derive(Serialize)]
pub struct SynthesizedFunction {
    pub name: String,
    pub circuit_info: CircuitInfo,
    #[serde(flatten)]
    pub metadata: Metadata,
}

/// Build a `DeployOutput` from transactions, stats, and optional broadcast results.
pub fn build_deploy_output<N: Network>(
    config: Option<Config>,
    transactions: &[(ProgramID<N>, Transaction<N>)],
    stats: &[DeploymentStats],
    broadcasts: &[BroadcastStats],
) -> DeployOutput {
    DeployOutput {
        config,
        deployments: transactions
            .iter()
            .zip(stats.iter().map(Some).chain(std::iter::repeat(None)))
            .zip(broadcasts.iter().map(Some).chain(std::iter::repeat(None)))
            .map(|(((program_id, transaction), stats), broadcast)| DeployedProgram {
                program_id: program_id.to_string(),
                transaction_id: transaction.id().to_string(),
                stats: stats.cloned(),
                broadcast: broadcast.cloned(),
            })
            .collect(),
    }
}
