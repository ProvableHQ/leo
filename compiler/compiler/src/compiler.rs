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

use crate::CompilerOptions;

pub use leo_ast::Ast;
use leo_ast::Stub;
use leo_errors::{CompilerError, Result, emitter::Handler};
use leo_passes::*;
use leo_span::{Symbol, source_map::FileName, symbol::with_session_globals};

use snarkvm::prelude::Network;

use indexmap::{IndexMap, IndexSet};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The primary entry point of the Leo compiler.
pub struct Compiler<N: Network> {
    /// The path to where the compiler outputs all generated files.
    output_directory: PathBuf,
    /// The program name,
    pub program_name: String,
    /// Options configuring compilation.
    compiler_options: CompilerOptions,
    /// State.
    state: CompilerState,
    /// The stubs for imported programs. Produced by `Retriever` module.
    import_stubs: IndexMap<Symbol, Stub>,
    /// How many statements were in the AST before DCE?
    pub statements_before_dce: u32,
    /// How many statements were in the AST after DCE?
    pub statements_after_dce: u32,
    // Allows the compiler to be generic over the network.
    phantom: std::marker::PhantomData<N>,
}

impl<N: Network> Compiler<N> {
    pub fn parse(&mut self, source: &str, filename: FileName) -> Result<()> {
        // Register the source in the source map.
        let source_file = with_session_globals(|s| s.source_map.new_source(source, filename));

        // Use the parser to construct the abstract syntax tree (ast).
        self.state.ast = leo_parser::parse_ast::<N>(
            self.state.handler.clone(),
            &self.state.node_builder,
            &source_file.src,
            source_file.start_pos,
        )?;

        // If the program is imported, then check that the name of its program scope matches the file name.
        // Note that parsing enforces that there is exactly one program scope in a file.
        // TODO: Clean up check.
        let program_scope = self.state.ast.ast.program_scopes.values().next().unwrap();
        self.program_name = program_scope.program_id.name.to_string();

        if self.compiler_options.output.initial_ast {
            self.write_ast_to_json("initial_ast.json")?;
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
    pub fn new(
        handler: Handler,
        output_directory: PathBuf,
        compiler_options: Option<CompilerOptions>,
        import_stubs: IndexMap<Symbol, Stub>,
    ) -> Self {
        Self {
            state: CompilerState { handler, ..Default::default() },
            output_directory,
            program_name: Default::default(),
            compiler_options: compiler_options.unwrap_or_default(),
            import_stubs,
            statements_before_dce: 0,
            statements_after_dce: 0,
            phantom: Default::default(),
        }
    }

    /// Runs the compiler stages.
    pub fn intermediate_passes(&mut self) -> Result<()> {
        SymbolTableCreation::do_pass((), &mut self.state)?;

        TypeChecking::do_pass(
            TypeCheckingInput {
                max_array_elements: N::MAX_ARRAY_ELEMENTS,
                max_mappings: N::MAX_MAPPINGS,
                max_functions: N::MAX_FUNCTIONS,
            },
            &mut self.state,
        )?;

        StaticAnalyzing::do_pass(
            StaticAnalyzingInput {
                max_depth: self.compiler_options.build.conditional_block_max_depth,
                conditional_branch_type_checking: !self.compiler_options.build.disable_conditional_branch_type_checking,
            },
            &mut self.state,
        )?;

        ConstPropagationAndUnrolling::do_pass((), &mut self.state)?;

        SsaForming::do_pass(SsaFormingInput { rename_defs: true }, &mut self.state)?;

        Destructuring::do_pass((), &mut self.state)?;

        SsaForming::do_pass(SsaFormingInput { rename_defs: false }, &mut self.state)?;

        Flattening::do_pass((), &mut self.state)?;

        FunctionInlining::do_pass((), &mut self.state)?;

        if self.compiler_options.build.dce_enabled {
            let output = DeadCodeEliminating::do_pass((), &mut self.state)?;
            self.statements_before_dce = output.statements_before;
            self.statements_after_dce = output.statements_after;
        }

        Ok(())
    }

    /// Returns a compiled Leo program.
    pub fn compile(&mut self, source: &str, filename: FileName) -> Result<String> {
        // Parse the program.
        self.parse(source, filename)?;
        // Copy the dependencies specified in `program.json` into the AST.
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
        if self.compiler_options.output.ast_spans_enabled {
            self.state
                .ast
                .to_json_file(self.output_directory.clone(), &format!("{}.{file_suffix}", self.program_name))?;
        } else {
            self.state.ast.to_json_file_without_keys(
                self.output_directory.clone(),
                &format!("{}.{file_suffix}", self.program_name),
                &["_span", "span"],
            )?;
        }
        Ok(())
    }

    /// Merges the dependencies defined in `program.json` with the dependencies imported in `.leo` file
    pub fn add_import_stubs(&mut self) -> Result<()> {
        // Create a list of both the explicit dependencies specified in the `.leo` file, as well as the implicit ones derived from those dependencies.
        let (mut unexplored, mut explored): (IndexSet<Symbol>, IndexSet<Symbol>) =
            (self.state.ast.ast.imports.keys().cloned().collect(), IndexSet::new());
        while !unexplored.is_empty() {
            let mut current_dependencies: IndexSet<Symbol> = IndexSet::new();
            for program_name in unexplored.iter() {
                if let Some(stub) = self.import_stubs.get(program_name) {
                    // Add the program to the explored set
                    explored.insert(*program_name);
                    for dependency in stub.imports.iter() {
                        // If dependency is already explored then don't need to re-explore it
                        if explored.insert(dependency.name.name) {
                            current_dependencies.insert(dependency.name.name);
                        }
                    }
                } else {
                    return Err(CompilerError::imported_program_not_found(
                        self.program_name.clone(),
                        *program_name,
                        self.state.ast.ast.imports[program_name].1,
                    )
                    .into());
                }
            }

            // Create next batch to explore
            unexplored = current_dependencies;
        }

        // Combine the dependencies from `program.json` and `.leo` file while preserving the post-order
        self.state.ast.ast.stubs =
            self.import_stubs.clone().into_iter().filter(|(program_name, _)| explored.contains(program_name)).collect();
        Ok(())
    }
}
