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

use crate::{compiler::Compiler, Output, OutputOptions};

use leo_asg::*;
use leo_errors::emitter::{Buffer, Emitter, Handler};
use leo_errors::LeoError;
use leo_span::symbol::create_session_if_not_set_then;
use leo_span::Span;
use leo_synthesizer::{CircuitSynthesizer, SerializedCircuit, SummarizedCircuit};
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};

use snarkvm_curves::bls12_377::Bls12_377;
use snarkvm_eval::Evaluator;
use snarkvm_ir::InputData;
use tracing::subscriber;

use core::fmt;
use indexmap::IndexMap;
use serde_yaml::Value;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{fs, io};

pub type TestCompiler<'a> = Compiler<'static, 'a>;

// Convenience function for tests, leaks memory.
pub(crate) fn make_test_context() -> AsgContext<'static> {
    let allocator = Box::leak(Box::new(new_alloc_context()));
    new_context(allocator)
}

fn new_compiler(handler: &Handler, path: PathBuf, theorem_options: Option<OutputOptions>) -> TestCompiler<'_> {
    let program_name = "test".to_string();
    let output_dir = PathBuf::from("/tmp/output/");
    fs::create_dir_all(output_dir.clone()).unwrap();

    TestCompiler::new(
        handler,
        program_name,
        path,
        output_dir,
        make_test_context(),
        None,
        IndexMap::new(),
        theorem_options,
    )
}

pub(crate) fn parse_program<'a>(
    handler: &'a Handler,
    program_string: &str,
    theorem_options: Option<OutputOptions>,
    cwd: Option<PathBuf>,
) -> Result<TestCompiler<'a>, LeoError> {
    let mut compiler = new_compiler(handler, cwd.unwrap_or_else(|| "compiler-test".into()), theorem_options);

    compiler.parse_program_from_string(program_string)?;

    Ok(compiler)
}

fn hash_file(path: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut file = fs::File::open(&Path::new(path)).unwrap();
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

/// The tracing writer used to capture the log
struct CaptiveWriter(Arc<Mutex<Vec<u8>>>);

impl io::Write for &CaptiveWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.lock().unwrap().flush()
    }
}

/// The tracing subscriber used to capture the log
#[derive(Default)]
struct CaptiveSubscriber {
    // `tracing` has harsh requirements on a custom writer.
    // All of the Arc, Mutex stuff is to make `tracing` happy
    output: Arc<Mutex<Vec<u8>>>,
}

impl CaptiveSubscriber {
    /// Returns a guard that, while live, captures the output of tracing.
    /// The output can then be retrieved using `captive_subscriber.output()`.
    fn capture(&self) -> subscriber::DefaultGuard {
        // Use a minimal format. e.g. "INFO hello"
        let fmt = tracing_subscriber::fmt::format()
            .with_target(false)
            .without_time()
            .with_ansi(false)
            .compact();

        // Build a subscriber that writes logs into `self.output`.
        let s = tracing_subscriber::fmt()
            .event_format(fmt)
            .with_writer(Arc::new(CaptiveWriter(self.output.clone())))
            .finish();

        tracing::subscriber::set_default(s)
    }

    /// Returns the captured output thus far.
    fn output(&self) -> Vec<u8> {
        self.output.lock().unwrap().clone()
    }
}

struct CompileNamespace;
struct ImportNamespace;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
struct OutputItem {
    pub input_file: String,
    pub output: Output,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_output: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
struct CompileOutput {
    pub circuit: SummarizedCircuit,
    pub output: Vec<OutputItem>,
    pub initial_ast: String,
    pub ir: String,
    pub imports_resolved_ast: String,
    pub canonicalized_ast: String,
    pub type_inferenced_ast: String,
}

type Input = (String, String);

/// Collect all inputs into `list` from `field`, if possible.
fn collect_inputs_list(list: &mut Vec<Input>, field: &[Value]) -> Result<(), String> {
    for map in field {
        for (name, value) in map.as_mapping().unwrap().iter() {
            // Try to parse string from 'inputs' map, else fail
            let value = if let serde_yaml::Value::String(value) = value {
                value
            } else {
                return Err("Expected string in 'inputs' map".to_string());
            };

            list.push((name.as_str().unwrap().to_string(), value.clone()));
        }
    }
    Ok(())
}

/// Read contents of `input_file` given in `input` into `list`.
fn read_input_file(list: &mut Vec<Input>, test: &Test, input: &Value) {
    let input_file: PathBuf = test.path.parent().expect("no test parent dir").into();
    if let Some(name) = input.as_str() {
        let mut input_file = input_file;
        input_file.push(input.as_str().expect("input_file was not a string or array"));
        list.push((
            name.to_string(),
            fs::read_to_string(&input_file).expect("failed to read test input file"),
        ));
    } else if let Some(seq) = input.as_sequence() {
        for name in seq {
            let mut input_file = input_file.clone();
            input_file.push(name.as_str().expect("input_file was not a string"));
            list.push((
                name.as_str().expect("input_file item was not a string").to_string(),
                fs::read_to_string(&input_file).expect("failed to read test input file"),
            ));
        }
    }
}

/// Collect and return all inputs, if possible.
fn collect_all_inputs(test: &Test) -> Result<Vec<Input>, String> {
    let mut list = vec![];

    if let Some(Value::Sequence(field)) = test.config.get("inputs") {
        collect_inputs_list(&mut list, field)?;
    }

    if let Some(input) = test.config.get("input_file") {
        read_input_file(&mut list, &test, input);
    }
    if list.is_empty() {
        list.push(("empty".to_string(), "".to_string()));
    }
    Ok(list)
}

fn read_state_file(test: &Test) -> String {
    if let Some(input) = test.config.get("state_file") {
        let mut input_file: PathBuf = test.path.parent().expect("no test parent dir").into();
        input_file.push(input.as_str().expect("state_file was not a string"));
        fs::read_to_string(&input_file).expect("failed to read test state file")
    } else {
        "".to_string()
    }
}

fn compile_and_process(
    parsed: TestCompiler<'_>,
    input: &Input,
    state: &str,
) -> Result<(snarkvm_ir::Program, InputData), LeoError> {
    let input_parsed = leo_parser::parse_program_input(&input.1, "input", &state, "state")?;
    let compiled = parsed.compile_ir(&input_parsed)?;
    let input_data = parsed.process_input(&input_parsed, &compiled.header)?;
    Ok((compiled, input_data))
}

// Errors used in this module.
enum LeoOrString {
    Leo(LeoError),
    String(String),
}

impl fmt::Display for LeoOrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Leo(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}

/// A buffer used to emit errors into.
#[derive(Clone)]
struct BufferEmitter(Rc<RefCell<Buffer<LeoOrString>>>);

impl Emitter for BufferEmitter {
    fn emit_err(&mut self, err: LeoError) {
        self.0.borrow_mut().push(LeoOrString::Leo(err));
    }
}

fn buffer_if_err<T>(buf: &BufferEmitter, res: Result<T, String>) -> Result<T, ()> {
    res.map_err(|err| buf.0.borrow_mut().push(LeoOrString::String(err)))
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
    // .unwrap_or(test.path.clone());

    let parsed = handler.extend_if_error(parse_program(
        &handler,
        &test.content,
        Some(OutputOptions {
            spans_enabled: false,
            ast_initial: true,
            ast_imports_resolved: true,
            ast_canonicalized: true,
            ast_type_inferenced: true,
            asg_initial: false,
            asg_constants_folded: false,
            asg_dead_code_eliminated: false,
            asg_exclude_edges: Vec::new(),
            asg_exclude_labels: Vec::new(),
            emit_ir: true,
        }),
        cwd,
    ))?;

    // (name, content)
    let inputs = buffer_if_err(&err_buf, collect_all_inputs(&test))?;

    let state = read_state_file(&test);

    let mut output_items = Vec::with_capacity(inputs.len());

    let mut last_circuit = None;
    let mut last_ir: Option<snarkvm_ir::Program> = None;
    for input in inputs {
        let console = CaptiveSubscriber::default();
        let console_guard = console.capture();

        let parsed = parsed.clone();
        let (compiled, input_data) = handler.extend_if_error(compile_and_process(parsed, &input, &state))?;

        let mut cs: CircuitSynthesizer<Bls12_377> = Default::default();
        let output = buffer_if_err(
            &err_buf,
            snarkvm_eval::SetupEvaluator::<_, snarkvm_eval::edwards_bls12::EdwardsGroupType, _>::new(&mut cs)
                .evaluate(&compiled, &input_data)
                .map_err(|e| e.to_string()),
        )?;

        // Drop the guard early to prevent unintended logging.
        // All `console` functions should have been evaluated before this point.
        drop(console_guard);

        let registers: Vec<_> = compiled.header.register_inputs.to_vec();
        let output = handler.extend_if_error(Output::new(&registers[..], output, &Span::default()))?;

        let circuit: SummarizedCircuit = SerializedCircuit::from(cs).into();

        // Add an error if circuit had no constraints.
        if circuit.num_constraints == 0 {
            let err = "- Circuit has no constraints, use inputs and registers in program to produce them";
            buffer_if_err(&err_buf, Err(err.to_string()))?;
        }

        // Set circuit if not set yet.
        // Otherwise, if circuit was changed, add an error.
        if let Some(last_circuit) = last_circuit.as_ref() {
            if last_circuit != &circuit {
                eprintln!(
                    "{}\n{}",
                    serde_yaml::to_string(last_circuit).unwrap(),
                    serde_yaml::to_string(&circuit).unwrap()
                );
                buffer_if_err::<()>(&err_buf, Err("- Circuit changed on different input files".to_string()))?;
            }
        } else {
            last_circuit = Some(circuit);
        }

        let capture_console = match test.config.get("capture_console") {
            Some(v) => v.as_bool().expect("`capture_console` is not bool"),
            None => false, // Ignore the stdout by default
        };

        let console_output = capture_console.then(|| {
            std::str::from_utf8(&console.output())
                .expect("failed to parse the console output")
                .to_string()
        });

        // Set IR if not set yet.
        // Otherwise, if IR was changed, add an error.
        if let Some(last_ir) = last_ir.as_ref() {
            if last_ir != &compiled {
                eprintln!("{}\n{}", last_ir, compiled);
                buffer_if_err::<()>(&err_buf, Err("- IR changed on different input files".to_string()))?;
            }
        } else {
            last_ir = Some(compiled);
        }

        output_items.push(OutputItem {
            input_file: input.0,
            output,
            console_output,
        });
    }

    let ir = hash_file("/tmp/output/test.leo.ir.json");
    let initial_ast = hash_file("/tmp/output/initial_ast.json");
    let imports_resolved_ast = hash_file("/tmp/output/imports_resolved_ast.json");
    let canonicalized_ast = hash_file("/tmp/output/canonicalization_ast.json");
    let type_inferenced_ast = hash_file("/tmp/output/type_inferenced_ast.json");

    if fs::read_dir("/tmp/output").is_ok() {
        fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
    }

    let final_output = CompileOutput {
        circuit: last_circuit.unwrap(),
        output: output_items,
        ir,
        initial_ast,
        imports_resolved_ast,
        canonicalized_ast,
        type_inferenced_ast,
    };
    Ok(serde_yaml::to_value(&final_output).expect("serialization failed"))
}

impl Namespace for CompileNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let err_buf = BufferEmitter(Rc::default());
        let handler = Handler::new(Box::new(err_buf.clone()));

        create_session_if_not_set_then(|_| {
            run_test(test, &handler, &err_buf).map_err(|()| err_buf.0.take().to_string())
        })
    }
}

impl Namespace for ImportNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let err_buf = BufferEmitter(Rc::default());
        let handler = Handler::new(Box::new(err_buf.clone()));

        // In import tests we only keep Error code to make error messages uniform accross
        // all platforms and exclude all platform-specific paths.
        create_session_if_not_set_then(|_| {
            run_test(test, &handler, &err_buf).map_err(|()| {
                let err_vec = err_buf.0.take().into_inner();
                if let LeoOrString::Leo(err) = err_vec.get(0).unwrap() {
                    err.error_code()
                } else {
                    panic!("Leo Error expected");
                }
            })
        })
    }
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Compile" => Box::new(CompileNamespace),
            "Import" => Box::new(ImportNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn compiler_tests() {
    leo_test_framework::run_tests(&TestRunner, "compiler");
}
