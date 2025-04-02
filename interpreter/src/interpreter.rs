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

use super::*;

use leo_errors::{CompilerError, Handler, InterpreterHalt, LeoError, Result};

/// Contains the state of interpretation, in the form of the `Cursor`,
/// as well as information needed to interact with the user, like
/// the breakpoints.
pub struct Interpreter {
    pub cursor: Cursor<'static>,
    actions: Vec<InterpreterAction>,
    handler: Handler,
    node_builder: NodeBuilder,
    breakpoints: Vec<Breakpoint>,
    pub watchpoints: Vec<Watchpoint>,
    saved_cursors: Vec<Cursor<'static>>,
    filename_to_program: HashMap<PathBuf, String>,
    parsed_inputs: u32,
}

#[derive(Clone, Debug)]
pub struct Breakpoint {
    pub program: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct Watchpoint {
    pub code: String,
    pub last_result: Option<String>,
}

#[derive(Clone, Debug)]
pub enum InterpreterAction {
    LeoInterpretInto(String),
    LeoInterpretOver(String),
    Watch(String),
    RunFuture(usize),
    Breakpoint(Breakpoint),
    PrintRegister(u64),
    Into,
    Over,
    Step,
    Run,
}

impl Interpreter {
    pub fn new<'a, P: 'a + AsRef<Path>, Q: 'a + AsRef<Path>>(
        leo_source_files: impl IntoIterator<Item = &'a P>,
        aleo_source_files: impl IntoIterator<Item = &'a Q>,
        signer: SvmAddress,
        block_height: u32,
    ) -> Result<Self> {
        Self::new_impl(
            &mut leo_source_files.into_iter().map(|p| p.as_ref()),
            &mut aleo_source_files.into_iter().map(|p| p.as_ref()),
            signer,
            block_height,
        )
    }

    fn get_ast(path: &Path, handler: &Handler, node_builder: &NodeBuilder) -> Result<Ast> {
        let text = fs::read_to_string(path).map_err(|e| CompilerError::file_read_error(path, e))?;
        let filename = FileName::Real(path.to_path_buf());
        let source_file = with_session_globals(|s| s.source_map.new_source(&text, filename));
        leo_parser::parse_ast::<TestnetV0>(handler.clone(), node_builder, &text, source_file.start_pos)
    }

    fn new_impl(
        leo_source_files: &mut dyn Iterator<Item = &Path>,
        aleo_source_files: &mut dyn Iterator<Item = &Path>,
        signer: SvmAddress,
        block_height: u32,
    ) -> Result<Self> {
        let handler = Handler::default();
        let node_builder = Default::default();
        let mut cursor: Cursor<'_> = Cursor::new(
            true, // really_async
            signer,
            block_height,
        );
        let mut filename_to_program = HashMap::new();
        for path in leo_source_files {
            let ast = Self::get_ast(path, &handler, &node_builder)?;
            // TODO: This leak is silly.
            let ast = Box::leak(Box::new(ast));
            for (&program, scope) in ast.ast.program_scopes.iter() {
                filename_to_program.insert(path.to_path_buf(), program.to_string());
                for (name, function) in scope.functions.iter() {
                    cursor.functions.insert(GlobalId { program, name: *name }, FunctionVariant::Leo(function));
                }

                for (name, composite) in scope.structs.iter() {
                    cursor.structs.insert(
                        GlobalId { program, name: *name },
                        composite.members.iter().map(|member| member.identifier.name).collect(),
                    );
                }

                for (name, _mapping) in scope.mappings.iter() {
                    cursor.mappings.insert(GlobalId { program, name: *name }, HashMap::new());
                }

                for (name, const_declaration) in scope.consts.iter() {
                    cursor.frames.push(Frame {
                        step: 0,
                        element: Element::Expression(&const_declaration.value),
                        user_initiated: false,
                    });
                    cursor.over()?;
                    let value = cursor.values.pop().unwrap();
                    cursor.globals.insert(GlobalId { program, name: *name }, value);
                }
            }
        }

        for path in aleo_source_files {
            let aleo_program = Self::get_aleo_program(path)?;
            // TODO: Another goofy leak.
            let aleo_program = Box::leak(Box::new(aleo_program));
            let program = snarkvm_identifier_to_symbol(aleo_program.id().name());
            filename_to_program.insert(path.to_path_buf(), program.to_string());

            for (name, struct_type) in aleo_program.structs().iter() {
                cursor.structs.insert(
                    GlobalId { program, name: snarkvm_identifier_to_symbol(name) },
                    struct_type.members().keys().map(snarkvm_identifier_to_symbol).collect(),
                );
            }

            for (name, record_type) in aleo_program.records().iter() {
                cursor.structs.insert(
                    GlobalId { program, name: snarkvm_identifier_to_symbol(name) },
                    record_type.entries().keys().map(snarkvm_identifier_to_symbol).collect(),
                );
            }

            for (name, _mapping) in aleo_program.mappings().iter() {
                cursor.mappings.insert(GlobalId { program, name: snarkvm_identifier_to_symbol(name) }, HashMap::new());
            }

            for (name, function) in aleo_program.functions().iter() {
                cursor.functions.insert(
                    GlobalId { program, name: snarkvm_identifier_to_symbol(name) },
                    FunctionVariant::AleoFunction(function),
                );
            }

            for (name, closure) in aleo_program.closures().iter() {
                cursor.functions.insert(
                    GlobalId { program, name: snarkvm_identifier_to_symbol(name) },
                    FunctionVariant::AleoClosure(closure),
                );
            }
        }

        Ok(Interpreter {
            cursor,
            handler,
            node_builder,
            actions: Vec::new(),
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
            saved_cursors: Vec::new(),
            filename_to_program,
            parsed_inputs: 0,
        })
    }

    pub fn save_cursor(&mut self) {
        self.saved_cursors.push(self.cursor.clone());
    }

    /// Returns false if there was no saved cursor to restore.
    pub fn restore_cursor(&mut self) -> bool {
        if let Some(old_cursor) = self.saved_cursors.pop() {
            self.cursor = old_cursor;
            true
        } else {
            false
        }
    }

    fn get_aleo_program(path: &Path) -> Result<Program<TestnetV0>> {
        let text = fs::read_to_string(path).map_err(|e| CompilerError::file_read_error(path, e))?;
        let program = text.parse()?;
        Ok(program)
    }

    /// Returns true if any watchpoints changed.
    pub fn update_watchpoints(&mut self) -> Result<bool> {
        let mut changed = false;
        let safe_cursor = self.cursor.clone();

        for i in 0..self.watchpoints.len() {
            let code = self.watchpoints[i].code.clone();
            let new_value = match self.action(InterpreterAction::LeoInterpretOver(code)) {
                Ok(None) => None,
                Ok(Some(ret)) => Some(ret.to_string()),
                Err(LeoError::InterpreterHalt(halt)) => {
                    self.cursor = safe_cursor.clone();
                    Some(halt.to_string())
                }
                Err(e) => return Err(e),
            };
            if self.watchpoints[i].last_result != new_value {
                changed = true;
                self.watchpoints[i].last_result = new_value;
            }
        }
        Ok(changed)
    }

    pub fn action(&mut self, act: InterpreterAction) -> Result<Option<Value>> {
        use InterpreterAction::*;

        let ret = match &act {
            RunFuture(n) => {
                let future = self.cursor.futures.remove(*n);
                for async_exec in future.0.into_iter().rev() {
                    self.cursor.values.extend(async_exec.arguments);
                    self.cursor.frames.push(Frame {
                        step: 0,
                        element: Element::DelayedCall(async_exec.function),
                        user_initiated: true,
                    });
                }
                self.cursor.step()?
            }
            LeoInterpretInto(s) | LeoInterpretOver(s) => {
                let filename = FileName::Custom(format!("user_input{:04}", self.parsed_inputs));
                self.parsed_inputs += 1;
                let source_file = with_session_globals(|globals| globals.source_map.new_source(s, filename));
                let s = s.trim();
                if s.ends_with(';') {
                    let statement = leo_parser::parse_statement::<TestnetV0>(
                        self.handler.clone(),
                        &self.node_builder,
                        s,
                        source_file.start_pos,
                    )
                    .map_err(|_e| {
                        LeoError::InterpreterHalt(InterpreterHalt::new("failed to parse statement".into()))
                    })?;
                    // TODO: This leak is silly.
                    let stmt = Box::leak(Box::new(statement));

                    // The spans of the code the user wrote at the REPL are meaningless, so get rid of them.
                    self.cursor.frames.push(Frame { step: 0, element: Element::Statement(stmt), user_initiated: true });
                } else {
                    let expression = leo_parser::parse_expression::<TestnetV0>(
                        self.handler.clone(),
                        &self.node_builder,
                        s,
                        source_file.start_pos,
                    )
                    .map_err(|e| {
                        LeoError::InterpreterHalt(InterpreterHalt::new(format!("Failed to parse expression: {e}")))
                    })?;
                    // TODO: This leak is silly.
                    let expr = Box::leak(Box::new(expression));

                    // The spans of the code the user wrote at the REPL are meaningless, so get rid of them.
                    self.cursor.frames.push(Frame {
                        step: 0,
                        element: Element::Expression(expr),
                        user_initiated: true,
                    });
                };

                if matches!(act, LeoInterpretOver(..)) {
                    self.cursor.over()?
                } else {
                    StepResult { finished: false, value: None }
                }
            }

            Step => self.cursor.whole_step()?,

            Into => self.cursor.step()?,

            Over => self.cursor.over()?,

            Breakpoint(breakpoint) => {
                self.breakpoints.push(breakpoint.clone());
                StepResult { finished: false, value: None }
            }

            Watch(code) => {
                self.watchpoints.push(Watchpoint { code: code.clone(), last_result: None });
                StepResult { finished: false, value: None }
            }

            PrintRegister(register_index) => {
                let Some(Frame { element: Element::AleoExecution { registers, .. }, .. }) = self.cursor.frames.last()
                else {
                    halt_no_span!("cannot print register - not currently interpreting Aleo VM code");
                };

                if let Some(value) = registers.get(register_index) {
                    StepResult { finished: false, value: Some(value.clone()) }
                } else {
                    halt_no_span!("no such register {register_index}");
                }
            }

            Run => {
                while !self.cursor.frames.is_empty() {
                    if let Some((program, line)) = self.current_program_and_line() {
                        if self.breakpoints.iter().any(|bp| bp.program == program && bp.line == line) {
                            return Ok(None);
                        }
                    }
                    self.cursor.step()?;
                    if self.update_watchpoints()? {
                        return Ok(None);
                    }
                }
                StepResult { finished: false, value: None }
            }
        };

        self.actions.push(act);

        Ok(ret.value)
    }

    pub fn view_current(&self) -> Option<impl Display> {
        if let Some(span) = self.current_span() {
            if span != Default::default() {
                return with_session_globals(|s| s.source_map.contents_of_span(span));
            }
        }

        Some(match self.cursor.frames.last()?.element {
            Element::Statement(statement) => format!("{statement}"),
            Element::Expression(expression) => format!("{expression}"),
            Element::Block { block, .. } => format!("{block}"),
            Element::DelayedCall(gid) => format!("Delayed call to {gid}"),
            Element::AleoExecution { context, instruction_index, .. } => match context {
                AleoContext::Closure(closure) => closure.instructions().get(instruction_index).map(|i| format!("{i}")),
                AleoContext::Function(function) => {
                    function.instructions().get(instruction_index).map(|i| format!("{i}"))
                }
                AleoContext::Finalize(finalize) => finalize.commands().get(instruction_index).map(|i| format!("{i}")),
            }
            .unwrap_or_else(|| "...".to_string()),
        })
    }

    pub fn view_current_in_context(&self) -> Option<(impl Display, usize, usize)> {
        if let Some(Frame { element: Element::AleoExecution { context, instruction_index, .. }, .. }) =
            self.cursor.frames.last()
        {
            // For Aleo VM code, there are no spans; just print out the code without referring to the source code.

            fn write_all<I: Display>(
                items: impl Iterator<Item = I>,
                instruction_index: usize,
                result: &mut String,
                start: &mut usize,
                stop: &mut usize,
            ) {
                for (i, item) in items.enumerate() {
                    if i == instruction_index {
                        *start = result.len();
                    }
                    writeln!(result, "    {item}").expect("write shouldn't fail");
                    if i == instruction_index {
                        *stop = result.len();
                    }
                }
            }

            let mut result = String::new();
            let mut start: usize = 0usize;
            let mut stop: usize = 0usize;

            match context {
                AleoContext::Closure(closure) => {
                    writeln!(&mut result, "closure {}", closure.name()).expect("write shouldn't fail");
                    write_all(closure.inputs().iter(), usize::MAX, &mut result, &mut 0usize, &mut 0usize);
                    write_all(closure.instructions().iter(), *instruction_index, &mut result, &mut start, &mut stop);
                    write_all(closure.outputs().iter(), usize::MAX, &mut result, &mut 0usize, &mut 0usize);
                }
                AleoContext::Function(function) => {
                    writeln!(&mut result, "function {}", function.name()).expect("write shouldn't fail");
                    write_all(function.inputs().iter(), usize::MAX, &mut result, &mut 0usize, &mut 0usize);
                    write_all(function.instructions().iter(), *instruction_index, &mut result, &mut start, &mut stop);
                    write_all(function.outputs().iter(), usize::MAX, &mut result, &mut 0usize, &mut 0usize);
                }
                AleoContext::Finalize(finalize) => {
                    writeln!(&mut result, "finalize {}", finalize.name()).expect("write shouldn't fail");
                    write_all(finalize.inputs().iter(), usize::MAX, &mut result, &mut 0usize, &mut 0usize);
                    write_all(finalize.commands().iter(), *instruction_index, &mut result, &mut start, &mut stop);
                }
            }

            Some((result, start, stop))
        } else {
            // For Leo code, we use spans to print the original source code.
            let span = self.current_span()?;
            if span == Default::default() {
                return None;
            }
            with_session_globals(|s| {
                let source_file = s.source_map.find_source_file(span.lo)?;
                let first_span = Span::new(source_file.start_pos, span.lo);
                let last_span = Span::new(span.hi, source_file.end_pos);
                let mut result = String::new();
                result.push_str(&s.source_map.contents_of_span(first_span)?);
                let start = result.len();
                result.push_str(&s.source_map.contents_of_span(span)?);
                let stop = result.len();
                result.push_str(&s.source_map.contents_of_span(last_span)?);
                Some((result, start, stop))
            })
        }
    }

    fn current_program_and_line(&self) -> Option<(String, usize)> {
        if let Some(span) = self.current_span() {
            if let Some(location) = with_session_globals(|s| s.source_map.span_to_location(span)) {
                let line = location.line_start;
                if let FileName::Real(name) = &location.source_file.name {
                    if let Some(program) = self.filename_to_program.get(name) {
                        return Some((program.clone(), line));
                    }
                }
            }
        }
        None
    }

    fn current_span(&self) -> Option<Span> {
        self.cursor.frames.last().map(|f| f.element.span())
    }
}
