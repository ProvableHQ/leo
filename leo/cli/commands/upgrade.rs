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

use super::*;
use std::{collections::HashSet, fs};

use leo_ast::NetworkName;
use leo_package::{Package, ProgramData, fetch_program_from_network};

#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};
use snarkvm::{
    circuit::{Aleo, AleoTestnetV0},
    ledger::query::{Query as SnarkVMQuery, QueryTrait},
    prelude::{
        Certificate,
        Deployment,
        Fee,
        Program,
        ProgramOwner,
        TestnetV0,
        VM,
        VerifyingKey,
        deployment_cost,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use crate::cli::{check_transaction::TransactionStatus, commands::deploy::validate_deployment_limits};
use aleo_std::StorageMode;
use colored::*;
use itertools::Itertools;
use leo_span::Symbol;
use snarkvm::{
    prelude::{ConsensusVersion, ProgramID, Stack, store::helpers::memory::BlockMemory},
    synthesizer::program::StackTrait,
};
use std::path::PathBuf;

/// Upgrades an Aleo program.
#[derive(Parser, Debug)]
pub struct LeoUpgrade {
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) extra: ExtraOptions,
    #[clap(long, help = "Skips the upgrade of any program that contains one of the given substrings.", value_delimiter = ',', num_args = 1..)]
    pub(crate) skip: Vec<String>,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
    #[clap(long, help = "Skips deployment certificate generation.")]
    pub(crate) skip_deploy_certificate: bool,
}

impl Command for LeoUpgrade {
    type Input = Package;
    type Output = DeployOutput;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        LeoBuild {
            env_override: self.env_override.clone(),
            options: {
                let mut options = self.build_options.clone();
                options.no_cache = true;
                options
            },
        }
        .execute(context)
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Get the network, accounting for overrides.
        let network = get_network(&self.env_override.network)?;
        // Handle each network with the appropriate parameterization.
        match network {
            NetworkName::TestnetV0 => handle_upgrade::<TestnetV0, AleoTestnetV0>(&self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_upgrade::<MainnetV0, snarkvm::circuit::AleoV0>(&self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_upgrade::<CanaryV0, snarkvm::circuit::AleoCanaryV0>(&self, context, network, input)
            }
        }
    }
}

// A helper function to handle upgrade logic.
fn handle_upgrade<N: Network, A: Aleo<Network = N>>(
    command: &LeoUpgrade,
    context: Context,
    network: NetworkName,
    package: Package,
) -> Result<<LeoDeploy as Command>::Output> {
    // Get the private key and associated address, accounting for overrides.
    let private_key = get_private_key(&command.env_override.private_key)?;
    let address =
        Address::try_from(&private_key).map_err(|e| CliError::custom(format!("Failed to parse address: {e}")))?;

    // Get the endpoint, accounting for overrides.
    let endpoint = get_endpoint(&command.env_override.endpoint)?;

    // Get whether the network is a devnet, accounting for overrides.
    let is_devnet = get_is_devnet(command.env_override.devnet);

    // If the consensus heights are provided, use them; otherwise, use the default heights for the network.
    let consensus_heights =
        command.env_override.consensus_heights.clone().unwrap_or_else(|| get_consensus_heights(network, is_devnet));
    // Validate the provided consensus heights.
    validate_consensus_heights(&consensus_heights)
        .map_err(|e| CliError::custom(format!("Invalid consensus heights: {e}")))?;
    // Print the consensus heights being used.
    let consensus_heights_string = consensus_heights.iter().format(",").to_string();
    println!(
        "\n📢 Using the following consensus heights: {consensus_heights_string}\n  To override, pass in `--consensus-heights` or override the environment variable `CONSENSUS_VERSION_HEIGHTS`.\n"
    );

    // Set the consensus heights in the environment.
    #[allow(unsafe_code)]
    unsafe {
        // SAFETY:
        //  - `CONSENSUS_VERSION_HEIGHTS` is only set once and is only read in `snarkvm::prelude::load_consensus_heights`.
        //  - There are no concurrent threads running at this point in the execution.
        // WHY:
        //  - This is needed because there is no way to set the desired consensus heights for a particular `VM` instance
        //    without using the environment variable `CONSENSUS_VERSION_HEIGHTS`. Which is itself read once, and stored in a `OnceLock`.
        std::env::set_var("CONSENSUS_VERSION_HEIGHTS", consensus_heights_string);
    }

    // Get all the programs but tests.
    let programs = package.programs.iter().filter(|program| !program.is_test).cloned();

    let programs_and_bytecode: Vec<(leo_package::Program, String)> = programs
        .into_iter()
        .map(|program| {
            let bytecode = match &program.data {
                ProgramData::Bytecode(s) => s.clone(),
                ProgramData::SourcePath { .. } => {
                    // We need to read the bytecode from the filesystem.
                    let aleo_name = format!("{}.aleo", program.name);
                    let aleo_path = if package.manifest.program == aleo_name {
                        // The main program in the package, so its .aleo file
                        // will be in the build directory.
                        package.build_directory().join("main.aleo")
                    } else {
                        // Some other dependency, so look in `imports`.
                        package.imports_directory().join(aleo_name)
                    };
                    fs::read_to_string(aleo_path.clone())
                        .map_err(|e| CliError::custom(format!("Failed to read file {}: {e}", aleo_path.display())))?
                }
            };

            Ok((program, bytecode))
        })
        .collect::<Result<_>>()?;

    // Parse the fee options.
    let fee_options = parse_fee_options(&private_key, &command.fee_options, programs_and_bytecode.len())?;

    let tasks: Vec<Task<N>> = programs_and_bytecode
        .into_iter()
        .zip(fee_options)
        .map(|((program, bytecode), (_base_fee, priority_fee, record))| {
            let id_str = format!("{}.aleo", program.name);
            let id =
                id_str.parse().map_err(|e| CliError::custom(format!("Failed to parse program ID {id_str}: {e}")))?;
            let bytecode_size = bytecode.len();
            let parsed_program =
                bytecode.parse().map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            Ok(Task {
                id,
                program: parsed_program,
                edition: program.edition,
                is_local: program.is_local,
                priority_fee,
                record,
                bytecode_size,
            })
        })
        .collect::<Result<_>>()?;

    // Get the program IDs.
    let program_ids = tasks.iter().map(|task| task.id).collect::<Vec<_>>();

    // Split the tasks into local and remote dependencies.
    let (local, remote) = tasks.into_iter().partition::<Vec<_>, _>(|task| task.is_local);

    // Get the skipped programs.
    let skipped: HashSet<ProgramID<N>> = local
        .iter()
        .filter_map(|task| {
            let id_string = task.id.to_string();
            command.skip.iter().any(|skip| id_string.contains(skip)).then_some(task.id)
        })
        .collect();

    // Get the consensus version.
    let consensus_version =
        get_consensus_version(&command.extra.consensus_version, &endpoint, network, &consensus_heights, &context)?;

    // Build the config for JSON output.
    let config = Some(Config {
        address: address.to_string(),
        network: network.to_string(),
        endpoint: Some(endpoint.clone()),
        consensus_version: Some(consensus_version as u8),
    });

    // Print a summary of the deployment plan.
    print_deployment_plan(
        &private_key,
        &address,
        &endpoint,
        &network,
        &local,
        &skipped,
        &remote,
        &check_tasks_for_warnings(&endpoint, network, &local, consensus_version, command),
        consensus_version,
        &command.into(),
    );

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with upgrade?", command.extra.yes)? {
        println!("❌ Upgrade aborted.");
        return Ok(DeployOutput::default());
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<N, ConsensusMemory<N>>::open(StorageMode::Production)?)?;

    // Load all the programs from the network into the VM.
    let mut programs_and_editions = Vec::with_capacity(program_ids.len());
    for id in &program_ids {
        // Load the program from the network.
        let Ok(program) = leo_package::Program::fetch(
            Symbol::intern(&id.name().to_string()),
            None,
            context.home()?,
            network,
            &endpoint,
            true,
        ) else {
            warn_and_confirm(&format!("Failed to fetch program {id} from the network."), command.extra.yes)?;
            continue;
        };

        let ProgramData::Bytecode(bytecode) = program.data else {
            panic!("Expected bytecode when fetching a remote program");
        };
        // Parse the program bytecode.
        let bytecode =
            Program::<N>::from_str(&bytecode).map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
        // Append the bytecode and edition.
        // Program::fetch should always set the edition after a successful fetch.
        let edition = program.edition.expect("Edition should be set after successful fetch");
        programs_and_editions.push((bytecode, edition));
    }
    // Check for programs that violate edition/constructor requirements.
    check_edition_constructor_requirements(&programs_and_editions, consensus_version, "upgrade")?;
    // Add the programs to the VM.
    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    // Print the programs and their editions in the VM.
    println!("Loaded the following programs into the VM:");
    for program_id in vm.process().read().program_ids() {
        let edition = *vm.process().read().get_stack(program_id)?.program_edition();
        if program_id.to_string() == "credits.aleo" {
            println!(" - credits.aleo (default)");
        } else {
            println!(" - {program_id} (edition {edition})");
        }
    }
    println!();

    // Remove version suffixes from the endpoint.
    let re = regex::Regex::new(r"v\d+$").unwrap();
    let query_endpoint = re.replace(&endpoint, "").to_string();

    // Specify the query.
    let query = SnarkVMQuery::<N, BlockMemory<N>>::from(
        query_endpoint
            .parse::<Uri>()
            .map_err(|e| CliError::custom(format!("Failed to parse endpoint URI '{endpoint}': {e}")))?,
    );

    // For each of the programs, generate a deployment transaction.
    let mut transactions = Vec::new();
    let mut all_stats = Vec::new();
    let mut all_broadcasts = Vec::new();
    for Task { id, program, priority_fee, record, bytecode_size, .. } in local {
        // If the program is a local dependency that is not skipped, generate a deployment transaction.
        if !skipped.contains(&id) {
            if command.skip_deploy_certificate {
                println!("⚠️  Skipping deployment certificate generation as per user request.\n");
                assert!(!program.functions().is_empty(), "Program `{}` has no functions", program.id());
                // Initialize a vector for the placeholder verifying keys and certificates.
                let mut verifying_keys = Vec::with_capacity(program.functions().len());
                for function_name in program.functions().keys() {
                    let (verifying_key, certificate) = {
                        // Use a placeholder verifying key.
                        let verifying_key = VerifyingKey::from_str(
                            "verifier1qygqqqqqqqqqqqyvxgqqqqqqqqq87vsqqqqqqqqqhe7sqqqqqqqqqma4qqqqqqqqqq65yqqqqqqqqqqvqqqqqqqqqqqgtlaj49fmrk2d8slmselaj9tpucgxv6awu6yu4pfcn5xa0yy0tpxpc8wemasjvvxr9248vt3509vpk3u60ejyfd9xtvjmudpp7ljq2csk4yqz70ug3x8xp3xn3ul0yrrw0mvd2g8ju7rts50u3smue03gp99j88f0ky8h6fjlpvh58rmxv53mldmgrxa3fq6spsh8gt5whvsyu2rk4a2wmeyrgvvdf29pwp02srktxnvht3k6ff094usjtllggva2ym75xc4lzuqu9xx8ylfkm3qc7lf7ktk9uu9du5raukh828dzgq26hrarq5ajjl7pz7zk924kekjrp92r6jh9dpp05mxtuffwlmvew84dvnqrkre7lw29mkdzgdxwe7q8z0vnkv2vwwdraekw2va3plu7rkxhtnkuxvce0qkgxcxn5mtg9q2c3vxdf2r7jjse2g68dgvyh85q4mzfnvn07lletrpty3vypus00gfu9m47rzay4mh5w9f03z9zgzgzhkv0mupdqsk8naljqm9tc2qqzhf6yp3mnv2ey89xk7sw9pslzzlkndfd2upzmew4e4vnrkr556kexs9qrykkuhsr260mnrgh7uv0sp2meky0keeukaxgjdsnmy77kl48g3swcvqdjm50ejzr7x04vy7hn7anhd0xeetclxunnl7pd6e52qxdlr3nmutz4zr8f2xqa57a2zkl59a28w842cj4783zpy9hxw03k6vz4a3uu7sm072uqknpxjk8fyq4vxtqd08kd93c2mt40lj9ag35nm4rwcfjayejk57m9qqu83qnkrj3sz90pw808srmf705n2yu6gvqazpvu2mwm8x6mgtlsntxfhr0qas43rqxnccft36z4ygty86390t7vrt08derz8368z8ekn3yywxgp4uq24gm6e58tpp0lcvtpsm3nkwpnmzztx4qvkaf6vk38wg787h8mfpqqqqqqqqqqt49m8x",
                        )?;
                        // Use a placeholder certificate.
                        let certificate = Certificate::from_str(
                            "certificate1qyqsqqqqqqqqqqxvwszp09v860w62s2l4g6eqf0kzppyax5we36957ywqm2dplzwvvlqg0kwlnmhzfatnax7uaqt7yqqqw0sc4u",
                        )?;

                        (verifying_key, certificate)
                    };
                    verifying_keys.push((*function_name, (verifying_key, certificate)));
                }
                // Increment the edition.
                let edition = *vm.process().read().get_stack(id)?.program_edition() + 1;
                println!("edition for deployed program: {}", edition);
                let mut deployment = Deployment::new(edition, program.clone(), verifying_keys, None, None).unwrap();

                // Set the program owner.
                deployment.set_program_owner_raw(Some(Address::try_from(&private_key)?));

                // Compute the checksum of the deployment.
                deployment.set_program_checksum_raw(Some(deployment.program().to_checksum()));

                // Compute the deployment ID.
                let deployment_id = deployment.to_deployment_id()?;

                // Construct the owner.
                let owner = ProgramOwner::new(&private_key, deployment_id, rng)?;

                // Construct the fee authorization.
                let (minimum_deployment_cost, _) =
                    deployment_cost(&vm.process().read(), &deployment, consensus_version)?;
                // Authorize the fee.
                let fee_authorization = match record {
                    Some(record) => vm.process().read().authorize_fee_private::<A, _>(
                        &private_key,
                        record,
                        minimum_deployment_cost,
                        priority_fee.unwrap_or(0),
                        deployment_id,
                        rng,
                    )?,
                    None => vm.process().read().authorize_fee_public::<A, _>(
                        &private_key,
                        minimum_deployment_cost,
                        priority_fee.unwrap_or(0),
                        deployment_id,
                        rng,
                    )?,
                };

                // Get the state root.
                let state_root = query.current_state_root()?;

                // Create a fee transition without a proof.
                let fee = Fee::from(fee_authorization.transitions().into_iter().next().unwrap().1, state_root, None)?;

                // Create the transaction.
                let transaction = Transaction::from_deployment(owner, deployment, fee)?;
                // Add the transaction to the transactions vector.
                transactions.push((id, transaction));
            } else {
                println!("📦 Creating deployment transaction for '{}'...\n", id.to_string().bold());
                // Generate the transaction.
                let transaction = vm
                    .deploy(&private_key, &program, record, priority_fee.unwrap_or(0), Some(&query), rng)
                    .map_err(|e| CliError::custom(format!("Failed to generate deployment transaction: {e}")))?;
                // Get the deployment.
                let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
                // Compute and print the deployment stats.
                let stats = print_deployment_stats(
                    &vm,
                    &id.to_string(),
                    deployment,
                    priority_fee,
                    consensus_version,
                    bytecode_size,
                )?;
                // Validate the deployment limits.
                validate_deployment_limits(deployment, &id, &network)?;
                // Save the transaction and stats.
                transactions.push((id, transaction));
                all_stats.push(stats);
            }
        }
        // Add the program to the VM.
        if let Err(e) = vm.process().write().add_program(&program) {
            warn_and_confirm(&format!("Failed to add program {id} to the VM. Error: {e}"), command.extra.yes)?;
        }
    }

    // If the `print` option is set, print the deployment transaction to the console.
    // The transaction is printed in JSON format.
    if command.action.print {
        for (program_name, transaction) in transactions.iter() {
            // Pretty-print the transaction.
            let transaction_json = serde_json::to_string_pretty(transaction)
                .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
            println!("🖨️ Printing deployment for {program_name}\n{transaction_json}")
        }
    }

    // If the `save` option is set, save each deployment transaction to a file in the specified directory.
    // The file format is `program_name.deployment.json`.
    // The directory is created if it doesn't exist.
    if let Some(path) = &command.action.save {
        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
        for (program_name, transaction) in transactions.iter() {
            // Save the transaction to a file.
            let file_path = PathBuf::from(path).join(format!("{program_name}.deployment.json"));
            println!("💾 Saving deployment for {program_name} at {}", file_path.display());
            let transaction_json = serde_json::to_string_pretty(transaction)
                .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
            std::fs::write(file_path, transaction_json)
                .map_err(|e| CliError::custom(format!("Failed to write transaction to file: {e}")))?;
        }
    }

    // If the `broadcast` option is set, broadcast each upgrade transaction to the network.
    if command.action.broadcast {
        for (i, (program_id, transaction)) in transactions.iter().enumerate() {
            println!("📡 Broadcasting upgrade for {program_id}...");
            // Get and confirm the fee with the user.
            let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
            if !confirm_fee(&fee, &private_key, &address, &endpoint, network, &context, command.extra.yes)? {
                println!("⏩ Upgrade skipped.");
                continue;
            }
            let fee_id = fee.id().to_string();
            let fee_transaction_id = Transaction::from_fee(fee.clone())?.id().to_string();
            let id = transaction.id().to_string();
            let height_before = check_transaction::current_height(&endpoint, network)?;
            // Broadcast the transaction to the network.
            let (message, status) = handle_broadcast(
                &format!("{endpoint}/{network}/transaction/broadcast"),
                transaction,
                &program_id.to_string(),
            )?;

            let fail_and_prompt = |msg| {
                println!("❌ Failed to upgrade program {program_id}: {msg}.");
                let count = transactions.len() - i - 1;
                // Check if the user wants to continue with the next upgrade.
                if count > 0 {
                    confirm("Do you want to continue with the next upgrade?", command.extra.yes)
                } else {
                    Ok(false)
                }
            };

            match status {
                200..=299 => {
                    let tx_status = check_transaction::check_transaction_with_message(
                        &id,
                        Some(&fee_id),
                        &endpoint,
                        network,
                        height_before + 1,
                        command.extra.max_wait,
                        command.extra.blocks_to_check,
                    )?;
                    let confirmed = tx_status == Some(TransactionStatus::Accepted);
                    if confirmed {
                        println!("✅ Upgrade confirmed!");
                    } else if fail_and_prompt("could not find the transaction on the network")? {
                        continue;
                    } else {
                        return Ok(build_deploy_output(config.clone(), &transactions, &all_stats, &all_broadcasts));
                    }
                    all_broadcasts.push(BroadcastStats {
                        fee_id: fee_id.clone(),
                        fee_transaction_id: fee_transaction_id.clone(),
                        confirmed,
                    });
                }
                _ => {
                    if fail_and_prompt(&message)? {
                        continue;
                    } else {
                        return Ok(build_deploy_output(config.clone(), &transactions, &all_stats, &all_broadcasts));
                    }
                }
            }
        }
    }

    Ok(build_deploy_output(config, &transactions, &all_stats, &all_broadcasts))
}

/// Check the tasks to warn the user about any potential issues.
/// The following properties are checked:
/// - If the transaction is to be broadcast:
///     - The program exists on the network and the new program is a valid upgrade.
///     - If the consensus version is less than V9, the program does not use V9 features.
///     - If the consensus version is V9 or greater, the program contains a constructor.
fn check_tasks_for_warnings<N: Network>(
    endpoint: &str,
    network: NetworkName,
    tasks: &[Task<N>],
    consensus_version: ConsensusVersion,
    command: &LeoUpgrade,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for Task { id, program, is_local, .. } in tasks {
        if !is_local || !command.action.broadcast {
            continue;
        }

        // Check if the program exists on the network.
        if let Ok(remote_program) = fetch_program_from_network(&id.to_string(), endpoint, network) {
            // Parse the program.
            let remote_program = match Program::<N>::from_str(&remote_program) {
                Ok(program) => program,
                Err(e) => {
                    warnings.push(format!("Could not parse '{id}' from the network. Error: {e}",));
                    continue;
                }
            };
            // Check if the program is a valid upgrade.
            if remote_program.contains_constructor() {
                if let Err(e) = Stack::check_upgrade_is_valid(&remote_program, program) {
                    warnings.push(format!(
                        "The program '{id}' is not a valid upgrade. The upgrade will likely fail. Error: {e}",
                    ));
                }
            } else if consensus_version >= ConsensusVersion::V8 {
                warnings.push(format!("The program '{id}' can only ever be upgraded once and its contents cannot be changed. Otherwise, the upgrade will likely fail."));
            } else {
                warnings.push(format!("The program '{id}' does not have a constructor and is not eligible for a one-time upgrade (>= `ConsensusVersion::V8`). The upgrade will likely fail."));
            }
        } else {
            warnings.push(format!("The program '{id}' does not exist on the network. The upgrade will likely fail.",));
        }
        // Check if the program has a valid naming scheme.
        if consensus_version >= ConsensusVersion::V7
            && let Err(e) = program.check_program_naming_structure()
        {
            warnings.push(format!(
                "The program '{id}' has an invalid naming scheme: {e}. The deployment will likely fail."
            ));
        }
        // Check if the program contains restricted keywords.
        if let Err(e) = program.check_restricted_keywords_for_consensus_version(consensus_version) {
            warnings.push(format!(
                "The program '{id}' contains restricted keywords for consensus version {}: {e}. The deployment will likely fail.",
                consensus_version as u8
            ));
        }
        // Check if the program uses V9 features.
        if consensus_version < ConsensusVersion::V9 && program.contains_v9_syntax() {
            warnings.push(format!("The program '{id}' uses V9 features but the consensus version is less than V9. The upgrade will likely fail"));
        }
        // Check if the program contains a constructor.
        if consensus_version >= ConsensusVersion::V9 && !program.contains_constructor() {
            warnings.push(format!("The program '{id}' does not contain a constructor. The upgrade will likely fail",));
        }
        // Check for a consensus version mismatch.
        if let Err(e) = check_consensus_version_mismatch(consensus_version, endpoint, network) {
            warnings.push(format!("{e}. In some cases, the deployment may fail"));
        }
    }
    warnings
}

// Convert the `LeoUpgrade` into a `LeoDeploy` command.
impl From<&LeoUpgrade> for LeoDeploy {
    fn from(upgrade: &LeoUpgrade) -> Self {
        Self {
            fee_options: upgrade.fee_options.clone(),
            action: upgrade.action.clone(),
            env_override: upgrade.env_override.clone(),
            extra: upgrade.extra.clone(),
            skip: upgrade.skip.clone(),
            build_options: upgrade.build_options.clone(),
            skip_deploy_certificate: upgrade.skip_deploy_certificate,
        }
    }
}
