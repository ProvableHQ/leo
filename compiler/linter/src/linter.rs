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

use std::{ffi::OsStr, fs, path::Path};

use indexmap::{IndexMap, IndexSet};
use leo_ast::{NetworkName, Stub};
use leo_errors::{CompilerError, Handler, LeoError, Result};
use leo_parser::{
    conversions::{to_main, to_module},
    parse_cst,
};
use leo_parser_lossless::SyntaxNode;
use leo_passes::{
    CompilerState,
    Pass,
    PathResolution,
    StaticAnalyzing,
    SymbolTableCreation,
    TypeChecking,
    TypeCheckingInput,
};
use leo_span::{Symbol, source_map::FileName, with_session_globals};
use walkdir::WalkDir;

use crate::{
    diagnostics::DiagnosticReport,
    passes::{
        early::{EarlyLinting, EarlyLintingInput},
        late::LateLinting,
    },
};

/// The primary entry point of the Leo linter.
pub struct Linter {
    state: CompilerState,
    program_name: Option<String>,
    import_stubs: IndexMap<Symbol, Stub>,
}

impl Linter {
    pub fn new(
        program_name: Option<String>,
        handler: Handler,
        is_test: bool,
        import_stubs: IndexMap<Symbol, Stub>,
        network: NetworkName,
    ) -> Linter {
        let state = CompilerState { is_test, handler, network, ..Default::default() };
        Linter { state, program_name, import_stubs }
    }

    pub fn lint_leo_source_directory(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        modules_directory: Option<impl AsRef<Path>>,
    ) -> Result<()> {
        // Walk all files under source_directory recursively, excluding the main source file itself.
        let files = if let Some(source_directory) = modules_directory {
            WalkDir::new(source_directory)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path() != entry_file_path.as_ref()
                        && e.path().extension() == Some(OsStr::new("leo"))
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let main = if entry_file_path.as_ref().try_exists().map_err(|e| LeoError::Anyhow(e.into()))? {
            // Read the contents of the main source file.
            let source = fs::read_to_string(&entry_file_path)
                .map_err(|e| CompilerError::file_read_error(entry_file_path.as_ref().display().to_string(), e))?;
            Some((source, FileName::Real(entry_file_path.as_ref().to_owned())))
        } else {
            None
        };

        let mut module_sources = Vec::new(); // Keep Strings alive for valid borrowing
        let mut modules = Vec::new(); // Parsed (source, filename) tuples for compilation

        // Read all module files and store their contents
        for file in &files {
            let source = fs::read_to_string(file.path())
                .map_err(|e| CompilerError::file_read_error(file.path().display().to_string(), e))?;
            module_sources.push(source); // Keep the String alive
        }

        // Create tuples of (&str, FileName) for the compiler
        for (i, file) in files.iter().enumerate() {
            let source = &module_sources[i]; // Borrow from the alive String
            modules.push((&source[..], FileName::Real(file.path().into())));
        }

        self.lint(main.as_ref().map(|(s, f)| (s.as_str(), f.clone())), modules.as_slice())
    }

    pub(crate) fn lint(&mut self, source: Option<(&str, FileName)>, modules: &[(&str, FileName)]) -> Result<()> {
        // Register the source in the source map.
        let source_file = with_session_globals(|s| source.map(|source| s.source_map.new_source(source.0, source.1)));

        // Register the sources of all the modules in the source map.
        let modules = modules
            .iter()
            .map(|(source, filename)| with_session_globals(|s| s.source_map.new_source(source, filename.clone())))
            .collect::<Vec<_>>();

        let parse_tree = parse_cst(&self.state.handler, source_file.as_deref(), &modules)?;

        let report =
            self.lint_early(parse_tree.0.as_ref(), parse_tree.1.iter().map(|m| &m.1).collect::<Vec<_>>().as_slice())?;

        let program_name = if let Some(main_tree) = parse_tree.0 {
            let program = to_main(&main_tree, &self.state.node_builder, &self.state.handler)?;
            let program_name = *program.program_scopes.first().unwrap().0;
            self.state.ast.ast = program;
            program_name
        } else {
            Symbol::default()
        };

        for (key, module_tree) in parse_tree.1 {
            let module_ast =
                to_module(&module_tree, &self.state.node_builder, program_name, key.clone(), &self.state.handler)?;
            self.state.ast.ast.modules.insert(key, module_ast);
        }

        if let Some(program_scope) = self.state.ast.ast.program_scopes.values().next()
            && self.program_name.is_none()
        {
            self.program_name = Some(program_scope.program_id.name.to_string());
        }

        self.add_import_stubs()?;

        self.intermediate_passes()?;

        self.lint_late(&report)?;

        for triggered_lint in report.consume() {
            self.state.handler.emit_warning(triggered_lint);
        }

        Ok(())
    }

    fn do_pass<P: Pass>(&mut self, input: P::Input) -> Result<P::Output> {
        let output = P::do_pass(input, &mut self.state)?;
        Ok(output)
    }

    fn intermediate_passes(&mut self) -> Result<()> {
        let type_checking_config = TypeCheckingInput::new(self.state.network);

        self.do_pass::<PathResolution>(())?;

        self.do_pass::<SymbolTableCreation>(())?;

        self.do_pass::<TypeChecking>(type_checking_config.clone())?;

        self.do_pass::<StaticAnalyzing>(())?;

        Ok(())
    }

    fn lint_early(&mut self, main_node: Option<&SyntaxNode>, module_nodes: &[&SyntaxNode]) -> Result<DiagnosticReport> {
        let input = EarlyLintingInput { module_trees: module_nodes, program_tree: main_node };
        self.do_pass::<EarlyLinting>(input)
    }

    fn lint_late(&mut self, report: &DiagnosticReport) -> Result<()> {
        self.do_pass::<LateLinting>(report)?;
        Ok(())
    }

    /// Merge the imported stubs which are dependencies of the current program into the AST
    /// in topological order.
    pub fn add_import_stubs(&mut self) -> Result<()> {
        let mut explored = IndexSet::<Symbol>::new();
        let mut to_explore: Vec<Symbol> = self.state.ast.ast.imports.keys().cloned().collect();

        while let Some(import) = to_explore.pop() {
            explored.insert(import);
            if let Some(stub) = self.import_stubs.get(&import) {
                for new_import_id in stub.imports.iter() {
                    if !explored.contains(&new_import_id.name.name) {
                        to_explore.push(new_import_id.name.name);
                    }
                }
            } else {
                if cfg!(test) {
                    return Ok(());
                }
                return Err(CompilerError::imported_program_not_found(
                    self.program_name.as_ref().unwrap(),
                    import,
                    self.state.ast.ast.imports[&import].1,
                )
                .into());
            }
        }

        // Iterate in the order of `import_stubs` to make sure they
        // stay topologically sorted.
        self.state.ast.ast.stubs = self
            .import_stubs
            .iter()
            .filter(|(symbol, _stub)| explored.contains(*symbol))
            .map(|(symbol, stub)| (*symbol, stub.clone()))
            .collect();
        Ok(())
    }
}
