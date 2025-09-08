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

//! These tests compare interpreter runs against ledger runs.

use leo_ast::{NetworkName, Stub, interpreter_value::Value};
use leo_compiler::{Compiler, run_with_ledger};
use leo_disassembler::disassemble_from_str;
use leo_errors::{BufferEmitter, Handler, Result};
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName};

use snarkvm::prelude::{Address, PrivateKey, TestnetV0};

use indexmap::IndexMap;
use itertools::Itertools as _;
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};
use walkdir::WalkDir;

use crate::interpreter::{Interpreter, InterpreterAction};

pub static TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

const PROGRAM_DELIMITER: &str = "// --- Next Program --- //";

type CurrentNetwork = TestnetV0;

fn whole_compile(source: &str, handler: &Handler, import_stubs: IndexMap<Symbol, Stub>) -> Result<(String, String)> {
    let mut compiler = Compiler::new(
        None,
        /* is_test (a Leo test) */ false,
        handler.clone(),
        "/fakedirectory-wont-use".into(),
        None,
        import_stubs,
        NetworkName::TestnetV0,
    );

    let filename = FileName::Custom("execution-test".into());

    let bytecode = compiler.compile(source, filename, &Vec::new())?;

    Ok((bytecode, compiler.program_name.unwrap()))
}

fn parse_cases(source: &str) -> (Vec<run_with_ledger::Case>, Vec<String>) {
    let mut cases: Vec<run_with_ledger::Case> = Vec::new();

    // Captures quote-delimited strings.
    let re_input = regex::Regex::new(r#""([^"]+)""#).unwrap();

    for line in source.lines() {
        if line.starts_with("[case]") {
            cases.push(Default::default());
        } else if let Some(rest) = line.strip_prefix("program = ") {
            cases.last_mut().unwrap().program_name = rest.trim_matches('"').into();
        } else if let Some(rest) = line.strip_prefix("function = ") {
            cases.last_mut().unwrap().function = rest.trim_matches('"').into();
        } else if let Some(rest) = line.strip_prefix("private_key = ") {
            cases.last_mut().unwrap().private_key = Some(rest.trim_matches('"').into());
        } else if let Some(rest) = line.strip_prefix("input = ") {
            // Get quote-delimited strings.
            cases.last_mut().unwrap().input = re_input.captures_iter(rest).map(|s| s[1].to_string()).collect();
        }
    }

    let sources = source.split(PROGRAM_DELIMITER).map(|s| s.trim().to_string()).collect();
    (cases, sources)
}

#[derive(Debug)]
pub struct TestResult {
    ledger_result: Vec<Value>,
    interpreter_result: Vec<Value>,
}

fn run_test(path: &Path, handler: &Handler, _buf: &BufferEmitter) -> Result<TestResult, ()> {
    let source = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read file {}: {e}.", path.display()));
    let (cases, sources) = parse_cases(&source);
    let mut import_stubs = IndexMap::new();
    let mut ledger_config = run_with_ledger::Config { seed: 2, start_height: None, programs: Vec::new() };
    for source in &sources {
        let (bytecode, name) = handler.extend_if_error(whole_compile(source, handler, import_stubs.clone()))?;

        let stub = handler
            .extend_if_error(disassemble_from_str::<CurrentNetwork>(&name, &bytecode).map_err(|err| err.into()))?;
        import_stubs.insert(Symbol::intern(&name), stub);

        ledger_config.programs.push(run_with_ledger::Program { bytecode, name });
    }

    let (ledger_handler, ledger_buf) = Handler::new_with_buf();

    let outcomes = handler.extend_if_error(run_with_ledger::run_with_ledger(
        &ledger_config,
        &cases,
        &ledger_handler,
        &ledger_buf,
    ))?;

    let private_key =
        PrivateKey::<CurrentNetwork>::from_str(TEST_PRIVATE_KEY).expect("Should be able to parse private key.");
    let address = Address::<CurrentNetwork>::try_from(&private_key).expect("Should be able to create address.");

    let tempdir = tempfile::tempdir().expect("tempdir");
    assert_eq!(sources.len(), 1, "For now we only support one program.");
    let paths: Vec<PathBuf> = sources
        .iter()
        .map(|source| {
            let path = tempdir.path().join("main.leo");
            fs::write(&path, source).expect("write failed");
            path
        })
        .collect();
    let empty: [&PathBuf; 0] = [];
    let mut interpreter =
        Interpreter::new(&[(paths[0].clone(), Vec::new())], empty, address.into(), 0, NetworkName::TestnetV0)
            .expect("creating interpreter");
    let interpreter_result = handler.extend_if_error(
        cases
            .iter()
            .map(|case| {
                interpreter
                    .action(InterpreterAction::LeoInterpretOver(format!(
                        "{}/{}({})",
                        case.program_name,
                        case.function,
                        case.input.iter().format(", ")
                    )))
                    .map(|opt_value| opt_value.unwrap_or(Value::make_unit()))
            })
            .collect::<Result<Vec<_>>>(),
    )?;
    Ok(TestResult { ledger_result: outcomes.into_iter().map(|outcome| outcome.output).collect(), interpreter_result })
}

#[test]
fn test_interpreter() {
    let tests_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "tests", "tests", "interpreter"]
        .iter()
        .collect::<PathBuf>()
        .canonicalize()
        .unwrap();

    let filter_string = std::env::var("TEST_FILTER").unwrap_or_default();

    let paths: Vec<PathBuf> = WalkDir::new(&tests_dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();

            let path_str = path.to_str().unwrap_or_else(|| panic!("Path not unicode: {}.", path.display()));

            if !path_str.contains(&filter_string) || !path_str.ends_with(".leo") {
                return None;
            }

            Some(path.into())
        })
        .collect();

    create_session_if_not_set_then(|_| {
        for path in paths.iter() {
            let mut test_result = {
                let buf = BufferEmitter::new();
                let handler = Handler::new(buf.clone());
                match run_test(path, &handler, &buf) {
                    Ok(result) => result,
                    Err(..) => {
                        let errs = buf.extract_errs();
                        panic!("{} {} ", errs.len(), errs);
                    }
                }
            };

            // Clear the `id`, for comparison against what snarkvm produced.
            test_result.interpreter_result.iter_mut().for_each(|value| value.id = None);
            if test_result.ledger_result != test_result.interpreter_result {
                println!("TEST {} Failed", path.display());
                println!("LEDGER: {:?}", test_result.ledger_result);
                println!("INTERPRETER: {:?}", test_result.interpreter_result);
                panic!("Test failure");
            }
        }
    })
}
