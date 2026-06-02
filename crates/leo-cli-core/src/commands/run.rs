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

//! Native-only `leo run` core. The `LeoRun` clap struct in `crates/leo`
//! resolves env defaults and calls into [`handle_run`].

#![cfg(not(target_arch = "wasm32"))]

use crate::{
    commands::{
        LOCAL_PROGRAM_DEFAULT_EDITION,
        query::load_latest_programs_from_network,
        util::{load_extra_programs_into_vm, parse_input, print_program_source},
    },
    errors,
};

use leo_ast::{NetworkName, TEST_PRIVATE_KEY};
use leo_errors::Result;
use leo_package::{Package, ProgramData};

use aleo_std_storage::StorageMode;

use snarkvm::{
    circuit::Aleo,
    prelude::{
        Identifier,
        PrivateKey,
        ProgramID,
        VM,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use serde::Serialize;
use std::{path::Path, str::FromStr};

/// Output shape for the `leo run` command.
#[derive(Serialize)]
pub struct RunOutput {
    pub program: String,
    pub function: String,
    pub outputs: Vec<String>,
}

/// Resolved inputs for [`handle_run`]. The CLI's `LeoRun` clap struct
/// extracts these from its parsed flags + context before calling.
pub struct RunArgs<'a> {
    /// `program/function` or `program::function` or just `function`.
    pub name: String,
    /// String-typed Leo inputs (e.g. `"1u32"`).
    pub inputs: Vec<String>,
    /// Optional `--with` entries (extra programs to load).
    pub with: &'a [String],
    /// Optional `--private-key`; defaults to `TEST_PRIVATE_KEY`.
    pub private_key: &'a Option<String>,
    /// Optional `--endpoint` (used to fetch the program if it isn't local).
    pub endpoint: &'a Option<String>,
    /// `--network-retries` value.
    pub network_retries: u32,
    /// Active network.
    pub network: NetworkName,
    /// Real-disk root for the on-disk program cache (typically `~/.aleo`).
    pub home_path: &'a Path,
    /// Optional already-loaded package (the CLI builds it via `LeoBuild`
    /// before reaching here).
    pub package: Option<&'a Package>,
}

/// Drive a `leo run`. Returns the `(program, function, outputs)` shape the
/// CLI surfaces as `RunOutput`.
pub fn handle_run<A: Aleo>(args: RunArgs<'_>) -> Result<RunOutput> {
    if let Some(package) = args.package
        && package.compilation_units.last().is_some_and(|p| p.kind.is_library())
    {
        return Err(errors::custom("Cannot run a library package. Only programs can be run.").into());
    }

    let private_key = match crate::options::get_private_key::<A::Network>(args.private_key) {
        Ok(private_key) => private_key,
        Err(_) => {
            println!("⚠️ No valid private key specified, defaulting to '{TEST_PRIVATE_KEY}'.");
            PrivateKey::<A::Network>::from_str(TEST_PRIVATE_KEY).expect("Failed to parse the test private key")
        }
    };

    let (program_name, function_name) = match args.name.split_once('/').or_else(|| args.name.split_once("::")) {
        Some((program_name, function_name)) => (program_name.to_string(), function_name.to_string()),
        None => match args.package {
            Some(package) => (
                package
                    .compilation_units
                    .last()
                    .expect("There must be at least one program in a Leo package")
                    .name
                    .to_string(),
                args.name.clone(),
            ),
            None => {
                return Err(errors::custom(format!(
                    "Running `leo execute {} ...`, without an explicit program name requires that your current working directory is a valid Leo project.",
                    args.name
                ))
                .into());
            }
        },
    };

    let program_id = ProgramID::<A::Network>::from_str(&program_name)
        .map_err(|e| errors::custom(format!("Failed to parse program name: {e}")))?;
    let function_id = Identifier::<A::Network>::from_str(&function_name)
        .map_err(|e| errors::custom(format!("Failed to parse function name: {e}")))?;

    let programs = if let Some(package) = args.package {
        package
            .compilation_units
            .iter()
            .filter(|unit| !unit.kind.is_library())
            .map(|unit| {
                let program_id = ProgramID::<A::Network>::from_str(&format!("{}", unit.name))
                    .map_err(|e| errors::custom(format!("Failed to parse program ID: {e}")))?;
                match &unit.data {
                    ProgramData::Bytecode(bytecode) => Ok((program_id, bytecode.to_string(), unit.edition)),
                    ProgramData::SourcePath { .. } => {
                        let bytecode_path = package.unit_bytecode_path(&unit.name.to_string());
                        let bytecode = std::fs::read_to_string(&bytecode_path).map_err(|e| {
                            errors::custom(format!("Failed to read bytecode at {}: {e}", bytecode_path.display()))
                        })?;
                        Ok((program_id, bytecode, unit.edition))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };

    let mut programs = programs
        .into_iter()
        .map(|(_, bytecode, edition)| {
            let program = snarkvm::prelude::Program::<A::Network>::from_str(&bytecode)
                .map_err(|e| errors::custom(format!("Failed to parse program: {e}")))?;
            Ok((program, edition))
        })
        .collect::<Result<Vec<_>>>()?;

    let is_local = programs.iter().any(|(program, _)| program.id() == &program_id);

    if is_local {
        let program = &programs.iter().find(|(program, _)| program.id() == &program_id).unwrap().0;
        if program.contains_view(&function_id) {
            return Err(errors::custom(format!(
                "`{function_name}` is a `view fn`; views are read-only and cannot be simulated by `leo run` \
                 (which evaluates against an empty in-memory finalize store)."
            ))
            .into());
        }
        if !program.contains_function(&function_id) {
            return Err(errors::custom(format!(
                "Function `{function_name}` does not exist in program `{program_name}`."
            ))
            .into());
        }
    }

    let inputs =
        args.inputs.into_iter().map(|string| parse_input(&string, &private_key)).collect::<Result<Vec<_>>>()?;

    let rng = &mut rand::rng();

    let vm = VM::from(ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?)?;

    if !is_local {
        let endpoint = crate::options::get_endpoint(args.endpoint)?;
        println!("⬇️ Downloading {program_name} and its dependencies from {endpoint}...");
        programs = load_latest_programs_from_network(
            args.home_path,
            program_id,
            args.network,
            &endpoint,
            args.network_retries,
        )?;
    };

    println!("\n➕Adding programs to the VM in the following order:");
    let programs_and_editions = programs
        .into_iter()
        .map(|(program, edition)| {
            print_program_source(&program.id().to_string(), edition);
            let edition = edition.unwrap_or(LOCAL_PROGRAM_DEFAULT_EDITION);
            (program, edition)
        })
        .collect::<Vec<_>>();
    vm.process().lock().add_programs_with_editions(&programs_and_editions)?;

    if !args.with.is_empty() {
        let endpoint = crate::options::get_endpoint(args.endpoint).ok();
        load_extra_programs_into_vm::<A::Network>(
            args.with,
            &vm,
            args.home_path,
            args.network,
            endpoint.as_deref(),
            args.network_retries,
        )?;
    }

    let authorization = vm
        .authorize(&private_key, program_id, function_id, inputs.iter(), rng)
        .map_err(|e| errors::custom(format!("Failed to authorize execution: {e}")))?;
    let response = vm
        .process()
        .evaluate::<A>(authorization)
        .map_err(|e| errors::custom(format!("Failed to evaluate program: {e}")))?;

    let outputs: Vec<String> = response.outputs().iter().map(|o| o.to_string()).collect();

    match outputs.len() {
        0 => (),
        1 => println!("\n➡️  Output\n"),
        _ => println!("\n➡️  Outputs\n"),
    };
    for output in &outputs {
        println!(" • {output}");
    }

    Ok(RunOutput { program: program_id.to_string(), function: function_id.to_string(), outputs })
}
