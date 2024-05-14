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

pub mod location;
pub use location::Location;

pub mod dependency;
pub use dependency::*;

pub mod lock_file_entry;
pub use lock_file_entry::*;

pub mod manifest;
pub use manifest::*;

pub mod network_name;
pub use network_name::*;

use leo_ast::Stub;
use leo_span::Symbol;

use indexmap::IndexSet;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct ProgramContext {
    name: Symbol,
    full_name: String,
    location: Location,
    network: Option<NetworkName>,
    path: Option<PathBuf>,
    full_path: Option<PathBuf>,
    compiled_file_path: Option<PathBuf>,
    dependencies: Option<Vec<Symbol>>,
    checksum: Option<String>,
    stub: Option<Stub>,
    post_order: Option<IndexSet<Symbol>>,
}

impl ProgramContext {
    pub fn new_main(name: Symbol, path: PathBuf, dependencies: Vec<Dependency>) -> Self {
        Self {
            name,
            full_name: format!("{}.aleo", name),
            location: Location::Local,
            network: None,
            path: Some(path.clone()),
            full_path: Some(path.clone()),
            compiled_file_path: Some(path.join("build/main.aleo")),
            dependencies: Some(dependencies.iter().map(Symbol::from).collect()),
            checksum: None,
            stub: None,
            post_order: None,
        }
    }

    // Method to extract 'name'
    pub fn name(&self) -> &Symbol {
        &self.name
    }

    // Method to extract 'name_with_network'
    pub fn full_name(&self) -> &String {
        &self.full_name
    }

    // Method to extract 'network', panics if `None`. Only safe to access if location is 'Network'
    pub fn network(&self) -> &NetworkName {
        self.network.as_ref().expect("ProgramContext network is None")
    }

    // Method to extract 'location'
    pub fn location(&self) -> &Location {
        &self.location
    }

    // Method to extract 'path', panics if `None`. Only safe to access if location is 'Local'.
    pub fn path(&self) -> &PathBuf {
        self.path.as_ref().expect("ProgramContext path is None")
    }

    // Method to extract 'full_path', panics if `None`. Only safe to access if location is 'Local'.
    pub fn full_path(&self) -> &PathBuf {
        self.full_path.as_ref().expect("ProgramContext full_path is None")
    }

    // Method to add 'full_path'.
    pub fn add_full_path(&mut self, full_path: &Path) {
        self.full_path = Some(PathBuf::from(full_path));
    }

    pub fn compiled_file_path(&self) -> &PathBuf {
        self.compiled_file_path.as_ref().expect("ProgramContext compiled_file_path is None")
    }

    pub fn add_compiled_file_path(&mut self, path: &Path) {
        self.compiled_file_path = Some(PathBuf::from(path));
    }

    // Method to extract 'checksum'.
    pub fn checksum(&self) -> &String {
        self.checksum.as_ref().expect("ProgramContext checksum is None")
    }

    // Method to add 'checksum'.
    pub fn add_checksum(&mut self) {
        if let Some(_c) = &self.checksum {
        } else {
            let file_str = std::fs::read_to_string(self.compiled_file_path()).expect("Unable to read file");
            let mut hasher = Sha256::new();
            hasher.update(file_str.as_bytes());
            let hash = hasher.finalize();

            // Convert the hash to a hexadecimal string
            let mut hash_str = String::new();
            for byte in hash {
                write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
            }

            self.checksum = Some(hash_str.clone());
        }
    }

    // Method to add 'stub'
    pub fn add_stub(&mut self, stub: Stub) -> bool {
        if self.stub.is_some() {
            return true;
        }
        self.stub = Some(stub);
        false
    }

    // Method to extract 'stub', panics if `None`. Safe after retrieve() for Network, and process_local() for Local
    pub fn stub(&self) -> &Stub {
        self.stub.as_ref().unwrap_or_else(|| panic!("{} has no stub set", self.name))
    }

    // Method to extract 'dependencies', panics if `None`
    pub fn dependencies(&self) -> Vec<Symbol> {
        self.dependencies.as_ref().unwrap_or_else(|| panic!("{}'s dependencies are not set", self.full_name)).clone()
    }

    // Method to add 'dependencies'
    pub fn add_dependencies(&mut self, dependencies: Vec<Symbol>) {
        self.dependencies = Some(dependencies);
    }

    // Method to extract 'post_order', panics if `None`
    pub fn post_order(&self) -> &IndexSet<Symbol> {
        self.post_order.as_ref().unwrap_or_else(|| panic!("{}'s post_order is None", self.full_name()))
    }

    pub fn add_post_order(&mut self, post_order: IndexSet<Symbol>) {
        self.post_order = Some(post_order);
    }
}

impl From<Dependency> for ProgramContext {
    fn from(dependency: Dependency) -> Self {
        Self {
            name: Symbol::from(&dependency),
            full_name: dependency.name().clone(),
            location: dependency.location().clone(),
            network: *dependency.network(),
            path: dependency.path().clone(),
            full_path: None,
            compiled_file_path: None,
            dependencies: None,
            checksum: None,
            stub: None,
            post_order: None,
        }
    }
}
