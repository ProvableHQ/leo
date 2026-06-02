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

pub use leo_ast::{Ast, DiGraph, Program};
use leo_ast::{Library, NetworkName, NodeBuilder, Stub};
use leo_errors::{Handler, Result};
use leo_passes::*;
use leo_span::{Span, Symbol, file_source::FileSource, source_map::FileName, with_session_globals};

use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use indexmap::IndexMap;

// On wasm we never write artifacts to disk; the AST-snapshot writers below
// become no-ops there, so suppress dead-code lints for the path field.
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

/// Borrowed frontend state after parsing and semantic frontend passes complete.
pub struct FrontendAnalysis<'a> {
    /// Parsed AST after import-stub registration and frontend passes.
    pub ast: &'a Ast,
    /// Name-resolution state produced by the frontend pipeline.
    pub symbol_table: &'a SymbolTable,
    /// Type information produced by semantic frontend passes.
    pub type_table: &'a TypeTable,
}

/// A single compiled program with its bytecode and ABI.
pub struct CompiledProgram {
    /// The program name (without `.aleo` suffix).
    pub name: String,
    /// The generated Aleo bytecode.
    pub bytecode: String,
    /// The ABI describing the program's public interface.
    pub abi: leo_abi::Program,
}

/// The result of compiling a Leo program.
pub struct Compiled {
    /// The primary program that was compiled.
    pub primary: CompiledProgram,
    /// Compiled programs for imports.
    pub imports: Vec<CompiledProgram>,
    /// Interface ABIs from the primary program.
    pub interfaces: Vec<leo_abi::interfaces::CompiledInterface>,
}

/// The primary entry point of the Leo compiler.
pub struct Compiler {
    /// The path to where the compiler outputs all generated files.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    output_directory: PathBuf,
    /// The name of the compilation unit (program or library).
    pub unit_name: Option<String>,
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
    /// Return the network selected for this compiler instance.
    pub fn network(&self) -> NetworkName {
        self.state.network
    }

    /// Parses the given source into a program AST and stores it in the compiler state.
    ///
    /// The source file and any provided module sources are first registered in the
    /// session source map so spans can be resolved correctly. The parser then
    /// constructs the program AST from the main source and its modules.
    ///
    /// After parsing, this verifies that the program scope name matches the expected
    /// program name (from `program.json` or the test filename). The resulting AST is
    /// stored in `self.state.ast`, and optionally written to disk if configured.
    pub fn parse_program(&mut self, source: &str, filename: FileName, modules: &[(&str, FileName)]) -> Result<()> {
        // Register the source in the source map.
        let source_file = with_session_globals(|s| s.source_map.new_source(source, filename.clone()));

        // Register the sources of all the modules in the source map.
        let modules = modules
            .iter()
            .map(|(source, filename)| with_session_globals(|s| s.source_map.new_source(source, filename.clone())))
            .collect::<Vec<_>>();

        // Use the parser to construct the abstract syntax tree (ast).
        let program = leo_parser::parse_program(
            self.state.handler.clone(),
            &self.state.node_builder,
            &source_file,
            &modules,
            self.state.network,
        )?;

        // Check that the name of its program scope matches the expected name.
        // Note that parsing enforces that there is exactly one program scope in a file.
        let program_scope = program.program_scopes.values().next().unwrap();
        if let Some(unit_name) = &self.unit_name {
            if unit_name != &program_scope.program_id.as_symbol().to_string() {
                return Err(crate::errors::program_name_should_match_file_name(
                    program_scope.program_id.as_symbol(),
                    // If this is a test, use the filename as the expected name.
                    if self.state.is_test {
                        format!(
                            "`{}` (the test file name)",
                            filename.to_string().split("/").last().expect("Could not get file name")
                        )
                    } else {
                        format!("`{unit_name}` (specified in `program.json`)")
                    },
                    program_scope.program_id.span(),
                )
                .into());
            }
        } else {
            self.unit_name = Some(program_scope.program_id.as_symbol().to_string());
        }

        self.state.ast = Ast::Program(program);

        if self.compiler_options.initial_ast {
            self.write_ast_to_json("initial.json")?;
            self.write_ast("initial.ast")?;
        }

        Ok(())
    }

    /// Simple wrapper around `parse_program` that also returns a program AST.
    pub fn parse_and_return_program(
        &mut self,
        source: &str,
        filename: FileName,
        modules: &[(&str, FileName)],
    ) -> Result<Program> {
        // Parse the program.
        self.parse_program(source, filename, modules)?;

        match &self.state.ast {
            Ast::Program(program) => Ok(program.clone()),
            Ast::Library(_) => unreachable!("expected Program AST"),
        }
    }

    /// Simple wrapper around `parse_library` that also returns a library AST.
    pub fn parse_and_return_library(
        &mut self,
        library_name: &str,
        source: &str,
        filename: FileName,
        modules: &[(&str, FileName)],
    ) -> Result<Library> {
        self.parse_library(Symbol::intern(library_name), source, filename, modules)?;

        match &self.state.ast {
            Ast::Program(_) => unreachable!("expected Library AST"),
            Ast::Library(library) => Ok(library.clone()),
        }
    }

    /// Parses a library source (and its submodules) into a library AST.
    ///
    /// All source strings are registered in the session source map so span information
    /// can be resolved correctly. The resulting AST is stored in `self.state.ast`.
    pub fn parse_library(
        &mut self,
        library_name: Symbol,
        source: &str,
        filename: FileName,
        modules: &[(&str, FileName)],
    ) -> Result<()> {
        let source_file = with_session_globals(|s| s.source_map.new_source(source, filename.clone()));

        // Register each module source in the source map.
        let module_files = modules
            .iter()
            .map(|(src, name)| with_session_globals(|s| s.source_map.new_source(src, name.clone())))
            .collect::<Vec<_>>();

        self.state.ast = Ast::Library(leo_parser::parse_library(
            self.state.handler.clone(),
            &self.state.node_builder,
            library_name,
            &source_file,
            &module_files,
            self.state.network,
        )?);

        // Downstream passes (e.g. `add_import_stubs`) read `unit_name` to identify the
        // current compilation target. Libraries don't embed their own name in the source the
        // way programs do, so adopt the name supplied by the caller if none was pre-set.
        if self.unit_name.is_none() {
            self.unit_name = Some(library_name.to_string());
        }

        Ok(())
    }

    /// Parses a package entry file, merges import stubs when applicable, and runs frontend passes.
    ///
    /// Unlike the full compile pipeline, this stops after semantic frontend
    /// analysis and returns borrowed access to the AST, symbol table, and type
    /// table. The LSP uses this to build semantic indices without running code
    /// generation or writing artifacts to disk.
    pub fn analyze_frontend_from_directory_with_file_source(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
        file_source: &dyn FileSource,
    ) -> Result<FrontendAnalysis<'_>> {
        self.analyze_frontend_from_directory_with_file_source_and_check(
            entry_file_path,
            source_directory,
            file_source,
            || Ok(()),
        )
    }

    /// Equivalent to [`Self::analyze_frontend_from_directory_with_file_source`], but checks
    /// `should_continue` at parse and pass boundaries so editor tooling can abandon
    /// stale work before completing the entire frontend pipeline.
    pub fn analyze_frontend_from_directory_with_file_source_and_check<C>(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
        file_source: &dyn FileSource,
        mut should_continue: C,
    ) -> Result<FrontendAnalysis<'_>>
    where
        C: FnMut() -> Result<()>,
    {
        should_continue()?;
        let is_library = self.unit_name.as_deref().is_some_and(|name| !name.ends_with(".aleo"));

        if is_library {
            let library_name = Symbol::intern(self.unit_name.as_deref().expect("library analysis requires a name"));
            self.parse_library_from_directory_with_file_source(
                library_name,
                &entry_file_path,
                &source_directory,
                file_source,
            )?;
        } else {
            self.parse_program_from_directory_with_file_source(&entry_file_path, &source_directory, file_source)?;
            self.add_import_stubs()?;
        }

        // Re-check after parsing/import setup so editor callers can drop stale
        // work before entering the semantic pass pipeline.
        should_continue()?;
        self.frontend_passes_with_check(&mut should_continue)?;

        Ok(FrontendAnalysis {
            ast: &self.state.ast,
            symbol_table: &self.state.symbol_table,
            type_table: &self.state.type_table,
        })
    }

    /// Returns a new Leo compiler.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expected_unit_name: Option<String>,
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
            unit_name: expected_unit_name,
            compiler_options: compiler_options.unwrap_or_default(),
            import_stubs,
            statements_before_dce: 0,
            statements_after_dce: 0,
        }
    }

    /// Run a compiler pass without an external cancellation check.
    pub fn do_pass<P: Pass>(&mut self, input: P::Input) -> Result<P::Output> {
        self.do_pass_with_check::<P, _>(input, &mut || Ok(()))
    }

    /// Runs a compiler pass and checks whether the caller still wants the
    /// result once the pass and any requested snapshots have completed.
    fn do_pass_with_check<P: Pass, C>(&mut self, input: P::Input, should_continue: &mut C) -> Result<P::Output>
    where
        C: FnMut() -> Result<()>,
    {
        let output = P::do_pass(input, &mut self.state)?;

        let write = match &self.compiler_options.ast_snapshots {
            AstSnapshots::All => true,
            AstSnapshots::Some(passes) => passes.contains(P::NAME),
        };

        if write {
            self.write_ast_to_json(&format!("{}.json", P::NAME))?;
            self.write_ast(&format!("{}.ast", P::NAME))?;
        }

        should_continue()?;
        Ok(output)
    }

    /// Runs all frontend passes: NameValidation through StaticAnalyzing.
    pub fn frontend_passes(&mut self) -> Result<()> {
        self.frontend_passes_with_check(|| Ok(()))
    }

    /// Runs all frontend passes while checking whether the caller still wants the result.
    pub fn frontend_passes_with_check<C>(&mut self, mut should_continue: C) -> Result<()>
    where
        C: FnMut() -> Result<()>,
    {
        // Bail out if the parser already found errors.  The error-recovering parser may have
        // produced ErrExpression nodes in the AST, which would cause panics in later passes.
        self.state.handler.last_err()?;

        self.do_pass_with_check::<NameValidation, _>((), &mut should_continue)?;
        self.do_pass_with_check::<GlobalVarsCollection, _>((), &mut should_continue)?;
        self.do_pass_with_check::<PathResolution, _>((), &mut should_continue)?;
        self.do_pass_with_check::<GlobalItemsCollection, _>((), &mut should_continue)?;
        self.do_pass_with_check::<CheckInterfaces, _>((), &mut should_continue)?;
        self.do_pass_with_check::<TypeChecking, _>(TypeCheckingInput::new(self.state.network), &mut should_continue)?;
        self.do_pass_with_check::<Disambiguate, _>((), &mut should_continue)?;
        self.do_pass_with_check::<CeiAnalyzing, _>((), &mut should_continue)?;
        self.do_pass_with_check::<ProcessingAsync, _>(
            TypeCheckingInput::new(self.state.network),
            &mut should_continue,
        )?;
        self.do_pass_with_check::<StaticAnalyzing, _>((), &mut should_continue)?;
        Ok(())
    }

    /// Runs the compiler stages.
    ///
    /// Returns the generated ABIs (primary and imports), which are captured
    /// immediately after monomorphisation to ensure all types are resolved,
    /// but not yet lowered.
    pub fn intermediate_passes(
        &mut self,
    ) -> Result<(leo_abi::Program, IndexMap<String, leo_abi::Program>, Vec<leo_abi::interfaces::CompiledInterface>)>
    {
        let type_checking_config = TypeCheckingInput::new(self.state.network);

        self.frontend_passes()?;

        self.do_pass::<ConstPropUnrollAndMorphing>(type_checking_config.clone())?;

        // Generate ABIs after monomorphization to capture concrete types.
        // Const generic structs are resolved to their monomorphized versions.
        let abis = self.generate_abi();

        self.do_pass::<StorageLowering>(type_checking_config.clone())?;

        self.do_pass::<OptionLowering>(type_checking_config)?;

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

        Ok(abis)
    }

    /// Generates ABIs for the primary program, all imports, and interfaces.
    ///
    /// Returns `(primary_abi, import_abis, interface_abis)` where `import_abis`
    /// maps program names to their ABIs.
    ///
    /// This method only expects program ASTs. Library ASTs cause this method to panic.
    fn generate_abi(
        &self,
    ) -> (leo_abi::Program, IndexMap<String, leo_abi::Program>, Vec<leo_abi::interfaces::CompiledInterface>) {
        let program = match &self.state.ast {
            Ast::Program(program) => program,
            Ast::Library(_) => panic!("expected Program AST"),
        };

        // Generate primary ABI (pruning happens inside generate).
        let primary_abi = leo_abi::generate(program);

        // Generate interface ABIs.
        let interface_abis = leo_abi::interfaces::generate_program_interfaces(program);

        // Generate import ABIs from stubs, ignoring libraries.
        let import_abis: IndexMap<String, leo_abi::Program> = program
            .stubs
            .iter()
            .filter(|(_, stub)| !matches!(stub, Stub::FromLibrary { .. }))
            .map(|(name, stub)| {
                let abi = match stub {
                    Stub::FromLeo { program, .. } => leo_abi::generate(program),
                    Stub::FromAleo { program, .. } => leo_abi::aleo::generate(program),
                    Stub::FromLibrary { .. } => unreachable!("filtered out"),
                };
                (name.to_string(), abi)
            })
            .collect();

        (primary_abi, import_abis, interface_abis)
    }

    /// Generates interface ABIs for a validated library.
    ///
    /// Must be called after `build_library()` since it reads the resolved AST.
    pub fn generate_library_interface_abis(&self) -> Vec<leo_abi::interfaces::CompiledInterface> {
        let Ast::Library(library) = &self.state.ast else {
            panic!("expected Library AST");
        };
        leo_abi::interfaces::generate_library_interfaces(library)
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
    /// * `Ok(CompiledPrograms)` containing the generated bytecode and ABI if compilation succeeds.
    /// * `Err(CompilerError)` if any stage of the pipeline fails.
    pub fn compile(&mut self, source: &str, filename: FileName, modules: &Vec<(&str, FileName)>) -> Result<Compiled> {
        // Parse the program.
        self.parse_program(source, filename, modules)?;
        // Merge the stubs into the AST.
        self.add_import_stubs()?;
        // Run the intermediate compiler stages, which also generates ABIs.
        let (primary_abi, import_abis, interfaces) = self.intermediate_passes()?;
        // Run code generation.
        let generated = self.do_pass::<CodeGenerating>(())?;
        // Run peephole optimization and serialize to bytecode.
        let bytecodes = self.do_pass::<PeepholeOptimizing>(generated)?;

        // Build the primary compiled program.
        let primary = CompiledProgram {
            name: self.unit_name.clone().unwrap(),
            bytecode: bytecodes.primary_bytecode,
            abi: primary_abi,
        };

        // Build compiled programs for imports, looking up ABIs by name.
        let imports: Vec<CompiledProgram> = bytecodes
            .import_bytecodes
            .into_iter()
            .map(|bc| {
                let abi = import_abis.get(&bc.program_name).expect("ABI should exist for all imports").clone();
                CompiledProgram { name: bc.program_name, bytecode: bc.bytecode, abi }
            })
            .collect();

        Ok(Compiled { primary, imports, interfaces })
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
        file_source: &dyn FileSource,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
    ) -> Result<(String, Vec<(String, FileName)>)> {
        let entry_file_path = entry_file_path.as_ref();
        let source_directory = source_directory.as_ref();

        // Read the contents of the main source file.
        let source = file_source
            .read_file(entry_file_path)
            .map_err(|e| crate::errors::file_read_error(entry_file_path.display().to_string(), e))?;

        let files = file_source
            .list_leo_files(source_directory, entry_file_path)
            .map_err(|e| crate::errors::file_read_error(source_directory.display().to_string(), e))?;

        let mut modules = Vec::with_capacity(files.len());
        for path in files {
            let module_source = file_source
                .read_file(&path)
                .map_err(|e| crate::errors::file_read_error(path.display().to_string(), e))?;
            modules.push((module_source, FileName::Real(path)));
        }

        Ok((source, modules))
    }

    /// Compiles a program from a source file using the given file source.
    pub fn compile_from_directory_with_file_source(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
        file_source: &dyn FileSource,
    ) -> Result<Compiled> {
        let (source, modules_owned) = Self::read_sources_and_modules(file_source, &entry_file_path, &source_directory)?;

        // Convert owned module sources into temporary (&str, FileName) tuples.
        let module_refs: Vec<(&str, FileName)> =
            modules_owned.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

        // Compile the main source along with all collected modules.
        self.compile(&source, FileName::Real(entry_file_path.as_ref().into()), &module_refs)
    }

    /// Parses a program from a source file using the given file source.
    pub fn parse_program_from_directory_with_file_source(
        &mut self,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
        file_source: &dyn FileSource,
    ) -> Result<Program> {
        let (source, modules_owned) = Self::read_sources_and_modules(file_source, &entry_file_path, &source_directory)?;

        // Convert owned module sources into temporary (&str, FileName) tuples.
        let module_refs: Vec<(&str, FileName)> =
            modules_owned.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

        // Parse the main source along with all collected modules.
        self.parse_program(&source, FileName::Real(entry_file_path.as_ref().into()), &module_refs)?;

        match &self.state.ast {
            Ast::Program(program) => Ok(program.clone()),
            Ast::Library(_) => unreachable!("expected Program AST"),
        }
    }

    /// Parses a library from a source file.
    pub fn parse_library_from_directory_with_file_source(
        &mut self,
        library_name: Symbol,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
        file_source: &dyn FileSource,
    ) -> Result<Library> {
        let (source, modules_owned) = Self::read_sources_and_modules(file_source, &entry_file_path, &source_directory)?;

        let module_refs: Vec<(&str, FileName)> =
            modules_owned.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

        self.parse_library(library_name, &source, FileName::Real(entry_file_path.as_ref().into()), &module_refs)?;

        match &self.state.ast {
            Ast::Library(library) => Ok(library.clone()),
            Ast::Program(_) => unreachable!("expected Library AST"),
        }
    }

    /// Writes the AST to a JSON file under the unit's snapshots directory.
    ///
    /// No-op on wasm: snapshot artifacts only exist on the native build path.
    fn write_ast_to_json(
        &self,
        #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] filename: &str,
    ) -> Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        if let Ast::Program(program) = &self.state.ast {
            // Snapshots are opt-in; create the directory lazily on first write.
            fs::create_dir_all(&self.output_directory)
                .map_err(|e| crate::errors::failed_ast_file(self.output_directory.display(), e))?;
            if self.compiler_options.ast_spans_enabled {
                program.to_json_file(self.output_directory.clone(), filename)?;
            } else {
                program.to_json_file_without_keys(self.output_directory.clone(), filename, &["_span", "span"])?;
            }
        }
        Ok(())
    }

    /// Writes the AST to a file (Leo syntax, not JSON) under the unit's snapshots directory.
    ///
    /// No-op on wasm: snapshot artifacts only exist on the native build path.
    fn write_ast(&self, #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] filename: &str) -> Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            fs::create_dir_all(&self.output_directory)
                .map_err(|e| crate::errors::failed_ast_file(self.output_directory.display(), e))?;
            let full_filename = self.output_directory.join(filename);
            let contents = match &self.state.ast {
                Ast::Program(program) => program.to_string(),
                Ast::Library(_) => String::new(), // empty for libraries
            };
            fs::write(&full_filename, contents)
                .map_err(|e| crate::errors::failed_ast_file(full_filename.display(), e))?;
        }
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
        use indexmap::IndexSet;

        // Track which programs we've already processed.
        let mut explored = IndexSet::<Symbol>::new();

        // Compute initial imports: explicit program imports + library dependencies
        let initial_imports: IndexMap<Symbol, Span> = match &self.state.ast {
            Ast::Program(program) => {
                let mut map: IndexMap<Symbol, Span> =
                    program.imports.iter().map(|(name, id)| (*name, id.span())).collect();
                // Add any libraries that have this program as a parent
                for (stub_name, stub) in &self.import_stubs {
                    if matches!(stub, Stub::FromLibrary { .. })
                        && stub.parents().contains(&Symbol::intern(self.unit_name.as_ref().unwrap()))
                    {
                        map.insert(
                            *stub_name,
                            Span::default(), // library dependencies are implicit
                        );
                    }
                }
                map
            }
            Ast::Library(_) => {
                // Libraries have no explicit `imports` field; their dependencies are expressed
                // indirectly through parent relations on the stubs map. A stub is a dep of this
                // library iff its parent set contains the library's own name.
                let library_name = Symbol::intern(self.unit_name.as_ref().unwrap());
                self.import_stubs
                    .iter()
                    .filter(|(_, stub)| stub.parents().contains(&library_name))
                    .map(|(name, _)| (*name, Span::default()))
                    .collect()
            }
        };

        // Initialize the exploration queue with the root’s direct imports.
        let mut to_explore: Vec<(Symbol, Span)> = initial_imports.iter().map(|(sym, span)| (*sym, *span)).collect();

        // If this is a named program, set the main program as the parent of its direct imports.
        if let Some(main_program_name) = self.unit_name.clone() {
            let main_symbol = Symbol::intern(&main_program_name);
            for import in initial_imports.keys() {
                if let Some(child_stub) = self.import_stubs.get_mut(import) {
                    child_stub.add_parent(main_symbol);
                }
            }
        }

        // Traverse the dependency graph breadth-first, populating parents
        while let Some((import_symbol, span)) = to_explore.pop() {
            // Mark this import as explored.
            explored.insert(import_symbol);

            // Look up the corresponding stub.
            let Some(stub) = self.import_stubs.get(&import_symbol) else {
                return Err(crate::errors::imported_program_not_found(
                    self.unit_name.as_ref().unwrap(),
                    import_symbol,
                    span,
                )
                .into());
            };

            // Combine imports: explicit stub.explicit_imports() + libraries that list this stub as parent
            let mut combined_imports: IndexMap<Symbol, Span> = stub.explicit_imports().collect();
            for (lib_name, lib_stub) in &self.import_stubs {
                if matches!(lib_stub, Stub::FromLibrary { .. }) && lib_stub.parents().contains(&import_symbol) {
                    combined_imports.insert(
                        *lib_name,
                        Span::default(), // library dependencies are implicit
                    );
                }
            }

            for (child_symbol, child_span) in combined_imports {
                // Record parent relationship
                if let Some(child_stub) = self.import_stubs.get_mut(&child_symbol) {
                    child_stub.add_parent(import_symbol);
                }

                // Schedule child for exploration if not yet visited.
                if explored.insert(child_symbol) {
                    to_explore.push((child_symbol, child_span));
                }
            }
        }

        // Collect all reachable stubs and store them on the AST.
        let reachable: IndexMap<Symbol, Stub> = self
            .import_stubs
            .iter()
            .filter(|(symbol, _)| explored.contains(*symbol))
            .map(|(symbol, stub)| (*symbol, stub.clone()))
            .collect();
        match &mut self.state.ast {
            Ast::Program(program) => program.stubs = reachable,
            Ast::Library(library) => library.stubs = reachable,
        }

        Ok(())
    }

    /// Builds a library: parses the source, resolves import stubs, and runs all frontend passes.
    ///
    /// Unlike [`Self::compile`], this does not run monomorphisation, lowerings, or code generation.
    /// No bytecode is produced. Returns the validated library AST, which callers can convert into
    /// a [`Stub`] for downstream units in the same build graph.
    pub fn build_library(
        &mut self,
        library_name: Symbol,
        source: &str,
        filename: FileName,
        modules: &[(&str, FileName)],
    ) -> Result<Library> {
        self.parse_library(library_name, source, filename, modules)?;
        self.add_import_stubs()?;
        self.frontend_passes()?;

        match &self.state.ast {
            Ast::Library(library) => Ok(library.clone()),
            Ast::Program(_) => unreachable!("expected Library AST"),
        }
    }

    /// Builds a library from a source file using the given file source.
    pub fn build_library_from_directory_with_file_source(
        &mut self,
        library_name: Symbol,
        entry_file_path: impl AsRef<Path>,
        source_directory: impl AsRef<Path>,
        file_source: &dyn FileSource,
    ) -> Result<Library> {
        let (source, modules_owned) = Self::read_sources_and_modules(file_source, &entry_file_path, &source_directory)?;

        let module_refs: Vec<(&str, FileName)> =
            modules_owned.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

        self.build_library(library_name, &source, FileName::Real(entry_file_path.as_ref().into()), &module_refs)
    }
}

/// Convert checked-in dependency bytecode into the same `Stub::FromAleo` shape
/// used for source dependencies so import consumers can stay agnostic to how a
/// dependency was declared.
///
/// Public so wasm callers (with no `leo-package` access) can build import stubs
/// from raw bytecode the same way the native build does.
pub fn disassemble_dependency_bytecode(program_name: Symbol, bytecode: &str, network: NetworkName) -> Result<Stub> {
    leo_disassembler::disassemble_from_str_for_network(program_name, bytecode, network)
        .map(Into::into)
        .map_err(|err| crate::errors::file_read_error(format!("dependency bytecode for `{program_name}`"), err).into())
}

#[cfg(test)]
mod tests {
    use super::Compiler;

    use leo_ast::{NetworkName, NodeBuilder};
    use leo_errors::{BufferEmitter, Handler};
    use leo_span::{Symbol, create_session_if_not_set_then, file_source::InMemoryFileSource};

    use std::{path::PathBuf, rc::Rc};

    use indexmap::IndexMap;

    /// Verifies library parsing can read every source file from an in-memory source.
    #[test]
    fn parse_library_from_directory_in_memory() {
        create_session_if_not_set_then(|_| {
            let mut source = InMemoryFileSource::new();
            source.set(
                PathBuf::from("/mylib/src/lib.leo"),
                concat!("const SCALE: u32 = 10u32;\n", "const OFFSET: u32 = SCALE + 1u32;\n",).into(),
            );

            let handler = Handler::default();
            let node_builder = Rc::new(NodeBuilder::default());
            let mut compiler = Compiler::new(
                None,
                false,
                handler,
                node_builder,
                PathBuf::from("/unused"),
                None,
                IndexMap::new(),
                NetworkName::TestnetV0,
            );

            let library = compiler
                .parse_library_from_directory_with_file_source(
                    Symbol::intern("mylib"),
                    "/mylib/src/lib.leo",
                    "/mylib/src",
                    &source,
                )
                .unwrap_or_else(|err| panic!("parsing library from in-memory file source failed: {err}"));

            assert_eq!(library.name, Symbol::intern("mylib"));
            assert_eq!(library.consts.len(), 2, "expected 2 consts, got {}", library.consts.len());
            assert!(
                library.consts.iter().any(|(name, _)| *name == Symbol::intern("SCALE")),
                "expected const `SCALE` in library"
            );
            assert!(
                library.consts.iter().any(|(name, _)| *name == Symbol::intern("OFFSET")),
                "expected const `OFFSET` in library"
            );
        });
    }

    /// Verifies in-memory library builds still reject type errors.
    #[test]
    fn build_library_from_directory_in_memory_rejects_type_error() {
        create_session_if_not_set_then(|_| {
            let mut source = InMemoryFileSource::new();
            // `true + 1u32` must be rejected by type checking.
            source
                .set(PathBuf::from("/badlib/src/lib.leo"), "fn broken() -> u32 {\n    return true + 1u32;\n}\n".into());

            // Capture errors in a buffer so the test can inspect them without writing to stderr.
            let emitter = BufferEmitter::new();
            let handler = Handler::new(emitter.clone());
            let node_builder = Rc::new(NodeBuilder::default());
            let mut compiler = Compiler::new(
                Some("badlib".into()),
                false,
                handler,
                node_builder,
                PathBuf::from("/unused"),
                None,
                IndexMap::new(),
                NetworkName::TestnetV0,
            );

            let result = compiler.build_library_from_directory_with_file_source(
                Symbol::intern("badlib"),
                "/badlib/src/lib.leo",
                "/badlib/src",
                &source,
            );

            assert!(result.is_err(), "expected build_library to fail on a library with a type error");

            let errors = emitter.extract_errs().to_string();
            assert!(errors.contains("ETYC"), "expected a type-checking error (prefix `ETYC`) but captured:\n{errors}");
        });
    }

    /// Verifies in-memory program parsing can load sibling modules.
    #[test]
    fn parse_program_from_directory_in_memory_with_module() {
        create_session_if_not_set_then(|_| {
            let mut source = InMemoryFileSource::new();
            source.set(
                PathBuf::from("/project/src/main.leo"),
                concat!(
                    "program test.aleo {\n",
                    "  fn main() -> u32 {\n",
                    "    return utils::helper();\n",
                    "  }\n",
                    "}\n",
                )
                .into(),
            );
            source.set(PathBuf::from("/project/src/utils.leo"), "fn helper() -> u32 {\n  return 42u32;\n}\n".into());

            let handler = Handler::default();
            let node_builder = Rc::new(NodeBuilder::default());
            let mut compiler = Compiler::new(
                Some("test.aleo".into()),
                false,
                handler,
                node_builder,
                PathBuf::from("/unused"),
                None,
                IndexMap::new(),
                NetworkName::TestnetV0,
            );

            let ast = compiler
                .parse_program_from_directory_with_file_source("/project/src/main.leo", "/project/src", &source)
                .unwrap_or_else(|err| panic!("parsing from in-memory file source failed: {err}"));
            let utils_key = vec![Symbol::intern("utils")];

            assert!(
                ast.modules.contains_key(&utils_key),
                "module `utils` should be loaded from the in-memory file source; found keys: {:?}",
                ast.modules.keys().collect::<Vec<_>>()
            );
        });
    }
}
