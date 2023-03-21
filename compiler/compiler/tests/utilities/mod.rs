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

use leo_compiler::{Compiler, CompilerOptions};
use leo_errors::{
    emitter::{Buffer, Emitter, Handler},
    LeoError, LeoWarning,
};
use leo_passes::{CodeGenerator, Pass};
use leo_span::source_map::FileName;
use leo_test_framework::Test;

use snarkvm::prelude::*;

use snarkvm::file::Manifest;
use snarkvm::package::Package;
use std::fs::File;
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

pub type Network = Testnet3;
#[allow(unused)]
pub type Aleo = snarkvm::circuit::AleoV0;

pub fn hash_asts() -> (String, String, String, String, String, String) {
    let initial_ast = hash_file("/tmp/output/test.initial_ast.json");
    let unrolled_ast = hash_file("/tmp/output/test.unrolled_ast.json");
    let ssa_ast = hash_file("/tmp/output/test.ssa_ast.json");
    let flattened_ast = hash_file("/tmp/output/test.flattened_ast.json");
    let inlined_ast = hash_file("/tmp/output/test.inlined_ast.json");
    let dce_ast = hash_file("/tmp/output/test.dce_ast.json");

    (initial_ast, unrolled_ast, ssa_ast, flattened_ast, inlined_ast, dce_ast)
}

pub fn get_cwd_option(test: &Test) -> Option<PathBuf> {
    // Check for CWD option:
    // ``` cwd: import ```
    // When set, uses different working directory for current file.
    // If not, uses file path as current working directory.
    test.config.extra.get("cwd").map(|val| {
        let mut cwd = test.path.clone();
        cwd.pop();
        cwd.join(val.as_str().unwrap())
    })
}

pub fn setup_build_directory(program_name: &str, bytecode: &String, handler: &Handler) -> Result<Package<Network>, ()> {
    // Initialize a temporary directory.
    let directory = temp_dir();

    // Create the program id.
    let program_id = ProgramID::<Network>::from_str(program_name).unwrap();

    // Write the program string to a file in the temporary directory.
    let path = directory.join("main.aleo");
    let mut file = File::create(path).unwrap();
    file.write_all(bytecode.as_bytes()).unwrap();

    // Create the manifest file.
    let _manifest_file = Manifest::create(&directory, &program_id).unwrap();

    // Create the build directory.
    let build_directory = directory.join("build");
    fs::create_dir_all(build_directory).unwrap();

    // Open the package at the temporary directory.
    handler.extend_if_error(Package::<Testnet3>::open(&directory).map_err(LeoError::Anyhow))
}

pub fn new_compiler(handler: &Handler, main_file_path: PathBuf) -> Compiler<'_> {
    let output_dir = PathBuf::from("/tmp/output/");
    fs::create_dir_all(output_dir.clone()).unwrap();

    Compiler::new(
        String::from("test"),
        String::from("aleo"),
        handler,
        main_file_path,
        output_dir,
        Some(CompilerOptions {
            spans_enabled: false,
            dce_enabled: true,
            initial_input_ast: true,
            initial_ast: true,
            unrolled_ast: true,
            ssa_ast: true,
            flattened_ast: true,
            inlined_ast: true,
            dce_ast: true,
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
pub struct BufferEmitter(
    pub Rc<RefCell<Buffer<LeoOrString>>>,
    pub Rc<RefCell<Buffer<LeoWarning>>>,
);

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

#[allow(unused)]
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

    let (st, struct_graph, call_graph) = parsed.type_checker_pass(st)?;

    let st = parsed.loop_unrolling_pass(st)?;

    let assigner = parsed.static_single_assignment_pass(&st)?;

    let assigner = parsed.flattening_pass(&st, assigner)?;

    let _ = parsed.function_inlining_pass(&call_graph, assigner)?;

    parsed.dead_code_elimination_pass()?;

    // Compile Leo program to bytecode.
    let bytecode = CodeGenerator::do_pass((&parsed.ast, &st, &struct_graph, &call_graph))?;

    Ok(bytecode)
}
