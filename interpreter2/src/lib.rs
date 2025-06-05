// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::{
    Location,
    interpreter_value::{Address, LeoValue},
};
use leo_errors::Result;
use leo_span::source_map::FileName;

use indexmap::IndexMap;
use std::path::Path;

#[cfg(test)]
mod test_interpreter;

mod core_function_helper;

mod cursor;

mod expression;

mod interpreter;

mod statement;

pub struct FunctionApplication {
    function: Location,
    arguments: Vec<String>,
}

pub fn run_functions<'a>(
    leo_programs: impl IntoIterator<Item = FileName>,
    aleo_programs: impl IntoIterator<Item = FileName>,
    signer: Address,
    block_height: u32,
    applications: impl Iterator<Item = FunctionApplication>,
) -> Result<Vec<Result<LeoValue>>> {
    let mut container = interpreter::Container::default();
    let mut interpreter =
        interpreter::Interpreter::new(&mut container, leo_programs, aleo_programs, signer, block_height)?;

    let mut results = Vec::new();

    for application in applications {
        let parsed = application.arguments.iter().map(|arg| arg.parse().unwrap()).collect::<Vec<LeoValue>>();
        interpreter.initiate_function(application.function, parsed)?;
        let result_value = interpreter.finish_function().map(|opt_value| opt_value.unwrap());
        results.push(result_value);
    }

    Ok(results)
}

// pub fn run_tests(
//     leo_programs: impl IntoIterator<Item = &str>,
//     aleo_programs: impl IntoIterator<Item = &str>,
//     signer: Address,
//     block_height: u32,
//     match_str: &str,
// ) -> Result<IndexMap<Location, Result<()>>> {
//     todo!()
// }

// pub fn interactive_interpreter(
//     leo_filenames: impl IntoIterator<Item = impl AsRef<Path>>,
//     aleo_filenames: impl IntoIterator<Item = impl AsRef<Path>>,
//     signer: Address,
//     block_height: u32,
//     tui: bool,
// ) -> Result<()> {
//     todo!()
// }
