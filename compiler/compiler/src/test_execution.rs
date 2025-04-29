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

use crate::Compiler;

use leo_ast::Stub;
use leo_disassembler::disassemble_from_str;
use leo_errors::{BufferEmitter, Handler, LeoError};
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName};

use aleo_std_storage::StorageMode;
use snarkvm::{
    prelude::{
        Address,
        Execution,
        Itertools,
        Ledger,
        PrivateKey,
        Process,
        ProgramID,
        TestnetV0,
        Transaction,
        VM,
        Value,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
    synthesizer::program::ProgramCore,
};

use indexmap::IndexMap;
use rand_chacha::{
    ChaCha20Rng,
    rand_core::{OsRng, RngCore, SeedableRng as _},
};
use serial_test::serial;
use std::{fmt::Write as _, str::FromStr};

pub type CurrentNetwork = TestnetV0;

pub const PROGRAM_DELIMITER: &str = "// --- Next Program --- //";

pub fn whole_compile(
    source: &str,
    handler: &Handler,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<(String, String), LeoError> {
    let mut compiler =
        Compiler::<CurrentNetwork>::new(handler.clone(), "/fakedirectory-wont-use".into(), None, import_stubs);

    let filename = FileName::Custom("execution-test".into());

    let bytecode = compiler.compile(source, filename)?;

    Ok((bytecode, compiler.program_name))
}

// Execution test configuration.
#[derive(Debug, Default)]
struct Config {
    pub seed: Option<u64>,
    pub min_height: Option<u32>,
    pub sources: Vec<String>,
}

// Execution test cases.
#[derive(Debug, Default)]
struct Case {
    program: String,
    function: String,
    private_key: Option<String>,
    input: Vec<String>,
}

fn execution_run_test(config: Config, handler: &Handler, buf: &BufferEmitter, cases: &[Case]) -> Result<String, ()> {
    // Initialize a `Process`. This should always succeed.
    let process = Process::<CurrentNetwork>::load().unwrap();

    // Initialize an rng.
    let mut rng = &mut ChaCha20Rng::seed_from_u64(config.seed.unwrap_or_else(|| {
        let mut seed = [0u8; 8];
        OsRng.try_fill_bytes(&mut seed).unwrap();
        u64::from_le_bytes(seed)
    }));

    let mut import_stubs = IndexMap::new();

    // Clone the process.
    let mut process = process.clone();

    // Initialize a `VM`. This should always succeed.
    let vm = VM::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::from(ConsensusStore::open(None).unwrap()).unwrap();

    // Initialize a genesis private key.
    let genesis_private_key = PrivateKey::new(&mut rng).unwrap();

    // Construct the genesis block.
    let genesis_block = vm.genesis_beacon(&genesis_private_key, &mut rng).unwrap();

    // Initialize a `Ledger`. This should always succeed.
    let ledger =
        Ledger::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::load(genesis_block, StorageMode::Production)
            .unwrap();

    // Compile and deploy each source file separately.
    for source in &config.sources {
        let (bytecode, program_name) = handler.extend_if_error(whole_compile(source, handler, import_stubs.clone()))?;

        // Parse the bytecode as an Aleo program.
        // Note that this function checks that the bytecode is well-formed.
        let aleo_program = handler.extend_if_error(ProgramCore::from_str(&bytecode).map_err(LeoError::Anyhow))?;

        // Add the program to the process.
        // Note that this function performs an additional validity check on the bytecode.
        handler.extend_if_error(process.add_program(&aleo_program).map_err(LeoError::Anyhow))?;

        // Add the bytecode to the import stubs.
        let stub = handler.extend_if_error(
            disassemble_from_str::<CurrentNetwork>(&program_name, &bytecode).map_err(|err| err.into()),
        )?;
        import_stubs.insert(Symbol::intern(&program_name), stub);

        if handler.err_count() != 0 || handler.warning_count() != 0 {
            return Err(());
        }

        // Parse the bytecode as an Aleo program.
        // Note that this function checks that the bytecode is well-formed.
        let aleo_program = handler.extend_if_error(ProgramCore::from_str(&bytecode).map_err(LeoError::Anyhow))?;

        // Add the program to the ledger.
        // Note that this function performs an additional validity check on the bytecode.
        let deployment = handler.extend_if_error(
            ledger.vm().deploy(&genesis_private_key, &aleo_program, None, 0, None, &mut rng).map_err(LeoError::Anyhow),
        )?;
        let block = handler.extend_if_error(
            ledger
                .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], vec![deployment], &mut rng)
                .map_err(LeoError::Anyhow),
        )?;
        handler.extend_if_error(ledger.advance_to_next_block(&block).map_err(LeoError::Anyhow))?;

        if handler.err_count() != 0 || handler.warning_count() != 0 {
            return Err(());
        }

        // Check that the deployment transaction was accepted.
        if block.transactions().num_accepted() != 1 {
            return Ok("Deployment transaction not accepted.".into());
        }
    }

    // Fund each private key used in the test cases with 1M ALEO.
    let transactions = cases
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
                        Value::from_str(&format!("{address}")).expect("Failed to parse recipient address"),
                        Value::from_str("1_000_000_000_000u64").expect("Failed to parse amount"),
                    ]
                    .iter(),
                    None,
                    0u64,
                    None,
                    rng,
                )
                .expect("Failed to generate funding transaction")
        })
        .collect_vec();

    // Create a block with the funding transactions.
    let block = ledger
        .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], transactions, &mut rng)
        .expect("Failed to prepare advance to next beacon block");
    // Assert that no transactions were aborted or rejected.
    assert!(block.aborted_transaction_ids().is_empty());
    assert_eq!(block.transactions().num_rejected(), 0);
    // Advance the ledger to the next block.
    ledger.advance_to_next_block(&block).expect("Failed to advance to next block");

    // Advance the ledger with empty blocks until the specified height.
    let target_height = config.min_height.unwrap_or(1);
    let current_height = ledger.vm().block_store().current_block_height();
    let num_blocks = target_height.saturating_sub(current_height).saturating_sub(1);
    for _ in 0..num_blocks {
        let block = ledger
            .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], vec![], &mut rng)
            .expect("Failed to prepare advance to next beacon block");
        ledger.advance_to_next_block(&block).expect("Failed to advance to next block");
    }

    println!("Current height: {}", ledger.vm().block_store().current_block_height());

    let mut output = String::new();
    for case in cases {
        if !ledger.vm().contains_program(&ProgramID::from_str(&case.program).unwrap()) {
            return Ok(format!("Program {} doesn't exist.", case.program));
        }

        let private_key = case
            .private_key
            .as_ref()
            .map(|key| PrivateKey::from_str(key).expect("Failed to parse private key."))
            .unwrap_or(genesis_private_key);

        let mut execution = None;
        let mut verified = false;
        let mut status = "none";

        let result = ledger
            .vm()
            .execute(&private_key, (&case.program, &case.function), case.input.iter(), None, 0, None, &mut rng)
            .and_then(|transaction| {
                verified = ledger.vm().check_transaction(&transaction, None, &mut rng).is_ok();
                execution = Some(transaction.clone());
                ledger.prepare_advance_to_next_beacon_block(&private_key, vec![], vec![], vec![transaction], &mut rng)
            })
            .and_then(|block| {
                status = match (block.aborted_transaction_ids().is_empty(), block.transactions().num_accepted() == 1) {
                    (false, _) => "aborted",
                    (true, true) => "accepted",
                    (true, false) => "rejected",
                };
                ledger.advance_to_next_block(&block)
            });

        if let Err(e) = result {
            handler.emit_err(LeoError::Anyhow(e));
        }

        // Extract the execution and remove the global state root.
        let execution = if let Some(Transaction::Execute(_, _, execution, _)) = execution {
            let proof = execution.proof().cloned();
            let transitions = execution.into_transitions();
            Some(Execution::from(transitions, Default::default(), proof).unwrap())
        } else {
            None
        };

        // These values are just to avoid spaces before the newline in `errors:` and `warnings`
        // in the output.
        let err_space = if handler.err_count() == 0 { "" } else { " " };
        let warning_space = if handler.warning_count() == 0 { "" } else { " " };

        write!(
            output,
            "verified: {verified}\nstatus: {status}\nerrors:{err_space}{}\nwarnings:{warning_space}{}\n",
            buf.extract_errs(),
            buf.extract_warnings()
        )
        .unwrap();
        writeln!(output, "{}\n", serde_json::to_string_pretty(&execution).expect("Serialization failure")).unwrap();
    }

    Ok(output)
}

fn execution_runner(source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());

    let mut config = Config::default();
    let mut cases = Vec::<Case>::new();

    // Captures quote-delimited strings.
    let re_input = regex::Regex::new(r#""([^"]+)""#).unwrap();

    for line in source.lines() {
        if line.starts_with("[case]") {
            cases.push(Default::default());
        } else if let Some(rest) = line.strip_prefix("program = ") {
            cases.last_mut().unwrap().program = rest.trim_matches('"').into();
        } else if let Some(rest) = line.strip_prefix("function = ") {
            cases.last_mut().unwrap().function = rest.trim_matches('"').into();
        } else if let Some(rest) = line.strip_prefix("private_key = ") {
            cases.last_mut().unwrap().private_key = Some(rest.trim_matches('"').into());
        } else if let Some(rest) = line.strip_prefix("input = ") {
            // Get quote-delimited strings.
            cases.last_mut().unwrap().input = re_input.captures_iter(rest).map(|s| s[1].to_string()).collect();
        } else if let Some(rest) = line.strip_prefix("seed = ") {
            config.seed = Some(rest.parse::<u64>().unwrap());
        } else if let Some(rest) = line.strip_prefix("min_height = ") {
            config.min_height = Some(rest.parse::<u32>().unwrap());
        }
    }

    // Parse the sources and add them to the config.
    let sources = source.split(PROGRAM_DELIMITER).map(|s| s.trim().to_string()).collect::<Vec<_>>();
    config.sources = sources;

    create_session_if_not_set_then(|_| match execution_run_test(config, &handler, &buf, &cases) {
        Ok(s) => s,
        Err(()) => format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
    })
}

#[test]
#[serial]
fn test_execution() {
    leo_test_framework::run_tests("execution", execution_runner);
}
