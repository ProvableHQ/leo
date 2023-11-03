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
use leo_passes::{common::DiGraph, DiGraphError};
use leo_span::{symbol::create_session_if_not_set_then, Symbol};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fmt,
    fs,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

const ALEO_EXPLORER_URL: &str = "https://api.explorer.aleo.org/v1";
const ALEO_REGISTRY_DIRECTORY: &str = "../tmp/.aleo";

// Struct representation of program's `program.json` specification
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgramSpecification {
    program: String,
    version: String,
    description: String,
    license: String,
    dependencies: Vec<Program>,
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
    explored: IndexSet<Program>,
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
    pub fn new(path: &Path) -> Self {
        let lock_path = path.to_path_buf().join("leo.lock");
        let mut file = File::open(lock_path).expect("Failed to open `leo.lock`.");

        // Read `leo.lock` into a string, and deserialize from TOML to a `LockFile` struct.
        let mut lock_file_contents = String::new();
        file.read_to_string(&mut lock_file_contents).expect("Failed to read `leo.lock`.");
        let parsed_lock_file: IndexMap<String, Vec<LockFileEntry>> =
            toml::from_str(&lock_file_contents).expect("Failed to parse `leo.lock`.");

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
        let mut file = File::open(path.join("program.json")).expect("Failed to open `program.json`.");

        // Read the file content
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read `program.json`.");

        // Deserialize the content into Program
        let program_data: ProgramSpecification =
            serde_json::from_str(&content).expect("Failed to deserialize `program.json`.");

        Self {
            programs: program_data.dependencies,
            path: path.to_path_buf(),
            stubs: IndexMap::new(),
            lock_file: lock_file_map,
            explored: IndexSet::new(),
            dependency_graph: DiGraph::new(IndexSet::new()),
        }
    }

    // Retrieve all dependencies for a program
    pub fn retrieve(&mut self) -> IndexMap<Symbol, Stub> {
        let mut programs_to_retrieve = self.programs.clone();

        while !programs_to_retrieve.is_empty() {
            let (mut results, mut dependencies) = (Vec::new(), Vec::new());
            // Visit all programs
            for program in programs_to_retrieve.iter() {
                match program.location {
                    Location::Network => results.push(retrieve_from_network(
                        self.path.clone(),
                        program.name.clone(),
                        program.network.clone(),
                    )),
                    Location::Git => panic!("Location::Git is not supported yet"),
                    Location::Local => panic!("Location::Local is not supported yet"),
                }

                // Mark as visited
                if !self.explored.insert(program.clone()) {
                    panic!("Should not ever explore same dependency twice.")
                }
            }

            for (stub, program, entry) in results {
                // Add dependencies to list of dependencies
                entry.dependencies.clone().iter().for_each(|dep| {
                    if !self.explored.contains(dep) {
                        dependencies.push(dep.clone());
                        self.dependency_graph
                            .add_edge(Symbol::intern(&program.name.clone()), Symbol::intern(&dep.name.clone()));
                    }
                });

                // Add stub to list of stubs
                if let Some(existing) = self.stubs.insert(stub.stub_id.name.name, stub.clone()) {
                    panic!(
                        "Should never be creating two stubs from the same program name. Existing: {:?}, New: {:?}",
                        existing,
                        stub.clone()
                    )
                }

                // Update lock file
                self.lock_file.insert(program, entry);
            }

            programs_to_retrieve = dependencies;
        }

        // Check for dependency cycles
        match self.dependency_graph.post_order() {
            Ok(_) => (),
            Err(DiGraphError::CycleDetected(cycle)) => panic!("Dependency cycle detected: {:?}", cycle),
        }

        // Write the finalized dependency information to `leo.lock`
        self.write_lock_file();
        self.stubs.clone()
    }

    // Write lock file
    fn write_lock_file(&self) {
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
        let toml_str = toml::to_string(&lock_file).expect("Failed to serialize lock file.");

        // Write the TOML string to a file
        std::fs::write(self.path.join("leo.lock"), toml_str).expect("Failed to write file to leo.lock");
    }
}

// Retrieve from network
fn retrieve_from_network(project_path: PathBuf, name: String, network: Network) -> (Stub, Program, LockContents) {
    // Check if the file is already cached in `~/.aleo/registry/{network}/{program}`
    let path_str = &format!("{}/{}/{}", ALEO_REGISTRY_DIRECTORY, network, name);
    let path = Path::new(&path_str);
    let mut file_str: String;
    if !path.exists() {
        // Create directories along the way if they don't exist
        std::fs::create_dir_all(Path::new(&format!("{}/{}", ALEO_REGISTRY_DIRECTORY, network)))
            .expect("Failed to create directory `~/.aleo/registry/{network}`.");

        // Fetch from network
        println!("Retrieving {} from {:?}.", name.clone(), network.clone());
        file_str = fetch_from_network(name.clone(), network.clone());
        file_str = file_str.replace("\\n", "\n").replace('\"', "");
        println!("Successfully retrieved {} from {:?}!", name, network);

        // Write file to cache
        std::fs::write(path, file_str.clone().replace("\\n", "\n")).expect("Failed to write file to ~/.aleo");
    } else {
        // Read file from cache
        file_str = fs::read_to_string(path).expect("Failed to read file.");
    }

    // Copy the file into build directory. We can assume build directory exists because of its initialization in `leo/cli/commands/build.rs`.
    let import_dir = project_path.join("build/imports");
    let import_dir_path = import_dir.as_path();
    std::fs::create_dir_all(import_dir_path).expect("Failed to create directory `~/.aleo/registry/{network}`.");
    let build_location = PathBuf::from(import_dir_path).join(name.clone());
    std::fs::write(build_location, file_str.clone()).expect("Failed to write file to build/imports.");

    // Hash the file contents
    let mut hasher = Sha256::new();
    hasher.update(file_str.as_bytes());
    let hash = hasher.finalize();

    // Disassemble into Stub
    let stub: Stub = disassemble_from_str(file_str);

    // Create entry for leo.lock
    (
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
    )
}

fn fetch_from_network(program: String, network: Network) -> String {
    let url = format!("{}/{}/program/{}", ALEO_EXPLORER_URL, network.clone(), program);
    let response = ureq::get(&url.clone()).call().unwrap();
    if response.status() == 200 { response.into_string().unwrap() } else { panic!("Failed to fetch from {url}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        const BUILD_DIRECTORY: &str = "../tmp/project";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let mut retriever = Retriever::new(&build_dir);
            retriever.retrieve();
        });
    }

    #[test]
    fn simple_test() {
        const BUILD_DIRECTORY: &str = "../tmp/simple";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let mut retriever = Retriever::new(&build_dir);
            retriever.retrieve();
        });
    }
    #[test]
    fn super_simple_test() {
        dbg!(std::env::current_dir().unwrap());
        const BUILD_DIRECTORY: &str = "../tmp/super_simple";
        create_session_if_not_set_then(|_| {
            let build_dir = PathBuf::from(BUILD_DIRECTORY);
            let mut retriever = Retriever::new(&build_dir);
            retriever.retrieve();
        });
    }
}
