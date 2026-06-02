// Copyright (C) 2019-2026 Provable Inc.
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

//! Wasm project loader.
//!
//! Layers an [`InMemoryFileSource`] over a wasm-side virtual file map and
//! defers all manifest reading, transitive dep walking, and topological
//! sorting to [`leo_package::Package`] via its
//! `from_directory_*_with_file_source` entry points. The Compiler then runs
//! against the same `FileSource`, so leo-wasm holds no parallel project
//! model — every command operates on a single shared `Package`.

use indexmap::IndexMap;
use leo_ast::{NetworkName, NodeBuilder, Stub};
use leo_compiler::{
    Compiled,
    Compiler,
    disassemble_dependency_bytecode,
    run::{Case, Config as RunConfig, EvaluationOutcome, Program as RunProgram, run_without_ledger},
};
use leo_errors::{BufferEmitter, Handler};
use leo_package::{Package, ProgramData};
use leo_span::{
    Symbol,
    file_source::{FileSource, InMemoryFileSource},
    source_map::FileName,
    sym,
    with_session_globals,
};
use std::{path::PathBuf, rc::Rc};

/// A wasm project: a fully resolved [`Package`] plus the in-memory file map
/// that fed it. Holding both lets the Compiler run against the same source
/// the package walker consulted.
pub struct Project {
    pub package: Package,
    pub file_source: InMemoryFileSource,
}

impl Project {
    /// Build a project from a `{ path: contents }` JSON blob and a `root`
    /// pointing at the package's `program.json` directory.
    pub fn from_files_json(files_json: &str, root: &str) -> Result<Self, String> {
        let file_source = build_file_source(files_json)?;
        let package = Package::from_directory_with_file_source(root, &file_source)
            .map_err(|e| format!("load package `{root}`: {e}"))?;
        Ok(Self { package, file_source })
    }
}

/// Materialise a `{path: contents}` JSON blob into an [`InMemoryFileSource`].
fn build_file_source(files_json: &str) -> Result<InMemoryFileSource, String> {
    let map: IndexMap<String, String> =
        serde_json::from_str(files_json).map_err(|e| format!("invalid files JSON: {e}"))?;
    let mut fs = InMemoryFileSource::new();
    for (path, contents) in map {
        fs.set(PathBuf::from(path), contents);
    }
    Ok(fs)
}

/// Iterate `package.compilation_units` (topologically sorted) and build a
/// stub map for every non-primary unit. Bytecode deps disassemble through
/// `disassemble_dependency_bytecode`; source deps go through the Leo parser
/// against the same `FileSource` the main compile sees.
///
/// `handler`/`node_builder` must be the same instances the Compiler will run
/// with so NodeIDs line up across stubs and the primary program.
fn build_import_stubs(
    project: &Project,
    primary_name: Symbol,
    handler: &Handler,
    buf: &BufferEmitter,
    node_builder: &Rc<NodeBuilder>,
    network: NetworkName,
) -> Result<IndexMap<Symbol, Stub>, String> {
    let mut stubs = IndexMap::with_capacity(project.package.compilation_units.len());
    for unit in &project.package.compilation_units {
        if unit.name == primary_name {
            continue;
        }
        let stub = match &unit.data {
            ProgramData::Bytecode(bytecode) => disassemble_dependency_bytecode(unit.name, bytecode, network)
                .map_err(|e| format!("disassemble `{}`: {e}", unit.name))?,
            ProgramData::SourcePath { source, .. } => {
                let src =
                    project.file_source.read_file(source).map_err(|e| format!("read `{}`: {e}", source.display()))?;
                let source_file =
                    with_session_globals(|s| s.source_map.new_source(&src, FileName::Real(source.clone())));
                let program = leo_parser::parse_program(handler.clone(), node_builder, &source_file, &[], network)
                    .map_err(|_| diag_or(buf, &format!("parse dependency `{}`", source.display())))?;
                if handler.had_errors() {
                    return Err(diag_or(buf, &format!("parse dependency `{}` had errors", source.display())));
                }
                Stub::from(program)
            }
        };
        stubs.insert(unit.name, stub);
    }
    Ok(stubs)
}

/// Pull `(name, src_dir, entry_file)` for the package's primary source unit.
fn primary_source(project: &Project) -> Result<(Symbol, PathBuf, PathBuf), String> {
    let primary = project.package.primary_unit().ok_or_else(|| "no primary unit in package".to_string())?;
    let ProgramData::SourcePath { directory, source } = &primary.data else {
        return Err(format!("primary unit `{}` is not a source package", primary.name));
    };
    Ok((primary.name, directory.join(leo_package::SOURCE_DIRECTORY), source.clone()))
}

/// Compile the project's primary unit to Aleo bytecode + ABI.
pub fn compile(project: &Project, is_test: bool, network: NetworkName) -> Result<Compiled, String> {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());

    let (primary_name, source_dir, entry) = primary_source(project)?;
    let import_stubs = build_import_stubs(project, primary_name, &handler, &buf, &node_builder, network)?;

    let mut compiler = Compiler::new(
        Some(primary_name.to_string()),
        is_test,
        handler,
        node_builder,
        PathBuf::new(), // unused on wasm: `write_ast` is a no-op
        None,
        import_stubs,
        network,
    );
    compiler
        .compile_from_directory_with_file_source(&entry, &source_dir, &project.file_source)
        .map_err(|e| diag_or(&buf, &e))
}

/// Compile the project as a test package: same as [`compile`] with
/// `is_test = true` and an optional `extra_stubs` merged into the stub map
/// (typically the main program's `Stub::FromLeo` so cross-program references
/// resolve).
pub fn compile_test(
    project: &Project,
    network: NetworkName,
    extra_stubs: IndexMap<Symbol, Stub>,
) -> Result<Compiled, String> {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());

    let (primary_name, source_dir, entry) = primary_source(project)?;
    let mut import_stubs = build_import_stubs(project, primary_name, &handler, &buf, &node_builder, network)?;
    for (k, v) in extra_stubs {
        import_stubs.entry(k).or_insert(v);
    }

    let mut compiler = Compiler::new(
        None,
        /* is_test */ true,
        handler,
        node_builder,
        PathBuf::new(),
        None,
        import_stubs,
        network,
    );
    compiler
        .compile_from_directory_with_file_source(&entry, &source_dir, &project.file_source)
        .map_err(|e| diag_or(&buf, &e))
}

fn diag(buf: &BufferEmitter) -> String {
    buf.extract_errs().into_inner().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
}

/// Prefer buffered diagnostics; fall back to the raw error string when
/// `Compiler` returned an error that bypassed the handler (e.g. file-IO
/// failures in `read_sources_and_modules`).
fn diag_or(buf: &BufferEmitter, fallback: &impl std::fmt::Display) -> String {
    let d = diag(buf);
    if d.is_empty() { fallback.to_string() } else { d }
}

// ---------------------------------------------------------------------------
// Execution
// ---------------------------------------------------------------------------

/// A `@test`-annotated entry function discovered in a test package's source.
#[derive(Debug, Clone)]
pub struct TestFn {
    /// Bare program name from the `program <name>.aleo` declaration.
    pub program: String,
    /// Function identifier.
    pub function: String,
    /// `@test(should_fail)` flips the pass criterion.
    pub should_fail: bool,
}

/// Build the `Vec<Program>` that `run_without_ledger` wants from a `Compiled`
/// project: every emitted dep bytecode first, then the primary program last,
/// so cross-program calls resolve in either direction.
pub fn stage_programs(compiled: &Compiled) -> Vec<RunProgram> {
    let mut programs: Vec<RunProgram> =
        compiled.imports.iter().map(|p| RunProgram { bytecode: p.bytecode.clone(), name: p.name.clone() }).collect();
    programs.push(RunProgram { bytecode: compiled.primary.bytecode.clone(), name: compiled.primary.name.clone() });
    programs
}

/// Evaluate a single function against the staged `programs` and return its
/// [`EvaluationOutcome`].
pub fn run_function(
    programs: Vec<RunProgram>,
    program_name: &str,
    function: &str,
    inputs: Vec<String>,
) -> Result<EvaluationOutcome, String> {
    let case = Case {
        program_name: program_name.to_string(),
        function: function.to_string(),
        private_key: None,
        input: inputs,
        seed_mapping: Vec::new(),
    };
    let mut outcomes =
        run_without_ledger(&RunConfig { seed: 0, start_height: None, programs, skip_proving: true }, &[case])
            .map_err(|e| format!("{e}"))?;
    Ok(outcomes.pop().expect("one case in, one outcome out"))
}

/// Parse the project's primary source (without running the full pipeline) and
/// list every `@test` entry function. Returns an empty vector when the source
/// fails to parse — the caller treats that the same as "no tests."
pub fn find_test_functions(project: &Project, network: NetworkName) -> Result<Vec<TestFn>, String> {
    let primary = project.package.primary_unit().ok_or_else(|| "no primary unit in package".to_string())?;
    let ProgramData::SourcePath { source, .. } = &primary.data else {
        return Err(format!("test package's primary unit `{}` is not a source package", primary.name));
    };
    let src = project.file_source.read_file(source).map_err(|e| format!("read `{}`: {e}", source.display()))?;
    let (handler, _) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());
    let source_file = with_session_globals(|s| s.source_map.new_source(&src, FileName::Real(source.clone())));
    let Ok(program) = leo_parser::parse_program(handler, &node_builder, &source_file, &[], network) else {
        return Ok(Vec::new());
    };

    let mut out = Vec::new();
    for (prog_sym, scope) in &program.program_scopes {
        let prog_name = prog_sym.to_string();
        for (fn_sym, function) in &scope.functions {
            if !function.annotations.iter().any(|a| a.identifier.name == sym::test) {
                continue;
            }
            let should_fail = function.annotations.iter().any(|a| a.identifier.name == sym::should_fail);
            out.push(TestFn { program: prog_name.clone(), function: fn_sym.to_string(), should_fail });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::create_session_if_not_set_then;
    use serde_json::json;

    fn fs_with(entries: &[(&str, &str)]) -> String {
        let mut map = serde_json::Map::new();
        for (path, body) in entries {
            map.insert((*path).into(), serde_json::Value::String((*body).into()));
        }
        serde_json::Value::Object(map).to_string()
    }

    fn manifest(name: &str, deps: serde_json::Value) -> String {
        json!({
            "program": name,
            "version": "0.0.0",
            "description": "",
            "license": "MIT",
            "dependencies": deps,
        })
        .to_string()
    }

    #[test]
    fn compile_with_modules_only() {
        let files = fs_with(&[
            ("/p/program.json", &manifest("hello.aleo", json!(null))),
            (
                "/p/src/main.leo",
                "program hello.aleo {\n    fn sum(public a: u32, public b: u32) -> u32 { return a + b; }\n    @noupgrade\n    constructor() {}\n}\n",
            ),
        ]);
        create_session_if_not_set_then(|_| {
            let proj = Project::from_files_json(&files, "/p").expect("project");
            let c = compile(&proj, false, NetworkName::TestnetV0).expect("compile");
            assert!(c.primary.bytecode.contains("hello.aleo"));
            assert!(c.imports.is_empty());
        });
    }

    #[test]
    fn compile_with_source_dependency() {
        // `app` depends on `lib`; lib::sum is invoked from app::main.
        let lib_main = "program lib.aleo {\n    fn sum(public a: u32, public b: u32) -> u32 { return a + b; }\n    @noupgrade\n    constructor() {}\n}\n";
        let app_main = "import lib.aleo;\n\nprogram app.aleo {\n    fn add(public a: u32, public b: u32) -> u32 { return lib.aleo::sum(a, b); }\n    @noupgrade\n    constructor() {}\n}\n";
        let app_manifest = manifest("app.aleo", json!([{ "name": "lib.aleo", "location": "local", "path": "lib" }]));
        let lib_manifest = manifest("lib.aleo", json!(null));

        let files = fs_with(&[
            ("/app/program.json", &app_manifest),
            ("/app/src/main.leo", app_main),
            ("/app/lib/program.json", &lib_manifest),
            ("/app/lib/src/main.leo", lib_main),
        ]);

        create_session_if_not_set_then(|_| {
            let proj = Project::from_files_json(&files, "/app").expect("project");
            let c = compile(&proj, false, NetworkName::TestnetV0).expect("compile");
            assert!(c.primary.bytecode.contains("program app.aleo"));
            // Codegen emits bytecode for the FromLeo dependency too.
            assert_eq!(c.imports.len(), 1);
            assert_eq!(c.imports[0].name, "lib.aleo");
            assert!(c.imports[0].bytecode.contains("program lib.aleo"));
        });
    }

    #[test]
    fn compile_with_transitive_source_dependency() {
        let leaf_src = "program leaf.aleo {\n    fn id(public n: u32) -> u32 { return n; }\n    @noupgrade\n    constructor() {}\n}\n";
        let mid_src = "import leaf.aleo;\n\nprogram mid.aleo {\n    fn bump(public n: u32) -> u32 { return leaf.aleo::id(n) + 1u32; }\n    @noupgrade\n    constructor() {}\n}\n";
        let top_src = "import mid.aleo;\n\nprogram top.aleo {\n    fn go(public n: u32) -> u32 { return mid.aleo::bump(n); }\n    @noupgrade\n    constructor() {}\n}\n";

        let top_manifest = manifest("top.aleo", json!([{ "name": "mid.aleo", "location": "local", "path": "mid" }]));
        let mid_manifest =
            manifest("mid.aleo", json!([{ "name": "leaf.aleo", "location": "local", "path": "../leaf" }]));
        let leaf_manifest = manifest("leaf.aleo", json!(null));

        let files = fs_with(&[
            ("/top/program.json", &top_manifest),
            ("/top/src/main.leo", top_src),
            ("/top/mid/program.json", &mid_manifest),
            ("/top/mid/src/main.leo", mid_src),
            ("/top/leaf/program.json", &leaf_manifest),
            ("/top/leaf/src/main.leo", leaf_src),
        ]);

        create_session_if_not_set_then(|_| {
            let proj = Project::from_files_json(&files, "/top").expect("project");
            let c = compile(&proj, false, NetworkName::TestnetV0).expect("compile");
            assert!(c.primary.bytecode.contains("program top.aleo"));
            // Both transitive deps emit bytecode.
            let names: Vec<&str> = c.imports.iter().map(|p| p.name.as_str()).collect();
            assert!(names.contains(&"mid.aleo"), "imports = {names:?}");
            assert!(names.contains(&"leaf.aleo"), "imports = {names:?}");
        });
    }

    #[test]
    fn compile_with_aleo_bytecode_dependency() {
        // Generate the dep bytecode at test time so the embedded snippet stays
        // valid as the disassembler/snarkVM version moves.
        let lib_src = "program lib.aleo {\n    fn id(public n: u32) -> u32 { return n; }\n    @noupgrade\n    constructor() {}\n}\n";
        let lib_bytecode = create_session_if_not_set_then(|_| {
            let p = Project::from_files_json(
                &fs_with(&[("/lib/program.json", &manifest("lib.aleo", json!(null))), ("/lib/src/main.leo", lib_src)]),
                "/lib",
            )
            .unwrap();
            compile(&p, false, NetworkName::TestnetV0).unwrap().primary.bytecode
        });

        let app_src = "import lib.aleo;\n\nprogram app.aleo {\n    fn echo(public n: u32) -> u32 { return lib.aleo::id(n); }\n    @noupgrade\n    constructor() {}\n}\n";
        // Dep is a `.aleo` bytecode file, not a source package.
        let app_manifest =
            manifest("app.aleo", json!([{ "name": "lib.aleo", "location": "local", "path": "deps/lib.aleo" }]));

        let files = fs_with(&[
            ("/app/program.json", &app_manifest),
            ("/app/src/main.leo", app_src),
            ("/app/deps/lib.aleo", &lib_bytecode),
        ]);
        create_session_if_not_set_then(|_| {
            let proj = Project::from_files_json(&files, "/app").expect("project");
            let c = compile(&proj, false, NetworkName::TestnetV0).expect("compile");
            assert!(c.primary.bytecode.contains("program app.aleo"));
            // `.aleo` deps come in pre-compiled; codegen doesn't re-emit them.
            assert!(c.imports.iter().all(|p| p.name != "lib.aleo"));
        });
    }

    #[test]
    fn compile_with_modules_in_src() {
        // Sibling `.leo` files in `src/` are auto-discovered as modules and
        // referenced with `module::item` syntax (no explicit `use` needed).
        let main_leo = "program demo.aleo {\n    fn sum(public a: u32, public b: u32) -> u32 { return utils::add(a, b); }\n    @noupgrade\n    constructor() {}\n}\n";
        let utils_leo = "fn add(a: u32, b: u32) -> u32 { return a + b; }\n";

        let files = fs_with(&[
            ("/p/program.json", &manifest("demo.aleo", json!(null))),
            ("/p/src/main.leo", main_leo),
            ("/p/src/utils.leo", utils_leo),
        ]);

        create_session_if_not_set_then(|_| {
            let proj = Project::from_files_json(&files, "/p").expect("project");
            let c = compile(&proj, false, NetworkName::TestnetV0).expect("compile");
            assert!(c.primary.bytecode.contains("program demo.aleo"));
        });
    }
}
