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

use super::*;

use leo_ast::{AleoProgram, DiGraph, DiGraphError, NetworkName};
use leo_errors::LeoError;

use snarkvm::{
    prelude::{CanaryV0, MainnetV0, Network, Process as SvmProcess, TestnetV0},
    synthesizer::program::Program as SvmProgram,
};

use indexmap::IndexMap;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

/// Generate ABI from an Aleo bytecode file.
#[derive(Parser, Debug)]
pub struct LeoAbi {
    /// Path to the .aleo file.
    #[clap(value_name = "FILE")]
    file: PathBuf,

    /// Network for parsing (mainnet, testnet, canary).
    #[clap(long, short, default_value = "testnet")]
    network: NetworkName,

    /// Output directory. Writes `<DIR>/<program>.abi.json` for the input and for each declared dependency. Created if
    /// missing; existing files are overwritten. When omitted, every ABI is printed to stdout, separated by
    /// `=== <name> ===` headers.
    #[clap(long, short, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Directory containing the program's `.aleo` imports. Two layouts are supported:
    /// per-unit (`<DIR>/<name>/<name>.aleo`) and legacy flat (`<DIR>/<name>.aleo`).
    /// Defaults to the parent's parent when the input is at `<root>/<unit>/<unit>.aleo`,
    /// otherwise a sibling `imports/` directory next to the input.
    #[clap(long, value_name = "DIR")]
    imports_dir: Option<PathBuf>,
}

impl Command for LeoAbi {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        if !self.file.exists() {
            return Err(crate::errors::cli_invalid_input(format!("File not found: {}", self.file.display())).into());
        }

        match self.file.extension().and_then(|s| s.to_str()) {
            Some("aleo") => {}
            _ => {
                return Err(crate::errors::cli_invalid_input(format!(
                    "Expected a .aleo file, got: {}",
                    self.file.display()
                ))
                .into());
            }
        }

        let content = std::fs::read_to_string(&self.file).map_err(crate::errors::cli_io_error)?;
        let file_name = self.file.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

        let imports_dir = self.imports_dir.clone().or_else(|| {
            let parent = self.file.parent()?;
            // Per-unit layout: file at `<root>/<unit>/<unit>.aleo` — use `<root>`.
            if parent.file_name() == self.file.file_stem() {
                return parent.parent().map(Path::to_path_buf);
            }
            // Legacy: sibling `imports/`.
            let legacy = parent.join("imports");
            legacy.is_dir().then_some(legacy)
        });

        // `Process::add_program` is contextual, so dependencies must be loaded in topological order before the main
        // program.
        let (main_aleo, dep_aleos) = match imports_dir {
            Some(dir) => disassemble_with_imports(file_name, &content, self.network, &dir)?,
            None => (
                leo_disassembler::disassemble_from_str_for_network(file_name, &content, self.network)
                    .map_err(|e| crate::errors::failed_to_parse_aleo_file(file_name, e))?,
                Vec::new(),
            ),
        };

        let main_abi = leo_abi::aleo::generate(&main_aleo);
        let dep_abis: IndexMap<String, _> =
            dep_aleos.into_iter().map(|(name, aleo)| (name, leo_abi::aleo::generate(&aleo))).collect();

        match self.output {
            Some(dir) => write_abis_to_directory(&dir, &main_abi, &dep_abis)?,
            None => print_abis_to_stdout(&main_abi, &dep_abis)?,
        }

        Ok(())
    }
}

/// Pretty-prints `abi` as JSON.
fn abi_to_json(abi: &leo_abi::Program) -> Result<String> {
    serde_json::to_string_pretty(abi).map_err(|e| crate::errors::failed_to_serialize_abi(e.to_string()).into())
}

/// Prints the main ABI followed by each dependency under a `=== <name> ===` header. The main entry is a bare JSON
/// document so callers that only need the no-imports case keep working unchanged.
fn print_abis_to_stdout(main: &leo_abi::Program, deps: &IndexMap<String, leo_abi::Program>) -> Result<()> {
    println!("{}", abi_to_json(main)?);
    for (name, abi) in deps {
        println!();
        println!("=== {name} ===");
        println!("{}", abi_to_json(abi)?);
    }
    Ok(())
}

/// Writes the main ABI and each dependency ABI to `<dir>/<name>.abi.json`. `dir` is created if missing; existing
/// files are overwritten.
fn write_abis_to_directory(
    dir: &Path,
    main: &leo_abi::Program,
    deps: &IndexMap<String, leo_abi::Program>,
) -> Result<()> {
    std::fs::create_dir_all(dir).map_err(crate::errors::failed_to_write_abi)?;
    let write = |name: &str, abi: &leo_abi::Program| -> Result<()> {
        let path = dir.join(format!("{name}.abi.json"));
        std::fs::write(&path, abi_to_json(abi)?).map_err(crate::errors::failed_to_write_abi)?;
        tracing::info!("ABI written to '{}'.", path.display());
        Ok(())
    };
    write(&main.program, main)?;
    for (name, abi) in deps {
        write(name, abi)?;
    }
    Ok(())
}

/// Disassembles `bytecode` and its declared transitive imports, resolving each dependency from `imports_dir`.
/// Returns the main program plus an ordered list of `(name, AleoProgram)` pairs where dependencies precede their
/// dependents (post-order). Names already loaded as network builtins (e.g. `credits.aleo`) are skipped silently.
fn disassemble_with_imports(
    name: &str,
    bytecode: &str,
    network: NetworkName,
    imports_dir: &Path,
) -> Result<(AleoProgram, Vec<(String, AleoProgram)>), LeoError> {
    match network {
        NetworkName::MainnetV0 => disassemble_with_imports_typed::<MainnetV0>(name, bytecode, imports_dir),
        NetworkName::TestnetV0 => disassemble_with_imports_typed::<TestnetV0>(name, bytecode, imports_dir),
        NetworkName::CanaryV0 => disassemble_with_imports_typed::<CanaryV0>(name, bytecode, imports_dir),
    }
}

/// Typed implementation of [`disassemble_with_imports`] specialised to a concrete `Network`. Parses the main
/// program once, walks its transitive imports from `imports_dir`, hands each parsed program to
/// `leo_disassembler::validate_and_disassemble` in topological order, and finally does the same for the main
/// program once all dependencies are loaded.
fn disassemble_with_imports_typed<N: Network>(
    name: &str,
    bytecode: &str,
    imports_dir: &Path,
) -> Result<(AleoProgram, Vec<(String, AleoProgram)>), LeoError> {
    let mut process = SvmProcess::<N>::load().map_err(crate::errors::failed_to_load_process)?;
    let main = SvmProgram::<N>::from_str(bytecode)
        .map_err(|_| crate::errors::failed_to_parse_aleo_file(name, "invalid Aleo bytecode"))?;
    let deps = load_and_disassemble_imports(&main, imports_dir, &mut process)?;
    let main_aleo = leo_disassembler::validate_and_disassemble(name, main, &mut process)?;
    Ok((main_aleo, deps))
}

/// Walks `program`'s transitive imports from `imports_dir`, hands each parsed program to
/// `leo_disassembler::validate_and_disassemble` in topological order, and returns the disassembled programs in
/// the same order. Names already loaded in `process` are skipped, so network builtins don't need to be on disk.
fn load_and_disassemble_imports<N: Network>(
    program: &SvmProgram<N>,
    imports_dir: &Path,
    process: &mut SvmProcess<N>,
) -> Result<Vec<(String, AleoProgram)>, LeoError> {
    let mut parsed: HashMap<String, SvmProgram<N>> = HashMap::new();
    let mut graph: DiGraph<String> = DiGraph::default();
    let mut worklist: Vec<String> = program
        .imports()
        .iter()
        .filter(|(id, _)| !process.contains_program(id))
        .map(|(id, _)| id.to_string())
        .collect();

    while let Some(name) = worklist.pop() {
        if parsed.contains_key(&name) {
            continue;
        }
        // Try the per-unit layout (`<dir>/<bare>/<name>.aleo`) first, falling back to flat.
        let bare = name.strip_suffix(".aleo").unwrap_or(&name);
        let per_unit = imports_dir.join(bare).join(&name);
        let path = if per_unit.exists() { per_unit } else { imports_dir.join(&name) };
        let text =
            std::fs::read_to_string(&path).map_err(|e| crate::errors::failed_to_read_import(path.display(), e))?;
        let imported = SvmProgram::<N>::from_str(&text)
            .map_err(|_| crate::errors::failed_to_parse_aleo_file(name.clone(), "invalid Aleo bytecode"))?;
        graph.add_node(name.clone());
        for (nested_id, _) in imported.imports() {
            if process.contains_program(nested_id) {
                continue;
            }
            let nested_name = nested_id.to_string();
            graph.add_edge(name.clone(), nested_name.clone());
            worklist.push(nested_name);
        }
        parsed.insert(name, imported);
    }

    let ordered = graph.post_order().map_err(|DiGraphError::CycleDetected(cycle)| {
        let path = cycle.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(" -> ");
        crate::errors::circular_import(path)
    })?;
    let mut disassembled: Vec<(String, AleoProgram)> = Vec::with_capacity(ordered.len());
    for name in ordered {
        // `post_order` returns only names inserted into `graph`, and every such name was inserted into `parsed` on
        // the same loop iteration, so `remove` cannot fail.
        let svm = parsed.remove(&name).expect("post_order yielded a name absent from `parsed`");
        let aleo = leo_disassembler::validate_and_disassemble(&name, svm, process)?;
        disassembled.push((name, aleo));
    }
    Ok(disassembled)
}

#[cfg(test)]
mod tests {
    use super::*;

    use leo_span::create_session_if_not_set_then;

    /// `leo abi` on a `.aleo` file that declares imports must load each dependency from an imports directory before
    /// disassembling.
    #[test]
    fn disassemble_with_imports_loads_dependencies() {
        create_session_if_not_set_then(|_| {
            let dep_src = "\
program dep.aleo;

function id:
    input r0 as u32.private;
    output r0 as u32.private;
";
            let main_src = "\
import dep.aleo;

program importer.aleo;

function call_id:
    input r0 as u32.private;
    call dep.aleo/id r0 into r1;
    output r1 as u32.private;
";

            let dir = tempfile::tempdir().unwrap();
            std::fs::write(dir.path().join("dep.aleo"), dep_src).unwrap();

            let (main, deps) = disassemble_with_imports("importer.aleo", main_src, NetworkName::TestnetV0, dir.path())
                .expect("expected disassembly with imports to succeed");
            assert_eq!(main.stub_id.to_string(), "importer.aleo");
            assert_eq!(main.imports.iter().map(|i| i.to_string()).collect::<Vec<_>>(), vec!["dep.aleo".to_string()]);
            assert_eq!(deps.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), vec!["dep.aleo"]);
            assert_eq!(deps[0].1.stub_id.to_string(), "dep.aleo");
        });
    }

    /// Transitive imports must load in dependency order: a leaf appears before any program that imports it.
    #[test]
    fn disassemble_with_imports_orders_transitive_dependencies() {
        create_session_if_not_set_then(|_| {
            let c_src = "\
program c.aleo;

function id_c:
    input r0 as u32.private;
    output r0 as u32.private;
";
            let b_src = "\
import c.aleo;

program b.aleo;

function id_b:
    input r0 as u32.private;
    call c.aleo/id_c r0 into r1;
    output r1 as u32.private;
";
            let a_src = "\
import b.aleo;

program a.aleo;

function id_a:
    input r0 as u32.private;
    call b.aleo/id_b r0 into r1;
    output r1 as u32.private;
";

            let dir = tempfile::tempdir().unwrap();
            std::fs::write(dir.path().join("b.aleo"), b_src).unwrap();
            std::fs::write(dir.path().join("c.aleo"), c_src).unwrap();

            let (main, deps) = disassemble_with_imports("a.aleo", a_src, NetworkName::TestnetV0, dir.path())
                .expect("expected transitive disassembly to succeed");
            assert_eq!(main.stub_id.to_string(), "a.aleo");
            assert_eq!(deps.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), vec!["c.aleo", "b.aleo"]);
        });
    }

    /// Network builtins like `credits.aleo` are already in the `Process`, so they must be skipped silently — the
    /// command must not require them to be present in the imports directory.
    #[test]
    fn disassemble_with_imports_handles_network_builtin() {
        create_session_if_not_set_then(|_| {
            let main_src = "\
import credits.aleo;

program network_user.aleo;

function noop:
    input r0 as u32.private;
    output r0 as u32.private;
";

            let dir = tempfile::tempdir().unwrap();
            let (main, deps) =
                disassemble_with_imports("network_user.aleo", main_src, NetworkName::TestnetV0, dir.path())
                    .expect("expected disassembly to succeed without credits.aleo on disk");
            assert_eq!(main.stub_id.to_string(), "network_user.aleo");
            assert!(deps.is_empty(), "network builtins must not appear in the dependency list");
        });
    }

    /// A cycle in the declared imports must surface a descriptive error (with the offending names) rather than a
    /// generic message or a panic.
    #[test]
    fn disassemble_with_imports_reports_circular_dependency() {
        create_session_if_not_set_then(|_| {
            let a_src = "\
import b.aleo;

program a.aleo;

function id_a:
    input r0 as u32.private;
    call b.aleo/id_b r0 into r1;
    output r1 as u32.private;
";
            let b_src = "\
import a.aleo;

program b.aleo;

function id_b:
    input r0 as u32.private;
    call a.aleo/id_a r0 into r1;
    output r1 as u32.private;
";

            let dir = tempfile::tempdir().unwrap();
            std::fs::write(dir.path().join("a.aleo"), a_src).unwrap();
            std::fs::write(dir.path().join("b.aleo"), b_src).unwrap();

            let err = disassemble_with_imports("a.aleo", a_src, NetworkName::TestnetV0, dir.path())
                .expect_err("expected disassembly to fail on a circular import");
            let msg = err.to_string();
            assert!(msg.contains("circular import"), "unexpected error: {msg}");
            assert!(msg.contains("a.aleo") && msg.contains("b.aleo"), "cycle path not surfaced: {msg}");
        });
    }

    /// A program that lists an import not present in the imports directory must surface a clear error rather than
    /// panicking.
    #[test]
    fn disassemble_with_imports_reports_missing_dependency() {
        create_session_if_not_set_then(|_| {
            let main_src = "\
import dep.aleo;

program importer.aleo;

function call_id:
    input r0 as u32.private;
    call dep.aleo/id r0 into r1;
    output r1 as u32.private;
";
            let dir = tempfile::tempdir().unwrap();
            let err = disassemble_with_imports("importer.aleo", main_src, NetworkName::TestnetV0, dir.path())
                .expect_err("expected disassembly to fail when an import is missing");
            assert!(err.to_string().contains("dep.aleo"), "unexpected error: {err}");
        });
    }

    /// Main bytecode that fails grammatical parsing must surface a clear error naming the main program.
    #[test]
    fn disassemble_with_imports_rejects_malformed_main() {
        create_session_if_not_set_then(|_| {
            let dir = tempfile::tempdir().unwrap();
            let err = disassemble_with_imports(
                "importer.aleo",
                "this is not valid Aleo bytecode",
                NetworkName::TestnetV0,
                dir.path(),
            )
            .expect_err("expected disassembly to fail on malformed main bytecode");
            assert!(err.to_string().contains("importer.aleo"), "unexpected error: {err}");
        });
    }

    /// A dependency file whose contents are not valid Aleo bytecode must surface a clear error naming the dep.
    #[test]
    fn disassemble_with_imports_rejects_malformed_dependency() {
        create_session_if_not_set_then(|_| {
            let main_src = "\
import dep.aleo;

program importer.aleo;

function call_id:
    input r0 as u32.private;
    call dep.aleo/id r0 into r1;
    output r1 as u32.private;
";
            let dir = tempfile::tempdir().unwrap();
            std::fs::write(dir.path().join("dep.aleo"), "this is not valid Aleo bytecode").unwrap();
            let err = disassemble_with_imports("importer.aleo", main_src, NetworkName::TestnetV0, dir.path())
                .expect_err("expected disassembly to fail on malformed dependency bytecode");
            assert!(err.to_string().contains("dep.aleo"), "unexpected error: {err}");
        });
    }

    /// A diamond-shaped import graph (`a → {b, c}`, `b → d`, `c → d`) must yield `d` exactly once, before both
    /// `b` and `c`. Exercises the worklist's already-parsed short-circuit.
    #[test]
    fn disassemble_with_imports_dedups_diamond_imports() {
        create_session_if_not_set_then(|_| {
            let d_src = "\
program d.aleo;

function id_d:
    input r0 as u32.private;
    output r0 as u32.private;
";
            let b_src = "\
import d.aleo;

program b.aleo;

function id_b:
    input r0 as u32.private;
    call d.aleo/id_d r0 into r1;
    output r1 as u32.private;
";
            let c_src = "\
import d.aleo;

program c.aleo;

function id_c:
    input r0 as u32.private;
    call d.aleo/id_d r0 into r1;
    output r1 as u32.private;
";
            let a_src = "\
import b.aleo;
import c.aleo;

program a.aleo;

function combine:
    input r0 as u32.private;
    call b.aleo/id_b r0 into r1;
    call c.aleo/id_c r1 into r2;
    output r2 as u32.private;
";

            let dir = tempfile::tempdir().unwrap();
            std::fs::write(dir.path().join("d.aleo"), d_src).unwrap();
            std::fs::write(dir.path().join("b.aleo"), b_src).unwrap();
            std::fs::write(dir.path().join("c.aleo"), c_src).unwrap();

            let (main, deps) = disassemble_with_imports("a.aleo", a_src, NetworkName::TestnetV0, dir.path())
                .expect("expected diamond-imports disassembly to succeed");
            assert_eq!(main.stub_id.to_string(), "a.aleo");

            let names: Vec<&str> = deps.iter().map(|(n, _)| n.as_str()).collect();
            assert_eq!(names.iter().filter(|&&n| n == "d.aleo").count(), 1, "d.aleo not deduped: {names:?}");
            assert_eq!(names.len(), 3, "expected exactly 3 deps, got: {names:?}");
            let pos = |needle: &str| {
                names.iter().position(|&n| n == needle).unwrap_or_else(|| panic!("{needle} missing from {names:?}"))
            };
            assert!(pos("d.aleo") < pos("b.aleo"), "topo order violated: {names:?}");
            assert!(pos("d.aleo") < pos("c.aleo"), "topo order violated: {names:?}");
        });
    }

    /// A network builtin reached transitively (`a → b`, `b → credits`) must be filtered out of the dependency
    /// walk too, not just at the top level. Exercises the nested-import builtin filter.
    #[test]
    fn disassemble_with_imports_skips_nested_network_builtin() {
        create_session_if_not_set_then(|_| {
            let b_src = "\
import credits.aleo;

program b.aleo;

function id_b:
    input r0 as u32.private;
    output r0 as u32.private;
";
            let a_src = "\
import b.aleo;

program a.aleo;

function id_a:
    input r0 as u32.private;
    call b.aleo/id_b r0 into r1;
    output r1 as u32.private;
";

            let dir = tempfile::tempdir().unwrap();
            std::fs::write(dir.path().join("b.aleo"), b_src).unwrap();

            let (main, deps) = disassemble_with_imports("a.aleo", a_src, NetworkName::TestnetV0, dir.path())
                .expect("expected disassembly with transitive credits.aleo import to succeed");
            assert_eq!(main.stub_id.to_string(), "a.aleo");
            let names: Vec<&str> = deps.iter().map(|(n, _)| n.as_str()).collect();
            assert_eq!(names, vec!["b.aleo"], "credits.aleo must be skipped silently, got: {names:?}");
        });
    }
}
