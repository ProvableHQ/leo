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
use sha2::{Digest, Sha256};
use std::{
    env,
    fmt,
    fs,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

const ALEO_EXPLORER_URL: &str = "https://api.explorer.aleo.org/v1";

// Struct representation of program's `program.json` specification
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgramSpecification {
    program: String,
    version: String,
    description: String,
    license: String,
    dependencies: Option<Vec<Program>>,
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
struct Program {
    name: String,
    location: Location,
    network: Network,
}

// Contents of a leo.lock entry for a program
#[derive(Debug, Clone)]
struct LockContents {
    dependencies: Vec<Program>,
    checksum: String,
}

// Retriever is responsible for retrieving external programs
pub struct Retriever {
    programs: Vec<Program>,
    path: PathBuf,
    lock_file: IndexMap<Program, LockContents>,
    stubs: IndexMap<Symbol, Stub>,
    dependency_graph: DiGraph<Symbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockFileEntry {
    name: String,
    network: Network,
    location: Location,
    checksum: String,
    dependencies: Vec<String>,
}

impl Retriever {
    // Initialize a new Retriever
    pub fn new(path: &Path) -> Result<Self, UtilError> {
        let lock_path = path.to_path_buf().join("leo.lock");
        if !lock_path.exists() {
            std::fs::create_dir_all(path).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?; // TODO: How to get rid of requirement for span?
            File::create(lock_path.clone()).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
        }

        let mut file = File::open(lock_path).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

        // Read `leo.lock` into a string, and deserialize from TOML to a `LockFile` struct.
        let mut lock_file_contents = String::new();
        file.read_to_string(&mut lock_file_contents)
            .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
        let parsed_lock_file: IndexMap<String, Vec<LockFileEntry>> = toml::from_str(&lock_file_contents)
            .map_err(|err| UtilError::toml_serizalization_error(err, Default::default()))?;

        // Construct a mapping of all programs in the `leo.lock` file to their specification.
        let mut lock_file_map = IndexMap::new();
        match parsed_lock_file.get("package") {
            None => (),
            Some(packages) => {
                for package in packages {
                    let program = Program {
                        name: package.name.clone(),
                        location: package.location.clone(),
                        network: package.network.clone(),
                    };
                    let lock_content = LockContents {
                        // Assign the dependency location and network to match the program's
                        dependencies: package
                            .dependencies
                            .clone()
                            .into_iter()
                            .map(|name| Program {
                                name,
                                location: package.location.clone(),
                                network: package.network.clone(),
                            })
                            .collect(),
                        checksum: package.checksum.clone(),
                    };
                    lock_file_map.insert(program, lock_content);
                }
            }
        }

        // Open `program.json` which is located at `package_path/program.json`.
        let mut file = File::open(path.join("program.json"))
            .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

        // Read the file content
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

        // Deserialize the content into Program
        let program_data: ProgramSpecification = serde_json::from_str(&content)
            .map_err(|err| UtilError::json_serialization_error(err, Default::default()))?;

        let dependencies = match program_data.dependencies {
            Some(deps) => deps,
            None => Vec::new(),
        };

        Ok(Self {
            programs: dependencies,
            path: path.to_path_buf(),
            stubs: IndexMap::new(),
            lock_file: lock_file_map,
            dependency_graph: DiGraph::new(IndexSet::new()),
        })
    }

    // Retrieve all dependencies for a program
    pub fn retrieve(&mut self) -> Result<IndexMap<Symbol, Stub>, UtilError> {
        let mut programs_to_retrieve = self.programs.clone();
        let mut explored: IndexSet<Program> = IndexSet::new();
        let mut solo_programs: IndexSet<Symbol> = IndexSet::new();

        while !programs_to_retrieve.is_empty() {
            let (mut results, mut dependencies) = (Vec::new(), Vec::new());
            // Visit all programs
            for program in programs_to_retrieve.iter() {
                match program.location {
                    Location::Network => results.push(retrieve_from_network(
                        self.path.clone(),
                        program.name.clone(),
                        program.network.clone(),
                    )?),
                    Location::Git => panic!("Location::Git is not supported yet"),
                    Location::Local => panic!("Location::Local is not supported yet"),
                }

                // Mark as visited
                if !explored.insert(program.clone()) {
                    Err(UtilError::circular_dependency_error(Default::default()))?;
                }
            }

            for (stub, program, entry) in results {
                // Add dependencies to list of dependencies
                entry.dependencies.clone().iter().for_each(|dep| {
                    if !explored.contains(dep) {
                        dependencies.push(dep.clone());
                        // Trim off `.aleo` from end of the program names to be consistent with formatting in AST
                        self.dependency_graph.add_edge(
                            Symbol::intern(&program.name.clone()[..program.name.len() - 5]),
                            Symbol::intern(&dep.name.clone()[..dep.name.len() - 5]),
                        );
                    }
                });

                // Add programs that do not have any dependencies to list of solo programs since they are not being added to dependency graph
                if entry.dependencies.is_empty() {
                    solo_programs.insert(Symbol::intern(&program.name.clone()[..program.name.len() - 5]));
                }

                // Add stub to list of stubs
                if let Some(existing) = self.stubs.insert(stub.stub_id.name.name, stub.clone()) {
                    Err(UtilError::duplicate_dependency_name_error(existing.stub_id.name.name, Default::default()))?;
                }

                // Update lock file
                self.lock_file.insert(program, entry);
            }

            programs_to_retrieve = dependencies;
        }

        // Write the finalized dependency information to `leo.lock`
        self.write_lock_file()?;

        // Check for dependency cycles
        match self.dependency_graph.post_order() {
            Ok(order) => {
                // Collect all the stubs in the order specified by the dependency graph
                let mut stubs: IndexMap<Symbol, Stub> = order
                    .iter()
                    .map(|id| match self.stubs.get(id) {
                        Some(s) => (*id, s.clone()),
                        None => panic!("Stub {id} not found"),
                    })
                    .collect();

                // Add all the stubs that do not have any dependencies
                solo_programs.iter().for_each(|id| {
                    match self.stubs.get(id) {
                        Some(s) => {
                            // Note that some programs will be added in twice if they are a dependency of another program but have no dependencies themselves.
                            stubs.insert(*id, s.clone());
                        }
                        None => panic!("Stub {id} not found"),
                    };
                });
                Ok(stubs)
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
                checksum: entry.checksum.clone(),
                dependencies: entry.dependencies.iter().map(|dep| dep.name.clone()).collect(),
            })
            .collect();
        lock_file.insert("package".to_string(), packages);

        // Serialize the data to a TOML string
        let toml_str =
            toml::to_string(&lock_file).map_err(|err| UtilError::toml_serizalization_error(err, Default::default()))?;

        // Write the TOML string to a file
        std::fs::write(self.path.join("leo.lock"), toml_str)
            .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;
        Ok(())
    }
}

// Retrieve from network
fn retrieve_from_network(
    project_path: PathBuf,
    name: String,
    network: Network,
) -> Result<(Stub, Program, LockContents), UtilError> {
    // Check if the file is already cached in `~/.aleo/registry/{network}/{program}`
    let registry_directory = &format!("{}/.aleo/registry/{}", env::var("HOME").unwrap(), network);
    let path_str = &format!("{}/{}", registry_directory, name);
    let path = Path::new(&path_str);
    let mut file_str: String;
    if !path.exists() {
        // Create directories along the way if they don't exist
        std::fs::create_dir_all(Path::new(&registry_directory))
            .map_err(|err| UtilError::util_file_io_error(err, Default::default()))?;

        // TODO: Refactor this so that we do the match statement here (instead of in `Retriever::retrieve()`)
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

    // Hash the file contents
    let mut hasher = Sha256::new();
    hasher.update(file_str.as_bytes());
    let hash = hasher.finalize();

    // Disassemble into Stub
    let stub: Stub = disassemble_from_str(file_str)?;

    // Create entry for leo.lock
    Ok((
        stub.clone(),
        Program { name: name.clone(), location: Location::Network, network: network.clone() },
        LockContents {
            dependencies: stub
                .imports
                .clone()
                .iter()
                .map(|id| Program {
                    name: id.name.name.to_string() + "." + id.network.name.to_string().as_str(),
                    location: Location::Network,
                    network: network.clone(),
                })
                .collect(),
            checksum: format!("{hash:x}"),
        },
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
        // Set $HOME to tmp directory so that tests do not modify users real home directory
        let original_home = env::var("HOME").unwrap();
        env::set_var("HOME", "../tmp");

        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/nested";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let mut retriever = Retriever::new(&build_dir).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
        });

        // Reset $HOME
        env::set_var("HOME", original_home);
    }
    #[test]
    #[ignore]
    fn simple_dir_test() {
        // Set $HOME to tmp directory so that tests do not modify users real home directory
        let original_home = env::var("HOME").unwrap();
        env::set_var("HOME", "../tmp");

        // Test pulling nested dependencies from network
        const BUILD_DIRECTORY: &str = "../tmp/simple";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let mut retriever = Retriever::new(&build_dir).expect("Failed to build retriever");
            retriever.retrieve().expect("failed to retrieve");
        });

        // Reset $HOME
        env::set_var("HOME", original_home);
    }
}
