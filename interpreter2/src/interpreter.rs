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

use super::{cursor::Cursor, *};

use cursor::FunctionCall;
use leo_ast::{Function, Location, NodeBuilder, interpreter_value::LeoValue};
use leo_errors::Result;
use leo_passes::{
    CompilerState,
    Pass as _,
    SymbolTable,
    SymbolTableCreation,
    TypeChecking,
    TypeCheckingInput,
    TypeTable,
};
use leo_span::{Symbol, source_map::FileName, with_session_globals};

use snarkvm::prelude::{Address, Closure as SvmClosureParam, Function as SvmFunctionParam, Network as _, TestnetV0};

use indexmap::IndexMap;
use std::cmp::Ordering;

pub type SvmFunction = SvmFunctionParam<TestnetV0>;
pub type SvmClosure = SvmClosureParam<TestnetV0>;

#[derive(Clone, Debug)]
pub enum FunctionVariant {
    Leo(Function),
    AleoClosure(SvmClosure),
    AleoFunction(SvmFunction),
}

#[derive(Default)]
pub struct Container {
    functions: IndexMap<Location, FunctionVariant>,
}

pub struct Interpreter<'a> {
    filename_to_program: IndexMap<FileName, Symbol>,
    node_builder: NodeBuilder,
    functions: &'a IndexMap<Location, FunctionVariant>,
    pub(crate) symbol_table: SymbolTable,
    pub(crate) type_table: TypeTable,
    pub(crate) cursor: Cursor<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(
        container: &'a mut Container,
        leo_filenames: impl IntoIterator<Item = FileName>,
        aleo_filenames: impl IntoIterator<Item = FileName>,
        signer: Address<TestnetV0>,
        block_height: u32,
    ) -> Result<Self> {
        Self::new_impl(container, &mut leo_filenames.into_iter(), &mut aleo_filenames.into_iter(), signer, block_height)
    }

    fn new_impl(
        container: &'a mut Container,
        leo_filenames: &mut dyn Iterator<Item = FileName>,
        _aleo_filenames: &mut dyn Iterator<Item = FileName>,
        signer: Address<TestnetV0>,
        block_height: u32,
    ) -> Result<Self> {
        container.functions.clear();
        let mut filename_to_program = IndexMap::new();
        let mut state = CompilerState { is_test: false, ..Default::default() };
        for leo_filename in leo_filenames {
            let source =
                with_session_globals(|globals| globals.source_map.source_file_by_filename(&leo_filename).unwrap());
            state.ast = leo_parser::parse_ast::<TestnetV0>(
                state.handler.clone(),
                &state.node_builder,
                &source.src,
                source.absolute_start,
            )?;
            assert_eq!(state.ast.ast.program_scopes.len(), 1);

            // Only run these two passes from the compiler to make sure that type inference runs and the `type_table`
            // and symbol table are filled out. Other compiler passes are not necessary at this stage.
            SymbolTableCreation::do_pass((), &mut state)?;
            TypeChecking::do_pass(
                TypeCheckingInput {
                    max_array_elements: TestnetV0::MAX_ARRAY_ELEMENTS,
                    max_mappings: TestnetV0::MAX_MAPPINGS,
                    max_functions: TestnetV0::MAX_FUNCTIONS,
                },
                &mut state,
            )?;

            let (program, scope) = state.ast.ast.program_scopes.into_iter().next().unwrap();
            filename_to_program.insert(leo_filename.clone(), program);

            for (name, function) in scope.functions.into_iter() {
                container.functions.insert(Location::new(program, name), FunctionVariant::Leo(function));
            }

            // TODO - mappings and consts
        }

        // TODO - aleo files

        Ok(Self {
            filename_to_program,
            node_builder: state.node_builder,
            functions: &container.functions,
            type_table: state.type_table,
            symbol_table: state.symbol_table,
            cursor: Cursor::new([], [], block_height, signer),
        })
    }

    pub fn initiate_function(
        &mut self,
        function_location: Location,
        arguments: impl IntoIterator<Item = LeoValue>,
    ) -> Result<()> {
        self.initiate_function_impl(function_location, &mut arguments.into_iter())
    }

    fn initiate_function_impl(
        &mut self,
        function_location: Location,
        arguments: &mut dyn Iterator<Item = LeoValue>,
    ) -> Result<()> {
        let leo_function = self.functions.get(&function_location).expect("Function should exist.");
        let call: cursor::FunctionCall<'_> = match leo_function {
            FunctionVariant::Leo(function) => {
                let mut input_names = function.input.iter().map(|input| input.identifier.name);
                let leo_function_call = cursor::LeoFunctionCall {
                    program: function_location.program,
                    caller: self.cursor.signer,
                    names: input_names.zip(arguments).collect(),
                    accumulated_futures: Default::default(),
                    is_async: false,
                    statement_frames: vec![(&function.block).into()],
                };
                // YYY - think about matching argument count to input count?
                leo_function_call.into()
            }
            FunctionVariant::AleoClosure(_closure_core) => todo!(),
            FunctionVariant::AleoFunction(_function_core) => todo!(),
        };

        self.cursor.function_call_stack.push(call);

        Ok(())
    }

    pub fn finish_expression(&mut self) -> Result<Option<LeoValue>> {
        let count_functions = |slf: &Self| slf.cursor.function_call_stack.len();
        let count_statements = |slf: &Self| match slf.cursor.function_call_stack.last() {
            Some(cursor::FunctionCall::Leo(leo_function_call)) => leo_function_call.statement_frames.len(),
            _ => 0,
        };
        let count_expressions = |slf: &Self| match slf.cursor.function_call_stack.last() {
            Some(cursor::FunctionCall::Leo(leo_function_call)) => match leo_function_call.statement_frames.last() {
                Some(statement_frame) => statement_frame.expression_frames.len(),
                None => 0,
            },
            _ => 0,
        };

        let function_count = count_functions(self);
        let statement_count = count_statements(self);
        let expression_count = count_expressions(self);

        if expression_count == 0 {
            return Ok(None);
        }

        loop {
            if count_functions(self) > function_count {
                self.finish_function()?;
            } else if count_statements(self) > statement_count {
                self.finish_statement()?;
            } else {
                self.step_expression()?;
            }

            match (
                count_functions(self).cmp(&function_count),
                count_statements(self).cmp(&statement_count),
                count_expressions(self).cmp(&expression_count),
            ) {
                (Ordering::Less, _, _)
                | (Ordering::Equal, Ordering::Less, _)
                | (Ordering::Equal, Ordering::Equal, Ordering::Less) => break,
                _ => {}
            }
        }

        let Some(cursor::FunctionCall::Leo(leo_function_call)) = self.cursor.function_call_stack.last() else {
            panic!("NO");
        };

        let Some(statement_frame) = leo_function_call.statement_frames.last() else {
            panic!("NO");
        };

        Ok(statement_frame.values.last().cloned())
    }

    pub fn finish_statement(&mut self) -> Result<()> {
        let count_functions = |slf: &Self| slf.cursor.function_call_stack.len();
        let count_statements = |slf: &Self| match slf.cursor.function_call_stack.last() {
            Some(cursor::FunctionCall::Leo(leo_function_call)) => leo_function_call.statement_frames.len(),
            _ => 0,
        };

        let start_functions = count_functions(self);
        let start_statements = count_statements(self);

        loop {
            self.step_statement()?;

            if count_functions(self) < start_functions || count_statements(self) < start_statements {
                break;
            }
        }

        Ok(())
    }

    pub fn finish_function(&mut self) -> Result<Option<LeoValue>> {
        let len = self.cursor.function_call_stack.len();

        if len == 0 {
            return Ok(None);
        }

        while self.cursor.function_call_stack.len() >= len {
            let Some(cursor::FunctionCall::Leo(leo_function_call)) = self.cursor.function_call_stack.last() else {
                panic!("NO");
            };

            if leo_function_call.statement_frames.is_empty() {
                if self.cursor.last_return_value.is_none() {
                    self.cursor.last_return_value = Some(LeoValue::Unit);
                }

                break;
            } else {
                self.finish_statement()?;
            }
        }

        Ok(self.cursor.last_return_value.clone())
    }
}
