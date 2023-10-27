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

mod check_unique_node_ids;
use check_unique_node_ids::*;

use leo_compiler::{BuildOptions, Compiler, CompilerOptions};
use leo_errors::{
    emitter::{Buffer, Emitter, Handler},
    LeoError,
    LeoWarning,
};
use leo_package::root::env::Env;
use leo_span::source_map::FileName;
use leo_test_framework::{test::TestConfig, Test};

use snarkvm::prelude::*;

use leo_ast::ProgramVisitor;
use snarkvm::{file::Manifest, package::Package};
use std::{
    cell::RefCell,
    fmt,
    fs,
    fs::File,
    path::{Path, PathBuf},
    rc::Rc,
};

pub type Network = Testnet3;
#[allow(unused)]
pub type Aleo = snarkvm::circuit::AleoV0;

pub fn hash_asts() -> (String, String, String, String, String, String, String) {
    let initial_ast = hash_file("/tmp/output/test.initial_ast.json");
    let unrolled_ast = hash_file("/tmp/output/test.unrolled_ast.json");
    let ssa_ast = hash_file("/tmp/output/test.ssa_ast.json");
    let flattened_ast = hash_file("/tmp/output/test.flattened_ast.json");
    let destructured_ast = hash_file("/tmp/output/test.destructured_ast.json");
    let inlined_ast = hash_file("/tmp/output/test.inlined_ast.json");
    let dce_ast = hash_file("/tmp/output/test.dce_ast.json");

    (initial_ast, unrolled_ast, ssa_ast, flattened_ast, destructured_ast, inlined_ast, dce_ast)
}

pub fn hash_symbol_tables() -> (String, String, String) {
    let initial_symbol_table = hash_file("/tmp/output/test.initial_symbol_table.json");
    let type_checked_symbol_table = hash_file("/tmp/output/test.type_checked_symbol_table.json");
    let unrolled_symbol_table = hash_file("/tmp/output/test.unrolled_symbol_table.json");
    (initial_symbol_table, type_checked_symbol_table, unrolled_symbol_table)
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

pub fn get_build_options(test_config: &TestConfig) -> Vec<BuildOptions> {
    match test_config.extra.get("configs") {
        Some(configs) => {
            // Parse the sequence of compiler configurations.
            configs
                .as_sequence()
                .unwrap()
                .iter()
                .map(|config| {
                    let config = config.as_mapping().expect("Expected the compiler configuration to be a mapping.");
                    assert_eq!(
                        config.len(),
                        1,
                        "A compiler configuration must have exactly one key-value pair. e.g. `dce_enabled`: true"
                    );
                    BuildOptions {
                        dce_enabled: config
                            .get(&serde_yaml::Value::String("dce_enabled".to_string()))
                            .expect("Expected key `dce_enabled`")
                            .as_bool()
                            .expect("Expected value to be a boolean."),
                    }
                })
                .collect()
        }
        None => vec![BuildOptions { dce_enabled: true }],
    }
}

#[allow(unused)]
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

    // Create the environment file.
    Env::<Network>::new().unwrap().write_to(&directory).unwrap();
    if Env::<Network>::exists_at(&directory) {
        println!(".env file created at {:?}", &directory);
    }

    // Create the build directory.
    let build_directory = directory.join("build");
    fs::create_dir_all(build_directory).unwrap();

    // Open the package at the temporary directory.
    handler.extend_if_error(Package::<Network>::open(&directory).map_err(LeoError::Anyhow))
}

pub fn new_compiler(
    handler: &Handler,
    main_file_path: PathBuf,
    compiler_options: Option<CompilerOptions>,
) -> Compiler<'_> {
    let output_dir = PathBuf::from("/tmp/output/");
    fs::create_dir_all(output_dir.clone()).unwrap();

    Compiler::new(String::from("test"), String::from("aleo"), handler, main_file_path, output_dir, compiler_options)
}

pub fn parse_program<'a>(
    handler: &'a Handler,
    program_string: &str,
    cwd: Option<PathBuf>,
    compiler_options: Option<CompilerOptions>,
) -> Result<Compiler<'a>, LeoError> {
    let mut compiler = new_compiler(handler, cwd.clone().unwrap_or_else(|| "compiler-test".into()), compiler_options);
    let name = cwd.map_or_else(|| FileName::Custom("compiler-test".into()), FileName::Real);
    compiler.parse_program_from_string(program_string, name)?;

    CheckUniqueNodeIds::new().visit_program(&compiler.ast.ast);

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

#[allow(unused)]
pub fn buffer_if_err<T>(buf: &BufferEmitter, res: Result<T, String>) -> Result<T, ()> {
    res.map_err(|err| buf.0.borrow_mut().push(LeoOrString::String(err)))
}

#[allow(unused)]
pub fn temp_dir() -> PathBuf {
    tempfile::tempdir().expect("Failed to open temporary directory").into_path()
}

pub fn compile_and_process<'a>(parsed: &'a mut Compiler<'a>) -> Result<String, LeoError> {
    let st = parsed.symbol_table_pass()?;

    CheckUniqueNodeIds::new().visit_program(&parsed.ast.ast);

    let (st, struct_graph, call_graph) = parsed.type_checker_pass(st)?;

    CheckUniqueNodeIds::new().visit_program(&parsed.ast.ast);

    let st = parsed.loop_unrolling_pass(st)?;

    parsed.static_single_assignment_pass(&st)?;

    parsed.flattening_pass(&st)?;

    parsed.destructuring_pass()?;

    parsed.function_inlining_pass(&call_graph)?;

    parsed.dead_code_elimination_pass()?;

    // Compile Leo program to bytecode.
    let bytecode = parsed.code_generation_pass(&st, &struct_graph, &call_graph)?;

    Ok(bytecode)
}

/// Returns the private key from the .env file specified in the directory.
#[allow(unused)]
pub fn dotenv_private_key(directory: &Path) -> Result<PrivateKey<Network>> {
    use std::str::FromStr;
    dotenvy::from_path(directory.join(".env")).map_err(|_| anyhow!("Missing a '.env' file in the test directory."))?;
    // Load the private key from the environment.
    let private_key = dotenvy::var("PRIVATE_KEY").map_err(|e| anyhow!("Missing PRIVATE_KEY - {e}"))?;
    // Parse the private key.
    PrivateKey::<Network>::from_str(&private_key)
}
