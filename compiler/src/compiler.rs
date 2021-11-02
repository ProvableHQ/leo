// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! Compiles a Leo program from a file path.

use crate::{asg_group_coordinate_to_ir, decode_address, CompilerOptions, Output, OutputFile, Program};
use crate::{OutputOptions, TypeInferencePhase};
pub use leo_asg::{new_context, AsgContext as Context, AsgContext};
use leo_asg::{Asg, AsgPass, CircuitMember, GroupValue, Program as AsgProgram};
use leo_ast::AstPass;
use leo_ast::{InputValue, IntegerType, Program as AstProgram};
use leo_errors::emitter::Handler;
use leo_errors::AsgError;
use leo_errors::SnarkVMError;
use leo_errors::StateError;
use leo_errors::{CompilerError, Result, Span};
use leo_imports::ImportParser;
use leo_input::LeoInputParser;
use leo_package::inputs::InputPairs;
use leo_parser::parse_ast;

use eyre::eyre;
use leo_synthesizer::CircuitSynthesizer;
use num_bigint::{BigInt, Sign};
use sha2::{Digest, Sha256};

use snarkvm_curves::bls12_377::Bls12_377;
use snarkvm_eval::edwards_bls12::EdwardsGroupType;
use snarkvm_eval::{Evaluator, GroupType, PrimeField};
use snarkvm_ir::InputData;
use snarkvm_ir::{Group, Integer, Type, Value};
use snarkvm_r1cs::ConstraintSystem;
use std::io::Write;
use std::{convert::TryFrom, fs, path::PathBuf};

use indexmap::IndexMap;

thread_local! {
    static THREAD_GLOBAL_CONTEXT: AsgContext<'static> = {
        let leaked = Box::leak(Box::new(leo_asg::new_alloc_context()));
        leo_asg::new_context(leaked)
    }
}

/// Convenience function to return a leaked thread-local global context. Should only be used for transient programs (like cli).
pub fn thread_leaked_context() -> AsgContext<'static> {
    THREAD_GLOBAL_CONTEXT.with(|f| *f)
}

#[derive(Debug)]
pub struct CompilationData {
    pub program: snarkvm_ir::Program,
    pub output: Output,
}

/// Stores information to compile a Leo program.
#[derive(Clone)]
pub struct Compiler<'a, 'b> {
    handler: &'b Handler,
    pub program_name: String,
    main_file_path: PathBuf,
    pub output_directory: PathBuf,
    program: AstProgram,
    context: AsgContext<'a>,
    asg: Option<AsgProgram<'a>>,
    options: CompilerOptions,
    imports_map: IndexMap<String, String>,
    output_options: OutputOptions,
}

impl<'a, 'b> Compiler<'a, 'b> {
    /// Returns a new Leo program compiler.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        handler: &'b Handler,
        package_name: String,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        context: AsgContext<'a>,
        options: Option<CompilerOptions>,
        imports_map: IndexMap<String, String>,
        output_options: Option<OutputOptions>,
    ) -> Self {
        // load static files
        // TODO remove this once we implement a determinstic include_dir
        leo_stdlib::static_include_stdlib();

        Self {
            handler,
            program_name: package_name.clone(),
            main_file_path,
            output_directory,
            program: AstProgram::new(package_name),
            asg: None,
            context,
            options: options.unwrap_or_default(),
            imports_map,
            output_options: output_options.unwrap_or_default(),
        }
    }

    /// Returns a new `Compiler` from the given main file path.
    ///
    /// Parses and stores a program from the main file path.
    /// Parses and stores all imported programs.
    /// Performs type inference checking on the program and imported programs.
    #[allow(clippy::too_many_arguments)]
    pub fn parse_program_without_input(
        handler: &'b Handler,
        package_name: String,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        context: AsgContext<'a>,
        options: Option<CompilerOptions>,
        imports_map: IndexMap<String, String>,
        output_options: Option<OutputOptions>,
    ) -> Result<Self> {
        let mut compiler = Self::new(
            handler,
            package_name,
            main_file_path,
            output_directory,
            context,
            options,
            imports_map,
            output_options,
        );

        compiler.parse_program()?;

        Ok(compiler)
    }

    pub fn set_options(&mut self, options: CompilerOptions) {
        self.options = options;
    }

    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    ///
    /// Parses and stores all programs imported by the main program file.
    pub fn parse_program(&mut self) -> Result<()> {
        // Load the program file.
        let content = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        self.parse_program_from_string(&content)
    }

    /// Equivalent to parse_and_check_program but uses the given program_string instead of a main
    /// file path.
    pub fn parse_program_from_string(&mut self, program_string: &str) -> Result<()> {
        // Use the parser to construct the abstract syntax tree (ast).

        let mut ast: leo_ast::Ast = parse_ast(
            self.handler,
            self.main_file_path.to_str().unwrap_or_default(),
            program_string,
        )?;

        if self.output_options.ast_initial {
            if self.output_options.spans_enabled {
                ast.to_json_file(self.output_directory.clone(), "initial_ast.json")?;
            } else {
                ast.to_json_file_without_keys(self.output_directory.clone(), "initial_ast.json", &["span"])?;
            }
        }

        // Preform import resolution.
        ast = leo_ast_passes::Importer::do_pass(
            leo_ast_passes::Importer::new(
                &mut ImportParser::new(self.handler, self.main_file_path.clone(), self.imports_map.clone()),
                "bls12_377",
                self.handler,
            ),
            ast.into_repr(),
        )?;

        if self.output_options.ast_imports_resolved {
            if self.output_options.spans_enabled {
                ast.to_json_file(self.output_directory.clone(), "imports_resolved_ast.json")?;
            } else {
                ast.to_json_file_without_keys(self.output_directory.clone(), "imports_resolved_ast.json", &["span"])?;
            }
        }

        // Preform canonicalization of AST always.
        ast = leo_ast_passes::Canonicalizer::do_pass(Default::default(), ast.into_repr())?;

        if self.output_options.ast_canonicalized {
            if self.output_options.spans_enabled {
                ast.to_json_file(self.output_directory.clone(), "canonicalization_ast.json")?;
            } else {
                ast.to_json_file_without_keys(self.output_directory.clone(), "canonicalization_ast.json", &["span"])?;
            }
        }

        // Store the main program file.
        self.program = ast.into_repr();
        self.program.name = self.program_name.clone();

        tracing::debug!("Program parsing complete\n{:#?}", self.program);

        // Create a new symbol table from the program, imported_programs, and program_input.
        let asg = Asg::new(self.context, &self.program)?;

        if self.output_options.ast_type_inferenced {
            let new_ast = TypeInferencePhase::default()
                .phase_ast(&self.program, &asg.clone().into_repr())
                .expect("Failed to produce type inference ast.");

            if self.output_options.spans_enabled {
                new_ast.to_json_file(self.output_directory.clone(), "type_inferenced_ast.json")?;
            } else {
                new_ast.to_json_file_without_keys(
                    self.output_directory.clone(),
                    "type_inferenced_ast.json",
                    &["span"],
                )?;
            }
        }

        tracing::debug!("ASG generation complete");

        // Store the ASG.
        self.asg = Some(asg.into_repr());

        self.do_asg_passes()?;

        Ok(())
    }

    /// Run compiler optimization passes on the program in asg format.
    fn do_asg_passes(&mut self) -> Result<()> {
        assert!(self.asg.is_some());

        // Do constant folding.
        if self.options.constant_folding_enabled {
            let asg = self.asg.take().unwrap();
            self.asg = Some(leo_asg_passes::ConstantFolding::do_pass(asg)?);
        }

        // Do dead code elimination.
        if self.options.dead_code_elimination_enabled {
            let asg = self.asg.take().unwrap();
            self.asg = Some(leo_asg_passes::DeadCodeElimination::do_pass(asg)?);
        }

        Ok(())
    }

    pub fn compile_ir(&self, input: &leo_ast::Input) -> Result<snarkvm_ir::Program> {
        let asg = self.asg.as_ref().unwrap().clone();
        let mut program = Program::new(asg);

        program.enforce_program(input)?;

        Ok(program.render(&self.options))
    }

    pub fn compile<F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        input: &leo_ast::Input,
    ) -> Result<CompilationData> {
        let compiled = self.compile_ir(input)?;

        if self.output_options.emit_ir {
            let writer = |extension: &str, data: Vec<u8>| {
                let mut f = std::fs::File::create(
                    self.output_directory
                        .clone()
                        .join(format!("{}{}", self.program_name, extension)),
                )
                .unwrap();
                f.write_all(&data).unwrap();
            };

            writer(".leo.ir", compiled.serialize().unwrap());
            writer(".leo.ir.fmt", compiled.to_string().as_bytes().to_vec());
            writer(
                ".leo.ir.input",
                self.process_input(input, &compiled.header)?.serialize().unwrap(),
            );
        }

        self.compile_inner::<F, G, CS>(cs, input, compiled)
    }

    pub fn compile_inner<F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        input: &leo_ast::Input,
        compiled: snarkvm_ir::Program,
    ) -> Result<CompilationData> {
        let input_data = self.process_input(input, &compiled.header)?;
        let mut evaluator = snarkvm_eval::SetupEvaluator::<F, G, CS>::new(cs);
        let output = evaluator
            .evaluate(&compiled, &input_data)
            .map_err(|e| SnarkVMError::from(eyre!(e)))?;

        let registers: Vec<_> = compiled.header.register_inputs.to_vec();
        let output = Output::new(&registers[..], output, &Span::default())?;

        Ok(CompilationData {
            program: compiled,
            output,
        })
    }

    fn compile_ir_test(
        &self,
        program: &Program<'a>,
        function: &'a leo_asg::Function<'a>,
        input: &InputPairs,
    ) -> Result<(leo_ast::Input, snarkvm_ir::Program, String)> {
        let program_name = program.asg.name.clone();
        let mut output_file_name = program_name.clone();

        let input_file = function.annotations.get("test").unwrap().arguments.get(0);
        // get input file name from annotation or use test_name
        let input_pair = match input_file {
            Some(file_id) => {
                let file_name = file_id.clone();
                let file_name_kebab = file_name.to_string().replace("_", "-");

                // transform "test_name" into "test-name"
                output_file_name = file_name.to_string();

                // searches for test_input (snake case) or for test-input (kebab case)
                match input
                    .pairs
                    .get(&file_name_kebab)
                    .or_else(|| input.pairs.get(&file_name_kebab))
                {
                    Some(pair) => pair.to_owned(),
                    None => {
                        return Err(CompilerError::invalid_test_context(file_name).into());
                    }
                }
            }
            None => input
                .pairs
                .get(&program_name)
                .ok_or_else(CompilerError::no_test_input)?,
        };

        // parse input files to abstract syntax trees
        let input_file = &input_pair.input_file;
        let state_file = &input_pair.state_file;

        let input_ast = LeoInputParser::parse_file(input_file)?;
        let state_ast = LeoInputParser::parse_file(state_file)?;

        // parse input files into input struct
        let mut input = leo_ast::Input::new();
        input.parse_input(input_ast)?;
        input.parse_state(state_ast)?;

        let secondary_functions: Vec<_> = program
            .asg
            .scope
            .get_functions()
            .iter()
            .filter(|(_, func)| !func.is_test())
            .map(|(_, f)| *f)
            .chain(program.asg.scope.get_circuits().iter().flat_map(|(_, circuit)| {
                circuit
                    .members
                    .borrow()
                    .iter()
                    .flat_map(|(_, member)| match member {
                        CircuitMember::Function(function) => Some(*function),
                        CircuitMember::Variable(_) => None,
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            }))
            .collect();

        // run test function on new program with input
        let mut temporary_program = program.clone();
        temporary_program.enforce_function(&program.asg, function, &secondary_functions, &input)?;
        Ok((input, temporary_program.render(&self.options), output_file_name))
    }

    pub fn compile_test(&self, input: InputPairs) -> Result<(u32, u32)> {
        let asg = self.asg.as_ref().unwrap().clone();
        let program = Program::new(asg);

        let program_name = program.asg.name.clone();
        let mut output_file_name = program_name.clone();

        let tests = program
            .asg
            .functions
            .iter()
            .filter(|(_name, func)| func.is_test())
            .collect::<Vec<_>>();

        tracing::info!("Running {} tests", tests.len());

        // Count passed and failed tests
        let mut passed = 0;
        let mut failed = 0;

        for (test_name, function) in tests.into_iter() {
            let mut cs = CircuitSynthesizer::<Bls12_377>::default();
            let full_test_name = format!("{}::{}", program_name, test_name);

            let result = match self.compile_ir_test(&program, function, &input) {
                Ok((input, compiled, output_name)) => {
                    output_file_name = output_name;
                    self.compile_inner::<_, EdwardsGroupType, _>(&mut cs, &input, compiled)
                }
                Err(e) => Err(e),
            };

            if result.is_ok() {
                tracing::info!("{} ... ok\n", full_test_name);

                // write result to file
                let output = result?;
                let output_file = OutputFile::new(&output_file_name);

                output_file
                    .write(&self.output_directory, output.output.to_string().as_bytes())
                    .unwrap();
                // increment passed tests
                passed += 1;
            } else {
                tracing::error!("{} failed due to error\n\n{}\n", full_test_name, result.unwrap_err());

                // increment failed tests
                failed += 1;
            }
        }

        Ok((passed, failed))
    }

    // ///
    // /// Synthesizes the circuit with program input to verify correctness.
    // ///
    // pub fn compile_constraints(&self, program: &mut Program) -> Result<Output, CompilerError> {
    //     generate_constraints(program, &self.asg.as_ref().unwrap(), &self.program_input)
    // }

    // ///
    // /// Synthesizes the circuit for test functions with program input.
    // ///
    // pub fn compile_test_constraints(self, input_pairs: InputPairs) -> Result<(u32, u32), CompilerError> {
    //     generate_test_constraints(&self.asg.as_ref().unwrap(), input_pairs, &self.output_directory)
    // }

    ///
    /// Returns a SHA256 checksum of the program file.
    pub fn checksum(&self) -> Result<String> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.update(unparsed_file.as_bytes());
        let hash = hasher.finalize();

        Ok(format!("{:x}", hash))
    }

    fn process_input_value(value: InputValue, type_: &Type, span: &Span) -> Result<Value> {
        Ok(match (type_, value) {
            (Type::Address, InputValue::Address(address)) => {
                let decoded = decode_address(&address, &Span::default())?;
                Value::Address(decoded)
            }
            (Type::Boolean, InputValue::Boolean(value)) => Value::Boolean(value),
            (Type::Field, InputValue::Field(value)) => {
                let parsed: BigInt = value.parse().map_err(|_| AsgError::invalid_int(value, span))?;
                Value::Field(snarkvm_ir::Field {
                    values: parsed.magnitude().iter_u64_digits().collect(),
                    negate: parsed.sign() == Sign::Minus,
                })
            }
            (Type::Char, InputValue::Char(c)) => Value::Char(match c.character {
                leo_ast::Char::Scalar(c) => c as u32,
                leo_ast::Char::NonScalar(c) => c,
            }),
            (Type::Group, InputValue::Group(group)) => {
                let asg_group = GroupValue::try_from(group)?;
                match asg_group {
                    GroupValue::Single(parsed) => Value::Group(Group::Single(snarkvm_ir::Field {
                        values: parsed.magnitude().iter_u64_digits().collect(),
                        negate: parsed.sign() == Sign::Minus,
                    })),
                    GroupValue::Tuple(left, right) => Value::Group(Group::Tuple(
                        asg_group_coordinate_to_ir(&left),
                        asg_group_coordinate_to_ir(&right),
                    )),
                }
            }
            (Type::U8, InputValue::Integer(IntegerType::U8, value)) => Value::Integer(Integer::U8(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::U16, InputValue::Integer(IntegerType::U16, value)) => Value::Integer(Integer::U16(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::U32, InputValue::Integer(IntegerType::U32, value)) => Value::Integer(Integer::U32(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::U64, InputValue::Integer(IntegerType::U64, value)) => Value::Integer(Integer::U64(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::U128, InputValue::Integer(IntegerType::U128, value)) => Value::Integer(Integer::U128(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::I8, InputValue::Integer(IntegerType::I8, value)) => Value::Integer(Integer::I8(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::I16, InputValue::Integer(IntegerType::I16, value)) => Value::Integer(Integer::I16(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::I32, InputValue::Integer(IntegerType::I32, value)) => Value::Integer(Integer::I32(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::I64, InputValue::Integer(IntegerType::I64, value)) => Value::Integer(Integer::I64(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::I128, InputValue::Integer(IntegerType::I128, value)) => Value::Integer(Integer::I128(
                value.parse().map_err(|_| AsgError::invalid_int(value, span))?,
            )),
            (Type::Array(inner, len), InputValue::Array(values)) => {
                if values.len() != len.unwrap() as usize {
                    return Err(CompilerError::invalid_input_array_dimensions(
                        len.unwrap() as usize,
                        values.len(),
                        span,
                    )
                    .into());
                }
                let mut out = Vec::with_capacity(values.len());
                for value in values {
                    out.push(Self::process_input_value(value, &**inner, span)?);
                }
                Value::Array(out)
            }
            (Type::Tuple(inner), InputValue::Tuple(values)) => {
                if inner.len() != values.len() {
                    return Err(CompilerError::input_tuple_size_mismatch(inner.len(), values.len(), span).into());
                }
                let mut out = Vec::with_capacity(values.len());
                for (value, type_) in values.into_iter().zip(inner.iter()) {
                    out.push(Self::process_input_value(value, type_, span)?);
                }
                Value::Tuple(out)
            }
            (Type::Circuit(_), _) => {
                return Err(CompilerError::circuit_as_input(span).into());
            }
            (type_, value) => return Err(AsgError::unexpected_type(type_, value, span).into()),
        })
    }

    pub fn process_input(&self, input: &leo_ast::Input, ir: &snarkvm_ir::Header) -> Result<InputData> {
        let program = self.asg.as_ref().unwrap();
        let main_function = *program.functions.get("main").expect("missing main function");
        let span = main_function.span.clone().unwrap_or_default();

        let mut out = InputData::default();
        for ir_input in &ir.main_inputs {
            let value = input
                .get(&*ir_input.name)
                .flatten()
                .ok_or_else(|| CompilerError::function_input_not_found("main", &ir_input.name, &span))?;
            out.main.insert(
                ir_input.name.clone(),
                Self::process_input_value(value, &ir_input.type_, &span)?,
            );
        }
        for ir_input in &ir.constant_inputs {
            let value = input
                .get_constant(&*ir_input.name)
                .flatten()
                .ok_or_else(|| CompilerError::function_input_not_found("main", &ir_input.name, &span))?;
            out.constants.insert(
                ir_input.name.clone(),
                Self::process_input_value(value, &ir_input.type_, &span)?,
            );
        }
        let mut registers = input.get_registers().raw_values();
        for ir_input in &ir.register_inputs {
            let value = registers
                .remove(&*ir_input.name)
                .ok_or_else(|| CompilerError::function_missing_input_register(&ir_input.name, &span))?;
            out.registers.insert(
                ir_input.name.clone(),
                Self::process_input_value(value, &ir_input.type_, &span)?,
            );
        }
        let mut public_states = input.get_state().raw_values();
        for ir_input in &ir.public_states {
            let value = public_states
                .remove(&*ir_input.name)
                .ok_or_else(|| StateError::missing_parameter(&ir_input.name))?;
            out.public_states.insert(
                ir_input.name.clone(),
                Self::process_input_value(value, &ir_input.type_, &span)?,
            );
        }
        let mut private_leaf_states = input.get_state_leaf().raw_values();
        for ir_input in &ir.private_leaf_states {
            let value = private_leaf_states
                .remove(&*ir_input.name)
                .ok_or_else(|| StateError::missing_parameter(&ir_input.name))?;
            out.private_leaf_states.insert(
                ir_input.name.clone(),
                Self::process_input_value(value, &ir_input.type_, &span)?,
            );
        }
        let mut private_record_states = input.get_record().raw_values();
        for ir_input in &ir.private_record_states {
            let value = private_record_states
                .remove(&*ir_input.name)
                .ok_or_else(|| StateError::missing_parameter(&ir_input.name))?;
            out.private_record_states.insert(
                ir_input.name.clone(),
                Self::process_input_value(value, &ir_input.type_, &span)?,
            );
        }
        Ok(out)
    }
}
