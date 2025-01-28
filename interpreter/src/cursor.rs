// Copyright (C) 2019-2025 Aleo Systems Inc.
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
    AccessExpression,
    AssertVariant,
    BinaryOperation,
    Block,
    CoreConstant,
    CoreFunction,
    Expression,
    Function,
    GroupLiteral,
    IntegerType,
    Literal,
    Statement,
    Type,
    UnaryOperation,
    Variant,
};
use leo_errors::{InterpreterHalt, Result};
use leo_span::{Span, Symbol, sym};

use snarkvm::prelude::{
    CastLossy as _,
    Closure as SvmClosure,
    Double as _,
    Finalize as SvmFinalize,
    Function as SvmFunctionParam,
    Inverse as _,
    Network as _,
    Pow as _,
    ProgramID,
    Square as _,
    SquareRoot as _,
    TestnetV0,
    ToBits,
};

use indexmap::{IndexMap, IndexSet};
use rand::Rng as _;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use std::{cmp::Ordering, collections::HashMap, fmt, mem, str::FromStr as _};

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

#[derive(Copy, Clone, Debug)]
pub enum AleoContext<'a> {
    Closure(&'a Closure),
    Function(&'a SvmFunction),
    Finalize(&'a Finalize),
}

/// A Leo construct to be evauated.
#[derive(Clone, Debug)]
pub enum Element<'a> {
    /// A Leo statement.
    Statement(&'a Statement),

    /// A Leo expression.
    Expression(&'a Expression),

    /// A Leo block.
    ///
    /// We have a separate variant for Leo blocks for two reasons:
    /// 1. In a ConditionalExpression, the `then` block is stored
    ///    as just a Block with no statement, and
    /// 2. We need to remember if a Block came from a function body,
    ///    so that if such a block ends, we know to push a `Unit` to
    ///    the values stack.
    Block {
        block: &'a Block,
        function_body: bool,
    },

    AleoExecution {
        context: AleoContext<'a>,
        registers: IndexMap<u64, Value>,
        instruction_index: usize,
    },

    DelayedCall(GlobalId),
}

impl Element<'_> {
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
pub struct Frame<'a> {
    pub step: usize,
    pub element: Element<'a>,
    pub user_initiated: bool,
}

/// Global values - such as mappings, functions, etc -
/// are identified by program and name.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct GlobalId {
    pub program: Symbol,
    pub name: Symbol,
}

impl fmt::Display for GlobalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.program, self.name)
    }
}

#[derive(Clone, Debug)]
pub enum FunctionVariant<'a> {
    Leo(&'a Function),
    AleoClosure(&'a Closure),
    AleoFunction(&'a SvmFunction),
}

/// Tracks the current execution state - a cursor into the running program.
#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    /// Stack of execution frames, with the one currently to be executed on top.
    pub frames: Vec<Frame<'a>>,

    /// Stack of values from evaluated expressions.
    ///
    /// Each time an expression completes evaluation, a value is pushed here.
    pub values: Vec<Value>,

    /// All functions (or transitions or inlines) in any program being interpreted.
    pub functions: HashMap<GlobalId, FunctionVariant<'a>>,

    /// Consts are stored here.
    pub globals: HashMap<GlobalId, Value>,

    pub user_values: HashMap<Symbol, Value>,

    pub mappings: HashMap<GlobalId, HashMap<Value, Value>>,

    /// For each struct type, we only need to remember the names of its members, in order.
    pub structs: HashMap<GlobalId, IndexSet<Symbol>>,

    pub futures: Vec<Future>,

    pub contexts: ContextStack,

    pub signer: SvmAddress,

    pub rng: ChaCha20Rng,

    pub block_height: u32,

    pub really_async: bool,

    pub program: Option<Symbol>,
}

impl<'a> Cursor<'a> {
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

    fn lookup_function(&self, program: Symbol, name: Symbol) -> Option<FunctionVariant<'a>> {
        self.functions.get(&GlobalId { program, name }).cloned()
    }

    fn set_variable(&mut self, symbol: Symbol, value: Value) {
        if self.contexts.len() > 0 {
            self.contexts.set(symbol, value);
        } else {
            self.user_values.insert(symbol, value);
        }
    }

    fn set_block_height(&mut self, block_height: u32) {
        self.block_height = block_height;
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

    pub fn step_block(&mut self, block: &'a Block, function_body: bool, step: usize) -> bool {
        let len = self.frames.len();

        let done = match step {
            0 => {
                for statement in block.statements.iter().rev() {
                    self.frames.push(Frame { element: Element::Statement(statement), step: 0, user_initiated: false });
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

    fn step_statement(&mut self, statement: &'a Statement, step: usize) -> Result<bool> {
        let len = self.frames.len();

        let mut push = |expression| {
            self.frames.push(Frame { element: Element::Expression(expression), step: 0, user_initiated: false })
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
                let Expression::Identifier(id) = &assign.place else { tc_fail!() };
                self.set_variable(id.name, value);
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
                        element: Element::Block { block: &conditional.then, function_body: false },
                        user_initiated: false,
                    }),
                    Value::Bool(false) => {
                        if let Some(otherwise) = conditional.otherwise.as_ref() {
                            self.frames.push(Frame {
                                step: 0,
                                element: Element::Statement(otherwise),
                                user_initiated: false,
                            })
                        }
                    }
                    _ => tc_fail!(),
                };
                false
            }
            Statement::Conditional(_) if step == 2 => true,
            Statement::Console(_) => todo!(),
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
                    Expression::Identifier(id) => self.set_variable(id.name, value),
                    Expression::Tuple(tuple) => {
                        let Value::Tuple(rhs) = value else {
                            tc_fail!();
                        };
                        for (name, val) in tuple.elements.iter().zip(rhs.into_iter()) {
                            let Expression::Identifier(id) = name else {
                                tc_fail!();
                            };
                            self.set_variable(id.name, val);
                        }
                    }
                    _ => tc_fail!(),
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
                        element: Element::Block { block: &iteration.block, function_body: false },
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

    fn step_expression(&mut self, expression: &'a Expression, step: usize) -> Result<bool> {
        let len = self.frames.len();

        macro_rules! push {
            () => {
                |expression| {
                    self.frames.push(Frame { element: Element::Expression(expression), step: 0, user_initiated: false })
                }
            };
        }

        if let Some(value) = match expression {
            Expression::Access(AccessExpression::Array(array)) if step == 0 => {
                push!()(&*array.index);
                push!()(&*array.array);
                None
            }
            Expression::Access(AccessExpression::Array(array)) if step == 1 => {
                let span = array.span();
                let array = self.pop_value()?;
                let index = self.pop_value()?;

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
            Expression::Access(AccessExpression::AssociatedConstant(constant)) if step == 0 => {
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
            Expression::Access(AccessExpression::Member(access)) => match &*access.inner {
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
                    push!()(&*access.inner);
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
            Expression::Access(AccessExpression::AssociatedFunction(function)) if step == 0 => {
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
            Expression::Access(AccessExpression::AssociatedFunction(function)) if step == 1 => {
                let Some(core_function) = CoreFunction::from_symbols(function.variant.name, function.name.name) else {
                    halt!(function.span(), "Unkown core function {function}");
                };

                let span = function.span();

                let value = self.evaluate_core_function(core_function.clone(), &function.arguments, span)?;

                if let CoreFunction::FutureAwait = core_function {
                    // For an await, we have one extra step - first we must evaluate the delayed call.
                    None
                } else {
                    Some(value)
                }
            }
            Expression::Access(AccessExpression::AssociatedFunction(function)) if step == 2 => {
                let Some(core_function) = CoreFunction::from_symbols(function.variant.name, function.name.name) else {
                    halt!(function.span(), "Unkown core function {function}");
                };
                assert!(core_function == CoreFunction::FutureAwait);
                Some(Value::Unit)
            }
            Expression::Access(AccessExpression::Tuple(tuple_access)) if step == 0 => {
                push!()(&*tuple_access.tuple);
                None
            }
            Expression::Access(AccessExpression::Tuple(tuple_access)) if step == 1 => {
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
            Expression::Binary(binary) if step == 0 => {
                push!()(&binary.right);
                push!()(&binary.left);
                None
            }
            Expression::Binary(binary) if step == 1 => {
                let rhs = self.pop_value()?;
                let lhs = self.pop_value()?;
                Some(evaluate_binary(binary.span, binary.op, lhs, rhs)?)
            }
            Expression::Call(call) if step == 0 => {
                call.arguments.iter().rev().for_each(push!());
                None
            }
            Expression::Call(call) if step == 1 => {
                let len = self.values.len();
                let (program, name) = match &*call.function {
                    Expression::Identifier(id) => {
                        let maybe_program = call.program.or_else(|| self.current_program());
                        if let Some(program) = maybe_program {
                            (program, id.name)
                        } else {
                            halt!(call.span, "No current program");
                        }
                    }
                    Expression::Locator(locator) => (locator.program.name.name, locator.name),
                    _ => tc_fail!(),
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
                push!()(&*cast.expression);
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
            Expression::Literal(literal) if step == 0 => Some(match literal {
                Literal::Boolean(b, ..) => Value::Bool(*b),
                Literal::Integer(IntegerType::U8, s, ..) => Value::U8(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::U16, s, ..) => Value::U16(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::U32, s, ..) => Value::U32(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::U64, s, ..) => Value::U64(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::U128, s, ..) => Value::U128(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::I8, s, ..) => Value::I8(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::I16, s, ..) => Value::I16(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::I32, s, ..) => Value::I32(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::I64, s, ..) => Value::I64(s.parse().expect_tc(literal.span())?),
                Literal::Integer(IntegerType::I128, s, ..) => Value::I128(s.parse().expect_tc(literal.span())?),
                Literal::Field(s, ..) => Value::Field(format!("{s}field").parse().expect_tc(literal.span())?),
                Literal::Group(group_literal) => match &**group_literal {
                    GroupLiteral::Single(s, ..) => Value::Group(format!("{s}group").parse().expect_tc(literal.span())?),
                    GroupLiteral::Tuple(_group_tuple) => todo!(),
                },
                Literal::Address(s, ..) => {
                    if s.ends_with(".aleo") {
                        let program_id = ProgramID::from_str(s)?;
                        Value::Address(program_id.to_address()?)
                    } else {
                        Value::Address(s.parse().expect_tc(literal.span())?)
                    }
                }
                Literal::Scalar(s, ..) => Value::Scalar(format!("{s}scalar").parse().expect_tc(literal.span())?),
                Literal::String(..) => tc_fail!(),
            }),
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
                push!()(&*ternary.condition);
                None
            }
            Expression::Ternary(ternary) if step == 1 => {
                let condition = self.pop_value()?;
                match condition {
                    Value::Bool(true) => push!()(&*ternary.if_true),
                    Value::Bool(false) => push!()(&*ternary.if_false),
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
                push!()(&*unary.receiver);
                None
            }
            Expression::Unary(unary) if step == 1 => {
                let value = self.pop_value()?;
                Some(evaluate_unary(unary.span, unary.op, value)?)
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

        let Frame { element, step, user_initiated } = self.frames.last().expect("there should be a frame");
        let user_initiated = *user_initiated;
        match element {
            Element::Block { block, function_body } => {
                let finished = self.step_block(block, *function_body, *step);
                Ok(StepResult { finished, value: None })
            }
            Element::Statement(statement) => {
                let finished = self.step_statement(statement, *step)?;
                Ok(StepResult { finished, value: None })
            }
            Element::Expression(expression) => {
                let finished = self.step_expression(expression, *step)?;
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
            Element::DelayedCall(gid) if *step == 0 => {
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
                            element: Element::Block { block: &function.block, function_body: true },
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
                                context: AleoContext::Finalize(finalize_f),
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
                assert_eq!(*step, 1);
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
                        element: Element::Block { block: &function.block, function_body: true },
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
                        context,
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
                    AleoContext::Finalize(finalize_f)
                } else {
                    AleoContext::Function(function)
                };
                self.frames.push(Frame {
                    step: 0,
                    element: Element::AleoExecution {
                        context,
                        registers: arguments.enumerate().map(|(i, v)| (i as u64, v)).collect(),
                        instruction_index: 0,
                    },
                    user_initiated: false,
                });
            }
        }

        Ok(())
    }

    pub fn evaluate_core_function(
        &mut self,
        core_function: CoreFunction,
        arguments: &[Expression],
        span: Span,
    ) -> Result<Value> {
        macro_rules! apply {
            ($func: expr, $value: ident, $to: ident) => {{
                let v = self.pop_value()?;
                let bits = v.$to();
                Value::$value($func(&bits).expect_tc(span)?)
            }};
        }

        macro_rules! apply_cast {
            ($func: expr, $value: ident, $to: ident) => {{
                let v = self.pop_value()?;
                let bits = v.$to();
                let group = $func(&bits).expect_tc(span)?;
                let x = group.to_x_coordinate();
                Value::$value(x.cast_lossy())
            }};
        }

        macro_rules! apply_cast_int {
            ($func: expr, $value: ident, $int_ty: ident, $to: ident) => {{
                let v = self.pop_value()?;
                let bits = v.$to();
                let group = $func(&bits).expect_tc(span)?;
                let x = group.to_x_coordinate();
                let bits = x.to_bits_le();
                let mut result: $int_ty = 0;
                for bit in 0..std::cmp::min($int_ty::BITS as usize, bits.len()) {
                    let setbit = (if bits[bit] { 1 } else { 0 }) << bit;
                    result |= setbit;
                }
                Value::$value(result)
            }};
        }

        macro_rules! apply_cast2 {
            ($func: expr, $value: ident) => {{
                let Value::Scalar(randomizer) = self.pop_value()? else {
                    tc_fail!();
                };
                let v = self.pop_value()?;
                let bits = v.to_bits_le();
                let group = $func(&bits, &randomizer).expect_tc(span)?;
                let x = group.to_x_coordinate();
                Value::$value(x.cast_lossy())
            }};
        }

        let value = match core_function {
            CoreFunction::BHP256CommitToAddress => {
                apply_cast2!(TestnetV0::commit_to_group_bhp256, Address)
            }
            CoreFunction::BHP256CommitToField => {
                apply_cast2!(TestnetV0::commit_to_group_bhp256, Field)
            }
            CoreFunction::BHP256CommitToGroup => {
                apply_cast2!(TestnetV0::commit_to_group_bhp256, Group)
            }
            CoreFunction::BHP256HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_bhp256, Address, to_bits_le)
            }
            CoreFunction::BHP256HashToField => apply!(TestnetV0::hash_bhp256, Field, to_bits_le),
            CoreFunction::BHP256HashToGroup => apply!(TestnetV0::hash_to_group_bhp256, Group, to_bits_le),
            CoreFunction::BHP256HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, I8, i8, to_bits_le)
            }
            CoreFunction::BHP256HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, I16, i16, to_bits_le)
            }
            CoreFunction::BHP256HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, I32, i32, to_bits_le)
            }
            CoreFunction::BHP256HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, I64, i64, to_bits_le)
            }
            CoreFunction::BHP256HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, I128, i128, to_bits_le)
            }
            CoreFunction::BHP256HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, U8, u8, to_bits_le)
            }
            CoreFunction::BHP256HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, U16, u16, to_bits_le)
            }
            CoreFunction::BHP256HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, U32, u32, to_bits_le)
            }
            CoreFunction::BHP256HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, U64, u64, to_bits_le)
            }
            CoreFunction::BHP256HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp256, U128, u128, to_bits_le)
            }
            CoreFunction::BHP256HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_bhp256, Scalar, to_bits_le)
            }
            CoreFunction::BHP512CommitToAddress => {
                apply_cast2!(TestnetV0::commit_to_group_bhp512, Address)
            }
            CoreFunction::BHP512CommitToField => {
                apply_cast2!(TestnetV0::commit_to_group_bhp512, Field)
            }
            CoreFunction::BHP512CommitToGroup => {
                apply_cast2!(TestnetV0::commit_to_group_bhp512, Group)
            }
            CoreFunction::BHP512HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_bhp512, Address, to_bits_le)
            }
            CoreFunction::BHP512HashToField => apply!(TestnetV0::hash_bhp512, Field, to_bits_le),
            CoreFunction::BHP512HashToGroup => apply!(TestnetV0::hash_to_group_bhp512, Group, to_bits_le),
            CoreFunction::BHP512HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, I8, i8, to_bits_le)
            }
            CoreFunction::BHP512HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, I16, i16, to_bits_le)
            }
            CoreFunction::BHP512HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, I32, i32, to_bits_le)
            }
            CoreFunction::BHP512HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, I64, i64, to_bits_le)
            }
            CoreFunction::BHP512HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, I128, i128, to_bits_le)
            }
            CoreFunction::BHP512HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, U8, u8, to_bits_le)
            }
            CoreFunction::BHP512HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, U16, u16, to_bits_le)
            }
            CoreFunction::BHP512HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, U32, u32, to_bits_le)
            }
            CoreFunction::BHP512HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, U64, u64, to_bits_le)
            }
            CoreFunction::BHP512HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp512, U128, u128, to_bits_le)
            }
            CoreFunction::BHP512HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_bhp512, Scalar, to_bits_le)
            }
            CoreFunction::BHP768CommitToAddress => {
                apply_cast2!(TestnetV0::commit_to_group_bhp768, Address)
            }
            CoreFunction::BHP768CommitToField => {
                apply_cast2!(TestnetV0::commit_to_group_bhp768, Field)
            }
            CoreFunction::BHP768CommitToGroup => {
                apply_cast2!(TestnetV0::commit_to_group_bhp768, Group)
            }
            CoreFunction::BHP768HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_bhp768, Address, to_bits_le)
            }
            CoreFunction::BHP768HashToField => apply!(TestnetV0::hash_bhp768, Field, to_bits_le),
            CoreFunction::BHP768HashToGroup => apply!(TestnetV0::hash_to_group_bhp768, Group, to_bits_le),
            CoreFunction::BHP768HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, I8, i8, to_bits_le)
            }
            CoreFunction::BHP768HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, I16, i16, to_bits_le)
            }
            CoreFunction::BHP768HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, I32, i32, to_bits_le)
            }
            CoreFunction::BHP768HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, I64, i64, to_bits_le)
            }
            CoreFunction::BHP768HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, I128, i128, to_bits_le)
            }
            CoreFunction::BHP768HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, U8, u8, to_bits_le)
            }
            CoreFunction::BHP768HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, U16, u16, to_bits_le)
            }
            CoreFunction::BHP768HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, U32, u32, to_bits_le)
            }
            CoreFunction::BHP768HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, U64, u64, to_bits_le)
            }
            CoreFunction::BHP768HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp768, U128, u128, to_bits_le)
            }
            CoreFunction::BHP768HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_bhp768, Scalar, to_bits_le)
            }
            CoreFunction::BHP1024CommitToAddress => {
                apply_cast2!(TestnetV0::commit_to_group_bhp1024, Address)
            }
            CoreFunction::BHP1024CommitToField => {
                apply_cast2!(TestnetV0::commit_to_group_bhp1024, Field)
            }
            CoreFunction::BHP1024CommitToGroup => {
                apply_cast2!(TestnetV0::commit_to_group_bhp1024, Group)
            }
            CoreFunction::BHP1024HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_bhp1024, Address, to_bits_le)
            }
            CoreFunction::BHP1024HashToField => apply!(TestnetV0::hash_bhp1024, Field, to_bits_le),
            CoreFunction::BHP1024HashToGroup => apply!(TestnetV0::hash_to_group_bhp1024, Group, to_bits_le),
            CoreFunction::BHP1024HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I8, i8, to_bits_le)
            }
            CoreFunction::BHP1024HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I16, i16, to_bits_le)
            }
            CoreFunction::BHP1024HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I32, i32, to_bits_le)
            }
            CoreFunction::BHP1024HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I64, i64, to_bits_le)
            }
            CoreFunction::BHP1024HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, I128, i128, to_bits_le)
            }
            CoreFunction::BHP1024HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U8, u8, to_bits_le)
            }
            CoreFunction::BHP1024HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U16, u16, to_bits_le)
            }
            CoreFunction::BHP1024HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U32, u32, to_bits_le)
            }
            CoreFunction::BHP1024HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U64, u64, to_bits_le)
            }
            CoreFunction::BHP1024HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_bhp1024, U128, u128, to_bits_le)
            }
            CoreFunction::BHP1024HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_bhp1024, Scalar, to_bits_le)
            }
            CoreFunction::ChaChaRandAddress => Value::Address(self.rng.gen()),
            CoreFunction::ChaChaRandBool => Value::Bool(self.rng.gen()),
            CoreFunction::ChaChaRandField => Value::Field(self.rng.gen()),
            CoreFunction::ChaChaRandGroup => Value::Group(self.rng.gen()),
            CoreFunction::ChaChaRandI8 => Value::I8(self.rng.gen()),
            CoreFunction::ChaChaRandI16 => Value::I16(self.rng.gen()),
            CoreFunction::ChaChaRandI32 => Value::I32(self.rng.gen()),
            CoreFunction::ChaChaRandI64 => Value::I64(self.rng.gen()),
            CoreFunction::ChaChaRandI128 => Value::I128(self.rng.gen()),
            CoreFunction::ChaChaRandU8 => Value::U8(self.rng.gen()),
            CoreFunction::ChaChaRandU16 => Value::U16(self.rng.gen()),
            CoreFunction::ChaChaRandU32 => Value::U32(self.rng.gen()),
            CoreFunction::ChaChaRandU64 => Value::U64(self.rng.gen()),
            CoreFunction::ChaChaRandU128 => Value::U128(self.rng.gen()),
            CoreFunction::ChaChaRandScalar => Value::Scalar(self.rng.gen()),
            CoreFunction::Keccak256HashToAddress => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                Address,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToField => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                Field,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToGroup => {
                apply!(
                    |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                    Group,
                    to_bits_le
                )
            }
            CoreFunction::Keccak256HashToI8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                I8,
                i8,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToI16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                I16,
                i16,
                to_bits_le
            ),

            CoreFunction::Keccak256HashToI32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                I32,
                i32,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToI64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                I64,
                i64,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToI128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                I128,
                i128,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToU8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                U8,
                u8,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToU16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                U16,
                u16,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToU32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                U32,
                u32,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToU64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                U64,
                u64,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToU128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                U128,
                u128,
                to_bits_le
            ),
            CoreFunction::Keccak256HashToScalar => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_keccak256(v).expect_tc(span)?),
                Scalar,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToAddress => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                Address,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToField => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                Field,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToGroup => {
                apply!(
                    |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                    Group,
                    to_bits_le
                )
            }
            CoreFunction::Keccak384HashToI8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                I8,
                i8,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToI16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                I16,
                i16,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToI32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                I32,
                i32,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToI64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                I64,
                i64,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToI128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                I128,
                i128,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToU8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                U8,
                u8,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToU16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                U16,
                u16,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToU32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                U32,
                u32,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToU64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                U64,
                u64,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToU128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                U128,
                u128,
                to_bits_le
            ),
            CoreFunction::Keccak384HashToScalar => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak384(v).expect_tc(span)?),
                Scalar,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToAddress => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                Address,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToField => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                Field,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToGroup => {
                apply!(
                    |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                    Group,
                    to_bits_le
                )
            }
            CoreFunction::Keccak512HashToI8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                I8,
                i8,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToI16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                I16,
                i16,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToI32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                I32,
                i32,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToI64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                I64,
                i64,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToI128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                I128,
                i128,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToU8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                U8,
                u8,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToU16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                U16,
                u16,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToU32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                U32,
                u32,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToU64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                U64,
                u64,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToU128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                U128,
                u128,
                to_bits_le
            ),
            CoreFunction::Keccak512HashToScalar => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_keccak512(v).expect_tc(span)?),
                Scalar,
                to_bits_le
            ),
            CoreFunction::Pedersen64CommitToAddress => {
                apply_cast2!(TestnetV0::commit_to_group_ped64, Address)
            }
            CoreFunction::Pedersen64CommitToField => {
                apply_cast2!(TestnetV0::commit_to_group_ped64, Field)
            }
            CoreFunction::Pedersen64CommitToGroup => {
                apply_cast2!(TestnetV0::commit_to_group_ped64, Group)
            }
            CoreFunction::Pedersen64HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_ped64, Address, to_bits_le)
            }
            CoreFunction::Pedersen64HashToField => apply!(TestnetV0::hash_ped64, Field, to_bits_le),
            CoreFunction::Pedersen64HashToGroup => apply!(TestnetV0::hash_to_group_ped64, Group, to_bits_le),
            CoreFunction::Pedersen64HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I8, i8, to_bits_le)
            }
            CoreFunction::Pedersen64HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I16, i16, to_bits_le)
            }
            CoreFunction::Pedersen64HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I32, i32, to_bits_le)
            }
            CoreFunction::Pedersen64HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I64, i64, to_bits_le)
            }
            CoreFunction::Pedersen64HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I128, i128, to_bits_le)
            }
            CoreFunction::Pedersen64HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U8, u8, to_bits_le)
            }
            CoreFunction::Pedersen64HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U16, u16, to_bits_le)
            }
            CoreFunction::Pedersen64HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U32, u32, to_bits_le)
            }
            CoreFunction::Pedersen64HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U64, u64, to_bits_le)
            }
            CoreFunction::Pedersen64HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U128, u128, to_bits_le)
            }
            CoreFunction::Pedersen64HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_ped64, Scalar, to_bits_le)
            }
            CoreFunction::Pedersen128HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_ped128, Address, to_bits_le)
            }
            CoreFunction::Pedersen128HashToField => {
                apply_cast!(TestnetV0::hash_to_group_ped128, Field, to_bits_le)
            }
            CoreFunction::Pedersen128HashToGroup => {
                apply_cast!(TestnetV0::hash_to_group_ped128, Group, to_bits_le)
            }
            CoreFunction::Pedersen128HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped128, I8, i8, to_bits_le)
            }
            CoreFunction::Pedersen128HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I16, i16, to_bits_le)
            }
            CoreFunction::Pedersen128HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped128, I32, i32, to_bits_le)
            }
            CoreFunction::Pedersen128HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, I64, i64, to_bits_le)
            }
            CoreFunction::Pedersen128HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped128, I128, i128, to_bits_le)
            }
            CoreFunction::Pedersen128HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped128, U8, u8, to_bits_le)
            }
            CoreFunction::Pedersen128HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U16, u16, to_bits_le)
            }
            CoreFunction::Pedersen128HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped128, U32, u32, to_bits_le)
            }
            CoreFunction::Pedersen128HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped64, U64, u64, to_bits_le)
            }
            CoreFunction::Pedersen128HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_ped128, U128, u128, to_bits_le)
            }
            CoreFunction::Pedersen128HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_ped128, Scalar, to_bits_le)
            }
            CoreFunction::Pedersen128CommitToAddress => {
                apply_cast2!(TestnetV0::commit_to_group_ped128, Address)
            }
            CoreFunction::Pedersen128CommitToField => {
                apply_cast2!(TestnetV0::commit_to_group_ped128, Field)
            }
            CoreFunction::Pedersen128CommitToGroup => {
                apply_cast2!(TestnetV0::commit_to_group_ped128, Group)
            }
            CoreFunction::Poseidon2HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_psd2, Address, to_fields)
            }
            CoreFunction::Poseidon2HashToField => {
                apply!(TestnetV0::hash_psd2, Field, to_fields)
            }
            CoreFunction::Poseidon2HashToGroup => {
                apply_cast!(TestnetV0::hash_to_group_psd2, Group, to_fields)
            }
            CoreFunction::Poseidon2HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, I8, i8, to_fields)
            }
            CoreFunction::Poseidon2HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, I16, i16, to_fields)
            }
            CoreFunction::Poseidon2HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, I32, i32, to_fields)
            }
            CoreFunction::Poseidon2HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, I64, i64, to_fields)
            }
            CoreFunction::Poseidon2HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, I128, i128, to_fields)
            }
            CoreFunction::Poseidon2HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, U8, u8, to_fields)
            }
            CoreFunction::Poseidon2HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, U16, u16, to_fields)
            }
            CoreFunction::Poseidon2HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, U32, u32, to_fields)
            }
            CoreFunction::Poseidon2HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, U64, u64, to_fields)
            }
            CoreFunction::Poseidon2HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd2, U128, u128, to_fields)
            }
            CoreFunction::Poseidon2HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_psd4, Scalar, to_fields)
            }
            CoreFunction::Poseidon4HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_psd4, Address, to_fields)
            }
            CoreFunction::Poseidon4HashToField => {
                apply!(TestnetV0::hash_psd4, Field, to_fields)
            }
            CoreFunction::Poseidon4HashToGroup => {
                apply_cast!(TestnetV0::hash_to_group_psd4, Group, to_fields)
            }
            CoreFunction::Poseidon4HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, I8, i8, to_fields)
            }
            CoreFunction::Poseidon4HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, I16, i16, to_fields)
            }
            CoreFunction::Poseidon4HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, I32, i32, to_fields)
            }
            CoreFunction::Poseidon4HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, I64, i64, to_fields)
            }
            CoreFunction::Poseidon4HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, I128, i128, to_fields)
            }
            CoreFunction::Poseidon4HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, U8, u8, to_fields)
            }
            CoreFunction::Poseidon4HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, U16, u16, to_fields)
            }
            CoreFunction::Poseidon4HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, U32, u32, to_fields)
            }
            CoreFunction::Poseidon4HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, U64, u64, to_fields)
            }
            CoreFunction::Poseidon4HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd4, U128, u128, to_fields)
            }
            CoreFunction::Poseidon4HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_psd4, Scalar, to_fields)
            }
            CoreFunction::Poseidon8HashToAddress => {
                apply_cast!(TestnetV0::hash_to_group_psd8, Address, to_fields)
            }
            CoreFunction::Poseidon8HashToField => {
                apply!(TestnetV0::hash_psd8, Field, to_fields)
            }
            CoreFunction::Poseidon8HashToGroup => {
                apply_cast!(TestnetV0::hash_to_group_psd8, Group, to_fields)
            }
            CoreFunction::Poseidon8HashToI8 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, I8, i8, to_fields)
            }
            CoreFunction::Poseidon8HashToI16 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, I16, i16, to_fields)
            }
            CoreFunction::Poseidon8HashToI32 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, I32, i32, to_fields)
            }
            CoreFunction::Poseidon8HashToI64 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, I64, i64, to_fields)
            }
            CoreFunction::Poseidon8HashToI128 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, I128, i128, to_fields)
            }
            CoreFunction::Poseidon8HashToU8 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, U8, u8, to_fields)
            }
            CoreFunction::Poseidon8HashToU16 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, U16, u16, to_fields)
            }
            CoreFunction::Poseidon8HashToU32 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, U32, u32, to_fields)
            }
            CoreFunction::Poseidon8HashToU64 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, U64, u64, to_fields)
            }
            CoreFunction::Poseidon8HashToU128 => {
                apply_cast_int!(TestnetV0::hash_to_group_psd8, U128, u128, to_fields)
            }
            CoreFunction::Poseidon8HashToScalar => {
                apply_cast!(TestnetV0::hash_to_group_psd8, Scalar, to_fields)
            }
            CoreFunction::SHA3_256HashToAddress => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                Address,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToField => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                Field,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToGroup => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                Group,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToI8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                I8,
                i8,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToI16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                I16,
                i16,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToI32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                I32,
                i32,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToI64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                I64,
                i64,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToI128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                I128,
                i128,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToU8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                U8,
                u8,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToU16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                U16,
                u16,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToU32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                U32,
                u32,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToU64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                U64,
                u64,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToU128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                U128,
                u128,
                to_bits_le
            ),
            CoreFunction::SHA3_256HashToScalar => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp256(&TestnetV0::hash_sha3_256(v).expect_tc(span)?),
                Scalar,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToAddress => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                Address,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToField => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                Field,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToGroup => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                Group,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToI8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                I8,
                i8,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToI16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                I16,
                i16,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToI32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                I32,
                i32,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToI64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                I64,
                i64,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToI128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                I128,
                i128,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToU8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                U8,
                u8,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToU16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                U16,
                u16,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToU32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                U32,
                u32,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToU64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                U64,
                u64,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToU128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                U128,
                u128,
                to_bits_le
            ),
            CoreFunction::SHA3_384HashToScalar => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_384(v).expect_tc(span)?),
                Scalar,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToAddress => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                Address,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToField => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                Field,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToGroup => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                Group,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToI8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                I8,
                i8,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToI16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                I16,
                i16,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToI32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                I32,
                i32,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToI64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                I64,
                i64,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToI128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                I128,
                i128,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToU8 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                U8,
                u8,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToU16 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                U16,
                u16,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToU32 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                U32,
                u32,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToU64 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                U64,
                u64,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToU128 => apply_cast_int!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                U128,
                u128,
                to_bits_le
            ),
            CoreFunction::SHA3_512HashToScalar => apply_cast!(
                |v| TestnetV0::hash_to_group_bhp512(&TestnetV0::hash_sha3_512(v).expect_tc(span)?),
                Scalar,
                to_bits_le
            ),
            CoreFunction::MappingGet => {
                let key = self.values.pop().expect_tc(span)?;
                let (program, name) = match &arguments[0] {
                    Expression::Identifier(id) => (None, id.name),
                    Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                    _ => tc_fail!(),
                };
                match self.lookup_mapping(program, name).and_then(|mapping| mapping.get(&key)) {
                    Some(v) => v.clone(),
                    None => halt!(span, "map lookup failure"),
                }
            }
            CoreFunction::MappingGetOrUse => {
                let use_value = self.values.pop().expect_tc(span)?;
                let key = self.values.pop().expect_tc(span)?;
                let (program, name) = match &arguments[0] {
                    Expression::Identifier(id) => (None, id.name),
                    Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                    _ => tc_fail!(),
                };
                match self.lookup_mapping(program, name).and_then(|mapping| mapping.get(&key)) {
                    Some(v) => v.clone(),
                    None => use_value,
                }
            }
            CoreFunction::MappingSet => {
                let value = self.pop_value()?;
                let key = self.pop_value()?;
                let (program, name) = match &arguments[0] {
                    Expression::Identifier(id) => (None, id.name),
                    Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                    _ => tc_fail!(),
                };
                if let Some(mapping) = self.lookup_mapping_mut(program, name) {
                    mapping.insert(key, value);
                } else {
                    tc_fail!();
                }
                Value::Unit
            }
            CoreFunction::MappingRemove => {
                let key = self.pop_value()?;
                let (program, name) = match &arguments[0] {
                    Expression::Identifier(id) => (None, id.name),
                    Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                    _ => tc_fail!(),
                };
                if let Some(mapping) = self.lookup_mapping_mut(program, name) {
                    mapping.remove(&key);
                } else {
                    tc_fail!();
                }
                Value::Unit
            }
            CoreFunction::MappingContains => {
                let key = self.pop_value()?;
                let (program, name) = match &arguments[0] {
                    Expression::Identifier(id) => (None, id.name),
                    Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                    _ => tc_fail!(),
                };
                if let Some(mapping) = self.lookup_mapping_mut(program, name) {
                    Value::Bool(mapping.contains_key(&key))
                } else {
                    tc_fail!();
                }
            }
            CoreFunction::GroupToXCoordinate => {
                let Value::Group(g) = self.pop_value()? else {
                    tc_fail!();
                };
                Value::Field(g.to_x_coordinate())
            }
            CoreFunction::GroupToYCoordinate => {
                let Value::Group(g) = self.pop_value()? else {
                    tc_fail!();
                };
                Value::Field(g.to_y_coordinate())
            }
            CoreFunction::SignatureVerify => todo!(),
            CoreFunction::FutureAwait => {
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
                Value::Unit
            }
            CoreFunction::CheatCodePrintMapping => {
                let (program, name) = match &arguments[0] {
                    Expression::Identifier(id) => (None, id.name),
                    Expression::Locator(locator) => (Some(locator.program.name.name), locator.name),
                    _ => tc_fail!(),
                };
                if let Some(mapping) = self.lookup_mapping(program, name) {
                    // TODO: What is the appropriate way to print this to the console.
                    // Print the name of the mapping.
                    println!(
                        "Mapping: {}",
                        if let Some(program) = program { format!("{}/{}", program, name) } else { name.to_string() }
                    );
                    // Print the contents of the mapping.
                    for (key, value) in mapping {
                        println!("  {} -> {}", key, value);
                    }
                } else {
                    tc_fail!();
                }
                Value::Unit
            }
            CoreFunction::CheatCodeSetBlockHeight => {
                let Value::U32(height) = self.pop_value()? else {
                    tc_fail!();
                };
                self.set_block_height(height);
                Value::Unit
            }
        };

        Ok(value)
    }
}

#[derive(Clone, Debug)]
pub struct StepResult {
    /// Has this element completely finished running?
    pub finished: bool,

    /// If the element was an expression, here's its value.
    pub value: Option<Value>,
}

/// Evaluate a binary operation.
pub fn evaluate_binary(span: Span, op: BinaryOperation, lhs: Value, rhs: Value) -> Result<Value> {
    let value = match op {
        BinaryOperation::Add => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_add(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_add(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_add(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_add(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_add(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_add(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_add(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_add(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_add(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_add(y).map(Value::I128),
                (Value::Group(x), Value::Group(y)) => Some(Value::Group(x + y)),
                (Value::Field(x), Value::Field(y)) => Some(Value::Field(x + y)),
                (Value::Scalar(x), Value::Scalar(y)) => Some(Value::Scalar(x + y)),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "add overflow");
            };
            value
        }
        BinaryOperation::AddWrapped => match (lhs, rhs) {
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_add(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_add(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_add(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_add(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_add(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_add(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_add(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_add(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_add(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_add(y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::And => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x && y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::BitwiseAnd => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x & y),
            (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
            (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
            (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
            (Value::U64(x), Value::U64(y)) => Value::U64(x & y),
            (Value::U128(x), Value::U128(y)) => Value::U128(x & y),
            (Value::I8(x), Value::I8(y)) => Value::I8(x & y),
            (Value::I16(x), Value::I16(y)) => Value::I16(x & y),
            (Value::I32(x), Value::I32(y)) => Value::I32(x & y),
            (Value::I64(x), Value::I64(y)) => Value::I64(x & y),
            (Value::I128(x), Value::I128(y)) => Value::I128(x & y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Div => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_div(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_div(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_div(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_div(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_div(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_div(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_div(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_div(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_div(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_div(y).map(Value::I128),
                (Value::Field(x), Value::Field(y)) => y.inverse().map(|y| Value::Field(x * y)).ok(),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "div overflow");
            };
            value
        }
        BinaryOperation::DivWrapped => match (lhs, rhs) {
            (Value::U8(_), Value::U8(0))
            | (Value::U16(_), Value::U16(0))
            | (Value::U32(_), Value::U32(0))
            | (Value::U64(_), Value::U64(0))
            | (Value::U128(_), Value::U128(0))
            | (Value::I8(_), Value::I8(0))
            | (Value::I16(_), Value::I16(0))
            | (Value::I32(_), Value::I32(0))
            | (Value::I64(_), Value::I64(0))
            | (Value::I128(_), Value::I128(0)) => halt!(span, "divide by 0"),
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_div(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_div(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_div(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_div(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_div(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_div(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_div(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_div(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_div(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_div(y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Eq => Value::Bool(lhs.eq(&rhs)?),
        BinaryOperation::Gte => Value::Bool(lhs.gte(&rhs)?),
        BinaryOperation::Gt => Value::Bool(lhs.gt(&rhs)?),
        BinaryOperation::Lte => Value::Bool(lhs.lte(&rhs)?),
        BinaryOperation::Lt => Value::Bool(lhs.lt(&rhs)?),
        BinaryOperation::Mod => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_rem(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_rem(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_rem(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_rem(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_rem(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_rem(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_rem(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_rem(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_rem(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_rem(y).map(Value::I128),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "mod overflow");
            };
            value
        }
        BinaryOperation::Mul => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_mul(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_mul(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_mul(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_mul(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_mul(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_mul(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_mul(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_mul(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_mul(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_mul(y).map(Value::I128),
                (Value::Field(x), Value::Field(y)) => Some(Value::Field(x * y)),
                (Value::Group(x), Value::Scalar(y)) => Some(Value::Group(x * y)),
                (Value::Scalar(x), Value::Group(y)) => Some(Value::Group(x * y)),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "mul overflow");
            };
            value
        }
        BinaryOperation::MulWrapped => match (lhs, rhs) {
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_mul(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_mul(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_mul(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_mul(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_mul(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_mul(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_mul(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_mul(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_mul(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_mul(y)),
            (Value::Field(_), Value::Field(_)) => todo!(),
            _ => halt!(span, "Type error"),
        },

        BinaryOperation::Nand => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(!(x & y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Neq => Value::Bool(lhs.neq(&rhs)?),
        BinaryOperation::Nor => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(!(x | y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Or => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x | y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::BitwiseOr => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x | y),
            (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
            (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
            (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
            (Value::U64(x), Value::U64(y)) => Value::U64(x | y),
            (Value::U128(x), Value::U128(y)) => Value::U128(x | y),
            (Value::I8(x), Value::I8(y)) => Value::I8(x | y),
            (Value::I16(x), Value::I16(y)) => Value::I16(x | y),
            (Value::I32(x), Value::I32(y)) => Value::I32(x | y),
            (Value::I64(x), Value::I64(y)) => Value::I64(x | y),
            (Value::I128(x), Value::I128(y)) => Value::I128(x | y),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Pow => {
            if let (Value::Field(x), Value::Field(y)) = (&lhs, &rhs) {
                Value::Field(x.pow(y))
            } else {
                let rhs: u32 = match rhs {
                    Value::U8(y) => y.into(),
                    Value::U16(y) => y.into(),
                    Value::U32(y) => y,
                    _ => tc_fail!(),
                };

                let Some(value) = (match lhs {
                    Value::U8(x) => x.checked_pow(rhs).map(Value::U8),
                    Value::U16(x) => x.checked_pow(rhs).map(Value::U16),
                    Value::U32(x) => x.checked_pow(rhs).map(Value::U32),
                    Value::U64(x) => x.checked_pow(rhs).map(Value::U64),
                    Value::U128(x) => x.checked_pow(rhs).map(Value::U128),
                    Value::I8(x) => x.checked_pow(rhs).map(Value::I8),
                    Value::I16(x) => x.checked_pow(rhs).map(Value::I16),
                    Value::I32(x) => x.checked_pow(rhs).map(Value::I32),
                    Value::I64(x) => x.checked_pow(rhs).map(Value::I64),
                    Value::I128(x) => x.checked_pow(rhs).map(Value::I128),
                    _ => halt!(span, "Type error"),
                }) else {
                    halt!(span, "pow overflow");
                };
                value
            }
        }
        BinaryOperation::PowWrapped => {
            let rhs: u32 = match rhs {
                Value::U8(y) => y.into(),
                Value::U16(y) => y.into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };

            match lhs {
                Value::U8(x) => Value::U8(x.wrapping_pow(rhs)),
                Value::U16(x) => Value::U16(x.wrapping_pow(rhs)),
                Value::U32(x) => Value::U32(x.wrapping_pow(rhs)),
                Value::U64(x) => Value::U64(x.wrapping_pow(rhs)),
                Value::U128(x) => Value::U128(x.wrapping_pow(rhs)),
                Value::I8(x) => Value::I8(x.wrapping_pow(rhs)),
                Value::I16(x) => Value::I16(x.wrapping_pow(rhs)),
                Value::I32(x) => Value::I32(x.wrapping_pow(rhs)),
                Value::I64(x) => Value::I64(x.wrapping_pow(rhs)),
                Value::I128(x) => Value::I128(x.wrapping_pow(rhs)),
                _ => halt!(span, "Type error"),
            }
        }
        BinaryOperation::Rem => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_rem(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_rem(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_rem(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_rem(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_rem(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_rem(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_rem(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_rem(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_rem(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_rem(y).map(Value::I128),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "rem error");
            };
            value
        }
        BinaryOperation::RemWrapped => match (lhs, rhs) {
            (Value::U8(_), Value::U8(0))
            | (Value::U16(_), Value::U16(0))
            | (Value::U32(_), Value::U32(0))
            | (Value::U64(_), Value::U64(0))
            | (Value::U128(_), Value::U128(0))
            | (Value::I8(_), Value::I8(0))
            | (Value::I16(_), Value::I16(0))
            | (Value::I32(_), Value::I32(0))
            | (Value::I64(_), Value::I64(0))
            | (Value::I128(_), Value::I128(0)) => halt!(span, "rem by 0"),
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_rem(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_rem(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_rem(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_rem(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_rem(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_rem(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_rem(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_rem(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_rem(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_rem(y)),
            _ => halt!(span, "Type error"),
        },
        BinaryOperation::Shl => {
            let rhs: u32 = match rhs {
                Value::U8(y) => y.into(),
                Value::U16(y) => y.into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };
            match lhs {
                Value::U8(_) | Value::I8(_) if rhs >= 8 => halt!(span, "shl overflow"),
                Value::U16(_) | Value::I16(_) if rhs >= 16 => halt!(span, "shl overflow"),
                Value::U32(_) | Value::I32(_) if rhs >= 32 => halt!(span, "shl overflow"),
                Value::U64(_) | Value::I64(_) if rhs >= 64 => halt!(span, "shl overflow"),
                Value::U128(_) | Value::I128(_) if rhs >= 128 => halt!(span, "shl overflow"),
                _ => {}
            }

            // Aleo's shl halts if set bits are shifted out.
            let shifted = lhs.simple_shl(rhs);
            let reshifted = shifted.simple_shr(rhs);
            if lhs.eq(&reshifted)? {
                shifted
            } else {
                halt!(span, "shl overflow");
            }
        }

        BinaryOperation::ShlWrapped => {
            let rhs: u32 = match rhs {
                Value::U8(y) => y.into(),
                Value::U16(y) => y.into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };
            match lhs {
                Value::U8(x) => Value::U8(x.wrapping_shl(rhs)),
                Value::U16(x) => Value::U16(x.wrapping_shl(rhs)),
                Value::U32(x) => Value::U32(x.wrapping_shl(rhs)),
                Value::U64(x) => Value::U64(x.wrapping_shl(rhs)),
                Value::U128(x) => Value::U128(x.wrapping_shl(rhs)),
                Value::I8(x) => Value::I8(x.wrapping_shl(rhs)),
                Value::I16(x) => Value::I16(x.wrapping_shl(rhs)),
                Value::I32(x) => Value::I32(x.wrapping_shl(rhs)),
                Value::I64(x) => Value::I64(x.wrapping_shl(rhs)),
                Value::I128(x) => Value::I128(x.wrapping_shl(rhs)),
                _ => halt!(span, "Type error"),
            }
        }

        BinaryOperation::Shr => {
            let rhs: u32 = match rhs {
                Value::U8(y) => y.into(),
                Value::U16(y) => y.into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };

            match lhs {
                Value::U8(_) | Value::I8(_) if rhs >= 8 => halt!(span, "shr overflow"),
                Value::U16(_) | Value::I16(_) if rhs >= 16 => halt!(span, "shr overflow"),
                Value::U32(_) | Value::I32(_) if rhs >= 32 => halt!(span, "shr overflow"),
                Value::U64(_) | Value::I64(_) if rhs >= 64 => halt!(span, "shr overflow"),
                Value::U128(_) | Value::I128(_) if rhs >= 128 => halt!(span, "shr overflow"),
                _ => {}
            }

            lhs.simple_shr(rhs)
        }

        BinaryOperation::ShrWrapped => {
            let rhs: u32 = match rhs {
                Value::U8(y) => y.into(),
                Value::U16(y) => y.into(),
                Value::U32(y) => y,
                _ => halt!(span, "Type error"),
            };

            match lhs {
                Value::U8(x) => Value::U8(x.wrapping_shr(rhs)),
                Value::U16(x) => Value::U16(x.wrapping_shr(rhs)),
                Value::U32(x) => Value::U32(x.wrapping_shr(rhs)),
                Value::U64(x) => Value::U64(x.wrapping_shr(rhs)),
                Value::U128(x) => Value::U128(x.wrapping_shr(rhs)),
                Value::I8(x) => Value::I8(x.wrapping_shr(rhs)),
                Value::I16(x) => Value::I16(x.wrapping_shr(rhs)),
                Value::I32(x) => Value::I32(x.wrapping_shr(rhs)),
                Value::I64(x) => Value::I64(x.wrapping_shr(rhs)),
                Value::I128(x) => Value::I128(x.wrapping_shr(rhs)),
                _ => halt!(span, "Type error"),
            }
        }

        BinaryOperation::Sub => {
            let Some(value) = (match (lhs, rhs) {
                (Value::U8(x), Value::U8(y)) => x.checked_sub(y).map(Value::U8),
                (Value::U16(x), Value::U16(y)) => x.checked_sub(y).map(Value::U16),
                (Value::U32(x), Value::U32(y)) => x.checked_sub(y).map(Value::U32),
                (Value::U64(x), Value::U64(y)) => x.checked_sub(y).map(Value::U64),
                (Value::U128(x), Value::U128(y)) => x.checked_sub(y).map(Value::U128),
                (Value::I8(x), Value::I8(y)) => x.checked_sub(y).map(Value::I8),
                (Value::I16(x), Value::I16(y)) => x.checked_sub(y).map(Value::I16),
                (Value::I32(x), Value::I32(y)) => x.checked_sub(y).map(Value::I32),
                (Value::I64(x), Value::I64(y)) => x.checked_sub(y).map(Value::I64),
                (Value::I128(x), Value::I128(y)) => x.checked_sub(y).map(Value::I128),
                (Value::Group(x), Value::Group(y)) => Some(Value::Group(x - y)),
                (Value::Field(x), Value::Field(y)) => Some(Value::Field(x - y)),
                _ => halt!(span, "Type error"),
            }) else {
                halt!(span, "sub overflow");
            };
            value
        }

        BinaryOperation::SubWrapped => match (lhs, rhs) {
            (Value::U8(x), Value::U8(y)) => Value::U8(x.wrapping_sub(y)),
            (Value::U16(x), Value::U16(y)) => Value::U16(x.wrapping_sub(y)),
            (Value::U32(x), Value::U32(y)) => Value::U32(x.wrapping_sub(y)),
            (Value::U64(x), Value::U64(y)) => Value::U64(x.wrapping_sub(y)),
            (Value::U128(x), Value::U128(y)) => Value::U128(x.wrapping_sub(y)),
            (Value::I8(x), Value::I8(y)) => Value::I8(x.wrapping_sub(y)),
            (Value::I16(x), Value::I16(y)) => Value::I16(x.wrapping_sub(y)),
            (Value::I32(x), Value::I32(y)) => Value::I32(x.wrapping_sub(y)),
            (Value::I64(x), Value::I64(y)) => Value::I64(x.wrapping_sub(y)),
            (Value::I128(x), Value::I128(y)) => Value::I128(x.wrapping_sub(y)),
            _ => halt!(span, "Type error"),
        },

        BinaryOperation::Xor => match (lhs, rhs) {
            (Value::Bool(x), Value::Bool(y)) => Value::Bool(x ^ y),
            (Value::U8(x), Value::U8(y)) => Value::U8(x ^ y),
            (Value::U16(x), Value::U16(y)) => Value::U16(x ^ y),
            (Value::U32(x), Value::U32(y)) => Value::U32(x ^ y),
            (Value::U64(x), Value::U64(y)) => Value::U64(x ^ y),
            (Value::U128(x), Value::U128(y)) => Value::U128(x ^ y),
            (Value::I8(x), Value::I8(y)) => Value::I8(x ^ y),
            (Value::I16(x), Value::I16(y)) => Value::I16(x ^ y),
            (Value::I32(x), Value::I32(y)) => Value::I32(x ^ y),
            (Value::I64(x), Value::I64(y)) => Value::I64(x ^ y),
            (Value::I128(x), Value::I128(y)) => Value::I128(x ^ y),
            _ => halt!(span, "Type error"),
        },
    };
    Ok(value)
}

/// Evaluate a unary operation.
pub fn evaluate_unary(span: Span, op: UnaryOperation, value: Value) -> Result<Value> {
    let value_result = match op {
        UnaryOperation::Abs => match value {
            Value::I8(x) => {
                if x == i8::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I8(x.abs())
                }
            }
            Value::I16(x) => {
                if x == i16::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I16(x.abs())
                }
            }
            Value::I32(x) => {
                if x == i32::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I32(x.abs())
                }
            }
            Value::I64(x) => {
                if x == i64::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I64(x.abs())
                }
            }
            Value::I128(x) => {
                if x == i128::MIN {
                    halt!(span, "abs overflow");
                } else {
                    Value::I128(x.abs())
                }
            }
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::AbsWrapped => match value {
            Value::I8(x) => Value::I8(x.unsigned_abs() as i8),
            Value::I16(x) => Value::I16(x.unsigned_abs() as i16),
            Value::I32(x) => Value::I32(x.unsigned_abs() as i32),
            Value::I64(x) => Value::I64(x.unsigned_abs() as i64),
            Value::I128(x) => Value::I128(x.unsigned_abs() as i128),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Double => match value {
            Value::Field(x) => Value::Field(x.double()),
            Value::Group(x) => Value::Group(x.double()),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Inverse => match value {
            Value::Field(x) => {
                let Ok(y) = x.inverse() else {
                    halt!(span, "attempt to invert 0field");
                };
                Value::Field(y)
            }
            _ => halt!(span, "Can only invert fields"),
        },
        UnaryOperation::Negate => match value {
            Value::I8(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I8(y),
            },
            Value::I16(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I16(y),
            },
            Value::I32(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I32(y),
            },
            Value::I64(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I64(y),
            },
            Value::I128(x) => match x.checked_neg() {
                None => halt!(span, "negation overflow"),
                Some(y) => Value::I128(y),
            },
            Value::Group(x) => Value::Group(-x),
            Value::Field(x) => Value::Field(-x),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Not => match value {
            Value::Bool(x) => Value::Bool(!x),
            Value::U8(x) => Value::U8(!x),
            Value::U16(x) => Value::U16(!x),
            Value::U32(x) => Value::U32(!x),
            Value::U64(x) => Value::U64(!x),
            Value::U128(x) => Value::U128(!x),
            Value::I8(x) => Value::I8(!x),
            Value::I16(x) => Value::I16(!x),
            Value::I32(x) => Value::I32(!x),
            Value::I64(x) => Value::I64(!x),
            Value::I128(x) => Value::I128(!x),
            _ => halt!(span, "Type error"),
        },
        UnaryOperation::Square => match value {
            Value::Field(x) => Value::Field(x.square()),
            _ => halt!(span, "Can only square fields"),
        },
        UnaryOperation::SquareRoot => match value {
            Value::Field(x) => {
                let Ok(y) = x.square_root() else {
                    halt!(span, "square root failure");
                };
                Value::Field(y)
            }
            _ => halt!(span, "Can only apply square_root to fields"),
        },
        UnaryOperation::ToXCoordinate => match value {
            Value::Group(x) => Value::Field(x.to_x_coordinate()),
            _ => tc_fail!(),
        },
        UnaryOperation::ToYCoordinate => match value {
            Value::Group(x) => Value::Field(x.to_y_coordinate()),
            _ => tc_fail!(),
        },
    };

    Ok(value_result)
}
