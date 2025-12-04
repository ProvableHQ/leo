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

//! Utilities for running Leo programs in test environments.
//!
//! Currently this is used by:
//! - the test runner in `test_execution.rs`,
//! - the interpreter tests in `interpreter/src/test_interpreter.rs`, and
//! - the `leo test` command in `cli/commands/test.rs`.
//!
//! `leo-compiler` is not necessarily the perfect place for it, but
//! it's the easiest place for now to make it accessible to all of these.
//!
//! Provides functions for:
//! - Running programs without a ledger (`run_without_ledger`). To be used for evaluating non-async code.
//! - Running programs with a full ledger (`run_with_ledger`), including setup of VM, blocks, and execution tracking.
//!   To be used for executing async code.
//!
//! Also defines types for program configuration, test cases, and outcomes.

use leo_ast::{TEST_PRIVATE_KEY, interpreter_value::Value};
use leo_errors::Result;

use aleo_std_storage::StorageMode;
use anyhow::anyhow;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng as _};
use rayon::prelude::*;
use serde_json;
use snarkvm::{
    circuit::AleoTestnetV0,
    prelude::{
        Address,
        ConsensusVersion,
        Execution,
        Identifier,
        Ledger,
        Network,
        PrivateKey,
        ProgramID,
        TestnetV0,
        Transaction,
        VM,
        Value as SvmValue,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
    synthesizer::program::ProgramCore,
};
use std::{
    fmt,
    panic::{AssertUnwindSafe, catch_unwind},
    str::FromStr as _,
};

type CurrentNetwork = TestnetV0;

/// Programs and configuration to run.
#[derive(Debug)]
pub struct Config {
    pub seed: u64,
    // If `None`, start at the height for the latest consensus version.
    pub start_height: Option<u32>,
    pub programs: Vec<Program>,
}

/// A program to deploy to the ledger.
#[derive(Clone, Debug, Default)]
pub struct Program {
    pub bytecode: String,
    pub name: String,
}

/// A particular case to run.
#[derive(Clone, Debug, Default)]
pub struct Case {
    pub program_name: String,
    pub function: String,
    pub private_key: Option<String>,
    pub input: Vec<String>,
}

/// The status of a case that was run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionStatus {
    None,
    Aborted,
    Accepted,
    Rejected,
    Halted(String),
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halted(s) => write!(f, "halted ({s})"),
            Self::None => write!(f, "none"),
            Self::Aborted => write!(f, "aborted"),
            Self::Accepted => write!(f, "accepted"),
            Self::Rejected => write!(f, "rejected"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvaluationStatus {
    Success,
    Failed(String),
}

impl fmt::Display for EvaluationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failed(e) => write!(f, "failed: {e}"),
        }
    }
}

/// Shared fields for all outcome types.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub program_name: String,
    pub function: String,
    pub output: Value,
}

impl Outcome {
    pub fn output(&self) -> Value {
        self.output.clone()
    }
}

/// Outcome of an evaluation-only run (no execution trace, no verification).
#[derive(Debug, Clone)]
pub struct EvaluationOutcome {
    pub outcome: Outcome,
    pub status: EvaluationStatus,
}

impl EvaluationOutcome {
    pub fn output(&self) -> Value {
        self.outcome.output()
    }
}

/// Outcome that includes execution and verification details.
#[derive(Debug, Clone)]
pub struct ExecutionOutcome {
    pub outcome: Outcome,
    pub verified: bool,
    pub execution: String,
    pub status: ExecutionStatus,
}

impl ExecutionOutcome {
    pub fn output(&self) -> Value {
        self.outcome.output()
    }
}

/// Evaluates a set of cases against some programs without using a ledger.
///
/// Each case is run in isolation, producing an `EvaluationOutcome` for its
/// output and success/failure status. Panics and errors in authorization or
/// evaluation are caught and reported as failures.
pub fn run_without_ledger(config: &Config, cases: &[Case]) -> Result<Vec<EvaluationOutcome>> {
    // Nothing to do
    if cases.is_empty() {
        return Ok(Vec::new());
    }

    let programs_and_editions: Vec<(snarkvm::prelude::Program<CurrentNetwork>, u16)> = config
        .programs
        .iter()
        .map(|Program { bytecode, name }| {
            let program = snarkvm::prelude::Program::<CurrentNetwork>::from_str(bytecode)
                .map_err(|e| anyhow!("Failed to parse bytecode of program {name}: {e}"))?;
            // Assume edition 1. We can consider parametrizing this in the future.
            let edition: u16 = 1;
            Ok((program, edition))
        })
        .collect::<Result<Vec<_>>>()?;

    let outcomes: Vec<EvaluationOutcome> = cases
        .par_iter()
        .map(|case| {
            let rng = &mut ChaCha20Rng::seed_from_u64(config.seed);

            // Helper to produce an EvaluationOutcome with `Failed` status
            let failed_outcome = |e: String| EvaluationOutcome {
                outcome: Outcome {
                    program_name: case.program_name.clone(),
                    function: case.function.clone(),
                    output: Value::make_unit(),
                },
                status: EvaluationStatus::Failed(e),
            };

            let vm = match ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(
                StorageMode::Production,
            ) {
                Ok(store) => match VM::from(store) {
                    Ok(vm) => vm,
                    Err(e) => return failed_outcome(format!("VM init error: {e}")),
                },
                Err(e) => return failed_outcome(format!("Consensus store open error: {e}")),
            };

            if let Err(e) = vm.process().write().add_programs_with_editions(&programs_and_editions) {
                return failed_outcome(format!("Failed to add programs: {e}"));
            }

            let private_key = match PrivateKey::from_str(leo_ast::TEST_PRIVATE_KEY) {
                Ok(pk) => pk,
                Err(e) => return failed_outcome(format!("Private key parse error: {e}")),
            };
            let program_id = match ProgramID::<CurrentNetwork>::from_str(&case.program_name) {
                Ok(pid) => pid,
                Err(e) => return failed_outcome(format!("ProgramID parse error: {e}")),
            };
            let function_id = match Identifier::<CurrentNetwork>::from_str(&case.function) {
                Ok(fid) => fid,
                Err(e) => return failed_outcome(format!("FunctionID parse error: {e}")),
            };
            let inputs = case.input.iter();

            // --- catch panics from authorize ---
            let authorization = match catch_unwind(AssertUnwindSafe(|| {
                vm.authorize(&private_key, program_id, function_id, inputs, rng)
            })) {
                Ok(Ok(auth)) => auth,
                Ok(Err(e)) => return failed_outcome(format!("{e}")),
                Err(e) => return failed_outcome(format!("{e:?}")),
            };

            // --- catch panics from evaluate ---
            let response =
                match catch_unwind(AssertUnwindSafe(|| vm.process().read().evaluate::<AleoTestnetV0>(authorization))) {
                    Ok(Ok(resp)) => resp,
                    Ok(Err(e)) => return failed_outcome(format!("{e}")),
                    Err(e) => return failed_outcome(format!("{e:?}")),
                };

            let outputs = response.outputs();
            let output = match outputs.len() {
                0 => Value::make_unit(),
                1 => outputs[0].clone().into(),
                _ => Value::make_tuple(outputs.iter().map(|x| x.clone().into())),
            };

            EvaluationOutcome {
                outcome: Outcome { program_name: case.program_name.clone(), function: case.function.clone(), output },
                status: EvaluationStatus::Success,
            }
        })
        .collect();

    Ok(outcomes)
}

/// Run the functions indicated by `cases` from the programs in `config`.
pub fn run_with_ledger(config: &Config, case_sets: &[Vec<Case>]) -> Result<Vec<Vec<ExecutionOutcome>>> {
    if case_sets.is_empty() {
        return Ok(Vec::new());
    }

    // Initialize an rng.
    let mut rng = ChaCha20Rng::seed_from_u64(config.seed);

    // Initialize a genesis private key.
    let genesis_private_key = PrivateKey::from_str(TEST_PRIVATE_KEY).unwrap();

    // Store all of the non-genesis blocks created during set up.
    let mut blocks = Vec::new();

    // Initialize a `VM` and construct the genesis block. This should always succeed.
    let genesis_block = VM::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::from(ConsensusStore::open(0).unwrap())
        .unwrap()
        .genesis_beacon(&genesis_private_key, &mut rng)
        .unwrap();

    // Initialize a `Ledger`. This should always succeed.
    let ledger =
        Ledger::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::load(genesis_block.clone(), StorageMode::Production)
            .unwrap();

    // Advance the `VM` to the start height, defaulting to the height for the latest consensus version.
    let latest_consensus_version = ConsensusVersion::latest();
    let start_height =
        config.start_height.unwrap_or(CurrentNetwork::CONSENSUS_HEIGHT(latest_consensus_version).unwrap());
    while ledger.latest_height() < start_height {
        let block = ledger
            .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], vec![], &mut rng)
            .map_err(|_| anyhow!("Failed to prepare advance to next beacon block"))?;
        ledger.advance_to_next_block(&block).map_err(|_| anyhow!("Failed to advance to next block"))?;
        blocks.push(block);
    }

    // Deploy each bytecode separately.
    for Program { bytecode, name } in &config.programs {
        // Parse the bytecode as an Aleo program.
        // Note that this function checks that the bytecode is well-formed.
        let aleo_program =
            ProgramCore::from_str(bytecode).map_err(|e| anyhow!("Failed to parse bytecode of program {name}: {e}"))?;

        let mut deploy = || -> Result<()> {
            // Add the program to the ledger.
            // Note that this function performs an additional validity check on the bytecode.
            let deployment = ledger
                .vm()
                .deploy(&genesis_private_key, &aleo_program, None, 0, None, &mut rng)
                .map_err(|e| anyhow!("Failed to deploy program {name}: {e}"))?;
            let block = ledger
                .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], vec![deployment], &mut rng)
                .map_err(|e| anyhow!("Failed to prepare to advance block for program {name}: {e}"))?;
            ledger
                .advance_to_next_block(&block)
                .map_err(|e| anyhow!("Failed to advance block for program {name}: {e}"))?;

            // Check that the deployment transaction was accepted.
            if block.transactions().num_accepted() != 1 {
                return Err(anyhow!("Deployment transaction for program {name} not accepted.").into());
            }

            // Store the block.
            blocks.push(block);

            Ok(())
        };

        // Deploy the program.
        deploy()?;
        // If the program does not have a constructor, deploy it twice to satisfy the edition requirement.
        if !aleo_program.contains_constructor() {
            deploy()?;
        }
    }

    // Initialize ledger instances for each case set.
    let mut indexed_ledgers = vec![(0, ledger)];
    indexed_ledgers.extend(
        (1..case_sets.len())
            .into_par_iter()
            .map(|i| {
                // Initialize a `Ledger`. This should always succeed.
                let l = Ledger::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::load(
                    genesis_block.clone(),
                    StorageMode::Production,
                )
                .expect("Failed to load copy of ledger");
                // Add the setup blocks.
                for block in blocks.iter() {
                    l.advance_to_next_block(block).expect("Failed to add setup block to ledger");
                }

                (i, l)
            })
            .collect::<Vec<_>>(),
    );

    // For each of the case sets, run the cases sequentially.
    let results = indexed_ledgers
        .into_par_iter()
        .map(|(index, ledger)| {
            // Get the cases for this ledger.
            let cases = &case_sets[index];
            // Clone the RNG.
            let mut rng = rng.clone();

            // Fund each private key used in the test cases with 1M ALEO.
            let transactions: Vec<Transaction<CurrentNetwork>> = cases
                .iter()
                .filter_map(|case| case.private_key.as_ref())
                .map(|key| {
                    // Parse the private key.
                    let private_key =
                        PrivateKey::<CurrentNetwork>::from_str(key).expect("Failed to parse private key.");
                    // Convert the private key to an address.
                    let address = Address::try_from(private_key).expect("Failed to convert private key to address.");
                    // Generate the transaction.
                    ledger
                        .vm()
                        .execute(
                            &genesis_private_key,
                            ("credits.aleo", "transfer_public"),
                            [
                                SvmValue::from_str(&format!("{address}")).expect("Failed to parse recipient address"),
                                SvmValue::from_str("1_000_000_000_000u64").expect("Failed to parse amount"),
                            ]
                            .iter(),
                            None,
                            0u64,
                            None,
                            &mut rng,
                        )
                        .expect("Failed to generate funding transaction")
                })
                .collect();

            // Create a block with the funding transactions.
            let block = ledger
                .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], transactions, &mut rng)
                .expect("Failed to prepare advance to next beacon block");
            // Assert that no transactions were aborted or rejected.
            assert!(block.aborted_transaction_ids().is_empty());
            assert_eq!(block.transactions().num_rejected(), 0);
            // Advance the ledger to the next block.
            ledger.advance_to_next_block(&block).expect("Failed to advance to next block");

            let mut case_outcomes = Vec::new();

            for case in cases {
                assert!(
                    ledger.vm().contains_program(&ProgramID::from_str(&case.program_name).unwrap()),
                    "Program {} should exist.",
                    case.program_name
                );

                let private_key = case
                    .private_key
                    .as_ref()
                    .map(|key| PrivateKey::from_str(key).expect("Failed to parse private key."))
                    .unwrap_or(genesis_private_key);

                let mut execution = None;
                let mut verified = false;
                let mut status = ExecutionStatus::None;

                // Halts are handled by panics, so we need to catch them.
                // I'm not thrilled about this usage of `AssertUnwindSafe`, but it seems to be
                // used frequently in SnarkVM anyway.
                let execute_output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    ledger.vm().execute_with_response(
                        &private_key,
                        (&case.program_name, &case.function),
                        case.input.iter(),
                        None,
                        0,
                        None,
                        &mut rng,
                    )
                }));

                if let Err(payload) = execute_output {
                    let s1 = payload.downcast_ref::<&str>().map(|s| s.to_string());
                    let s2 = payload.downcast_ref::<String>().cloned();
                    let s = s1.or(s2).unwrap_or_else(|| "Unknown panic payload".to_string());

                    case_outcomes.push(ExecutionOutcome {
                        outcome: Outcome {
                            program_name: case.program_name.clone(),
                            function: case.function.clone(),
                            output: Value::make_unit(),
                        },
                        status: ExecutionStatus::Halted(s),
                        verified: false,
                        execution: "".to_string(),
                    });

                    continue;
                }

                let result = execute_output.unwrap().and_then(|(transaction, response)| {
                    verified = ledger.vm().check_transaction(&transaction, None, &mut rng).is_ok();
                    execution = Some(transaction.clone());
                    let block = ledger.prepare_advance_to_next_beacon_block(
                        &private_key,
                        vec![],
                        vec![],
                        vec![transaction],
                        &mut rng,
                    )?;
                    status =
                        match (block.aborted_transaction_ids().is_empty(), block.transactions().num_accepted() == 1) {
                            (false, _) => ExecutionStatus::Aborted,
                            (true, true) => ExecutionStatus::Accepted,
                            (true, false) => ExecutionStatus::Rejected,
                        };
                    ledger.advance_to_next_block(&block)?;
                    Ok(response)
                });

                let output = match result {
                    Ok(response) => {
                        let outputs = response.outputs();
                        match outputs.len() {
                            0 => Value::make_unit(),
                            1 => outputs[0].clone().into(),
                            _ => Value::make_tuple(outputs.iter().map(|x| x.clone().into())),
                        }
                    }
                    Err(e) => Value::make_string(format!("Failed to extract output: {e}")),
                };

                // Extract the execution, removing the global state root and proof.
                // This is necessary as they are not deterministic across runs, even with RNG fixed.
                let execution = if let Some(Transaction::Execute(_, _, execution, _)) = execution {
                    Some(Execution::from(execution.into_transitions(), Default::default(), None).unwrap())
                } else {
                    None
                };

                case_outcomes.push(ExecutionOutcome {
                    outcome: Outcome {
                        program_name: case.program_name.clone(),
                        function: case.function.clone(),
                        output,
                    },
                    status,
                    verified,
                    execution: serde_json::to_string_pretty(&execution).expect("Serialization failure"),
                });
            }

            Ok((index, case_outcomes))
        })
        .collect::<Result<Vec<_>>>()?;

    // Reorder results to match input order.
    let mut ordered_results: Vec<Vec<ExecutionOutcome>> = vec![Default::default(); case_sets.len()];
    for (index, outcomes) in results.into_iter() {
        ordered_results[index] = outcomes;
    }

    Ok(ordered_results)
}
