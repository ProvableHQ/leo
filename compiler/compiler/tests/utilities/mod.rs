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

use leo_compiler::{Compiler, OutputOptions};
use leo_errors::{
    emitter::{Buffer, Emitter, Handler},
    LeoError, LeoWarning,
};
use leo_span::{source_map::FileName};
use leo_test_framework::{
    Test,
};
use leo_passes::{CodeGenerator, Pass};

use snarkvm::prelude::*;

use serde_yaml::Value;
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

pub fn new_compiler(handler: &Handler, main_file_path: PathBuf) -> Compiler<'_> {
    let output_dir = PathBuf::from("/tmp/output/");
    fs::create_dir_all(output_dir.clone()).unwrap();

    Compiler::new(
        String::from("test"),
        String::from("aleo"),
        handler,
        main_file_path,
        output_dir,
        Some(OutputOptions {
            spans_enabled: false,
            initial_input_ast: true,
            initial_ast: true,
            unrolled_ast: true,
            ssa_ast: true,
            flattened_ast: true,
        }),
    )
}

pub fn parse_program<'a>(
    handler: &'a Handler,
    program_string: &str,
    cwd: Option<PathBuf>,
) -> Result<Compiler<'a>, LeoError> {
    let mut compiler = new_compiler(handler, cwd.clone().unwrap_or_else(|| "compiler-test".into()));
    let name = cwd.map_or_else(|| FileName::Custom("compiler-test".into()), FileName::Real);
    compiler.parse_program_from_string(program_string, name)?;

    Ok(compiler)
}

pub fn hash_content(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize();

    format!("{hash:x}")
}

pub fn hash_file(path: &str) -> String {
    let file = fs::read_to_string(Path::new(path)).unwrap();
    hash_content(&file)
}

/// Get the path of the `input_file` given in `input` into `list`.
pub fn get_input_file_paths(list: &mut Vec<PathBuf>, test: &Test, input: &Value) {
    let input_file: PathBuf = test.path.parent().expect("no test parent dir").into();
    if input.as_str().is_some() {
        let mut input_file = input_file;
        input_file.push(input.as_str().expect("input_file was not a string or array"));
        list.push(input_file.clone());
    } else if let Some(seq) = input.as_sequence() {
        for name in seq {
            let mut input_file = input_file.clone();
            input_file.push(name.as_str().expect("input_file was not a string"));
            list.push(input_file.clone());
        }
    }
}

/// Collect and return all inputs, if possible.
pub fn collect_all_inputs(test: &Test) -> Result<Vec<PathBuf>, String> {
    let mut list = vec![];

    if let Some(input) = test.config.get("input_file") {
        get_input_file_paths(&mut list, test, input);
    }

    Ok(list)
}

/// Errors used in this module.
pub enum LeoOrString {
    Leo(LeoError),
    String(String),
}

impl Display for LeoOrString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Leo(x) => std::fmt::Display::fmt(&x, f),
            Self::String(x) => std::fmt::Display::fmt(&x, f),
        }
    }
}

/// A buffer used to emit errors into.
#[derive(Clone)]
pub struct BufferEmitter(pub Rc<RefCell<Buffer<LeoOrString>>>, pub Rc<RefCell<Buffer<LeoWarning>>>);

impl Emitter for BufferEmitter {
    fn emit_err(&mut self, err: LeoError) {
        self.0.borrow_mut().push(LeoOrString::Leo(err));
    }

    fn last_emitted_err_code(&self) -> Option<i32> {
        let temp = &*self.0.borrow();
        temp.last_entry().map(|entry| match entry {
            LeoOrString::Leo(err) => err.exit_code(),
            _ => 0,
        })
    }

    fn emit_warning(&mut self, warning: leo_errors::LeoWarning) {
        self.1.borrow_mut().push(warning);
    }
}

pub fn buffer_if_err<T>(buf: &BufferEmitter, res: Result<T, String>) -> Result<T, ()> {
    res.map_err(|err| buf.0.borrow_mut().push(LeoOrString::String(err)))
}

pub fn temp_dir() -> PathBuf {
    tempfile::tempdir()
        .expect("Failed to open temporary directory")
        .into_path()
}

pub fn compile_and_process<'a>(parsed: &'a mut Compiler<'a>) -> Result<String, LeoError> {
    let st = parsed.symbol_table_pass()?;
    let st = parsed.type_checker_pass(st)?;
    let st = parsed.loop_unrolling_pass(st)?;
    let assigner = parsed.static_single_assignment_pass(&st)?;

    parsed.flattening_pass(&st, assigner)?;

    // Compile Leo program to bytecode.
    let bytecode = CodeGenerator::do_pass((&parsed.ast, &st))?;

    Ok(bytecode)
}

