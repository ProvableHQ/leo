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

use check_transaction::TransactionStatus;
use leo_ast::NetworkName;
use leo_package::{Package, ProgramData, fetch_program_from_network};

use aleo_std::StorageMode;
#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};
use snarkvm::{
    circuit::{Aleo, AleoTestnetV0},
    ledger::{
        query::{Query as SnarkVMQuery, QueryTrait},
        store::helpers::memory::BlockMemory,
    },
    prelude::{
        Certificate,
        ConsensusVersion,
        Deployment,
        Fee,
        Program,
        ProgramID,
        ProgramOwner,
        TestnetV0,
        VM,
        VerifyingKey,
        deployment_cost,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use colored::*;
use itertools::Itertools;
use std::{collections::HashSet, fs, path::PathBuf};

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct LeoDeploy {
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) extra: ExtraOptions,
    #[clap(long, help = "Skips deployment of any program that contains one of the given substrings.", value_delimiter = ',', num_args = 1..)]
    pub(crate) skip: Vec<String>,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
    #[clap(long, help = "Use placeholder certificate and verifying keys during deployment.", default_value = "false")]
    pub(crate) skip_deploy_certificate: bool,
}

pub struct Task<N: Network> {
    pub id: ProgramID<N>,
    pub program: Program<N>,
    pub edition: Option<u16>,
    pub is_local: bool,
    pub priority_fee: Option<u64>,
    pub record: Option<Record<N, Plaintext<N>>>,
    pub bytecode_size: usize,
}

impl Command for LeoDeploy {
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
            NetworkName::TestnetV0 => handle_deploy::<TestnetV0, AleoTestnetV0>(&self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_deploy::<MainnetV0, snarkvm::circuit::AleoV0>(&self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_deploy::<CanaryV0, snarkvm::circuit::AleoCanaryV0>(&self, context, network, input)
            }
        }
    }
}

// A helper function to handle deployment logic.
fn handle_deploy<N: Network, A: Aleo<Network = N>>(
    command: &LeoDeploy,
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
        .map_err(|e| CliError::custom(format!("⚠️ Invalid consensus heights: {e}")))?;
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
        command,
    );

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with deployment?", command.extra.yes)? {
        println!("❌ Deployment aborted.");
        return Ok(DeployOutput::default());
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<N, ConsensusMemory<N>>::open(StorageMode::Production)?)?;

    // Load the remote dependencies into the VM.
    let programs_and_editions = remote
        .into_iter()
        .map(|task| {
            // Get the actual edition from the network if not specified.
            let edition = match task.edition {
                Some(e) => e,
                None => leo_package::fetch_latest_edition(&task.id.to_string(), &endpoint, network)?,
            };
            Ok((task.program, edition))
        })
        .collect::<Result<Vec<_>>>()?;

    // Check for programs that violate edition/constructor requirements.
    check_edition_constructor_requirements(&programs_and_editions, consensus_version, "deploy")?;

    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    // Remove version suffixes from the endpoint.
    let re = regex::Regex::new(r"v\d+$").unwrap();
    let query_endpoint = re.replace(&endpoint, "").to_string();

    // Specify the query
    let query = SnarkVMQuery::<N, BlockMemory<N>>::from(
        query_endpoint
            .parse::<Uri>()
            .map_err(|e| CliError::custom(format!("Failed to parse endpoint URI '{endpoint}': {e}")))?,
    );

    // For each of the programs, generate a deployment transaction.
    let mut transactions = Vec::new();
    let mut all_stats = Vec::new();
    let mut all_broadcasts = Vec::new();
    for Task { id, program, edition, priority_fee, record, bytecode_size, .. } in local {
        // If the program is a local dependency that is not skipped, generate a deployment transaction.
        if !skipped.contains(&id) {
            // If the program contains an upgrade config, confirm with the user that they want to proceed.
            if let Some(constructor) = program.constructor() {
                println!(
                    r"
🔧 Your program '{}' has the following constructor.
──────────────────────────────────────────────
{constructor}
──────────────────────────────────────────────
Once it is deployed, it CANNOT be changed.
",
                    id.to_string().bold()
                );
                if !confirm("Would you like to proceed?", command.extra.yes)? {
                    println!("❌ Deployment aborted.");
                    return Ok(DeployOutput::default());
                }
            }
            println!("📦 Creating deployment transaction for '{}'...\n", id.to_string().bold());
            if command.skip_deploy_certificate {
                println!("⚠️  Skipping deployment certificate and verifier key generation as per user request.\n");
                assert!(!program.functions().is_empty(), "Program `{}` has no functions", program.id());
                // Initialize a vector for the placeholder  verifying keys and certificates.
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
                // Create the deployment.
                let mut deployment =
                    Deployment::new(edition.unwrap_or(0), program.clone(), verifying_keys, None, None).unwrap();

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
                // Save the transaction.
                transactions.push((id, transaction));
                all_stats.push(stats);
            }

            if !command.skip_deploy_certificate {
                for (program_id, transaction) in transactions.iter() {
                    // Validate the deployment limits.
                    let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
                    validate_deployment_limits(deployment, program_id, &network)?;
                }
            }
        }

        // Add the program to the VM.
        vm.process().write().add_program(&program)?;
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

    // If the `broadcast` option is set, broadcast each deployment transaction to the network.
    if command.action.broadcast {
        for (i, (program_id, transaction)) in transactions.iter().enumerate() {
            println!("\n📡 Broadcasting deployment for {}...", program_id.to_string().bold());
            // Get and confirm the fee with the user.
            let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
            if !confirm_fee(&fee, &private_key, &address, &endpoint, network, &context, command.extra.yes)? {
                println!("⏩ Deployment skipped.");
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
                println!("❌ Failed to deploy program {program_id}: {msg}.");
                let count = transactions.len() - i - 1;
                // Check if the user wants to continue with the next deployment.
                if count > 0 {
                    confirm("Do you want to continue with the next deployment?", command.extra.yes)
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
                        println!("✅ Deployment confirmed!");
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
///     - The program does not exist on the network.
///     - If the consensus version is less than V9, the program does not use V9 features.
///     - If the consensus version is V9 or greater, the program contains a constructor.
///     - The program size is approaching the limit.
fn check_tasks_for_warnings<N: Network>(
    endpoint: &str,
    network: NetworkName,
    tasks: &[Task<N>],
    consensus_version: ConsensusVersion,
    command: &LeoDeploy,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for Task { id, is_local, program, bytecode_size, .. } in tasks {
        if !is_local || !command.action.broadcast {
            continue;
        }
        // Check if the program exists on the network.
        if fetch_program_from_network(&id.to_string(), endpoint, network).is_ok() {
            warnings
                .push(format!("The program '{id}' already exists on the network. Please use `leo upgrade` instead.",));
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
            warnings.push(format!("The program '{id}' uses V9 features but the consensus version is less than V9. The deployment will likely fail"));
        }
        // Check if the program contains a constructor.
        if consensus_version >= ConsensusVersion::V9 && !program.contains_constructor() {
            warnings
                .push(format!("The program '{id}' does not contain a constructor. The deployment will likely fail",));
        }
        // Check if the program size is approaching the limit.
        if let (_, _, Some(msg)) = format_program_size(*bytecode_size, N::MAX_PROGRAM_SIZE) {
            warnings.push(format!("The program '{id}' is {msg}."));
        }
    }
    // Check for a consensus version mismatch.
    if let Err(e) = check_consensus_version_mismatch(consensus_version, endpoint, network) {
        warnings.push(format!("{e}. In some cases, the deployment may fail"));
    }
    warnings
}

/// Check if the number of variables and constraints are within the limits.
pub(crate) fn validate_deployment_limits<N: Network>(
    deployment: &Deployment<N>,
    program_id: &ProgramID<N>,
    network: &NetworkName,
) -> Result<()> {
    // Check if the number of variables is within the limits.
    let combined_variables = deployment.num_combined_variables()?;
    if combined_variables > N::MAX_DEPLOYMENT_VARIABLES {
        return Err(CliError::variable_limit_exceeded(
            program_id,
            combined_variables,
            N::MAX_DEPLOYMENT_VARIABLES,
            network,
        )
        .into());
    }

    // Check if the number of constraints is within the limits.
    let constraints = deployment.num_combined_constraints()?;
    if constraints > N::MAX_DEPLOYMENT_CONSTRAINTS {
        return Err(CliError::constraint_limit_exceeded(
            program_id,
            constraints,
            N::MAX_DEPLOYMENT_CONSTRAINTS,
            network,
        )
        .into());
    }

    Ok(())
}

/// Pretty‑print the deployment plan without using a table.
#[allow(clippy::too_many_arguments)]
pub(crate) fn print_deployment_plan<N: Network>(
    private_key: &PrivateKey<N>,
    address: &Address<N>,
    endpoint: &str,
    network: &NetworkName,
    local: &[Task<N>],
    skipped: &HashSet<ProgramID<N>>,
    remote: &[Task<N>],
    warnings: &[String],
    consensus_version: ConsensusVersion,
    command: &LeoDeploy,
) {
    use colored::*;

    println!("\n{}", "🛠️  Deployment Plan Summary".bold());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    // ── Configuration ────────────────────────────────────────────────────
    println!("{}", "🔧 Configuration:".bold());
    println!("  {:20}{}", "Private Key:".cyan(), format!("{}...", &private_key.to_string()[..24]).yellow());
    println!("  {:20}{}", "Address:".cyan(), format!("{}...", &address.to_string()[..24]).yellow());
    println!("  {:20}{}", "Endpoint:".cyan(), endpoint.yellow());
    println!("  {:20}{}", "Network:".cyan(), network.to_string().yellow());
    println!("  {:20}{}", "Consensus Version:".cyan(), (consensus_version as u8).to_string().yellow());

    // ── Deployment tasks (bullet list) ───────────────────────────────────
    println!("\n{}", "📦 Deployment Tasks:".bold());
    if local.is_empty() {
        println!("  (none)");
    } else {
        for Task { id, priority_fee, record, .. } in local.iter().filter(|task| !skipped.contains(&task.id)) {
            let priority_fee_str = priority_fee.map_or("0".into(), |v| v.to_string());
            let record_str = if record.is_some() { "yes" } else { "no (public fee)" };
            println!(
                "  • {}  │ priority fee: {}  │ fee record: {}",
                id.to_string().cyan(),
                priority_fee_str,
                record_str
            );
        }
    }

    // ── Skipped programs ─────────────────────────────────────────────────
    if !skipped.is_empty() {
        println!("\n{}", "🚫 Skipped Programs:".bold().red());
        for symbol in skipped {
            println!("  • {}", symbol.to_string().dimmed());
        }
    }

    // ── Remote dependencies ──────────────────────────────────────────────
    if !remote.is_empty() {
        println!("\n{}", "🌐 Remote Dependencies:".bold().red());
        println!("{}", "(Leo will not generate transactions for these programs)".bold().red());
        for Task { id, .. } in remote {
            println!("  • {}", id.to_string().dimmed());
        }
    }

    // ── Actions ──────────────────────────────────────────────────────────
    println!("\n{}", "⚙️ Actions:".bold());
    if command.action.print {
        println!("  • Transaction(s) will be printed to the console.");
    } else {
        println!("  • Transaction(s) will NOT be printed to the console.");
    }
    if let Some(path) = &command.action.save {
        println!("  • Transaction(s) will be saved to {}", path.bold());
    } else {
        println!("  • Transaction(s) will NOT be saved to a file.");
    }
    if command.action.broadcast {
        println!("  • Transaction(s) will be broadcast to {}", endpoint.bold());
    } else {
        println!("  • Transaction(s) will NOT be broadcast to the network.");
    }

    // ── Warnings ─────────────────────────────────────────────────────────
    if !warnings.is_empty() {
        println!("\n{}", "⚠️ Warnings:".bold().red());
        for warning in warnings {
            println!("  • {}", warning.dimmed());
        }
    }

    println!("{}", "──────────────────────────────────────────────\n".dimmed());
}

/// Compute deployment statistics.
pub(crate) fn compute_deployment_stats<N: Network>(
    vm: &VM<N, ConsensusMemory<N>>,
    deployment: &Deployment<N>,
    priority_fee: Option<u64>,
    consensus_version: ConsensusVersion,
    bytecode_size: usize,
) -> Result<DeploymentStats> {
    let variables = deployment.num_combined_variables()?;
    let constraints = deployment.num_combined_constraints()?;
    let (_, (storage_cost, synthesis_cost, constructor_cost, namespace_cost)) =
        deployment_cost(&vm.process().read(), deployment, consensus_version)?;

    Ok(DeploymentStats {
        program_size_bytes: bytecode_size,
        max_program_size_bytes: N::MAX_PROGRAM_SIZE,
        total_variables: variables,
        total_constraints: constraints,
        max_variables: N::MAX_DEPLOYMENT_VARIABLES,
        max_constraints: N::MAX_DEPLOYMENT_CONSTRAINTS,
        cost: CostBreakdown::for_deployment(
            storage_cost,
            synthesis_cost,
            namespace_cost,
            constructor_cost,
            priority_fee.unwrap_or(0),
        ),
    })
}

/// Pretty-print deployment statistics.
pub(crate) fn print_deployment_stats<N: Network>(
    vm: &VM<N, ConsensusMemory<N>>,
    program_id: &str,
    deployment: &Deployment<N>,
    priority_fee: Option<u64>,
    consensus_version: ConsensusVersion,
    bytecode_size: usize,
) -> Result<DeploymentStats> {
    use colored::*;

    let stats = compute_deployment_stats(vm, deployment, priority_fee, consensus_version, bytecode_size)?;

    println!("\n{} {}", "📊 Deployment Summary for".bold(), program_id.bold());
    println!("{}", "──────────────────────────────────────────────".dimmed());
    print!("{stats}");
    println!("{}", "──────────────────────────────────────────────".dimmed());

    Ok(stats)
}
