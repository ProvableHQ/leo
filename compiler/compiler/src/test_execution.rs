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

use crate::run;

use leo_ast::{Bytecode, NodeBuilder};
use leo_errors::{BufferEmitter, Handler, Result};
use leo_span::{Symbol, create_session_if_not_set_then};

use indexmap::IndexMap;
use itertools::Itertools as _;

use std::{fmt::Write as _, rc::Rc};

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
    cases: &[run::Case],
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
) -> Result<String> {
    let mut import_stubs = IndexMap::new();

    let mut ledger_config = run::Config { seed: config.seed, start_height: config.start_height, programs: Vec::new() };

    // We assume config.sources is non-empty.
    let (last, rest) = config.sources.split_last().expect("non-empty sources");

    // Parse-only for intermediate programs.
    for source in rest {
        let (program, program_name) = super::test_utils::parse(source, handler, node_builder, import_stubs.clone())?;

        import_stubs.insert(Symbol::intern(&program_name), program.into());
    }

    // Full compile for the final program.
    let (compiled_programs, program_name) =
        super::test_utils::whole_compile(last, handler, node_builder, import_stubs.clone())?;

    // Add imports.
    let mut requires_ledger = false;
    for Bytecode { program_name, bytecode } in compiled_programs.import_bytecodes {
        requires_ledger |= bytecode.contains("async");
        ledger_config.programs.push(run::Program { bytecode, name: program_name });
    }

    // Add main program.
    let primary_bytecode = compiled_programs.primary_bytecode.clone();
    requires_ledger |= primary_bytecode.contains("async");
    ledger_config.programs.push(run::Program { bytecode: primary_bytecode, name: program_name });

    let mut result = ledger_config
        .programs
        .clone()
        .into_iter()
        .map(|program| program.bytecode)
        .format(&format!("{}\n", super::test_utils::PROGRAM_DELIMITER))
        .to_string();

    if requires_ledger {
        // Note: We wrap cases in a slice to run them all in one ledger instance.
        let outcomes =
            run::run_with_ledger(&ledger_config, &[cases.to_vec()])?.into_iter().flatten().collect::<Vec<_>>();

        assert_eq!(outcomes.len(), cases.len());

        for outcome in outcomes {
            write!(result, "verified: {}\nstatus: {}\n", outcome.verified, outcome.status).unwrap();
            writeln!(result, "{}\n", outcome.execution).unwrap();
        }
    } else {
        let outcomes = run::run_without_ledger(&ledger_config, cases)?;
        assert_eq!(outcomes.len(), cases.len());

        for outcome in outcomes {
            write!(result, "status: {}\noutput: {}\n", outcome.status, outcome.outcome.output).unwrap();
        }
    }

    Ok(result)
}

fn execution_runner(source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());
    let node_builder = Rc::new(NodeBuilder::default());

    let mut config = Config::default();
    let mut cases = Vec::<run::Case>::new();

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

    create_session_if_not_set_then(|_| match execution_run_test(&config, &cases, &handler, &node_builder) {
        Ok(s) => s,
        Err(e) => {
            format!("Error while running execution tests:\n{e}\n\nErrors:\n{}", buf.extract_errs())
        }
    })
}

#[cfg(test)]
mod execution_tests {
    include!(concat!(env!("OUT_DIR"), "/execution_tests.rs"));
}
