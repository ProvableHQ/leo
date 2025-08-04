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

//! The compiler for Leo programs.
//!
//! The [`Compiler`] type compiles Leo programs into R1CS circuits.

use crate::{AstSnapshots, CompilerOptions};

pub use leo_ast::Ast;
use leo_ast::{NetworkName, Stub};
use leo_errors::{CompilerError, Handler, Result};
use leo_passes::*;
use leo_span::{Symbol, source_map::FileName, with_session_globals};

use indexmap::{IndexMap, IndexSet};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
    pub fn parse(&mut self, source: &str, filename: FileName) -> Result<()> {
        // Register the source in the source map.
        let source_file = with_session_globals(|s| s.source_map.new_source(source, filename));

        // Use the parser to construct the abstract syntax tree (ast).
        self.state.ast = leo_parser::parse_ast(
            self.state.handler.clone(),
            &self.state.node_builder,
            &source_file.src,
            source_file.absolute_start,
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
                self.program_name.as_ref().unwrap(),
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

    pub fn parse_from_file(&mut self, source_file_path: impl AsRef<Path>) -> Result<()> {
        // Load the program file.
        let source = fs::read_to_string(&source_file_path)
            .map_err(|e| CompilerError::file_read_error(source_file_path.as_ref().display().to_string(), e))?;
        self.parse(&source, FileName::Real(source_file_path.as_ref().into()))
    }

    /// Returns a new Leo compiler.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expected_program_name: Option<String>,
        is_test: bool,
        handler: Handler,
        output_directory: PathBuf,
        compiler_options: Option<CompilerOptions>,
        import_stubs: IndexMap<Symbol, Stub>,
        network: NetworkName,
    ) -> Self {
        Self {
            state: CompilerState { handler, is_test, network, ..Default::default() },
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

        self.do_pass::<SymbolTableCreation>(())?;

        self.do_pass::<TypeChecking>(type_checking_config.clone())?;

        self.do_pass::<ProcessingAsync>(type_checking_config.clone())?;

        self.do_pass::<StaticAnalyzing>(())?;

        self.do_pass::<ConstPropUnrollAndMorphing>(type_checking_config)?;

        self.do_pass::<ProcessingScript>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: true })?;

        self.do_pass::<Destructuring>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: false })?;

        self.do_pass::<WriteTransforming>(())?;

        self.do_pass::<SsaForming>(SsaFormingInput { rename_defs: false })?;

        self.do_pass::<Flattening>(())?;

        self.do_pass::<FunctionInlining>(())?;

        let output = self.do_pass::<DeadCodeEliminating>(())?;
        self.statements_before_dce = output.statements_before;
        self.statements_after_dce = output.statements_after;

        Ok(())
    }

    /// Returns a compiled Leo program.
    pub fn compile(&mut self, source: &str, filename: FileName) -> Result<String> {
        // Parse the program.
        self.parse(source, filename)?;
        // Merge the stubs into the AST.
        self.add_import_stubs()?;
        // Run the intermediate compiler stages.
        self.intermediate_passes()?;
        // Run code generation.
        let bytecode = CodeGenerating::do_pass((), &mut self.state)?;
        Ok(bytecode)
    }

    pub fn compile_from_file(&mut self, source_file_path: impl AsRef<Path>) -> Result<String> {
        let source = fs::read_to_string(&source_file_path)
            .map_err(|e| CompilerError::file_read_error(source_file_path.as_ref().display().to_string(), e))?;
        self.compile(&source, FileName::Real(source_file_path.as_ref().into()))
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
