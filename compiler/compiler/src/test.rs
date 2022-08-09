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

use crate::{Compiler, OutputOptions};

use leo_errors::{
    emitter::{Buffer, Emitter, Handler},
    LeoError, LeoWarning,
};
use leo_span::{source_map::FileName, symbol::create_session_if_not_set_then};
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};
use snarkvm::file::Manifest;
use snarkvm::package::Package;
use snarkvm::prelude::*;

use leo_passes::{CodeGenerator, Pass};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};
use std::{fs::File, io::Write};

type CurrentNetwork = Testnet3;

fn new_compiler(handler: &Handler, main_file_path: PathBuf) -> Compiler<'_> {
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
        }),
    )
}

fn parse_program<'a>(
    handler: &'a Handler,
    program_string: &str,
    cwd: Option<PathBuf>,
) -> Result<Compiler<'a>, LeoError> {
    let mut compiler = new_compiler(handler, cwd.clone().unwrap_or_else(|| "compiler-test".into()));
    let name = cwd.map_or_else(|| FileName::Custom("compiler-test".into()), FileName::Real);
    compiler.parse_program_from_string(program_string, name)?;

    Ok(compiler)
}

fn hash_content(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

fn hash_file(path: &str) -> String {
    let file = fs::read_to_string(&Path::new(path)).unwrap();
    hash_content(&file)
}

struct CompileNamespace;

impl Namespace for CompileNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let buf = BufferEmitter(Rc::default(), Rc::default());
        let handler = Handler::new(Box::new(buf.clone()));

        create_session_if_not_set_then(|_| {
            run_test(test, &handler, &buf).map_err(|()| buf.0.take().to_string() + &buf.1.take().to_string())
        })
    }
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct OutputItem {
    pub initial_input_ast: String,
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct CompileOutput {
    pub warnings: String,
    pub output: Vec<OutputItem>,
    pub initial_ast: String,
    pub unrolled_ast: String,
    pub ssa_ast: String,
}

/// Get the path of the `input_file` given in `input` into `list`.
fn get_input_file_paths(list: &mut Vec<PathBuf>, test: &Test, input: &Value) {
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
fn collect_all_inputs(test: &Test) -> Result<Vec<PathBuf>, String> {
    let mut list = vec![];

    if let Some(input) = test.config.get("input_file") {
        get_input_file_paths(&mut list, test, input);
    }

    Ok(list)
}

// Errors used in this module.
enum LeoOrString {
    LeoError(LeoError),
    String(String),
}

impl Display for LeoOrString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeoError(x) => std::fmt::Display::fmt(&x, f),
            Self::String(x) => std::fmt::Display::fmt(&x, f),
        }
    }
}

/// A buffer used to emit errors into.
#[derive(Clone)]
struct BufferEmitter(Rc<RefCell<Buffer<LeoOrString>>>, Rc<RefCell<Buffer<LeoWarning>>>);

impl Emitter for BufferEmitter {
    fn emit_err(&mut self, err: LeoError) {
        self.0.borrow_mut().push(LeoOrString::LeoError(err));
    }

    fn last_emitted_err_code(&self) -> Option<i32> {
        let temp = &*self.0.borrow();
        temp.last_entry().map(|entry| match entry {
            LeoOrString::LeoError(err) => err.exit_code(),
            _ => 0,
        })
    }

    fn emit_warning(&mut self, warning: LeoWarning) {
        self.1.borrow_mut().push(warning);
    }
}

fn buffer_if_err<T>(buf: &BufferEmitter, res: Result<T, String>) -> Result<T, ()> {
    res.map_err(|err| buf.0.borrow_mut().push(LeoOrString::String(err)))
}

fn temp_dir() -> PathBuf {
    tempfile::tempdir()
        .expect("Failed to open temporary directory")
        .into_path()
}

fn compile_and_process<'a>(parsed: &'a mut Compiler<'a>, handler: &Handler) -> Result<String, LeoError> {
    let st = parsed.symbol_table_pass()?;
    let st = parsed.type_checker_pass(st)?;
    let _st = parsed.loop_unrolling_pass(st)?;
    parsed.static_single_assignment_pass()?;

    // Compile Leo program to bytecode.
    let bytecode = CodeGenerator::do_pass((&parsed.ast, handler))?;

    Ok(bytecode)
}

fn run_test(test: Test, handler: &Handler, err_buf: &BufferEmitter) -> Result<Value, ()> {
    // Check for CWD option:
    // ``` cwd: import ```
    // When set, uses different working directory for current file.
    // If not, uses file path as current working directory.
    let cwd = test.config.get("cwd").map(|val| {
        let mut cwd = test.path.clone();
        cwd.pop();
        cwd.join(&val.as_str().unwrap())
    });

    let mut parsed = handler.extend_if_error(parse_program(handler, &test.content, cwd))?;

    // (name, content)
    let inputs = buffer_if_err(err_buf, collect_all_inputs(&test))?;

    let mut output_items = Vec::with_capacity(inputs.len());

    if inputs.is_empty() {
        output_items.push(OutputItem {
            initial_input_ast: "no input".to_string(),
        });
    } else {
        // Parse one or more input files to execute the program with.
        for input in inputs {
            let mut parsed = parsed.clone();
            handler.extend_if_error(parsed.parse_input(input))?;
            let initial_input_ast = hash_file("/tmp/output/initial_input_ast.json");

            output_items.push(OutputItem { initial_input_ast });
        }
    };

    // Compile the program to bytecode.
    let program_name = format!("{}.aleo", parsed.program_name);
    let bytecode = handler.extend_if_error(compile_and_process(&mut parsed, handler))?;

    // Run snarkvm package.
    {
        // Initialize a temporary directory.
        let directory = temp_dir();

        // Create the program id.
        let program_id = ProgramID::<CurrentNetwork>::from_str(&program_name).unwrap();

        // Write the program string to a file in the temporary directory.
        let path = directory.join("main.aleo");
        let mut file = File::create(&path).unwrap();
        file.write_all(bytecode.as_bytes()).unwrap();

        // Create the manifest file.
        let _manifest_file = Manifest::create(&directory, &program_id).unwrap();

        // Create the build directory.
        let build_directory = directory.join("build");
        std::fs::create_dir_all(&build_directory).unwrap();

        // Open the package at the temporary directory.
        let _package = handler.extend_if_error(Package::<Testnet3>::open(&directory).map_err(LeoError::Anyhow))?;

        // Commented out since it bottlenecks the test framework.
        // // Get the program process and check all instructions.
        // handler.extend_if_error(package.get_process().map_err(LeoError::Anyhow))?;
    }

    let initial_ast = hash_file("/tmp/output/initial_ast.json");
    let unrolled_ast = hash_file("/tmp/output/unrolled_ast.json");
    let ssa_ast = hash_file("/tmp/output/ssa_ast.json");

    if fs::read_dir("/tmp/output").is_ok() {
        fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
    }

    let final_output = CompileOutput {
        warnings: err_buf.1.take().to_string(),
        output: output_items,
        initial_ast,
        unrolled_ast,
        ssa_ast,
    };
    Ok(serde_yaml::to_value(&final_output).expect("serialization failed"))
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Compile" => Box::new(CompileNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn compiler_tests() {
    leo_test_framework::run_tests(&TestRunner, "compiler");
}
