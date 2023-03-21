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

//! The compiler for Leo programs.
//!
//! The [`Compiler`] type compiles Leo programs into R1CS circuits.
use leo_ast::Program;
pub use leo_ast::{Ast, InputAst};
use leo_errors::emitter::Handler;
use leo_errors::{CompilerError, Result};
pub use leo_passes::SymbolTable;
use leo_passes::*;
use leo_span::source_map::FileName;
use leo_span::symbol::with_session_globals;

use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

use crate::CompilerOptions;

/// The primary entry point of the Leo compiler.
#[derive(Clone)]
pub struct Compiler<'a> {
    /// The handler is used for error and warning emissions.
    handler: &'a Handler,
    /// The path to the main leo file.
    main_file_path: PathBuf,
    /// The path to where the compiler outputs all generated files.
    output_directory: PathBuf,
    /// The program name,
    pub program_name: String,
    /// The network name,
    pub network: String,
    /// The AST for the program.
    pub ast: Ast,
    /// The input ast for the program if it exists.
    pub input_ast: Option<InputAst>,
    /// Options configuring compilation.
    compiler_options: CompilerOptions,
}

impl<'a> Compiler<'a> {
    /// Returns a new Leo compiler.
    pub fn new(
        program_name: String,
        network: String,
        handler: &'a Handler,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        compiler_options: Option<CompilerOptions>,
    ) -> Self {
        Self {
            handler,
            main_file_path,
            output_directory,
            program_name,
            network,
            ast: Ast::new(Program::default()),
            input_ast: None,
            compiler_options: compiler_options.unwrap_or_default(),
        }
    }

    /// Returns a SHA256 checksum of the program file.
    pub fn checksum(&self) -> Result<String> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.update(unparsed_file.as_bytes());
        let hash = hasher.finalize();

        Ok(format!("{hash:x}"))
    }

    /// Parses and stores a program file content from a string, constructs a syntax tree, and generates a program.
    pub fn parse_program_from_string(&mut self, program_string: &str, name: FileName) -> Result<()> {
        // Register the source (`program_string`) in the source map.
        let prg_sf = with_session_globals(|s| s.source_map.new_source(program_string, name));

        // Use the parser to construct the abstract syntax tree (ast).
        self.ast = leo_parser::parse_ast(self.handler, &prg_sf.src, prg_sf.start_pos)?;

        // If the program is imported, then check that the name of its program scope matches the file name.
        // Note that parsing enforces that there is exactly one program scope in a file.
        // TODO: Clean up check.
        let program_scope = self.ast.ast.program_scopes.values().next().unwrap();
        let program_scope_name = format!("{}", program_scope.program_id.name);
        if program_scope_name != self.program_name {
            return Err(CompilerError::program_scope_name_does_not_match(
                program_scope_name,
                self.program_name.clone(),
                program_scope.program_id.name.span,
            )
            .into());
        }

        if self.compiler_options.initial_ast {
            self.write_ast_to_json("initial_ast.json")?;
        }

        Ok(())
    }

    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<()> {
        // Load the program file.
        let program_string = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(&self.main_file_path, e))?;

        self.parse_program_from_string(&program_string, FileName::Real(self.main_file_path.clone()))
    }

    /// Parses and stores the input file, constructs a syntax tree, and generates a program input.
    pub fn parse_input(&mut self, input_file_path: PathBuf) -> Result<()> {
        if input_file_path.exists() {
            // Load the input file into the source map.
            let input_sf = with_session_globals(|s| s.source_map.load_file(&input_file_path))
                .map_err(|e| CompilerError::file_read_error(&input_file_path, e))?;

            // Parse and serialize it.
            let input_ast = leo_parser::parse_input(self.handler, &input_sf.src, input_sf.start_pos)?;
            if self.compiler_options.initial_ast {
                // Write the input AST snapshot post parsing.
                if self.compiler_options.spans_enabled {
                    input_ast.to_json_file(
                        self.output_directory.clone(),
                        &format!("{}.initial_input_ast.json", self.program_name),
                    )?;
                } else {
                    input_ast.to_json_file_without_keys(
                        self.output_directory.clone(),
                        &format!("{}.initial_input_ast.json", self.program_name),
                        &["span"],
                    )?;
                }
            }

            self.input_ast = Some(input_ast);
        }
        Ok(())
    }

    /// Runs the symbol table pass.
    pub fn symbol_table_pass(&self) -> Result<SymbolTable> {
        SymbolTableCreator::do_pass((&self.ast, self.handler))
    }

    /// Runs the type checker pass.
    pub fn type_checker_pass(&'a self, symbol_table: SymbolTable) -> Result<(SymbolTable, StructGraph, CallGraph)> {
        TypeChecker::do_pass((&self.ast, self.handler, symbol_table))
    }

    /// Runs the loop unrolling pass.
    pub fn loop_unrolling_pass(&mut self, symbol_table: SymbolTable) -> Result<SymbolTable> {
        let (ast, symbol_table) = Unroller::do_pass((std::mem::take(&mut self.ast), self.handler, symbol_table))?;
        self.ast = ast;

        if self.compiler_options.unrolled_ast {
            self.write_ast_to_json("unrolled_ast.json")?;
        }

        Ok(symbol_table)
    }

    /// Runs the static single assignment pass.
    pub fn static_single_assignment_pass(&mut self, symbol_table: &SymbolTable) -> Result<Assigner> {
        let (ast, assigner) = StaticSingleAssigner::do_pass((std::mem::take(&mut self.ast), symbol_table))?;
        self.ast = ast;

        if self.compiler_options.ssa_ast {
            self.write_ast_to_json("ssa_ast.json")?;
        }

        Ok(assigner)
    }

    /// Runs the flattening pass.
    pub fn flattening_pass(&mut self, symbol_table: &SymbolTable, assigner: Assigner) -> Result<Assigner> {
        let (ast, assigner) = Flattener::do_pass((std::mem::take(&mut self.ast), symbol_table, assigner))?;
        self.ast = ast;

        if self.compiler_options.flattened_ast {
            self.write_ast_to_json("flattened_ast.json")?;
        }

        Ok(assigner)
    }

    /// Runs the function inlining pass.
    pub fn function_inlining_pass(&mut self, call_graph: &CallGraph, assigner: Assigner) -> Result<Assigner> {
        let (ast, assigner) = FunctionInliner::do_pass((std::mem::take(&mut self.ast), call_graph, assigner))?;
        self.ast = ast;

        if self.compiler_options.inlined_ast {
            self.write_ast_to_json("inlined_ast.json")?;
        }

        Ok(assigner)
    }

    /// Runs the dead code elimination pass.
    pub fn dead_code_elimination_pass(&mut self) -> Result<()> {
        if self.compiler_options.dce_enabled {
            self.ast = DeadCodeEliminator::do_pass(std::mem::take(&mut self.ast))?;
        }

        if self.compiler_options.dce_ast {
            self.write_ast_to_json("dce_ast.json")?;
        }

        Ok(())
    }

    /// Runs the code generation pass.
    pub fn code_generation_pass(
        &mut self,
        symbol_table: &SymbolTable,
        struct_graph: &StructGraph,
        call_graph: &CallGraph,
    ) -> Result<String> {
        CodeGenerator::do_pass((&self.ast, symbol_table, struct_graph, call_graph))
    }

    /// Runs the compiler stages.
    pub fn compiler_stages(&mut self) -> Result<(SymbolTable, StructGraph, CallGraph)> {
        let st = self.symbol_table_pass()?;
        let (st, struct_graph, call_graph) = self.type_checker_pass(st)?;

        // TODO: Make this pass optional.
        let st = self.loop_unrolling_pass(st)?;

        // TODO: Make this pass optional.
        let assigner = self.static_single_assignment_pass(&st)?;

        let assigner = self.flattening_pass(&st, assigner)?;

        let _ = self.function_inlining_pass(&call_graph, assigner)?;

        self.dead_code_elimination_pass()?;

        Ok((st, struct_graph, call_graph))
    }

    /// Returns a compiled Leo program.
    pub fn compile(&mut self) -> Result<(SymbolTable, String)> {
        // Parse the program.
        self.parse_program()?;
        // Run the intermediate compiler stages.
        let (symbol_table, struct_graph, call_graph) = self.compiler_stages()?;
        // Run code generation.
        let bytecode = self.code_generation_pass(&symbol_table, &struct_graph, &call_graph)?;
        Ok((symbol_table, bytecode))
    }

    /// Writes the AST to a JSON file.
    fn write_ast_to_json(&self, file_suffix: &str) -> Result<()> {
        // Remove `Span`s if they are not enabled.
        if self.compiler_options.spans_enabled {
            self.ast.to_json_file(
                self.output_directory.clone(),
                &format!("{}.{file_suffix}", self.program_name),
            )?;
        } else {
            self.ast.to_json_file_without_keys(
                self.output_directory.clone(),
                &format!("{}.{file_suffix}", self.program_name),
                &["span"],
            )?;
        }
        Ok(())
    }
}
