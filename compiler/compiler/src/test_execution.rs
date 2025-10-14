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

use crate::run_with_ledger;

use leo_disassembler::disassemble_from_str;
use leo_errors::{BufferEmitter, Handler, Result};
use leo_span::{Symbol, create_session_if_not_set_then};

use snarkvm::prelude::TestnetV0;

use indexmap::IndexMap;
use itertools::Itertools as _;
use serial_test::serial;
use std::fmt::Write as _;

type CurrentNetwork = TestnetV0;

// Execution test configuration.
#[derive(Debug)]
struct Config {
    seed: u64,
    start_height: Option<u32>,
    sources: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { seed: 1234567890, start_height: None, sources: Vec::new() }
    }
}

fn execution_run_test(
    config: &Config,
    cases: &[run_with_ledger::Case],
    handler: &Handler,
    buf: &BufferEmitter,
) -> Result<String> {
    let mut import_stubs = IndexMap::new();

    let mut ledger_config =
        run_with_ledger::Config { seed: config.seed, start_height: config.start_height, programs: Vec::new() };

    // Compile each source file.
    for source in &config.sources {
        let (bytecode, name) = super::test_utils::whole_compile(source, handler, import_stubs.clone())?;

        let stub = disassemble_from_str::<CurrentNetwork>(&name, &bytecode)?;
        import_stubs.insert(Symbol::intern(&name), stub);

        ledger_config.programs.push(run_with_ledger::Program { bytecode, name });
    }

    let outcomes = run_with_ledger::run_with_ledger(&ledger_config, cases, handler, buf)?;

    assert_eq!(outcomes.len(), cases.len());

    // Output bytecode.
    let mut output = ledger_config
        .programs
        .into_iter()
        .map(|program| program.bytecode)
        .format(&format!("{}\n", super::test_utils::PROGRAM_DELIMITER))
        .to_string();

    // Output each case outcome.
    for outcome in outcomes {
        let err_space = if outcome.errors.is_empty() { "" } else { " " };
        let warning_space = if outcome.warnings.is_empty() { "" } else { " " };

        write!(
            output,
            "verified: {verified}\nstatus: {status}\nerrors:{err_space}{errors}\nwarnings:{warning_space}{warnings}\n",
            verified = outcome.verified,
            status = outcome.status,
            errors = outcome.errors,
            warnings = outcome.warnings,
        )
        .unwrap();
        writeln!(output, "{}\n", outcome.execution).unwrap();
    }

    Ok(output)
}

fn execution_runner(source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());

    let mut config = Config::default();
    let mut cases = Vec::<run_with_ledger::Case>::new();

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
        } else if let Some(rest) = line.strip_prefix("seed = ") {
            config.seed = rest.parse::<u64>().unwrap();
        } else if let Some(rest) = line.strip_prefix("start_height = ") {
            config.start_height = Some(rest.parse::<u32>().unwrap())
        }
    }

    // Split the sources and add them to the config.
    config.sources = source.split(super::test_utils::PROGRAM_DELIMITER).map(|s| s.trim().to_string()).collect();

    create_session_if_not_set_then(|_| match execution_run_test(&config, &cases, &handler, &buf) {
        Ok(s) => s,
        Err(e) => {
            format!("Error while running execution tests:\n{e}\n\nErrors:\n{}", buf.extract_errs())
        }
    })
}

#[test]
#[serial]
fn test_execution() {
    leo_test_framework::run_tests("execution", execution_runner);
}
