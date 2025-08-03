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

use leo_ast::interpreter_value::Value;
use leo_errors::{BufferEmitter, ErrBuffer, Handler, LeoError, Result, WarningBuffer};

use aleo_std_storage::StorageMode;
use anyhow::anyhow;
use snarkvm::{
    prelude::{
        Address,
        Execution,
        Ledger,
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

use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng as _};
use serde_json;
use snarkvm::prelude::{ConsensusVersion, Network};
use std::{fmt, str::FromStr as _};

type CurrentNetwork = TestnetV0;

/// Programs and configuration to run.
#[derive(Debug)]
pub struct Config {
    pub seed: u64,
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
pub enum Status {
    None,
    Aborted,
    Accepted,
    Rejected,
    Halted(String),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Halted(s) => write!(f, "halted ({s})"),
            Status::None => "none".fmt(f),
            Status::Aborted => "aborted".fmt(f),
            Status::Accepted => "accepted".fmt(f),
            Status::Rejected => "rejected".fmt(f),
        }
    }
}

/// All details about the result of a case that was run.
#[derive(Debug)]
pub struct CaseOutcome {
    pub status: Status,
    pub verified: bool,
    pub errors: ErrBuffer,
    pub warnings: WarningBuffer,
    pub execution: String,
    pub output: Value,
}

/// Run the functions indicated by `cases` from the programs in `config`.
// Currently this is used both by the test runner in `test_execution.rs`
// as well as the Leo test in `cli/commands/test.rs`.
// `leo-compiler` is not necessarily the perfect place for it, but
// it's the easiest place for now to make it accessible to both of those.
pub fn run_with_ledger(
    config: &Config,
    cases: &[Case],
    handler: &Handler,
    buf: &BufferEmitter,
) -> Result<Vec<CaseOutcome>> {
    if cases.is_empty() {
        return Ok(Vec::new());
    }

    // Initialize an rng.
    let mut rng = ChaCha20Rng::seed_from_u64(config.seed);

    // Initialize a genesis private key.
    let genesis_private_key = PrivateKey::new(&mut rng).unwrap();

    // Initialize a `VM` and construct the genesis block. This should always succeed.
    let genesis_block = VM::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::from(ConsensusStore::open(0).unwrap())
        .unwrap()
        .genesis_beacon(&genesis_private_key, &mut rng)
        .unwrap();

    // Initialize a `Ledger`. This should always succeed.
    let ledger =
        Ledger::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::load(genesis_block, StorageMode::Production)
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
            Ok(())
        };

        // Deploy the program.
        deploy()?;
        // If the program does not have a constructor, deploy it twice to satisfy the edition requirement.
        if !aleo_program.contains_constructor() {
            deploy()?;
        }
    }

    // Fund each private key used in the test cases with 1M ALEO.
    let transactions: Vec<Transaction<CurrentNetwork>> = cases
        .iter()
        .filter_map(|case| case.private_key.as_ref())
        .map(|key| {
            // Parse the private key.
            let private_key = PrivateKey::<CurrentNetwork>::from_str(key).expect("Failed to parse private key.");
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
        let mut status = Status::None;

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

            case_outcomes.push(CaseOutcome {
                status: Status::Halted(s),
                verified: false,
                errors: buf.extract_errs(),
                warnings: buf.extract_warnings(),
                execution: "".to_string(),
                output: Value::make_unit(),
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
            status = match (block.aborted_transaction_ids().is_empty(), block.transactions().num_accepted() == 1) {
                (false, _) => Status::Aborted,
                (true, true) => Status::Accepted,
                (true, false) => Status::Rejected,
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
            Err(e) => {
                handler.emit_err(LeoError::Anyhow(e));
                Value::make_unit()
            }
        };

        // Extract the execution, removing the global state root and proof.
        // This is necessary as they are not deterministic across runs, even with RNG fixed.
        let execution = if let Some(Transaction::Execute(_, _, execution, _)) = execution {
            Some(Execution::from(execution.into_transitions(), Default::default(), None).unwrap())
        } else {
            None
        };

        case_outcomes.push(CaseOutcome {
            status,
            verified,
            errors: buf.extract_errs(),
            warnings: buf.extract_warnings(),
            execution: serde_json::to_string_pretty(&execution).expect("Serialization failure"),
            output,
        });
    }

    Ok(case_outcomes)
}
