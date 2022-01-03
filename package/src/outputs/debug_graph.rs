// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::PackageFile;

use serde::Deserialize;
use std::fmt;

/// Enum to handle all 3 types of snapshots.
#[derive(Deserialize)]
pub enum DebugGraph {
    Initial,
    ConstantsFolded,
    DeadCodeEliminated,
}

impl fmt::Display for DebugGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Initial => "initial_asg.dot",
                Self::ConstantsFolded => "constants_folded_asg.dot",
                Self::DeadCodeEliminated => "dead_code_eliminated_asg.dot",
            }
        )
    }
}

pub static ASG_DEBUG_GRAPH_FILE_EXTENSION: &str = ".dot";

/// Generic Snapshot file wrapper. Each package can have up to 3
/// different snapshots: initial_ast, canonicalization_ast and type_inferenced_ast;
#[derive(Deserialize)]
pub struct DebugGraphFile {
    pub package_name: String,
    pub debug_graph: DebugGraph,
}

impl PackageFile for DebugGraphFile {
    type ParentDirectory = super::OutputsDirectory;

    fn template(&self) -> String {
        unimplemented!("Debug graph files don't have templates.");
    }
}

impl std::fmt::Display for DebugGraphFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug_graph)
    }
}

impl DebugGraphFile {
    pub fn new(package_name: &str, debug_graph: DebugGraph) -> Self {
        Self {
            package_name: package_name.to_string(),
            debug_graph,
        }
    }
}
