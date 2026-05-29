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

//! Multi-file / multi-dependency project support for `leo-wasm`.
//!
//! The browser passes a virtual file system (path → contents) plus a project
//! root path. This module:
//!
//! - Materialises the file map as an [`InMemoryFileSource`].
//! - Walks each project's `program.json` to collect dependencies, including
//!   transitive ones, distinguishing source packages from `.aleo` bytecode
//!   stubs.
//! - Delegates the actual compilation to
//!   [`leo_compiler::Compiler::compile_from_directory_with_file_source`], so
//!   module discovery and the pass pipeline are not duplicated.

use indexmap::{IndexMap, IndexSet};
use leo_ast::{NetworkName, NodeBuilder, Stub};
use leo_compiler::{Compiled, Compiler, disassemble_dependency_bytecode};
use leo_errors::{BufferEmitter, Handler};
use leo_span::{
    Symbol,
    file_source::{FileSource, InMemoryFileSource},
    source_map::FileName,
    with_session_globals,
};
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

/// Source-directory name inside a Leo package (mirrors `leo_package::SOURCE_DIRECTORY`).
const SOURCE_DIRECTORY: &str = "src";
/// Entry-point name for a program package (mirrors `leo_package::MAIN_FILENAME`).
const MAIN_FILENAME: &str = "main.leo";
/// Entry-point name for a library package (mirrors `leo_package::LIB_FILENAME`).
const LIB_FILENAME: &str = "lib.leo";
/// Manifest filename (mirrors `leo_package::MANIFEST_FILENAME`).
const MANIFEST_FILENAME: &str = "program.json";

/// A virtual project rooted at `root`, with all files preloaded into a shared
/// [`InMemoryFileSource`]. The root must be the directory that contains
/// `program.json`.
pub struct Project {
    pub root: PathBuf,
    pub file_source: InMemoryFileSource,
}

impl Project {
    /// Build a project from a `{ path: contents }` JSON blob.
    ///
    /// Paths are stored verbatim; callers choose the prefix convention. Every
    /// entry passed in becomes addressable through the returned
    /// [`InMemoryFileSource`].
    pub fn from_files_json(files_json: &str, root: &str) -> Result<Self, String> {
        let map: IndexMap<String, String> =
            serde_json::from_str(files_json).map_err(|e| format!("invalid files JSON: {e}"))?;
        let mut file_source = InMemoryFileSource::new();
        for (path, contents) in map {
            file_source.set(PathBuf::from(path), contents);
        }
        Ok(Self { root: PathBuf::from(root), file_source })
    }

    /// Path to the project's `program.json`.
    pub fn manifest_path(&self) -> PathBuf {
        self.root.join(MANIFEST_FILENAME)
    }

    /// Path to the project's source directory.
    pub fn source_dir(&self) -> PathBuf {
        self.root.join(SOURCE_DIRECTORY)
    }

    /// Resolve the entry file (`src/main.leo` or `src/lib.leo`).
    pub fn entry_file(&self) -> Result<PathBuf, String> {
        entry_file_in(&self.file_source, &self.root)
    }
}

/// Minimal manifest shape needed to walk dependencies.
///
/// Inlined rather than depending on `leo-package`: that crate pulls in the full
/// `snarkvm` umbrella and is native-only.
#[derive(Debug, serde::Deserialize)]
struct ManifestLite {
    #[allow(dead_code)] // captured for symmetry with `leo-package::Manifest`; not used directly.
    program: String,
    #[serde(default)]
    dependencies: Option<Vec<DependencyLite>>,
}

#[derive(Debug, serde::Deserialize)]
struct DependencyLite {
    name: String,
    /// Path relative to the manifest's directory. Only path-based deps are
    /// resolvable from a virtual FS; network deps require the JS side to bake
    /// the bytecode into the file map and point `path` at it.
    path: Option<PathBuf>,
}

/// Resolve a package's entry file (`main.leo` or `lib.leo`) inside `root`.
fn entry_file_in(file_source: &InMemoryFileSource, root: &Path) -> Result<PathBuf, String> {
    let src = root.join(SOURCE_DIRECTORY);
    let main = src.join(MAIN_FILENAME);
    let lib = src.join(LIB_FILENAME);
    match (file_source.is_file(&main), file_source.is_file(&lib)) {
        (true, true) => Err(format!("ambiguous entry in `{}`: both `main.leo` and `lib.leo` present", root.display())),
        (true, false) => Ok(main),
        (false, true) => Ok(lib),
        (false, false) => Err(format!("missing entry in `{}`: neither `main.leo` nor `lib.leo` found", root.display())),
    }
}

/// Walk `project`'s dependency graph and build the import-stub map that
/// `Compiler::new` expects.
///
/// Path-based deps are loaded one of two ways:
///
/// * Path ends in `.aleo` and points to a file → disassemble that bytecode
///   (`Stub::FromAleo`).
/// * Path points at a directory containing `program.json` → parse the package's
///   entry file with the *same* node builder as the parent so NodeIDs line up
///   (`Stub::FromLeo`), then recurse for its own deps.
pub fn collect_import_stubs(
    project: &Project,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    network: NetworkName,
) -> Result<IndexMap<Symbol, Stub>, String> {
    let mut stubs = IndexMap::new();
    let mut visited = IndexSet::<PathBuf>::new();
    walk_deps(&project.file_source, &project.root, handler, node_builder, network, &mut stubs, &mut visited)?;
    Ok(stubs)
}

fn walk_deps(
    file_source: &InMemoryFileSource,
    manifest_dir: &Path,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    network: NetworkName,
    stubs: &mut IndexMap<Symbol, Stub>,
    visited: &mut IndexSet<PathBuf>,
) -> Result<(), String> {
    let manifest_path = manifest_dir.join(MANIFEST_FILENAME);
    if !visited.insert(manifest_path.clone()) {
        return Ok(());
    }
    let manifest = read_manifest(file_source, &manifest_path)?;

    for dep in manifest.dependencies.unwrap_or_default() {
        let Some(rel_path) = dep.path else {
            // Network-only deps are unsupported in the virtual FS: callers should
            // bake the bytecode into the file map and point `path` at it.
            continue;
        };
        let abs_path = normalize_path(&if rel_path.is_absolute() { rel_path } else { manifest_dir.join(&rel_path) });

        if abs_path.extension().and_then(|s| s.to_str()) == Some("aleo") {
            // `.aleo` bytecode dep — disassemble in place. The disassembler
            // emits the stub's `program_id` (with `.aleo` suffix), which is
            // also the symbol the parser puts in `program.imports`, so the
            // lookup in `add_import_stubs` matches.
            let key = Symbol::intern(&dep.name);
            if stubs.contains_key(&key) {
                continue;
            }
            let bytecode =
                file_source.read_file(&abs_path).map_err(|e| format!("read `{}`: {e}", abs_path.display()))?;
            let stub = disassemble_dependency_bytecode(key, &bytecode, network)
                .map_err(|e| format!("disassemble `{}`: {e}", abs_path.display()))?;
            stubs.insert(key, stub);
        } else {
            // Source-package dep — parse the entry file and recurse.
            let entry = entry_file_in(file_source, &abs_path)?;
            let source = file_source.read_file(&entry).map_err(|e| format!("read `{}`: {e}", entry.display()))?;
            let source_file = with_session_globals(|s| s.source_map.new_source(&source, FileName::Real(entry.clone())));
            let program = leo_parser::parse_program(handler.clone(), node_builder, &source_file, &[], network)
                .map_err(|_| format!("parse dependency `{}`", entry.display()))?;
            if handler.had_errors() {
                return Err(format!("parse dependency `{}` had errors", entry.display()));
            }
            // `add_import_stubs` looks the stub up by the same Symbol the parser
            // installs as the program's scope key — use that, not the manifest's
            // `name` field, so cross-program references resolve regardless of
            // whether the manifest dropped the `.aleo` suffix.
            let scope_key = program
                .program_scopes
                .first()
                .map(|(k, _)| *k)
                .ok_or_else(|| format!("dependency `{}` has no program scope", entry.display()))?;
            if stubs.contains_key(&scope_key) {
                continue;
            }
            stubs.insert(scope_key, Stub::from(program));

            walk_deps(file_source, &abs_path, handler, node_builder, network, stubs, visited)?;
        }
    }
    Ok(())
}

/// Collapse `.` and `..` components without touching the filesystem.
///
/// `Path::canonicalize` would consult the real disk; the virtual file source
/// has no real disk to consult, so we resolve manifest-relative paths like
/// `../leaf` to `/top/leaf` here.
fn normalize_path(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in p.components() {
        use std::path::Component::*;
        match comp {
            CurDir => {}
            ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn read_manifest(file_source: &InMemoryFileSource, path: &Path) -> Result<ManifestLite, String> {
    let raw = file_source.read_file(path).map_err(|e| format!("read manifest `{}`: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse manifest `{}`: {e}", path.display()))
}

/// Compile a project to bytecode + ABI.
///
/// Returns the [`Compiled`] result including the primary program and every
/// import bytecode emitted by codegen.
pub fn compile(project: &Project, is_test: bool, network: NetworkName) -> Result<Compiled, String> {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());

    // Get the program name up front so the compiler's name-mismatch check fires
    // with a meaningful "expected vs got" message.
    let manifest = read_manifest(&project.file_source, &project.manifest_path())?;
    let expected_unit_name = Some(manifest.program);

    let import_stubs = collect_import_stubs(project, &handler, &node_builder, network)?;

    let entry = project.entry_file()?;
    let src_dir = project.source_dir();
    let mut compiler = Compiler::new(
        expected_unit_name,
        is_test,
        handler,
        node_builder,
        PathBuf::new(), // unused: `write_ast` is a no-op on wasm32
        None,
        import_stubs,
        network,
    );
    compiler
        .compile_from_directory_with_file_source(&entry, &src_dir, &project.file_source)
        .map_err(|e| diag_or(&buf, &e))
}

/// Compile `project` as a test: same as [`compile`] but with `is_test = true`
/// and additional import stubs populated by `extra_stubs` (typically the main
/// program's `Stub::FromLeo` so cross-program references resolve).
pub fn compile_test(
    project: &Project,
    network: NetworkName,
    extra_stubs: IndexMap<Symbol, Stub>,
) -> Result<Compiled, String> {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());

    let mut import_stubs = collect_import_stubs(project, &handler, &node_builder, network)?;
    for (k, v) in extra_stubs {
        import_stubs.entry(k).or_insert(v);
    }

    let entry = project.entry_file()?;
    let src_dir = project.source_dir();
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
        .compile_from_directory_with_file_source(&entry, &src_dir, &project.file_source)
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
