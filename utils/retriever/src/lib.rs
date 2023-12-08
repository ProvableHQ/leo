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

use disassembler::disassemble_from_str;
use indexmap::{IndexMap, IndexSet};
use leo_ast::Stub;
use leo_errors::UtilError;
use leo_passes::{common::DiGraph, DiGraphError};
use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{fmt, fs, fs::File, io::Read, path::PathBuf};

const ALEO_EXPLORER_URL: &str = "https://api.explorer.aleo.org/v1";

// Struct representation of program's `program.json` specification
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgramSpecification {
    program: String,
    version: String,
    description: String,
    license: String,
    dependencies: Option<Vec<ProgramContext>>,
}

// Retrievable locations for an external program
#[derive(Debug, Clone, std::cmp::Eq, PartialEq, Hash, Serialize, Deserialize)]
enum Location {
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "git")]
    Git,
}

// Retrievable networks for an external program
#[derive(Debug, Clone, std::cmp::Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "testnet3")]
    Testnet3,
    #[serde(rename = "mainnet")]
    Mainnet,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Network::Testnet3 => write!(f, "testnet3"),
            Network::Mainnet => write!(f, "mainnet"),
        }
    }
}

// Information required to retrieve external program
#[derive(Debug, Clone, std::cmp::Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProgramContext {
    name: String,
    location: Location,
    network: Option<Network>,
    path: Option<PathBuf>,
}

impl From<&ProgramContext> for Symbol {
    fn from(context: &ProgramContext) -> Self {
        Symbol::intern(&context.name.clone()[..context.name.len() - 5])
    }
}

// Contents of a leo.lock entry for a program
#[derive(Debug, Clone)]
struct LockContents {
    dependencies: Vec<ProgramContext>,
    checksum: String,
}

// Retriever is responsible for retrieving external programs
pub struct Retriever {
    initial_dependencies: Vec<ProgramContext>,
    program_map: IndexMap<Symbol, ProgramContext>,
    local_paths: IndexMap<Symbol, PathBuf>,
    project_path: PathBuf,
    registry_path: PathBuf,
    lock_file: IndexMap<ProgramContext, LockContents>,
    stubs: IndexMap<Symbol, Stub>,
    dependency_graph: DiGraph<Symbol>,
    pub name: Symbol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockFileEntry {
    name: String,
    network: Option<Network>,
    location: Location,
    path: Option<PathBuf>,
    checksum: String,
    dependencies: Vec<String>,
}

impl Retriever {
    // Initialize a new Retriever
    pub fn new(name: Symbol, path: &PathBuf, home: &PathBuf) -> Result<Self, UtilError> {
        // Initialize the local paths of all direct dependencies
        let dependencies = retrieve_local(path.clone())?;
        let mut local_paths = IndexMap::from([(name, path.clone())]);
        for dep in &dependencies {
            if dep.location == Location::Local {
                match &dep.path {
                    Some(p) => {
                        local_paths.insert(Symbol::from(dep), path.clone().join(p.clone()));
                    }
                    None => return Err(UtilError::missing_path_error(dep.name.clone(), Default::default()))?,
                }
            }
        }
        Ok(Self {
            name,
            initial_dependencies: dependencies,
            program_map: IndexMap::new(),
            local_paths,
            project_path: path.clone(),
            registry_path: home.clone(),
            stubs: IndexMap::new(),
            lock_file: IndexMap::new(),
            dependency_graph: DiGraph::new(IndexSet::new()),
        })
    }

    // Retrieve all dependencies for a program
    pub fn retrieve(&mut self) -> Result<Vec<ProgramContext>, UtilError> {
        let mut programs_to_retrieve = self.initial_dependencies.clone();
        let mut explored: IndexSet<Symbol> = IndexSet::new();
        let mut solo_programs: IndexSet<Symbol> = IndexSet::new();

        // Loop retrieving all nested dependencies for the current set of dependencies
        // Only adding non-duplicates to be searched in the future
        while !programs_to_retrieve.is_empty() {
            let mut dependencies: Vec<ProgramContext> = Vec::new();
            // Visit all programs
            for program in programs_to_retrieve.iter() {
                // Split into cases based on network dependency or local dependency
                let nested_dependencies = match program.location {
                    Location::Network => {
                        let network = match &program.network {
                            Some(n) => n,
                            None => {
                                return Err(UtilError::missing_network_error(program.name.clone(), Default::default()))?;
                            }
                        };
                        let (stub, nested_dependencies) = retrieve_from_network(
                            self.project_path.clone(),
                            self.registry_path.clone(),
                            program.name.clone(),
                            network.clone(),
                        )?;

                        // Cache the stubs
                        if let Some(existing) = self.stubs.insert(stub.stub_id.name.name, stub.clone()) {
                            Err(UtilError::duplicate_dependency_name_error(
                                existing.stub_id.name.name,
                                Default::default(),
                            ))?;
                        }

                        nested_dependencies
                    }
                    Location::Local => {
                        let cur_path = self
                            .local_paths
                            .get(&Symbol::from(program))
                            .expect("Local path must have been processed before its dependency")
                            .clone();
                        let nested_local_deps = retrieve_local(cur_path.clone())?;

                        // Append child's local path to parent's local path, and add to mapping
                        for nested_dep in &nested_local_deps {
                            match nested_dep.location {
                                Location::Local => match &nested_dep.path {
                                    Some(p) => {
                                        self.local_paths
                                            .insert(Symbol::from(nested_dep), cur_path.clone().join(p.clone()));
                                    }
                                    None => {
                                        return Err(UtilError::missing_path_error(
                                            nested_dep.name.clone(),
                                            Default::default(),
                                        ))?;
                                    }
                                },
                                _ => (),
                            };
                        }

                        nested_local_deps
                    }
                    Location::Git => panic!("Location::Git is not supported yet"),
                };

                // Mark as visited
                let sym = Symbol::from(program);
                if !explored.insert(sym) {
                    panic!("Should never visit same dependency twice");
                }

                // Add (Symbol, Program) to map
                self.program_map.insert(sym, program.clone());

                for dep in &nested_dependencies {
                    let dep_sym = Symbol::from(dep);

                    // Don't add a new dependency to check if it has already been processed, or will be processed in the future
                    if !explored.contains(&dep_sym)
                        && !dependencies.contains(dep)
                        && !programs_to_retrieve.contains(dep)
                    {
                        dependencies.push(dep.clone());
                    }

                    // Update dependecy graph
                    self.dependency_graph.add_edge(sym, dep_sym);
                }

                // Add programs that do not have any dependencies to list of solo programs since they are not being added to dependency graph
                if nested_dependencies.is_empty() {
                    solo_programs.insert(sym);
                }
            }

            programs_to_retrieve = dependencies;
        }

        // Compute post order of dependency graph
        match self.dependency_graph.post_order() {
            Ok(mut order) => {
                // Collect all solo programs that are not already in ordered list
                solo_programs.iter().for_each(|id| {
                    match self.program_map.get(id) {
                        Some(_p) => {
                            // Some of the programs w/o dependencies are still dependencies to other programs, so can't move them into wrong order.
                            if !order.contains(id) {
                                order.insert(*id);
                            }
                        }
                        None => panic!("Program {id} not found"),
                    };
                });

                // Collect list of ProgramContexts in post order, filtering out all network dependencies
                Ok(order
                    .iter()
                    .map(|id| match self.program_map.get(id) {
                        Some(p) => p.clone(),
                        None => panic!("Program {id} not found"),
                    })
                    .filter(|p| p.location == Location::Local)
                    .collect())
            }
            Err(DiGraphError::CycleDetected(_)) => Err(UtilError::circular_dependency_error(Default::default()))?,
        }
    }

    // Write lock file
    fn write_lock_file(&self) -> Result<(), UtilError> {
        // Create struct representation of lock file
        let mut lock_file: IndexMap<String, Vec<LockFileEntry>> = IndexMap::new();
        let packages: Vec<LockFileEntry> = self
            .lock_file
            .iter()
            .map(|(program, entry)| LockFileEntry {
                name: program.name.clone(),
                network: program.network.clone(),
                location: program.location.clone(),
                path: program.path.clone(),
                checksum: entry.checksum.clone(),
                dependencies: entry.dependencies.iter().map(|dep| dep.name.clone()).collect(),
            })
            .collect();
        lock_file.insert("package".to_string(), packages);

        // Serialize the data to a TOML string
        let toml_str =
            toml::to_string(&lock_file).map_err(|err| UtilError::toml_serizalization_error(err, Default::default()))?;

        // Write the TOML string to a file
        std::fs::write(self.project_path.join("leo.lock"), toml_str)
            .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
        Ok(())
    }
}

// Retrieve local
fn retrieve_local(path: PathBuf) -> Result<Vec<ProgramContext>, UtilError> {
    // Create the lock file if it doesn't exist
    let lock_path = path.join("leo.lock");
    if !lock_path.exists() {
        std::fs::create_dir_all(&path).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?; // TODO: How to get rid of requirement for span?
        File::create(lock_path.clone()).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
    }

    // Open `program.json` which is located at `package_path/program.json`.
    let mut file =
        File::open(path.join("program.json")).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

    // Read the file content
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

    // Deserialize the content into Program
    let program_data: ProgramSpecification =
        serde_json::from_str(&content).map_err(|err| UtilError::json_serialization_error(err, Default::default()))?;

    let dependencies = match program_data.dependencies {
        Some(deps) => deps,
        None => Vec::new(),
    };

    Ok(dependencies)
}

// Retrieve from network
fn retrieve_from_network(
    project_path: PathBuf,
    home_path: PathBuf,
    name: String,
    network: Network,
) -> Result<(Stub, Vec<ProgramContext>), UtilError> {
    // Check if the file is already cached in `~/.aleo/registry/{network}/{program}`
    let move_to_path = home_path.join(format!("{network}"));
    let path = move_to_path.join(name.clone());
    let mut file_str: String;
    if !path.exists() {
        // Create directories along the way if they don't exist
        std::fs::create_dir_all(&move_to_path).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

        // Fetch from network
        println!("Retrieving {} from {:?}.", name.clone(), network.clone());
        file_str = fetch_from_network(name.clone(), network.clone())?;
        file_str = file_str.replace("\\n", "\n").replace('\"', "");
        println!("Successfully retrieved {} from {:?}!", name, network);

        // Write file to cache
        std::fs::write(path, file_str.clone().replace("\\n", "\n"))
            .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
    } else {
        // Read file from cache
        file_str = fs::read_to_string(path).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
    }

    // Copy the file into build directory. We can assume build directory exists because of its initialization in `leo/cli/commands/build.rs`.
    let import_dir = project_path.join("build/imports");
    let import_dir_path = import_dir.as_path();
    std::fs::create_dir_all(import_dir_path).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
    let build_location = PathBuf::from(import_dir_path).join(name.clone());
    std::fs::write(build_location, file_str.clone())
        .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

    // Disassemble into Stub
    let stub: Stub = disassemble_from_str(file_str)?;

    // Create entry for leo.lock
    Ok((
        stub.clone(),
        stub.imports
            .clone()
            .iter()
            .map(|id| ProgramContext {
                name: id.name.name.to_string() + "." + id.network.name.to_string().as_str(),
                location: Location::Network,
                network: Some(network.clone()),
                path: None,
            })
            .collect(),
    ))
}

fn fetch_from_network(program: String, network: Network) -> Result<String, UtilError> {
    let url = format!("{}/{}/program/{}", ALEO_EXPLORER_URL, network.clone(), program);
    let response = ureq::get(&url.clone()).call().unwrap();
    if response.status() == 200 {
        Ok(response.into_string().unwrap())
    } else {
        Err(UtilError::network_error(url, response.status(), Default::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::symbol::create_session_if_not_set_then;

    #[test]
    #[ignore]
    fn temp_dir_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/nested";
        const HOME_DIRECTORY: &str = "../tmp/.aleo/registry";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let home_dir = PathBuf::from(HOME_DIRECTORY);
            let mut retriever =
                Retriever::new(Symbol::intern("nested"), &build_dir, &home_dir).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
        });
    }
    #[test]
    #[ignore]
    fn simple_dir_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/simple";
        const HOME_DIRECTORY: &str = "../tmp/.aleo/registry";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let home_dir = PathBuf::from(HOME_DIRECTORY);
            let mut retriever =
                Retriever::new(Symbol::intern("simple"), &build_dir, &home_dir).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
        });
    }

    #[test]
    #[ignore]
    fn local_dir_test() {
        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/local_test";
        const HOME_DIRECTORY: &str = "../tmp/.aleo/registry";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let home_dir = PathBuf::from(HOME_DIRECTORY);
            let mut retriever =
                Retriever::new(Symbol::intern("local_test"), &build_dir, &home_dir).expect("Failed to build retriever");
            let deps = retriever.retrieve().expect("failed to retrieve");
            dbg!(deps);
            dbg!(retriever.local_paths);
        });
    }
}
