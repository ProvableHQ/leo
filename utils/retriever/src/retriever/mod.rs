// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{Dependency, Location, LockFileEntry, Manifest, NetworkName, ProgramContext};

use leo_ast::Stub;
use leo_disassembler::disassemble_from_str;
use leo_errors::UtilError;
use leo_passes::{common::DiGraph, DiGraphError};
use leo_span::Symbol;

use snarkvm::prelude::{Network, Program};

use indexmap::{IndexMap, IndexSet};
use std::{
    fs,
    fs::File,
    io::Read,
    marker::PhantomData,
    path::{Path, PathBuf},
    str::FromStr,
};

// Retriever is responsible for retrieving external programs
pub struct Retriever<N: Network> {
    name: Symbol,
    contexts: IndexMap<Symbol, ProgramContext>,
    project_path: PathBuf,
    registry_path: PathBuf,
    endpoint: String,
    phantom: PhantomData<N>,
}

impl<N: Network> Retriever<N> {
    // Initialize a new Retriever.
    pub fn new(name: Symbol, path: &PathBuf, home: &Path, endpoint: String) -> Result<Self, UtilError> {
        // Starting point is all of the dependencies specified in the main `program.json` file
        let dependencies = retrieve_local(&format!("{name}.aleo"), path)?;
        let mut contexts = IndexMap::from([(name, ProgramContext::new_main(name, path.clone(), dependencies.clone()))]);
        for dep in dependencies {
            contexts.insert(Symbol::from(&dep), ProgramContext::from(dep));
        }

        Ok(Self {
            name,
            contexts,
            project_path: path.clone(),
            registry_path: home.join("registry"),
            endpoint: endpoint.clone(),
            phantom: Default::default(),
        })
    }

    pub fn get_context(&self, name: &Symbol) -> &ProgramContext {
        self.contexts.get(name).expect("Could not find program context")
    }

    // Retrieve all dependencies for a program.
    // Pull all network dependencies, and cache their stubs.
    // Construct post order traversal of all local dependencies to be compiled sequentially.
    pub fn retrieve(&mut self) -> Result<Vec<Symbol>, UtilError> {
        let mut contexts = self.contexts.clone();
        let mut unexplored = IndexSet::from([self.name]);
        let mut explored: IndexSet<Symbol> = IndexSet::new();
        let mut cur_exploring: IndexSet<Symbol> = IndexSet::new();
        let mut dependency_graph: DiGraph<Symbol> = DiGraph::new(IndexSet::new());

        // Loop retrieving all nested dependencies for the current set of dependencies
        // Only adding non-duplicates to be searched in the future
        while !unexplored.is_empty() {
            let mut new_unexplored: IndexSet<Symbol> = IndexSet::new();
            let mut new_contexts: IndexMap<Symbol, ProgramContext> = IndexMap::new();
            // Visit all programs
            for program in unexplored {
                let cur_context =
                    contexts.get_mut(&program.clone()).expect("Program must have been processed before its dependency");
                // Split into cases based on network dependency or local dependency
                let nested_dependencies = match cur_context.location() {
                    Location::Network => {
                        let (stub, nested_dependencies) = retrieve_from_network::<N>(
                            &self.project_path,
                            &self.registry_path,
                            cur_context.full_name(),
                            &self.endpoint,
                        )?;

                        // Cache the stubs
                        if cur_context.add_stub(stub.clone()) {
                            Err(UtilError::duplicate_dependency_name_error(
                                stub.stub_id.name.name,
                                Default::default(),
                            ))?;
                        }

                        cur_context.add_checksum();

                        nested_dependencies
                    }
                    Location::Local => {
                        // Programs add an entry for their dependencies in the self.contexts mapping, so we can use that information to learn the current path
                        let cur_full_path = cur_context.full_path();
                        retrieve_local(cur_context.full_name(), cur_full_path)?
                    }
                    Location::Git => panic!("Location::Git is not supported yet"),
                };

                // Mark as visited
                if !explored.insert(program) {
                    panic!("Should never visit same dependency twice");
                }

                for dep in &nested_dependencies {
                    let dep_sym = Symbol::from(dep);
                    // Dependency's can be processed before their parent, so we need to make sure not to process twice
                    if !explored.contains(&dep_sym) {
                        // Create new ProgramContext for each dependency
                        let mut dep_context = ProgramContext::from(dep.clone());

                        // Add full_path for dependency if they are local
                        match dep_context.location() {
                            Location::Local => {
                                // Impossible for a network dependency to import a local dependency
                                dep_context.add_full_path(&cur_context.full_path().join(dep_context.path()));
                                dep_context
                                    .add_compiled_file_path(&dep_context.full_path().join("build").join("main.aleo"));
                            }
                            Location::Network => {
                                dep_context.add_compiled_file_path(&self.registry_path.join(format!(
                                    "{}/{}",
                                    dep_context.network(),
                                    dep_context.full_name()
                                )));
                            }
                            _ => panic!("Location::Git is not supported yet"),
                        }

                        // Don't add a new dependency to check if it has already been processed, or will be processed in the future
                        if !explored.contains(&dep_sym)
                            && !new_unexplored.contains(&dep_sym)
                            && !cur_exploring.contains(&dep_sym)
                        {
                            new_unexplored.insert(dep_sym);
                        }

                        // Update dependency graph
                        dependency_graph.add_edge(program, dep_sym);

                        new_contexts.insert(dep_sym, dep_context);
                    }
                }

                cur_context.add_dependencies(nested_dependencies.clone().iter().map(Symbol::from).collect());

                new_contexts.insert(program, cur_context.clone());
            }

            // Update contexts
            for (name, context) in new_contexts {
                contexts.insert(name, context);
            }

            cur_exploring = new_unexplored.clone();
            unexplored = new_unexplored;
        }

        // Compute post order of dependency graph
        match dependency_graph.post_order() {
            Ok(mut order) => {
                // Remove the main program
                order.remove(&self.name);

                // Cache order
                contexts
                    .get_mut(&self.name)
                    .expect("Retriever must be initialized with main program")
                    .add_post_order(order.clone());

                // Filter out all network dependencies
                let local_order: Vec<Symbol> =
                    order.iter().cloned().filter(|p| contexts.get(p).unwrap().location() == &Location::Local).collect();

                // Save the local contexts
                self.contexts = contexts;

                Ok(local_order)
            }
            Err(DiGraphError::CycleDetected(_)) => Err(UtilError::circular_dependency_error(Default::default()))?,
        }
    }

    // Prepares all the stubs of the program's dependencies in post order, so that the program can be compiled
    pub fn prepare_local(&mut self, name: Symbol) -> Result<(PathBuf, IndexMap<Symbol, Stub>), UtilError> {
        // Get the post order of the program
        let post_order = if name == self.name {
            // The main program already has its post order cached
            self.get_context(&name).post_order().clone()
        } else {
            // Construct local post order
            let mut unexplored = self.get_context(&name).dependencies();
            let mut local_digraph: DiGraph<Symbol> = DiGraph::new(IndexSet::new());
            let mut solo_programs: IndexSet<Symbol> = IndexSet::new();
            while !unexplored.is_empty() {
                let mut new_unexplored: Vec<Symbol> = Vec::new();
                // Visit all programs
                for program in unexplored {
                    for dep in self.get_context(&program).dependencies() {
                        // Don't add a new dependency to check if it has already been processed, or will be processed in the future
                        if !new_unexplored.contains(&dep) && !local_digraph.contains_node(dep) {
                            new_unexplored.push(dep);
                        }

                        // Update dependency graph
                        local_digraph.add_edge(program, dep);
                    }

                    // Make sure to include solo programs to dependency graph
                    if self.get_context(&program).dependencies().is_empty() {
                        solo_programs.insert(program);
                    }
                }

                unexplored = new_unexplored;
            }

            // Return the order
            match local_digraph.post_order() {
                Ok(mut order) => {
                    order.extend(solo_programs);
                    // Cache order
                    self.contexts.get_mut(&name).unwrap().add_post_order(order.clone());
                    order
                }
                Err(DiGraphError::CycleDetected(_)) => Err(UtilError::circular_dependency_error(Default::default()))?,
            }
        };

        let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();
        let project_path = self.get_context(&name).full_path().clone();
        // Prepare build directory for compilation
        for dep in post_order {
            let dep_context = self.get_context(&dep);
            // Fetch stubs. They must exist in cache.
            stubs.insert(dep, dep_context.stub().clone());

            let imports_path = project_path.join("build").join("imports");

            if !imports_path.exists() {
                std::fs::create_dir_all(&imports_path)
                    .unwrap_or_else(|_| panic!("Failed to create build/imports directory for `{name}`"));
            }

            let destination_path = imports_path.join(dep_context.full_name());

            // Move all dependencies to local build directory
            let source_string = fs::read_to_string(dep_context.compiled_file_path()).unwrap_or_else(|_| {
                panic!(
                    "Failed to read `{name}` from `{path}`",
                    name = dep_context.full_name(),
                    path = dep_context.compiled_file_path().to_str().unwrap()
                )
            });
            fs::write(destination_path.clone(), source_string).unwrap_or_else(|_| {
                panic!(
                    "Failed to write `{name}` to `{path}`",
                    name = dep_context.full_name(),
                    path = destination_path.to_str().unwrap()
                )
            });
            // fs::copy(dep_context.compiled_file_path(), destination_path).unwrap_or_else(|_| {
            //     panic!("Failed to copy `{name}` to build directory", name = dep_context.full_name())
            // });
        }

        Ok((project_path, stubs))
    }

    // Creates the stub of the program, caches it, and writes the local `leo.lock` file
    pub fn process_local(&mut self, name: Symbol, recursive: bool) -> Result<(), UtilError> {
        let cur_context = self.contexts.get_mut(&name).unwrap();
        // Don't need to disassemble the main file
        if name != self.name {
            // Disassemble the program
            let compiled_path = cur_context.compiled_file_path();
            if !compiled_path.exists() {
                return Err(UtilError::build_file_does_not_exist(compiled_path.to_str().unwrap(), Default::default()));
            }
            let mut file = File::open(compiled_path).unwrap_or_else(|_| {
                panic!("Failed to open file {}", cur_context.compiled_file_path().to_str().unwrap())
            });
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|err| {
                UtilError::util_file_io_error(
                    format!("Could not read {}", cur_context.compiled_file_path().to_str().unwrap()),
                    err,
                    Default::default(),
                )
            })?;

            // Cache the disassembled stub
            let stub: Stub = disassemble_from_str::<N>(&name.to_string(), &content)?;
            if cur_context.add_stub(stub.clone()) {
                Err(UtilError::duplicate_dependency_name_error(stub.stub_id.name.name, Default::default()))?;
            }

            // Cache the hash
            cur_context.add_checksum();

            // Only write lock file when recursive building
            if recursive {
                self.write_lock_file(&name)?;
            }
        } else {
            // Write lock file
            self.write_lock_file(&name)?;
        }

        Ok(())
    }

    // Write lock file
    fn write_lock_file(&self, name: &Symbol) -> Result<(), UtilError> {
        // Add entry for all dependencies
        let mut lock_file: IndexMap<String, Vec<LockFileEntry>> = IndexMap::new();
        let packages: Vec<LockFileEntry> = self
            .get_context(name)
            .post_order()
            .iter()
            .map(|program| {
                let context = self.get_context(program);
                LockFileEntry::from(context)
            })
            .collect();
        lock_file.insert("package".to_string(), packages);

        // Serialize the data to a TOML string
        let toml_str =
            toml::to_string(&lock_file).map_err(|err| UtilError::toml_serizalization_error(err, Default::default()))?;

        // Write the TOML string to a file
        std::fs::write(self.get_context(name).full_path().join("leo.lock"), toml_str).map_err(|err| {
            UtilError::util_file_io_error(
                format!("Could not read {}", self.get_context(name).full_path().join("leo.lock").to_str().unwrap()),
                err,
                Default::default(),
            )
        })?;
        Ok(())
    }
}

// Retrieve local
fn retrieve_local(name: &String, path: &PathBuf) -> Result<Vec<Dependency>, UtilError> {
    // Create the lock file if it doesn't exist
    let lock_path = path.join("leo.lock");
    if !lock_path.exists() {
        std::fs::create_dir_all(path).map_err(|err| {
            UtilError::util_file_io_error(
                format!("Couldn't create directory {}", lock_path.to_str().unwrap()),
                err,
                Default::default(),
            )
        })?;
        File::create(lock_path.clone()).map_err(|err| {
            UtilError::util_file_io_error(
                format!("Couldn't create file {}", lock_path.to_str().unwrap()),
                err,
                Default::default(),
            )
        })?;
    }

    // Open `program.json` which is located at `package_path/program.json`.
    let mut file = File::open(path.join("program.json")).map_err(|err| {
        UtilError::util_file_io_error(
            format!("Could not open path {}", path.join("program.json").to_str().unwrap()),
            err,
            Default::default(),
        )
    })?;

    // Read the file content
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|err| {
        UtilError::util_file_io_error(
            format!("Could not read path {}", path.join("program.json").to_str().unwrap()),
            err,
            Default::default(),
        )
    })?;

    // Deserialize the content into Program
    let program_data: Manifest =
        serde_json::from_str(&content).map_err(|err| UtilError::json_serialization_error(err, Default::default()))?;

    // Throw error in the event of a name mismatch
    if program_data.program() != name {
        Err(UtilError::program_name_mismatch_error(
            program_data.program(),
            name,
            path.to_str().unwrap_or_default(),
            Default::default(),
        ))?;
    }

    let dependencies = match program_data.dependencies() {
        Some(deps) => deps.clone(),
        None => Vec::new(),
    };

    Ok(dependencies)
}

// Retrieve from network
fn retrieve_from_network<N: Network>(
    project_path: &Path,
    home_path: &Path,
    name: &String,
    endpoint: &String,
) -> Result<(Stub, Vec<Dependency>), UtilError> {
    // Get the network being used.
    let network = match N::ID {
        snarkvm::console::network::MainnetV0::ID => NetworkName::MainnetV0,
        snarkvm::console::network::TestnetV0::ID => NetworkName::TestnetV0,
        _ => NetworkName::MainnetV0,
    };

    // Check if the file is already cached in `~/.aleo/registry/{network}/{program}`
    let move_to_path = home_path.join(network.to_string());
    let path = move_to_path.join(name.clone());
    let file_str: String;
    if !path.exists() {
        // Create directories along the way if they don't exist
        std::fs::create_dir_all(&move_to_path).map_err(|err| {
            UtilError::util_file_io_error(
                format!("Could not write path {}", move_to_path.to_str().unwrap()),
                err,
                Default::default(),
            )
        })?;

        // Fetch from network
        println!("Retrieving {name} from {endpoint} on {network}.");
        file_str = fetch_from_network(&format!("{endpoint}/{network}/program/{}", &name))?;
        verify_valid_program::<N>(name, &file_str)?;
        println!("Successfully retrieved {} from {:?}!", name, endpoint);

        // Write file to cache
        std::fs::write(path.clone(), file_str.clone().replace("\\n", "\n")).map_err(|err| {
            UtilError::util_file_io_error(
                format!("Could not open path {}", path.to_str().unwrap()),
                err,
                Default::default(),
            )
        })?;
    } else {
        // Read file from cache
        file_str = fs::read_to_string(path.clone()).map_err(|err| {
            UtilError::util_file_io_error(
                format!("Could not read path {}", path.clone().to_str().unwrap()),
                err,
                Default::default(),
            )
        })?;
    }

    // Copy the file into build directory. We can assume build directory exists because of its initialization in `leo/cli/commands/build.rs`.
    let import_dir = project_path.join("build").join("imports");
    let import_dir_path = import_dir.as_path();
    std::fs::create_dir_all(import_dir_path).map_err(|err| {
        UtilError::util_file_io_error(
            format!("Could not create path {}", import_dir_path.to_str().unwrap()),
            err,
            Default::default(),
        )
    })?;
    let build_location = PathBuf::from(import_dir_path).join(name.clone());
    std::fs::write(build_location.clone(), file_str.clone()).map_err(|err| {
        UtilError::util_file_io_error(
            format!("Could not write to path {}", build_location.to_str().unwrap()),
            err,
            Default::default(),
        )
    })?;

    // Disassemble into Stub
    let stub: Stub = disassemble_from_str::<N>(name, &file_str)?;

    // Create entry for leo.lock
    Ok((
        stub.clone(),
        stub.imports
            .clone()
            .iter()
            .map(|id| {
                Dependency::new(
                    id.name.name.to_string() + "." + id.network.name.to_string().as_str(),
                    Location::Network,
                    Some(network),
                    None,
                )
            })
            .collect(),
    ))
}

// Fetch the given endpoint url and return the sanitized response.
pub fn fetch_from_network(url: &str) -> Result<String, UtilError> {
    let response = ureq::get(url)
        .set(&format!("X-Aleo-Leo-{}", env!("CARGO_PKG_VERSION")), "true")
        .call()
        .map_err(|err| UtilError::failed_to_retrieve_from_endpoint(err, Default::default()))?;
    if response.status() == 200 {
        Ok(response.into_string().unwrap().replace("\\n", "\n").replace('\"', ""))
    } else {
        Err(UtilError::network_error(url, response.status(), Default::default()))
    }
}

// Verify that a fetched program is valid aleo instructions.
pub fn verify_valid_program<N: Network>(name: &str, program: &str) -> Result<(), UtilError> {
    match Program::<N>::from_str(program) {
        Ok(_) => Ok(()),
        Err(_) => Err(UtilError::snarkvm_parsing_error(name, Default::default())),
    }
}
