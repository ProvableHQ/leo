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

use leo_ast::{
    AssertVariant,
    Block,
    CoreConstant,
    CoreFunction,
    DefinitionPlace,
    Expression,
    Function,
    Statement,
    Type,
    Variant,
    interpreter_value::{
        AsyncExecution,
        CoreFunctionHelper,
        Future,
        GlobalId,
        StructContents,
        SvmAddress,
        Value,
        evaluate_binary,
        evaluate_core_function,
        evaluate_unary,
        literal_to_value,
    },
};
use leo_errors::{InterpreterHalt, Result};
use leo_passes::TypeTable;
use leo_span::{Span, Symbol, sym};

use snarkvm::prelude::{
    Closure as SvmClosure,
    Finalize as SvmFinalize,
    Function as SvmFunctionParam,
    ProgramID,
    TestnetV0,
};

use indexmap::{IndexMap, IndexSet};
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use std::{cmp::Ordering, collections::HashMap, mem, str::FromStr as _};

pub type Closure = SvmClosure<TestnetV0>;
pub type Finalize = SvmFinalize<TestnetV0>;
pub type SvmFunction = SvmFunctionParam<TestnetV0>;

/// Names associated to values in a function being executed.
#[derive(Clone, Debug)]
pub struct FunctionContext {
    program: Symbol,
    pub caller: SvmAddress,
    names: HashMap<Symbol, Value>,
    accumulated_futures: Future,
    is_async: bool,
}

/// A stack of contexts, building with the function call stack.
#[derive(Clone, Debug, Default)]
pub struct ContextStack {
    contexts: Vec<FunctionContext>,
    current_len: usize,
}

impl ContextStack {
    fn len(&self) -> usize {
        self.current_len
    }

    fn push(&mut self, program: Symbol, caller: SvmAddress, is_async: bool) {
        if self.current_len == self.contexts.len() {
            self.contexts.push(FunctionContext {
                program,
                caller,
                names: HashMap::new(),
                accumulated_futures: Default::default(),
                is_async,
            });
        }
        self.contexts[self.current_len].accumulated_futures.0.clear();
        self.contexts[self.current_len].names.clear();
        self.contexts[self.current_len].caller = caller;
        self.contexts[self.current_len].program = program;
        self.contexts[self.current_len].is_async = is_async;
        self.current_len += 1;
    }

    pub fn pop(&mut self) {
        // We never actually pop the underlying Vec
        // so we can reuse the storage of the hash
        // tables.
        assert!(self.len() > 0);
        self.current_len -= 1;
        self.contexts[self.current_len].names.clear();
    }

    /// Get the future accumulated by awaiting futures in the current function call.
    ///
    /// If the current code being interpreted is not in an async function, this
    /// will of course be empty.
    fn get_future(&mut self) -> Future {
        assert!(self.len() > 0);
        mem::take(&mut self.contexts[self.current_len - 1].accumulated_futures)
    }

    fn set(&mut self, symbol: Symbol, value: Value) {
        assert!(self.current_len > 0);
        self.last_mut().unwrap().names.insert(symbol, value);
    }

    pub fn add_future(&mut self, future: Future) {
        assert!(self.current_len > 0);
        self.contexts[self.current_len - 1].accumulated_futures.0.extend(future.0);
    }

    /// Are we currently in an async function?
    fn is_async(&self) -> bool {
        assert!(self.current_len > 0);
        self.last().unwrap().is_async
    }

    pub fn current_program(&self) -> Option<Symbol> {
        self.last().map(|c| c.program)
    }

    pub fn last(&self) -> Option<&FunctionContext> {
        self.len().checked_sub(1).and_then(|i| self.contexts.get(i))
    }

    fn last_mut(&mut self) -> Option<&mut FunctionContext> {
        self.len().checked_sub(1).and_then(|i| self.contexts.get_mut(i))
    }
}

#[derive(Clone, Debug)]
pub enum AleoContext {
    Closure(Closure),
    Function(SvmFunction),
    Finalize(Finalize),
}

/// A Leo construct to be evauated.
#[derive(Clone, Debug)]
pub enum Element {
    /// A Leo statement.
    Statement(Statement),

    /// A Leo expression.
    Expression(Expression),

    /// A Leo block.
    ///
    /// We have a separate variant for Leo blocks for two reasons:
    /// 1. In a ConditionalExpression, the `then` block is stored
    ///    as just a Block with no statement, and
    /// 2. We need to remember if a Block came from a function body,
    ///    so that if such a block ends, we know to push a `Unit` to
    ///    the values stack.
    Block {
        block: Block,
        function_body: bool,
    },

    AleoExecution {
        context: Box<AleoContext>,
        registers: IndexMap<u64, Value>,
        instruction_index: usize,
    },

    DelayedCall(GlobalId),
}

impl Element {
    pub fn span(&self) -> Span {
        use Element::*;
        match self {
            Statement(statement) => statement.span(),
            Expression(expression) => expression.span(),
            Block { block, .. } => block.span(),
            AleoExecution { .. } | DelayedCall(..) => Default::default(),
        }
    }
}

/// A frame of execution, keeping track of the Element next to
/// be executed and the number of steps we've done so far.
#[derive(Clone, Debug)]
pub struct Frame {
    pub step: usize,
    pub element: Element,
    pub user_initiated: bool,
}

#[derive(Clone, Debug)]
pub enum FunctionVariant {
    Leo(Function),
    AleoClosure(Closure),
    AleoFunction(SvmFunction),
}

/// Tracks the current execution state - a cursor into the running program.
#[derive(Clone, Debug)]
pub struct Cursor {
    /// Stack of execution frames, with the one currently to be executed on top.
    pub frames: Vec<Frame>,

    /// Stack of values from evaluated expressions.
    ///
    /// Each time an expression completes evaluation, a value is pushed here.
    pub values: Vec<Value>,

    /// All functions (or transitions or inlines) in any program being interpreted.
    pub functions: HashMap<GlobalId, FunctionVariant>,

    /// Consts are stored here.
    pub globals: HashMap<GlobalId, Value>,

    pub user_values: HashMap<Symbol, Value>,

    pub mappings: HashMap<GlobalId, HashMap<Value, Value>>,

    /// For each struct type, we only need to remember the names of its members, in order.
    pub structs: HashMap<GlobalId, IndexSet<Symbol>>,

    pub futures: Vec<Future>,

    pub contexts: ContextStack,

    pub type_table: TypeTable,

    pub signer: SvmAddress,

    pub rng: ChaCha20Rng,

    pub block_height: u32,

    pub really_async: bool,

    pub program: Option<Symbol>,
}

impl CoreFunctionHelper for Cursor {
    fn pop_value_impl(&mut self) -> Option<Value> {
        self.values.pop()
    }

    fn set_block_height(&mut self, height: u32) {
        self.block_height = height;
    }

    fn lookup_mapping(&self, program: Option<Symbol>, name: Symbol) -> Option<&HashMap<Value, Value>> {
        Cursor::lookup_mapping(self, program, name)
    }

    fn lookup_mapping_mut(&mut self, program: Option<Symbol>, name: Symbol) -> Option<&mut HashMap<Value, Value>> {
        Cursor::lookup_mapping_mut(self, program, name)
    }

    fn rng(&mut self) -> Option<&mut ChaCha20Rng> {
        Some(&mut self.rng)
    }
}

impl Cursor {
    /// `really_async` indicates we should really delay execution of async function calls until the user runs them.
    pub fn new(really_async: bool, signer: SvmAddress, block_height: u32) -> Self {
        Cursor {
            frames: Default::default(),
            values: Default::default(),
            functions: Default::default(),
            globals: Default::default(),
            user_values: Default::default(),
            mappings: Default::default(),
            structs: Default::default(),
            contexts: Default::default(),
            futures: Default::default(),
            type_table: Default::default(),
            rng: ChaCha20Rng::from_entropy(),
            signer,
            block_height,
            really_async,
            program: None,
        }
    }

    pub fn set_program(&mut self, program: &str) {
        let p = program.strip_suffix(".aleo").unwrap_or(program);
        self.program = Some(Symbol::intern(p));
    }

    pub fn current_program(&self) -> Option<Symbol> {
        self.contexts.current_program().or(self.program)
    }

    pub fn increment_step(&mut self) {
        let Some(Frame { step, .. }) = self.frames.last_mut() else {
            panic!("frame expected");
        };
        *step += 1;
    }

    fn new_caller(&self) -> SvmAddress {
        if let Some(function_context) = self.contexts.last() {
            let program_id = ProgramID::<TestnetV0>::from_str(&format!("{}.aleo", function_context.program))
                .expect("should be able to create ProgramID");
            program_id.to_address().expect("should be able to convert to address")
        } else {
            self.signer
        }
    }

    fn pop_value(&mut self) -> Result<Value> {
        match self.values.pop() {
            Some(v) => Ok(v),
            None => {
                Err(InterpreterHalt::new("value expected - this may be a bug in the Leo interpreter".to_string())
                    .into())
            }
        }
    }

    fn lookup(&self, name: Symbol) -> Option<Value> {
        if let Some(context) = self.contexts.last() {
            let option_value =
                context.names.get(&name).or_else(|| self.globals.get(&GlobalId { program: context.program, name }));
            if option_value.is_some() {
                return option_value.cloned();
            }
        };

        self.user_values.get(&name).cloned()
    }

    pub fn lookup_mapping(&self, program: Option<Symbol>, name: Symbol) -> Option<&HashMap<Value, Value>> {
        let Some(program) = program.or_else(|| self.current_program()) else {
            panic!("no program for mapping lookup");
        };
        self.mappings.get(&GlobalId { program, name })
    }

    pub fn lookup_mapping_mut(&mut self, program: Option<Symbol>, name: Symbol) -> Option<&mut HashMap<Value, Value>> {
        let Some(program) = program.or_else(|| self.current_program()) else {
            panic!("no program for mapping lookup");
        };
        self.mappings.get_mut(&GlobalId { program, name })
    }

    fn lookup_function(&self, program: Symbol, name: Symbol) -> Option<FunctionVariant> {
        self.functions.get(&GlobalId { program, name }).cloned()
    }

    fn set_variable(&mut self, symbol: Symbol, value: Value) {
        if self.contexts.len() > 0 {
            self.contexts.set(symbol, value);
        } else {
            self.user_values.insert(symbol, value);
        }
    }

    /// Execute the whole step of the current Element.
    ///
    /// That is, perform a step, and then finish all statements and expressions that have been pushed,
    /// until we're ready for the next step of the current Element (if there is one).
    pub fn whole_step(&mut self) -> Result<StepResult> {
        let frames_len = self.frames.len();
        let initial_result = self.step()?;
        if !initial_result.finished {
            while self.frames.len() > frames_len {
                self.step()?;
            }
        }
        Ok(initial_result)
    }

    /// Step `over` the current Element.
    ///
    /// That is, continue executing until the current Element is finished.
    pub fn over(&mut self) -> Result<StepResult> {
        let frames_len = self.frames.len();
        loop {
            match self.frames.len().cmp(&frames_len) {
                Ordering::Greater => {
                    self.step()?;
                }
                Ordering::Equal => {
                    let result = self.step()?;
                    if result.finished {
                        return Ok(result);
                    }
                }
                Ordering::Less => {
                    // This can happen if, for instance, a `return` was encountered,
                    // which means we exited the function we were evaluating and the
                    // frame stack was truncated.
                    return Ok(StepResult { finished: true, value: None });
                }
            }
        }
    }

    pub fn step_block(&mut self, block: &Block, function_body: bool, step: usize) -> bool {
        let len = self.frames.len();

        let done = match step {
            0 => {
                for statement in block.statements.iter().rev() {
                    self.frames.push(Frame {
                        element: Element::Statement(statement.clone()),
                        step: 0,
                        user_initiated: false,
                    });
                }
                false
            }
            1 if function_body => {
                self.values.push(Value::Unit);
                self.contexts.pop();
                true
            }
            1 => true,
            _ => unreachable!(),
        };

        if done {
            assert_eq!(len, self.frames.len());
            self.frames.pop();
        } else {
            self.frames[len - 1].step += 1;
        }

        done
    }

    fn step_statement(&mut self, statement: &Statement, step: usize) -> Result<bool> {
        let len = self.frames.len();

        let mut push = |expression: &Expression| {
            self.frames.push(Frame { element: Element::Expression(expression.clone()), step: 0, user_initiated: false })
        };

        let done = match statement {
            Statement::Assert(assert) if step == 0 => {
                match &assert.variant {
                    AssertVariant::Assert(x) => push(x),
                    AssertVariant::AssertEq(x, y) | AssertVariant::AssertNeq(x, y) => {
                        push(y);
                        push(x);
                    }
                };
                false
            }
            Statement::Assert(assert) if step == 1 => {
                match &assert.variant {
                    AssertVariant::Assert(..) => {
                        let value = self.pop_value()?;
                        match value {
                            Value::Bool(true) => {}
                            Value::Bool(false) => halt!(assert.span(), "assert failure"),
                            _ => tc_fail!(),
                        }
                    }
                    AssertVariant::AssertEq(..) | AssertVariant::AssertNeq(..) => {
                        let x = self.pop_value()?;
                        let y = self.pop_value()?;
                        let b =
                            if matches!(assert.variant, AssertVariant::AssertEq(..)) { x.eq(&y)? } else { x.neq(&y)? };
                        if !b {
                            halt!(assert.span(), "assert failure");
                        }
                    }
                };
                true
            }
            Statement::Assign(assign) if step == 0 => {
                push(&assign.value);
                false
            }
            Statement::Assign(assign) if step == 1 => {
                let value = self.values.pop().unwrap();
                match &assign.place {
                    Expression::Identifier(name) => self.set_variable(name.name, value),
                    Expression::TupleAccess(tuple_access) => {
                        let Expression::Identifier(identifier) = tuple_access.tuple else {
                            halt!(assign.span(), "tuple assignments must refer to identifiers.");
                        };
                        let mut current_tuple = self.lookup(identifier.name).expect_tc(identifier.span())?;
                        let Value::Tuple(tuple) = &mut current_tuple else {
                            halt!(tuple_access.span(), "Type error: this must be a tuple.");
                        };
                        tuple[tuple_access.index.value()] = value;
                        self.set_variable(identifier.name, current_tuple);
                    }
                    _ => halt!(assign.span(), "Invalid assignment place."),
                }
                true
            }
            Statement::Block(block) => return Ok(self.step_block(block, false, step)),
            Statement::Conditional(conditional) if step == 0 => {
                push(&conditional.condition);
                false
            }
            Statement::Conditional(conditional) if step == 1 => {
                match self.pop_value()? {
                    Value::Bool(true) => self.frames.push(Frame {
                        step: 0,
                        element: Element::Block { block: conditional.then.clone(), function_body: false },
                        user_initiated: false,
                    }),
                    Value::Bool(false) => {
                        if let Some(otherwise) = conditional.otherwise.as_ref() {
                            self.frames.push(Frame {
                                step: 0,
                                element: Element::Statement(Statement::clone(otherwise)),
                                user_initiated: false,
                            })
                        }
                    }
                    _ => tc_fail!(),
                };
                false
            }
            Statement::Conditional(_) if step == 2 => true,
            Statement::Const(const_) if step == 0 => {
                push(&const_.value);
                false
            }
            Statement::Const(const_) if step == 1 => {
                let value = self.pop_value()?;
                self.set_variable(const_.place.name, value);
                true
            }
            Statement::Definition(definition) if step == 0 => {
                push(&definition.value);
                false
            }
            Statement::Definition(definition) if step == 1 => {
                let value = self.pop_value()?;
                match &definition.place {
                    DefinitionPlace::Single(id) => self.set_variable(id.name, value),
                    DefinitionPlace::Multiple(ids) => {
                        let Value::Tuple(rhs) = value else {
                            tc_fail!();
                        };
                        for (id, val) in ids.iter().zip(rhs.into_iter()) {
                            self.set_variable(id.name, val);
                        }
                    }
                }
                true
            }
            Statement::Expression(expression) if step == 0 => {
                push(&expression.expression);
                false
            }
            Statement::Expression(_) if step == 1 => {
                self.values.pop();
                true
            }
            Statement::Iteration(iteration) if step == 0 => {
                assert!(!iteration.inclusive);
                push(&iteration.stop);
                push(&iteration.start);
                false
            }
            Statement::Iteration(iteration) => {
                // Currently there actually isn't a syntax in Leo for inclusive ranges.
                let stop = self.pop_value()?;
                let start = self.pop_value()?;
                if start.eq(&stop)? {
                    true
                } else {
                    let new_start = start.inc_wrapping();
                    self.set_variable(iteration.variable.name, start);
                    self.frames.push(Frame {
                        step: 0,
                        element: Element::Block { block: iteration.block.clone(), function_body: false },
                        user_initiated: false,
                    });
                    self.values.push(new_start);
                    self.values.push(stop);
                    false
                }
            }
            Statement::Return(return_) if step == 0 => {
                push(&return_.expression);
                false
            }
            Statement::Return(_) if step == 1 => loop {
                let last_frame = self.frames.last().expect("a frame should be present");
                match last_frame.element {
                    Element::Expression(Expression::Call(_)) | Element::DelayedCall(_) => {
                        if self.contexts.is_async() {
                            // Get rid of the Unit we previously pushed, and replace it with a Future.
                            self.values.pop();
                            self.values.push(Value::Future(self.contexts.get_future()));
                        }
                        self.contexts.pop();
                        return Ok(true);
                    }
                    _ => {
                        self.frames.pop();
                    }
                }
            },
            _ => unreachable!(),
        };

        if done {
            assert_eq!(len, self.frames.len());
            self.frames.pop();
        } else {
            self.frames[len - 1].step += 1;
        }

        Ok(done)
    }

    fn step_expression(&mut self, expression: &Expression, step: usize) -> Result<bool> {
        let len = self.frames.len();

        macro_rules! push {
            () => {
                |expression: &Expression| {
                    self.frames.push(Frame {
                        element: Element::Expression(expression.clone()),
                        step: 0,
                        user_initiated: false,
                    })
                }
            };
        }

        if let Some(value) = match expression {
            Expression::ArrayAccess(array) if step == 0 => {
                push!()(&array.index);
                push!()(&array.array);
                None
            }
            Expression::ArrayAccess(array) if step == 1 => {
                let span = array.span();
                let index = self.pop_value()?;
                let array = self.pop_value()?;

                let index_usize: usize = match index {
                    Value::U8(x) => x.into(),
                    Value::U16(x) => x.into(),
                    Value::U32(x) => x.try_into().expect_tc(span)?,
                    Value::U64(x) => x.try_into().expect_tc(span)?,
                    Value::U128(x) => x.try_into().expect_tc(span)?,
                    Value::I8(x) => x.try_into().expect_tc(span)?,
                    Value::I16(x) => x.try_into().expect_tc(span)?,
                    Value::I32(x) => x.try_into().expect_tc(span)?,
                    Value::I64(x) => x.try_into().expect_tc(span)?,
                    Value::I128(x) => x.try_into().expect_tc(span)?,
                    _ => halt!(expression.span(), "invalid array index {index}"),
                };
                let Value::Array(vec_array) = array else { tc_fail!() };
                Some(vec_array.get(index_usize).expect_tc(span)?.clone())
            }
            Expression::MemberAccess(access) => match &access.inner {
                Expression::Identifier(identifier) if identifier.name == sym::SelfLower => match access.name.name {
                    sym::signer => Some(Value::Address(self.signer)),
                    sym::caller => {
                        if let Some(function_context) = self.contexts.last() {
                            Some(Value::Address(function_context.caller))
                        } else {
                            Some(Value::Address(self.signer))
                        }
                    }
                    _ => halt!(access.span(), "unknown member of self"),
                },
                Expression::Identifier(identifier) if identifier.name == sym::block => match access.name.name {
                    sym::height => Some(Value::U32(self.block_height)),
                    _ => halt!(access.span(), "unknown member of block"),
                },

                // Otherwise, we just have a normal struct member access.
                _ if step == 0 => {
                    push!()(&access.inner);
                    None
                }
                _ if step == 1 => {
                    let Some(Value::Struct(struct_)) = self.values.pop() else {
                        tc_fail!();
                    };
                    let value = struct_.contents.get(&access.name.name).cloned();
                    if value.is_none() {
                        tc_fail!();
                    }
                    value
                }
                _ => unreachable!("we've actually covered all possible patterns above"),
            },
            Expression::TupleAccess(tuple_access) if step == 0 => {
                push!()(&tuple_access.tuple);
                None
            }
            Expression::TupleAccess(tuple_access) if step == 1 => {
                let Some(value) = self.values.pop() else { tc_fail!() };
                let Value::Tuple(tuple) = value else {
                    halt!(tuple_access.span(), "Type error");
                };
                if let Some(result) = tuple.get(tuple_access.index.value()) {
                    Some(result.clone())
                } else {
                    halt!(tuple_access.span(), "Tuple index out of range");
                }
            }
            Expression::Array(array) if step == 0 => {
                array.elements.iter().rev().for_each(push!());
                None
            }
            Expression::Array(array) if step == 1 => {
                let len = self.values.len();
                let array_values = self.values.drain(len - array.elements.len()..).collect();
                Some(Value::Array(array_values))
            }
            Expression::AssociatedConstant(constant) if step == 0 => {
                let Type::Identifier(type_ident) = constant.ty else {
                    tc_fail!();
                };
                let Some(core_constant) = CoreConstant::from_symbols(type_ident.name, constant.name.name) else {
                    halt!(constant.span(), "Unknown constant {constant}");
                };
                match core_constant {
                    CoreConstant::GroupGenerator => Some(Value::generator()),
                }
            }
            Expression::AssociatedFunction(function) if step == 0 => {
                let Some(core_function) = CoreFunction::from_symbols(function.variant.name, function.name.name) else {
                    halt!(function.span(), "Unkown core function {function}");
                };

                // We want to push expressions for each of the arguments... except for mappings,
                // because we don't look them up as Values.
                match core_function {
                    CoreFunction::MappingGet | CoreFunction::MappingRemove | CoreFunction::MappingContains => {
                        push!()(&function.arguments[1]);
                    }
                    CoreFunction::MappingGetOrUse | CoreFunction::MappingSet => {
                        push!()(&function.arguments[2]);
                        push!()(&function.arguments[1]);
                    }
                    CoreFunction::CheatCodePrintMapping => {
                        // Do nothing, as we don't need to evaluate the mapping.
                    }
                    _ => function.arguments.iter().rev().for_each(push!()),
                }
                None
            }
            Expression::AssociatedFunction(function) if step == 1 => {
                let Some(core_function) = CoreFunction::from_symbols(function.variant.name, function.name.name) else {
                    halt!(function.span(), "Unkown core function {function}");
                };

                let span = function.span();

                if let CoreFunction::FutureAwait = core_function {
                    let value = self.pop_value()?;
                    let Value::Future(future) = value else {
                        halt!(span, "Invalid value for await: {value}");
                    };
                    for async_execution in future.0 {
                        self.values.extend(async_execution.arguments.into_iter());
                        self.frames.push(Frame {
                            step: 0,
                            element: Element::DelayedCall(async_execution.function),
                            user_initiated: false,
                        });
                    }
                    // For an await, we have one extra step - first we must evaluate the delayed call.
                    None
                } else {
                    let value = evaluate_core_function(self, core_function.clone(), &function.arguments, span)?;
                    assert!(value.is_some());
                    value
                }
            }
            Expression::AssociatedFunction(function) if step == 2 => {
                let Some(core_function) = CoreFunction::from_symbols(function.variant.name, function.name.name) else {
                    halt!(function.span(), "Unkown core function {function}");
                };
                assert!(core_function == CoreFunction::FutureAwait);
                Some(Value::Unit)
            }
            Expression::Binary(binary) if step == 0 => {
                push!()(&binary.right);
                push!()(&binary.left);
                None
            }
            Expression::Binary(binary) if step == 1 => {
                let rhs = self.pop_value()?;
                let lhs = self.pop_value()?;
                Some(evaluate_binary(binary.span, binary.op, &lhs, &rhs)?)
            }
            Expression::Call(call) if step == 0 => {
                call.arguments.iter().rev().for_each(push!());
                None
            }
            Expression::Call(call) if step == 1 => {
                let len = self.values.len();
                let (program, name) = {
                    let maybe_program = call.program.or_else(|| self.current_program());
                    if let Some(program) = maybe_program {
                        (program, call.function.name)
                    } else {
                        halt!(call.span, "No current program");
                    }
                };
                // It's a bit cheesy to collect the arguments into a Vec first, but it's the easiest way
                // to handle lifetimes here.
                let arguments: Vec<Value> = self.values.drain(len - call.arguments.len()..).collect();
                self.do_call(
                    program,
                    name,
                    arguments.into_iter(),
                    false, // finalize
                    call.span(),
                )?;
                None
            }
            Expression::Call(_call) if step == 2 => Some(self.pop_value()?),
            Expression::Cast(cast) if step == 0 => {
                push!()(&cast.expression);
                None
            }
            Expression::Cast(cast) if step == 1 => {
                let span = cast.span();
                let arg = self.pop_value()?;
                match arg.cast(&cast.type_) {
                    Some(value) => Some(value),
                    None => return Err(InterpreterHalt::new_spanned("cast failure".to_string(), span).into()),
                }
            }
            Expression::Err(_) => todo!(),
            Expression::Identifier(identifier) if step == 0 => {
                Some(self.lookup(identifier.name).expect_tc(identifier.span())?)
            }
            Expression::Literal(literal) if step == 0 => {
                Some(literal_to_value(literal, &self.type_table.get(&expression.id()))?)
            }
            Expression::Locator(_locator) => todo!(),
            Expression::Struct(struct_) if step == 0 => {
                struct_.members.iter().flat_map(|init| init.expression.as_ref()).for_each(push!());
                None
            }
            Expression::Struct(struct_) if step == 1 => {
                // Collect all the key/value pairs into a HashMap.
                let mut contents_tmp = HashMap::with_capacity(struct_.members.len());
                for initializer in struct_.members.iter() {
                    let name = initializer.identifier.name;
                    let value = if initializer.expression.is_some() {
                        self.pop_value()?
                    } else {
                        self.lookup(name).expect_tc(struct_.span())?
                    };
                    contents_tmp.insert(name, value);
                }

                // And now put them into an IndexMap in the correct order.
                let program = self.current_program().expect("there should be a current program");
                let id = GlobalId { program, name: struct_.name.name };
                let struct_type = self.structs.get(&id).expect_tc(struct_.span())?;
                let contents = struct_type
                    .iter()
                    .map(|sym| (*sym, contents_tmp.remove(sym).expect("we just inserted this")))
                    .collect();

                Some(Value::Struct(StructContents { name: struct_.name.name, contents }))
            }
            Expression::Ternary(ternary) if step == 0 => {
                push!()(&ternary.condition);
                None
            }
            Expression::Ternary(ternary) if step == 1 => {
                let condition = self.pop_value()?;
                match condition {
                    Value::Bool(true) => push!()(&ternary.if_true),
                    Value::Bool(false) => push!()(&ternary.if_false),
                    _ => halt!(ternary.span(), "Invalid type for ternary expression {ternary}"),
                }
                None
            }
            Expression::Ternary(_) if step == 2 => Some(self.pop_value()?),
            Expression::Tuple(tuple) if step == 0 => {
                tuple.elements.iter().rev().for_each(push!());
                None
            }
            Expression::Tuple(tuple) if step == 1 => {
                let len = self.values.len();
                let tuple_values = self.values.drain(len - tuple.elements.len()..).collect();
                Some(Value::Tuple(tuple_values))
            }
            Expression::Unary(unary) if step == 0 => {
                push!()(&unary.receiver);
                None
            }
            Expression::Unary(unary) if step == 1 => {
                let value = self.pop_value()?;
                Some(evaluate_unary(unary.span, unary.op, &value)?)
            }
            Expression::Unit(_) if step == 0 => Some(Value::Unit),
            x => unreachable!("Unexpected expression {x}"),
        } {
            assert_eq!(self.frames.len(), len);
            self.frames.pop();
            self.values.push(value);
            Ok(true)
        } else {
            self.frames[len - 1].step += 1;
            Ok(false)
        }
    }

    /// Execute one step of the current element.
    ///
    /// Many Leo constructs require multiple steps. For instance, when executing a conditional,
    /// the first step will push the condition expression to the stack. Once that has executed
    /// and we've returned to the conditional, we push the `then` or `otherwise` block to the
    /// stack. Once that has executed and we've returned to the conditional, the final step
    /// does nothing.
    pub fn step(&mut self) -> Result<StepResult> {
        if self.frames.is_empty() {
            return Err(InterpreterHalt::new("no execution frames available".into()).into());
        }

        let Frame { element, step, user_initiated } = self.frames.last().expect("there should be a frame").clone();
        match element {
            Element::Block { block, function_body } => {
                let finished = self.step_block(&block, function_body, step);
                Ok(StepResult { finished, value: None })
            }
            Element::Statement(statement) => {
                let finished = self.step_statement(&statement, step)?;
                Ok(StepResult { finished, value: None })
            }
            Element::Expression(expression) => {
                let finished = self.step_expression(&expression, step)?;
                let value = match (finished, user_initiated) {
                    (false, _) => None,
                    (true, false) => self.values.last().cloned(),
                    (true, true) => self.values.pop(),
                };
                let maybe_future = if let Some(Value::Tuple(vals)) = &value { vals.last() } else { value.as_ref() };

                if let Some(Value::Future(future)) = &maybe_future {
                    if user_initiated && !future.0.is_empty() {
                        self.futures.push(future.clone());
                    }
                }
                Ok(StepResult { finished, value })
            }
            Element::AleoExecution { .. } => {
                self.step_aleo()?;
                Ok(StepResult { finished: true, value: None })
            }
            Element::DelayedCall(gid) if step == 0 => {
                match self.lookup_function(gid.program, gid.name).expect("function should exist") {
                    FunctionVariant::Leo(function) => {
                        assert!(function.variant == Variant::AsyncFunction);
                        let len = self.values.len();
                        let values: Vec<Value> = self.values.drain(len - function.input.len()..).collect();
                        self.contexts.push(
                            gid.program,
                            self.signer,
                            true, // is_async
                        );
                        let param_names = function.input.iter().map(|input| input.identifier.name);
                        for (name, value) in param_names.zip(values) {
                            self.set_variable(name, value);
                        }
                        self.frames.last_mut().unwrap().step = 1;
                        self.frames.push(Frame {
                            step: 0,
                            element: Element::Block { block: function.block.clone(), function_body: true },
                            user_initiated: false,
                        });
                        Ok(StepResult { finished: false, value: None })
                    }
                    FunctionVariant::AleoFunction(function) => {
                        let Some(finalize_f) = function.finalize_logic() else {
                            panic!("must have finalize logic for a delayed call");
                        };
                        let len = self.values.len();
                        let values_iter = self.values.drain(len - finalize_f.inputs().len()..);
                        self.contexts.push(
                            gid.program,
                            self.signer,
                            true, // is_async
                        );
                        self.frames.last_mut().unwrap().step = 1;
                        self.frames.push(Frame {
                            step: 0,
                            element: Element::AleoExecution {
                                context: AleoContext::Finalize(finalize_f.clone()).into(),
                                registers: values_iter.enumerate().map(|(i, v)| (i as u64, v)).collect(),
                                instruction_index: 0,
                            },
                            user_initiated: false,
                        });
                        Ok(StepResult { finished: false, value: None })
                    }
                    FunctionVariant::AleoClosure(..) => panic!("A call to a closure can't be delayed"),
                }
            }
            Element::DelayedCall(_gid) => {
                assert_eq!(step, 1);
                let value = self.values.pop();
                self.frames.pop();
                Ok(StepResult { finished: true, value })
            }
        }
    }

    pub fn do_call(
        &mut self,
        function_program: Symbol,
        function_name: Symbol,
        arguments: impl Iterator<Item = Value>,
        finalize: bool,
        span: Span,
    ) -> Result<()> {
        let Some(function_variant) = self.lookup_function(function_program, function_name) else {
            halt!(span, "unknown function {function_program}.aleo/{function_name}");
        };
        match function_variant {
            FunctionVariant::Leo(function) => {
                let caller = if matches!(function.variant, Variant::Transition | Variant::AsyncTransition) {
                    self.new_caller()
                } else {
                    self.signer
                };
                if self.really_async && function.variant == Variant::AsyncFunction {
                    // Don't actually run the call now.
                    let async_ex = AsyncExecution {
                        function: GlobalId { name: function_name, program: function_program },
                        arguments: arguments.collect(),
                    };
                    self.values.push(Value::Future(Future(vec![async_ex])));
                } else {
                    let is_async = function.variant == Variant::AsyncFunction;
                    self.contexts.push(function_program, caller, is_async);
                    let param_names = function.input.iter().map(|input| input.identifier.name);
                    for (name, value) in param_names.zip(arguments) {
                        self.set_variable(name, value);
                    }
                    self.frames.push(Frame {
                        step: 0,
                        element: Element::Block { block: function.block.clone(), function_body: true },
                        user_initiated: false,
                    });
                }
            }
            FunctionVariant::AleoClosure(closure) => {
                self.contexts.push(function_program, self.signer, false);
                let context = AleoContext::Closure(closure);
                self.frames.push(Frame {
                    step: 0,
                    element: Element::AleoExecution {
                        context: context.into(),
                        registers: arguments.enumerate().map(|(i, v)| (i as u64, v)).collect(),
                        instruction_index: 0,
                    },
                    user_initiated: false,
                });
            }
            FunctionVariant::AleoFunction(function) => {
                let caller = self.new_caller();
                self.contexts.push(function_program, caller, false);
                let context = if finalize {
                    let Some(finalize_f) = function.finalize_logic() else {
                        panic!("finalize call with no finalize logic");
                    };
                    AleoContext::Finalize(finalize_f.clone())
                } else {
                    AleoContext::Function(function)
                };
                self.frames.push(Frame {
                    step: 0,
                    element: Element::AleoExecution {
                        context: context.into(),
                        registers: arguments.enumerate().map(|(i, v)| (i as u64, v)).collect(),
                        instruction_index: 0,
                    },
                    user_initiated: false,
                });
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct StepResult {
    /// Has this element completely finished running?
    pub finished: bool,

    /// If the element was an expression, here's its value.
    pub value: Option<Value>,
}
