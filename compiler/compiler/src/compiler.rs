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

//! The compiler for Leo programs.
//!
//! The [`Compiler`] type compiles Leo programs into R1CS circuits.

use crate::{AstSnapshots, CompilerOptions};

pub use leo_ast::{Ast, CompiledPrograms, Program};
use leo_ast::{NetworkName, NodeBuilder, Stub};
use leo_errors::{CompilerError, Handler, Result};
use leo_passes::*;
use leo_span::{Symbol, source_map::FileName, with_session_globals};

use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use indexmap::{IndexMap, IndexSet};
use walkdir::WalkDir;

/// The primary entry point of the Leo compiler.
pub struct Compiler {
    /// The path to where the compiler outputs all generated files.
    output_directory: PathBuf,
    /// The program name,
    pub program_name: Option<String>,
    /// Options configuring compilation.
    compiler_options: CompilerOptions,
    /// State.
    state: CompilerState,
    /// The stubs for imported programs.
    import_stubs: IndexMap<Symbol, Stub>,
    /// How many statements were in the AST before DCE?
    pub statements_before_dce: u32,
    /// How many statements were in the AST after DCE?
    pub statements_after_dce: u32,
}

impl Compiler {
    pub fn parse(&mut self, source: &str, filename: FileName, modules: &[(&str, FileName)]) -> Result<()> {
        // Register the source in the source map.
        let source_file = with_session_globals(|s| s.source_map.new_source(source, filename.clone()));

        // Register the sources of all the modules in the source map.
        let modules = modules
            .iter()
            .map(|(source, filename)| with_session_globals(|s| s.source_map.new_source(source, filename.clone())))
            .collect::<Vec<_>>();

        // Use the parser to construct the abstract syntax tree (ast).
        self.state.ast = leo_parser::parse_ast(
            self.state.handler.clone(),
            &self.state.node_builder,
            &source_file,
            &modules,
            self.state.network,
        )?;

        // Check that the name of its program scope matches the expected name.
        // Note that parsing enforces that there is exactly one program scope in a file.
        let program_scope = self.state.ast.ast.program_scopes.values().next().unwrap();
        if self.program_name.is_none() {
            self.program_name = Some(program_scope.program_id.name.to_string());
        } else if self.program_name != Some(program_scope.program_id.name.to_string()) {
            return Err(CompilerError::program_name_should_match_file_name(
                program_scope.program_id.name,
                // If this is a test, use the filename as the expected name.
                if self.state.is_test {
                    format!(
                        "`{}` (the test file name)",
                        filename.to_string().split("/").last().expect("Could not get file name")
                    )
                } else {
                    format!("`{}` (specified in `program.json`)", self.program_name.as_ref().unwrap())
                },
                program_scope.program_id.name.span,
            )
            .into());
        }

        if self.compiler_options.initial_ast {
            self.write_ast_to_json("initial.json")?;
            self.write_ast("initial.ast")?;
        }

        Ok(())
    }

    /// Simple wrapper around `parse` that also returns the AST.
    pub fn parse_and_return_ast(
        &mut self,
        source: &str,
        filename: FileName,
        modules: &[(&str, FileName)],
    ) -> Result<Program> {
        // Parse the program.
        self.parse(source, filename, modules)?;

        Ok(self.state.ast.ast.clone())
    }

    /// Returns a new Leo compiler.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expected_program_name: Option<String>,
        is_test: bool,
        handler: Handler,
        node_builder: Rc<NodeBuilder>,
        output_directory: PathBuf,
        compiler_options: Option<CompilerOptions>,
        import_stubs: IndexMap<Symbol, Stub>,
        network: NetworkName,
    ) -> Self {
        Self {
            state: CompilerState {
                handler,
                node_builder: Rc::clone(&node_builder),
                is_test,
                network,
                ..Default::default()
            },
            output_directory,
            program_name: expected_program_name,
            compiler_options: compiler_options.unwrap_or_default(),
            import_stubs,
            statements_before_dce: 0,
            statements_after_dce: 0,
        }
    }

    fn do_pass<P: Pass>(&mut self, input: P::Input) -> Result<P::Output> {
        let output = P::do_pass(input, &mut self.state)?;

        let write = match &self.compiler_options.ast_snapshots {
            AstSnapshots::All => true,
            AstSnapshots::Some(passes) => passes.contains(P::NAME),
        };

        if write {
            self.write_ast_to_json(&format!("{}.json", P::NAME))?;
            self.write_ast(&format!("{}.ast", P::NAME))?;
        }

        Ok(output)
    }

    /// Runs the compiler stages.
    pub fn intermediate_passes(&mut self) -> Result<()> {
        let type_checking_config = TypeCheckingInput::new(self.state.network);

        self.do_pass::<NameValidation>(())?;

        self.do_pass::<GlobalVarsCollection>(())?;

        self.do_pass::<PathResolution>(())?;

        self.do_pass::<GlobalItemsCollection>(())?;

        self.do_pass::<TypeChecking>(type_checking_config.clone())?;

        self.do_pass::<Disambiguate>(())?;

        self.do_pass::<ProcessingAsync>(type_checking_config.clone())?;

        self.do_pass::<StaticAnalyzing>(())?;

        self.do_pass::<ConstPropUnrollAndMorphing>(type_checking_config.clone())?;

        self.do_pass::<StorageLowering>(type_checking_config.clone())?;

        self.do_pass::<OptionLowering>(type_checking_config)?;

        self.do_pass::<ProcessingScript>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: true })?;

        self.do_pass::<Destructuring>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: false })?;

        self.do_pass::<WriteTransforming>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: false })?;

        self.do_pass::<Flattening>(())?;

        self.do_pass::<FunctionInlining>(())?;

        // Flattening may produce ternary expressions not in SSA form.
        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: false })?;

        self.do_pass::<SsaConstPropagation>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: false })?;

        self.do_pass::<CommonSubexpressionEliminating>(())?;

        let output = self.do_pass::<DeadCodeEliminating>(())?;
        self.statements_before_dce = output.statements_before;
        self.statements_after_dce = output.statements_after;

        Ok(())
    }

    /// Compiles a program from a given source string and a list of module sources.
    ///
    /// # Arguments
    ///
    /// * `source` - The main source code as a string slice.
    /// * `filename` - The name of the main source file.
    /// * `modules` - A vector of tuples where each tuple contains:
    ///     - A module source as a string slice.
    ///     - Its associated `FileName`.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the generated bytecode if compilation succeeds.
    /// * `Err(CompilerError)` if any stage of the pipeline fails.
    pub fn compile(
        &mut self,
        source: &str,
        filename: FileName,
        modules: &Vec<(&str, FileName)>,
    ) -> Result<CompiledPrograms> {
        // Parse the program.
        self.parse(source, filename, modules)?;
        // Merge the stubs into the AST.
        self.add_import_stubs()?;
        // Run the intermediate compiler stages.
        self.intermediate_passes()?;
        // Run code generation.
        CodeGenerating::do_pass((), &mut self.state)
    }

    /// Reads the main source file and all module files in the same directory tree.
    ///
    /// This helper walks all `.leo` files under `source_directory` (excluding the main file itself),
    /// reads their contents, and returns:
    /// - The main file’s source as a `String`.
    /// - A vector of module tuples `(String, FileName)` suitable for compilation or parsing.
    ///
    /// # Arguments
    ///
    /// * `entry_file_path` - The main source file.
    /// * `source_directory` - The directory root for discovering `.leo` module files.
    ///
    /// # Errors
    ///
    /// Returns `Err(CompilerError)` if reading any file fails.
    fn read_sources_and_modules(
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
    ) -> Result<(String, Vec<(String, FileName)>)> {
        // Read the contents of the main source file.
        let source = fs::read_to_string(&entry_file_path)
            .map_err(|e| CompilerError::file_read_error(entry_file_path.as_ref().display().to_string(), e))?;

        // Walk all files under source_directory recursively, excluding the main source file itself.
        let files = WalkDir::new(source_directory)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file()
                    && e.path() != entry_file_path.as_ref()
                    && e.path().extension() == Some(OsStr::new("leo"))
            })
            .collect::<Vec<_>>();

        // Read all module files and pair with FileName immediately
        let mut modules = Vec::new();
        for file in &files {
            let module_source = fs::read_to_string(file.path())
                .map_err(|e| CompilerError::file_read_error(file.path().display().to_string(), e))?;
            modules.push((module_source, FileName::Real(file.path().into())));
        }

        Ok((source, modules))
    }

    /// Compiles a program from a source file and its associated module files in the same directory tree.
    pub fn compile_from_directory(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
    ) -> Result<CompiledPrograms> {
        let (source, modules_owned) = Self::read_sources_and_modules(&entry_file_path, &source_directory)?;

        // Convert owned module sources into temporary (&str, FileName) tuples.
        let module_refs: Vec<(&str, FileName)> =
            modules_owned.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

        // Compile the main source along with all collected modules.
        self.compile(&source, FileName::Real(entry_file_path.as_ref().into()), &module_refs)
    }

    /// Parses a program from a source file and its associated module files in the same directory tree.
    pub fn parse_from_directory(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
    ) -> Result<Program> {
        let (source, modules_owned) = Self::read_sources_and_modules(&entry_file_path, &source_directory)?;

        // Convert owned module sources into temporary (&str, FileName) tuples.
        let module_refs: Vec<(&str, FileName)> =
            modules_owned.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

        // Parse the main source along with all collected modules.
        self.parse(&source, FileName::Real(entry_file_path.as_ref().into()), &module_refs)?;
        Ok(self.state.ast.ast.clone())
    }

    /// Writes the AST to a JSON file.
    fn write_ast_to_json(&self, file_suffix: &str) -> Result<()> {
        // Remove `Span`s if they are not enabled.
        if self.compiler_options.ast_spans_enabled {
            self.state.ast.to_json_file(
                self.output_directory.clone(),
                &format!("{}.{file_suffix}", self.program_name.as_ref().unwrap()),
            )?;
        } else {
            self.state.ast.to_json_file_without_keys(
                self.output_directory.clone(),
                &format!("{}.{file_suffix}", self.program_name.as_ref().unwrap()),
                &["_span", "span"],
            )?;
        }
        Ok(())
    }

    /// Writes the AST to a file (Leo syntax, not JSON).
    fn write_ast(&self, file_suffix: &str) -> Result<()> {
        let filename = format!("{}.{file_suffix}", self.program_name.as_ref().unwrap());
        let full_filename = self.output_directory.join(&filename);
        let contents = self.state.ast.ast.to_string();
        fs::write(&full_filename, contents).map_err(|e| CompilerError::failed_ast_file(full_filename.display(), e))?;
        Ok(())
    }

    /// Resolves and registers all import stubs for the current program.
    ///
    /// This method performs a graph traversal over the program’s import relationships to:
    /// 1. Establish parent–child relationships between stubs based on imports.
    /// 2. Collect all reachable stubs in traversal order.
    /// 3. Store the explored stubs back into `self.state.ast.ast.stubs`.
    ///
    /// The traversal starts from the imports of the main program and recursively follows
    /// their transitive dependencies. Any missing stub during traversal results in an error.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all imports are successfully resolved and stubs are collected.
    /// * `Err(CompilerError)` if any imported program cannot be found.
    pub fn add_import_stubs(&mut self) -> Result<()> {
        // Track which programs we've already processed.
        let mut explored = IndexSet::<Symbol>::new();

        // Initialize the exploration queue with the main program’s direct imports.
        let mut to_explore: Vec<Symbol> = self.state.ast.ast.imports.keys().cloned().collect();

        // If this is a named program, set the main program as the parent of its direct imports.
        if let Some(main_program_name) = self.program_name.clone() {
            let main_symbol = Symbol::intern(&main_program_name);
            for import in self.state.ast.ast.imports.keys() {
                if let Some(child_stub) = self.import_stubs.get_mut(import) {
                    child_stub.add_parent(main_symbol);
                }
            }
        }

        // Traverse the import graph breadth-first, collecting dependencies.
        while let Some(import_symbol) = to_explore.pop() {
            // Mark this import as explored.
            explored.insert(import_symbol);

            // Look up the corresponding stub.
            let Some(stub) = self.import_stubs.get(&import_symbol) else {
                return Err(CompilerError::imported_program_not_found(
                    self.program_name.as_ref().unwrap(),
                    import_symbol,
                    self.state.ast.ast.imports[&import_symbol],
                )
                .into());
            };

            for child_symbol in stub.imports().cloned().collect::<Vec<_>>() {
                // Record parent relationship.
                if let Some(child_stub) = self.import_stubs.get_mut(&child_symbol) {
                    child_stub.add_parent(import_symbol);
                }

                // Schedule child for exploration if not yet visited.
                if explored.insert(child_symbol) {
                    to_explore.push(child_symbol);
                }
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
