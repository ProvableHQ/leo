// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use leo_ast::{BinaryOperation, CoreFunction, IntegerType, Type, UnaryOperation};

use snarkvm::{
    prelude::{
        Access,
        Boolean,
        Field,
        Identifier,
        Literal,
        LiteralType,
        Network as _,
        PlaintextType,
        Register,
        TestnetV0,
        ToBits as _,
        ToBytes as _,
        integers::Integer,
    },
    synthesizer::{Command, Instruction},
};
use snarkvm_synthesizer_program::{CallOperator, CastType, Operand};

use rand::Rng as _;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use std::mem;

impl Cursor<'_> {
    fn mapping_by_call_operator(&self, operator: &CallOperator<TestnetV0>) -> Option<&HashMap<Value, Value>> {
        let (program, name) = match operator {
            CallOperator::Locator(locator) => {
                (Some(snarkvm_identifier_to_symbol(locator.name())), snarkvm_identifier_to_symbol(locator.resource()))
            }
            CallOperator::Resource(id) => (None, snarkvm_identifier_to_symbol(id)),
        };
        self.lookup_mapping(program, name)
    }

    fn get_register(&self, reg: &Register<TestnetV0>) -> &Value {
        let Some(Frame { element: Element::AleoExecution { registers, .. }, .. }) = self.frames.last() else {
            panic!();
        };
        match reg {
            Register::Locator(index) => {
                registers.get(index).expect("valid .aleo code doesn't access undefined registers")
            }
            Register::Access(index, accesses) => {
                let mut current_value =
                    registers.get(index).expect("valid .aleo code doesn't access undefined registers");
                for access in accesses.iter() {
                    match access {
                        Access::Member(id) => {
                            if let Value::Struct(current_struct) = current_value {
                                let name = snarkvm_identifier_to_symbol(id);
                                current_value = current_struct.contents.get(&name).expect("struct missing member");
                            } else {
                                tc_fail!();
                            }
                        }
                        Access::Index(index) => {
                            if let Value::Array(current_array) = current_value {
                                current_value = current_array.get(**index as usize).expect("array index out of bounds");
                            } else {
                                tc_fail!();
                            }
                        }
                    }
                }

                current_value
            }
        }
    }

    fn set_register(&mut self, reg: Register<TestnetV0>, value: Value) {
        let Some(Frame { element: Element::AleoExecution { registers, .. }, .. }) = self.frames.last_mut() else {
            panic!();
        };

        match reg {
            Register::Locator(index) => {
                registers.insert(index, value);
            }
            Register::Access(_, _) => todo!(),
        }
    }

    fn instructions_len(&self) -> usize {
        let Some(Frame { element: Element::AleoExecution { context, .. }, .. }) = self.frames.last() else {
            panic!();
        };
        match context {
            AleoContext::Closure(closure) => closure.instructions().len(),
            AleoContext::Function(function) => function.instructions().len(),
            AleoContext::Finalize(finalize) => finalize.commands().len(),
        }
    }

    fn increment_instruction_index(&mut self) {
        let Some(Frame { element: Element::AleoExecution { instruction_index, .. }, .. }) = self.frames.last_mut()
        else {
            panic!();
        };
        *instruction_index += 1;
    }

    fn execution_complete(&self) -> bool {
        let Some(Frame { element: Element::AleoExecution { instruction_index, .. }, .. }) = self.frames.last() else {
            panic!();
        };
        *instruction_index >= self.instructions_len()
    }

    fn next_instruction(&self) -> Option<&Instruction<TestnetV0>> {
        let Some(Frame { element: Element::AleoExecution { instruction_index, context, .. }, .. }) = self.frames.last()
        else {
            panic!();
        };
        match context {
            AleoContext::Closure(closure) => closure.instructions().get(*instruction_index),
            AleoContext::Function(function) => function.instructions().get(*instruction_index),
            AleoContext::Finalize(_) => None,
        }
    }

    fn next_command(&self) -> Option<&Command<TestnetV0>> {
        let Some(Frame { element: Element::AleoExecution { instruction_index, context, .. }, .. }) = self.frames.last()
        else {
            panic!();
        };
        match context {
            AleoContext::Closure(_) | AleoContext::Function(_) => None,
            AleoContext::Finalize(finalize) => finalize.commands().get(*instruction_index),
        }
    }

    fn operand_value(&self, operand: &Operand<TestnetV0>) -> Value {
        match operand {
            Operand::Literal(literal) => match literal {
                Literal::Address(x) => Value::Address(*x),
                Literal::Boolean(x) => Value::Bool(**x),
                Literal::Field(x) => Value::Field(*x),
                Literal::Group(x) => Value::Group(*x),
                Literal::I8(x) => Value::I8(**x),
                Literal::I16(x) => Value::I16(**x),
                Literal::I32(x) => Value::I32(**x),
                Literal::I64(x) => Value::I64(**x),
                Literal::I128(x) => Value::I128(**x),
                Literal::U8(x) => Value::U8(**x),
                Literal::U16(x) => Value::U16(**x),
                Literal::U32(x) => Value::U32(**x),
                Literal::U64(x) => Value::U64(**x),
                Literal::U128(x) => Value::U128(**x),
                Literal::Scalar(x) => Value::Scalar(*x),
                Literal::Signature(_) => todo!(),
                Literal::String(_) => todo!(),
            },
            Operand::Register(register) => self.get_register(register).clone(),
            Operand::ProgramID(_) => todo!(),
            Operand::Signer => Value::Address(self.signer),
            Operand::Caller => {
                if let Some(function_context) = self.contexts.last() {
                    Value::Address(function_context.caller)
                } else {
                    Value::Address(self.signer)
                }
            }
            Operand::BlockHeight => Value::U32(self.block_height),
            Operand::NetworkID => todo!(),
        }
    }

    fn step_aleo_instruction(&mut self, instruction: Instruction<TestnetV0>) -> Result<()> {
        // The Aleo VM code is a linear sequence of instructions, so we don't need to keep
        // a stack of Elements (except for calls). Just run instructions in order.
        use Instruction::*;

        let Some(Frame { step, .. }) = self.frames.last() else {
            panic!("frame expected");
        };

        macro_rules! unary {
            ($svm_op: expr, $op: ident) => {{
                let operand = self.operand_value(&$svm_op.operands()[0]);
                let value = evaluate_unary(Default::default(), UnaryOperation::$op, operand)?;
                self.increment_instruction_index();
                (value, $svm_op.destinations()[0].clone())
            }};
        }

        macro_rules! binary {
            ($svm_op: expr, $op: ident) => {{
                let operand0 = self.operand_value(&$svm_op.operands()[0]);
                let operand1 = self.operand_value(&$svm_op.operands()[1]);
                let value = evaluate_binary(Default::default(), BinaryOperation::$op, operand0, operand1)?;
                self.increment_instruction_index();
                (value, $svm_op.destinations()[0].clone())
            }};
        }

        macro_rules! commit_function {
            ($commit: expr,
             $to_address: ident,
             $to_field: ident,
             $to_group: ident,
            ) => {{
                let core_function = match $commit.destination_type() {
                    LiteralType::Address => CoreFunction::$to_address,
                    LiteralType::Field => CoreFunction::$to_field,
                    LiteralType::Group => CoreFunction::$to_group,
                    _ => panic!("invalid commit destination type"),
                };

                let randomizer_value = self.operand_value(&$commit.operands()[0]);
                let operand_value = self.operand_value(&$commit.operands()[1]);
                self.values.push(randomizer_value);
                self.values.push(operand_value);
                let value = self.evaluate_core_function(core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value, $commit.destinations()[0].clone())
            }};
        }

        macro_rules! hash_function {
            ($hash: expr,
             $to_address: ident,
             $to_field: ident,
             $to_group: ident,
             $to_i8: ident,
             $to_i16: ident,
             $to_i32: ident,
             $to_i64: ident,
             $to_i128: ident,
             $to_u8: ident,
             $to_u16: ident,
             $to_u32: ident,
             $to_u64: ident,
             $to_u128: ident,
             $to_scalar: ident,
            ) => {{
                let core_function = match $hash.destination_type() {
                    PlaintextType::Literal(LiteralType::Address) => CoreFunction::$to_address,
                    PlaintextType::Literal(LiteralType::Field) => CoreFunction::$to_field,
                    PlaintextType::Literal(LiteralType::Group) => CoreFunction::$to_group,
                    PlaintextType::Literal(LiteralType::I8) => CoreFunction::$to_i8,
                    PlaintextType::Literal(LiteralType::I16) => CoreFunction::$to_i16,
                    PlaintextType::Literal(LiteralType::I32) => CoreFunction::$to_i32,
                    PlaintextType::Literal(LiteralType::I64) => CoreFunction::$to_i64,
                    PlaintextType::Literal(LiteralType::I128) => CoreFunction::$to_i128,
                    PlaintextType::Literal(LiteralType::U8) => CoreFunction::$to_u8,
                    PlaintextType::Literal(LiteralType::U16) => CoreFunction::$to_u16,
                    PlaintextType::Literal(LiteralType::U32) => CoreFunction::$to_u32,
                    PlaintextType::Literal(LiteralType::U64) => CoreFunction::$to_u64,
                    PlaintextType::Literal(LiteralType::U128) => CoreFunction::$to_u128,
                    PlaintextType::Literal(LiteralType::Scalar) => CoreFunction::$to_scalar,
                    _ => panic!("invalid hash destination type"),
                };
                let operand_value = self.operand_value(&$hash.operands()[0]);
                self.values.push(operand_value);
                let value = self.evaluate_core_function(core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value, $hash.destinations()[0].clone())
            }};
        }

        let (value, destination) = match instruction {
            Abs(abs) => unary!(abs, Abs),
            AbsWrapped(abs_wrapped) => unary!(abs_wrapped, AbsWrapped),
            Add(add) => binary!(add, Add),
            AddWrapped(add_wrapped) => binary!(add_wrapped, AddWrapped),
            And(and) => binary!(and, BitwiseAnd),
            AssertEq(assert_eq) => {
                let operand0 = self.operand_value(&assert_eq.operands()[0]);
                let operand1 = self.operand_value(&assert_eq.operands()[1]);
                if operand0.neq(&operand1)? {
                    halt_no_span!("assertion failure: {operand0} != {operand1}");
                }
                self.increment_instruction_index();
                return Ok(());
            }
            AssertNeq(assert_neq) => {
                let operand0 = self.operand_value(&assert_neq.operands()[0]);
                let operand1 = self.operand_value(&assert_neq.operands()[1]);
                if operand0.eq(&operand1)? {
                    halt_no_span!("assertion failure: {operand0} != {operand1}");
                }
                self.increment_instruction_index();
                return Ok(());
            }
            Async(async_) if *step == 0 => {
                let program = self.contexts.current_program().expect("there should be a program");
                let name = snarkvm_identifier_to_symbol(async_.function_name());
                let arguments: Vec<Value> = async_.operands().iter().map(|op| self.operand_value(op)).collect();
                if self.really_async {
                    self.increment_instruction_index();
                    let async_ex = AsyncExecution { function: GlobalId { name, program }, arguments };
                    (Value::Future(Future(vec![async_ex])), async_.destinations()[0].clone())
                } else {
                    self.do_call(
                        program,
                        name,
                        arguments.into_iter(),
                        true, // finalize
                        Span::default(),
                    )?;
                    self.increment_step();
                    return Ok(());
                }
            }
            Call(call) if *step == 0 => {
                let (program, name) = match call.operator() {
                    CallOperator::Locator(locator) => (
                        snarkvm_identifier_to_symbol(locator.resource()),
                        snarkvm_identifier_to_symbol(locator.program_id().name()),
                    ),
                    CallOperator::Resource(id) => (
                        snarkvm_identifier_to_symbol(id),
                        self.contexts.current_program().expect("there should be a program"),
                    ),
                };
                let arguments: Vec<Value> = call.operands().iter().map(|op| self.operand_value(op)).collect();
                self.do_call(
                    program,
                    name,
                    arguments.into_iter(),
                    false, // finalize
                    Span::default(),
                )?;
                self.increment_step();
                return Ok(());
            }
            Async(async_) if *step == 1 => {
                // We've done a call, and the result is on the value stack.
                self.values.pop();
                self.set_register(async_.destinations()[0].clone(), Value::Future(Future(Vec::new())));
                self.increment_instruction_index();
                return Ok(());
            }
            Call(call) if *step == 1 => {
                // We've done a call, and the result is on the value stack.
                let Some(result) = self.values.pop() else {
                    panic!("should have a result");
                };
                if call.destinations().len() == 1 {
                    self.set_register(call.destinations()[0].clone(), result);
                } else {
                    let Value::Tuple(tuple) = result else {
                        panic!("function returning multiple values should create a tuple");
                    };
                    for (dest, value) in call.destinations().iter().zip(tuple.into_iter()) {
                        self.set_register(dest.clone(), value);
                    }
                }
                self.increment_instruction_index();
                return Ok(());
            }
            Call(_) | Async(_) => unreachable!("all cases covered above"),
            Cast(cast) => {
                let destination = cast.destinations()[0].clone();

                self.increment_instruction_index();

                let make_struct = |program, name_identifier| {
                    let name = snarkvm_identifier_to_symbol(name_identifier);
                    let id = GlobalId { program, name };
                    let struct_type = self.structs.get(&id).expect("struct type should exist");
                    let operands = cast.operands().iter().map(|op| self.operand_value(op));
                    Value::Struct(StructContents {
                        name,
                        contents: struct_type.iter().cloned().zip(operands).collect(),
                    })
                };

                match cast.cast_type() {
                    CastType::GroupXCoordinate => {
                        let Value::Group(g) = self.operand_value(&cast.operands()[0]) else {
                            tc_fail!();
                        };
                        let value = Value::Field(g.to_x_coordinate());
                        (value, destination)
                    }
                    CastType::GroupYCoordinate => {
                        let Value::Group(g) = self.operand_value(&cast.operands()[0]) else {
                            tc_fail!();
                        };
                        let value = Value::Field(g.to_y_coordinate());
                        (value, destination)
                    }
                    CastType::Plaintext(PlaintextType::Array(_array)) => {
                        let value = Value::Array(cast.operands().iter().map(|op| self.operand_value(op)).collect());
                        (value, destination)
                    }
                    CastType::Plaintext(PlaintextType::Literal(literal_type)) => {
                        let operand = self.operand_value(&cast.operands()[0]);
                        let value = match operand.cast(&snarkvm_literal_type_to_type(*literal_type)) {
                            Some(value) => value,
                            None => halt_no_span!("cast failure"),
                        };
                        (value, destination)
                    }
                    CastType::Record(struct_name) | CastType::Plaintext(PlaintextType::Struct(struct_name)) => {
                        let program = self.contexts.current_program().expect("there should be a current program");
                        let value = make_struct(program, struct_name);
                        (value, destination)
                    }
                    CastType::ExternalRecord(locator) => {
                        let program = snarkvm_identifier_to_symbol(locator.program_id().name());
                        let value = make_struct(program, locator.name());
                        (value, destination)
                    }
                }
            }
            CastLossy(cast_lossy) => {
                match cast_lossy.cast_type() {
                    CastType::Plaintext(PlaintextType::Literal(literal_type)) => {
                        // This is the only variant supported for lossy casts.
                        let operand = self.operand_value(&cast_lossy.operands()[0]);
                        let operand_literal = value_to_snarkvm_literal(operand);
                        let result_literal = match operand_literal.cast_lossy(*literal_type) {
                            Ok(result_literal) => result_literal,
                            Err(_) => halt_no_span!("cast failure"),
                        };
                        let destination = cast_lossy.destinations()[0].clone();
                        self.increment_instruction_index();
                        (snarkvm_literal_to_value(result_literal), destination)
                    }
                    _ => tc_fail!(),
                }
            }
            CommitBHP256(commit) => {
                commit_function!(commit, BHP256CommitToAddress, BHP256CommitToField, BHP256CommitToGroup,)
            }
            CommitBHP512(commit) => {
                commit_function!(commit, BHP512CommitToAddress, BHP512CommitToField, BHP512CommitToGroup,)
            }
            CommitBHP768(commit) => {
                commit_function!(commit, BHP768CommitToAddress, BHP768CommitToField, BHP768CommitToGroup,)
            }
            CommitBHP1024(commit) => {
                commit_function!(commit, BHP1024CommitToAddress, BHP1024CommitToField, BHP1024CommitToGroup,)
            }
            CommitPED64(commit) => {
                commit_function!(commit, Pedersen64CommitToAddress, Pedersen64CommitToField, Pedersen64CommitToGroup,)
            }
            CommitPED128(commit) => {
                commit_function!(commit, Pedersen128CommitToAddress, Pedersen128CommitToField, Pedersen128CommitToGroup,)
            }
            Div(div) => binary!(div, Div),
            DivWrapped(div_wrapped) => binary!(div_wrapped, DivWrapped),
            Double(double) => unary!(double, Double),
            GreaterThan(gt) => binary!(gt, Gt),
            GreaterThanOrEqual(gte) => binary!(gte, Gte),
            HashBHP256(hash) => hash_function!(
                hash,
                BHP256HashToAddress,
                BHP256HashToField,
                BHP256HashToGroup,
                BHP256HashToI8,
                BHP256HashToI16,
                BHP256HashToI32,
                BHP256HashToI64,
                BHP256HashToI128,
                BHP256HashToU8,
                BHP256HashToU16,
                BHP256HashToU32,
                BHP256HashToU64,
                BHP256HashToU128,
                BHP256HashToScalar,
            ),
            HashBHP512(hash) => hash_function!(
                hash,
                BHP512HashToAddress,
                BHP512HashToField,
                BHP512HashToGroup,
                BHP512HashToI8,
                BHP512HashToI16,
                BHP512HashToI32,
                BHP512HashToI64,
                BHP512HashToI128,
                BHP512HashToU8,
                BHP512HashToU16,
                BHP512HashToU32,
                BHP512HashToU64,
                BHP512HashToU128,
                BHP512HashToScalar,
            ),
            HashBHP768(hash) => hash_function!(
                hash,
                BHP768HashToAddress,
                BHP768HashToField,
                BHP768HashToGroup,
                BHP768HashToI8,
                BHP768HashToI16,
                BHP768HashToI32,
                BHP768HashToI64,
                BHP768HashToI128,
                BHP768HashToU8,
                BHP768HashToU16,
                BHP768HashToU32,
                BHP768HashToU64,
                BHP768HashToU128,
                BHP768HashToScalar,
            ),
            HashBHP1024(hash) => hash_function!(
                hash,
                BHP1024HashToAddress,
                BHP1024HashToField,
                BHP1024HashToGroup,
                BHP1024HashToI8,
                BHP1024HashToI16,
                BHP1024HashToI32,
                BHP1024HashToI64,
                BHP1024HashToI128,
                BHP1024HashToU8,
                BHP1024HashToU16,
                BHP1024HashToU32,
                BHP1024HashToU64,
                BHP1024HashToU128,
                BHP1024HashToScalar,
            ),
            HashKeccak256(hash) => hash_function!(
                hash,
                Keccak256HashToAddress,
                Keccak256HashToField,
                Keccak256HashToGroup,
                Keccak256HashToI8,
                Keccak256HashToI16,
                Keccak256HashToI32,
                Keccak256HashToI64,
                Keccak256HashToI128,
                Keccak256HashToU8,
                Keccak256HashToU16,
                Keccak256HashToU32,
                Keccak256HashToU64,
                Keccak256HashToU128,
                Keccak256HashToScalar,
            ),
            HashKeccak384(hash) => hash_function!(
                hash,
                Keccak384HashToAddress,
                Keccak384HashToField,
                Keccak384HashToGroup,
                Keccak384HashToI8,
                Keccak384HashToI16,
                Keccak384HashToI32,
                Keccak384HashToI64,
                Keccak384HashToI128,
                Keccak384HashToU8,
                Keccak384HashToU16,
                Keccak384HashToU32,
                Keccak384HashToU64,
                Keccak384HashToU128,
                Keccak384HashToScalar,
            ),
            HashKeccak512(hash) => hash_function!(
                hash,
                Keccak512HashToAddress,
                Keccak512HashToField,
                Keccak512HashToGroup,
                Keccak512HashToI8,
                Keccak512HashToI16,
                Keccak512HashToI32,
                Keccak512HashToI64,
                Keccak512HashToI128,
                Keccak512HashToU8,
                Keccak512HashToU16,
                Keccak512HashToU32,
                Keccak512HashToU64,
                Keccak512HashToU128,
                Keccak512HashToScalar,
            ),
            HashPED64(hash) => hash_function!(
                hash,
                Pedersen64HashToAddress,
                Pedersen64HashToField,
                Pedersen64HashToGroup,
                Pedersen64HashToI8,
                Pedersen64HashToI16,
                Pedersen64HashToI32,
                Pedersen64HashToI64,
                Pedersen64HashToI128,
                Pedersen64HashToU8,
                Pedersen64HashToU16,
                Pedersen64HashToU32,
                Pedersen64HashToU64,
                Pedersen64HashToU128,
                Pedersen64HashToScalar,
            ),
            HashPED128(hash) => hash_function!(
                hash,
                Pedersen128HashToAddress,
                Pedersen128HashToField,
                Pedersen128HashToGroup,
                Pedersen128HashToI8,
                Pedersen128HashToI16,
                Pedersen128HashToI32,
                Pedersen128HashToI64,
                Pedersen128HashToI128,
                Pedersen128HashToU8,
                Pedersen128HashToU16,
                Pedersen128HashToU32,
                Pedersen128HashToU64,
                Pedersen128HashToU128,
                Pedersen128HashToScalar,
            ),
            HashPSD2(hash) => hash_function!(
                hash,
                Poseidon2HashToAddress,
                Poseidon2HashToField,
                Poseidon2HashToGroup,
                Poseidon2HashToI8,
                Poseidon2HashToI16,
                Poseidon2HashToI32,
                Poseidon2HashToI64,
                Poseidon2HashToI128,
                Poseidon2HashToU8,
                Poseidon2HashToU16,
                Poseidon2HashToU32,
                Poseidon2HashToU64,
                Poseidon2HashToU128,
                Poseidon2HashToScalar,
            ),
            HashPSD4(hash) => hash_function!(
                hash,
                Poseidon4HashToAddress,
                Poseidon4HashToField,
                Poseidon4HashToGroup,
                Poseidon4HashToI8,
                Poseidon4HashToI16,
                Poseidon4HashToI32,
                Poseidon4HashToI64,
                Poseidon4HashToI128,
                Poseidon4HashToU8,
                Poseidon4HashToU16,
                Poseidon4HashToU32,
                Poseidon4HashToU64,
                Poseidon4HashToU128,
                Poseidon4HashToScalar,
            ),
            HashPSD8(hash) => hash_function!(
                hash,
                Poseidon8HashToAddress,
                Poseidon8HashToField,
                Poseidon8HashToGroup,
                Poseidon8HashToI8,
                Poseidon8HashToI16,
                Poseidon8HashToI32,
                Poseidon8HashToI64,
                Poseidon8HashToI128,
                Poseidon8HashToU8,
                Poseidon8HashToU16,
                Poseidon8HashToU32,
                Poseidon8HashToU64,
                Poseidon8HashToU128,
                Poseidon8HashToScalar,
            ),
            HashSha3_256(hash) => hash_function!(
                hash,
                SHA3_256HashToAddress,
                SHA3_256HashToField,
                SHA3_256HashToGroup,
                SHA3_256HashToI8,
                SHA3_256HashToI16,
                SHA3_256HashToI32,
                SHA3_256HashToI64,
                SHA3_256HashToI128,
                SHA3_256HashToU8,
                SHA3_256HashToU16,
                SHA3_256HashToU32,
                SHA3_256HashToU64,
                SHA3_256HashToU128,
                SHA3_256HashToScalar,
            ),
            HashSha3_384(hash) => hash_function!(
                hash,
                SHA3_384HashToAddress,
                SHA3_384HashToField,
                SHA3_384HashToGroup,
                SHA3_384HashToI8,
                SHA3_384HashToI16,
                SHA3_384HashToI32,
                SHA3_384HashToI64,
                SHA3_384HashToI128,
                SHA3_384HashToU8,
                SHA3_384HashToU16,
                SHA3_384HashToU32,
                SHA3_384HashToU64,
                SHA3_384HashToU128,
                SHA3_384HashToScalar,
            ),
            HashSha3_512(hash) => hash_function!(
                hash,
                SHA3_512HashToAddress,
                SHA3_512HashToField,
                SHA3_512HashToGroup,
                SHA3_512HashToI8,
                SHA3_512HashToI16,
                SHA3_512HashToI32,
                SHA3_512HashToI64,
                SHA3_512HashToI128,
                SHA3_512HashToU8,
                SHA3_512HashToU16,
                SHA3_512HashToU32,
                SHA3_512HashToU64,
                SHA3_512HashToU128,
                SHA3_512HashToScalar,
            ),
            HashManyPSD2(_) | HashManyPSD4(_) | HashManyPSD8(_) => panic!("these instructions don't exist yet"),
            Inv(inv) => unary!(inv, Inverse),
            IsEq(eq) => binary!(eq, Eq),
            IsNeq(neq) => binary!(neq, Neq),
            LessThan(lt) => binary!(lt, Lt),
            LessThanOrEqual(lte) => binary!(lte, Lte),
            Modulo(modulo) => binary!(modulo, Mod),
            Mul(mul) => binary!(mul, Mul),
            MulWrapped(mul_wrapped) => binary!(mul_wrapped, MulWrapped),
            Nand(nand) => binary!(nand, Nand),
            Neg(neg) => unary!(neg, Negate),
            Nor(nor) => binary!(nor, Nor),
            Not(not) => unary!(not, Not),
            Or(or) => binary!(or, BitwiseOr),
            Pow(pow) => binary!(pow, Pow),
            PowWrapped(pow_wrapped) => binary!(pow_wrapped, PowWrapped),
            Rem(rem) => binary!(rem, Rem),
            RemWrapped(rem_wrapped) => binary!(rem_wrapped, RemWrapped),
            Shl(shl) => binary!(shl, Shl),
            ShlWrapped(shl_wrapped) => binary!(shl_wrapped, ShlWrapped),
            Shr(shr) => binary!(shr, Shr),
            ShrWrapped(shr_wrapped) => binary!(shr_wrapped, ShrWrapped),
            SignVerify(_) => todo!(),
            Square(square) => unary!(square, Square),
            SquareRoot(sqrt) => unary!(sqrt, SquareRoot),
            Sub(sub) => binary!(sub, Sub),
            SubWrapped(sub_wrapped) => binary!(sub_wrapped, SubWrapped),
            Ternary(ternary) => {
                let condition = self.operand_value(&ternary.operands()[0]);
                let result = match condition {
                    Value::Bool(true) => &ternary.operands()[1],
                    Value::Bool(false) => &ternary.operands()[2],
                    _ => panic!(),
                };
                self.increment_instruction_index();
                (self.operand_value(result), ternary.destinations()[0].clone())
            }
            Xor(xor) => binary!(xor, Xor),
        };

        self.set_register(destination, value);

        Ok(())
    }

    fn outputs(&self) -> Vec<Value> {
        let Some(Frame { element, .. }) = self.frames.last() else {
            panic!("frame expected");
        };
        let Element::AleoExecution { context, .. } = element else {
            panic!("aleo execution expected");
        };

        let mut result = match context {
            AleoContext::Closure(closure) => {
                closure.outputs().iter().map(|output| self.operand_value(output.operand())).collect()
            }
            AleoContext::Function(function) => {
                function.outputs().iter().map(|output| self.operand_value(output.operand())).collect()
            }
            AleoContext::Finalize(_finalize) => Vec::new(),
        };

        if result.is_empty() {
            result.push(Value::Unit);
        }
        result
    }

    fn step_aleo_command(&mut self, command: Command<TestnetV0>) -> Result<()> {
        use Command::*;

        let (value, destination) = match command {
            Instruction(instruction) => {
                self.step_aleo_instruction(instruction)?;
                return Ok(());
            }
            Await(await_) => {
                let Value::Future(future) = self.get_register(await_.register()) else {
                    halt_no_span!("attempted to await a non-future");
                };
                self.contexts.add_future(future.clone());
                self.increment_instruction_index();
                return Ok(());
            }
            Contains(contains) => {
                let mapping = self.mapping_by_call_operator(contains.mapping()).expect("mapping should be present");
                let key = self.operand_value(contains.key());
                let result = Value::Bool(mapping.contains_key(&key));
                self.increment_instruction_index();
                (result, contains.destination().clone())
            }
            Get(get) => {
                let key = self.operand_value(get.key());
                let value = self.mapping_by_call_operator(get.mapping()).and_then(|mapping| mapping.get(&key)).cloned();
                self.increment_instruction_index();

                match value {
                    Some(v) => (v, get.destination().clone()),
                    None => halt_no_span!("map access failure: {key}"),
                }
            }
            GetOrUse(get_or_use) => {
                let key = self.operand_value(get_or_use.key());
                let value =
                    self.mapping_by_call_operator(get_or_use.mapping()).and_then(|mapping| mapping.get(&key)).cloned();

                let use_value = value.unwrap_or_else(|| self.operand_value(get_or_use.default()));
                self.increment_instruction_index();

                (use_value, get_or_use.destination().clone())
            }
            Remove(remove) => {
                let key = self.operand_value(remove.key());
                let mapping_name = snarkvm_identifier_to_symbol(remove.mapping_name());
                let maybe_mapping = self.lookup_mapping_mut(None, mapping_name);
                match maybe_mapping {
                    None => halt_no_span!("no such mapping {mapping_name}"),
                    Some(mapping) => {
                        mapping.remove(&key);
                    }
                }
                self.increment_instruction_index();
                return Ok(());
            }
            Set(set) => {
                let key = self.operand_value(set.key());
                let value = self.operand_value(set.value());
                let mapping_name = snarkvm_identifier_to_symbol(set.mapping_name());
                let maybe_mapping = self.lookup_mapping_mut(None, mapping_name);
                match maybe_mapping {
                    None => halt_no_span!("no such mapping {mapping_name}"),
                    Some(mapping) => {
                        mapping.insert(key, value);
                    }
                }
                self.increment_instruction_index();
                return Ok(());
            }
            RandChaCha(rand) => {
                // If there are operands, they are additional seeds.
                let mut bits = Vec::new();
                for value in rand.operands().iter().map(|op| self.operand_value(op)) {
                    value.write_bits_le(&mut bits);
                }
                let field: Field<TestnetV0> = self.rng.gen();
                field.write_bits_le(&mut bits);
                let seed_vec = TestnetV0::hash_bhp1024(&bits)?.to_bytes_le()?;
                let mut seed = [0u8; 32];
                seed.copy_from_slice(&seed_vec[..32]);
                let mut rng = ChaCha20Rng::from_seed(seed);
                let value = match rand.destination_type() {
                    LiteralType::Address => Value::Address(rng.gen()),
                    LiteralType::Boolean => Value::Bool(rng.gen()),
                    LiteralType::Field => Value::Field(rng.gen()),
                    LiteralType::Group => Value::Group(rng.gen()),
                    LiteralType::I8 => Value::I8(rng.gen()),
                    LiteralType::I16 => Value::I16(rng.gen()),
                    LiteralType::I32 => Value::I32(rng.gen()),
                    LiteralType::I64 => Value::I64(rng.gen()),
                    LiteralType::I128 => Value::I128(rng.gen()),
                    LiteralType::U8 => Value::U8(rng.gen()),
                    LiteralType::U16 => Value::U16(rng.gen()),
                    LiteralType::U32 => Value::U32(rng.gen()),
                    LiteralType::U64 => Value::U64(rng.gen()),
                    LiteralType::U128 => Value::U128(rng.gen()),
                    LiteralType::Scalar => Value::Scalar(rng.gen()),
                    LiteralType::Signature => halt_no_span!("Cannot create a random signature"),
                    LiteralType::String => halt_no_span!("Cannot create a random string"),
                };
                self.increment_instruction_index();
                (value, rand.destination().clone())
            }
            BranchEq(branch_eq) => {
                let first = self.operand_value(branch_eq.first());
                let second = self.operand_value(branch_eq.second());
                if first.eq(&second)? {
                    self.branch(branch_eq.position());
                } else {
                    self.increment_instruction_index();
                }
                return Ok(());
            }
            BranchNeq(branch_neq) => {
                let first = self.operand_value(branch_neq.first());
                let second = self.operand_value(branch_neq.second());
                if first.neq(&second)? {
                    self.branch(branch_neq.position());
                } else {
                    self.increment_instruction_index();
                }
                return Ok(());
            }
            Position(_) => return Ok(()),
        };

        self.set_register(destination, value);

        Ok(())
    }

    fn branch(&mut self, label: &Identifier<TestnetV0>) {
        let Some(Frame {
            element: Element::AleoExecution { instruction_index, context: AleoContext::Finalize(finalize), .. },
            ..
        }) = self.frames.last_mut()
        else {
            panic!();
        };
        for (i, cmd) in finalize.commands().iter().enumerate() {
            if let Command::Position(position) = cmd {
                if position.name() == label {
                    *instruction_index = i;
                    return;
                }
            }
        }
        panic!("branch to nonexistent label {}", label);
    }

    pub fn step_aleo(&mut self) -> Result<()> {
        if let Some(command) = self.next_command().cloned() {
            self.step_aleo_command(command)?;
        } else if let Some(instruction) = self.next_instruction().cloned() {
            self.step_aleo_instruction(instruction)?;
        }

        if self.execution_complete() {
            let mut outputs = self.outputs();
            self.frames.pop();
            self.contexts.pop();
            if outputs.len() > 1 {
                self.values.push(Value::Tuple(outputs));
            } else {
                self.values.push(mem::take(&mut outputs[0]));
            }
        }

        Ok(())
    }
}

fn snarkvm_literal_type_to_type(snarkvm_type: LiteralType) -> Type {
    use Type::*;
    match snarkvm_type {
        LiteralType::Address => Address,
        LiteralType::Boolean => Boolean,
        LiteralType::Field => Field,
        LiteralType::Group => Group,
        LiteralType::I8 => Integer(IntegerType::I8),
        LiteralType::I16 => Integer(IntegerType::I16),
        LiteralType::I32 => Integer(IntegerType::I32),
        LiteralType::I64 => Integer(IntegerType::I64),
        LiteralType::I128 => Integer(IntegerType::I128),
        LiteralType::U8 => Integer(IntegerType::U8),
        LiteralType::U16 => Integer(IntegerType::U16),
        LiteralType::U32 => Integer(IntegerType::U32),
        LiteralType::U64 => Integer(IntegerType::U64),
        LiteralType::U128 => Integer(IntegerType::U128),
        LiteralType::Scalar => Scalar,
        LiteralType::Signature => todo!(),
        LiteralType::String => todo!(),
    }
}

fn snarkvm_literal_to_value(literal: Literal<TestnetV0>) -> Value {
    match literal {
        Literal::Address(x) => Value::Address(x),
        Literal::Boolean(x) => Value::Bool(*x),
        Literal::Field(x) => Value::Field(x),
        Literal::Group(x) => Value::Group(x),
        Literal::I8(x) => Value::I8(*x),
        Literal::I16(x) => Value::I16(*x),
        Literal::I32(x) => Value::I32(*x),
        Literal::I64(x) => Value::I64(*x),
        Literal::I128(x) => Value::I128(*x),
        Literal::U8(x) => Value::U8(*x),
        Literal::U16(x) => Value::U16(*x),
        Literal::U32(x) => Value::U32(*x),
        Literal::U64(x) => Value::U64(*x),
        Literal::U128(x) => Value::U128(*x),
        Literal::Scalar(x) => Value::Scalar(x),
        Literal::Signature(_) | Literal::String(_) => tc_fail!(),
    }
}

fn value_to_snarkvm_literal(value: Value) -> Literal<TestnetV0> {
    match value {
        Value::Bool(x) => Literal::Boolean(Boolean::new(x)),
        Value::U8(x) => Literal::U8(Integer::new(x)),
        Value::U16(x) => Literal::U16(Integer::new(x)),
        Value::U32(x) => Literal::U32(Integer::new(x)),
        Value::U64(x) => Literal::U64(Integer::new(x)),
        Value::U128(x) => Literal::U128(Integer::new(x)),
        Value::I8(x) => Literal::I8(Integer::new(x)),
        Value::I16(x) => Literal::I16(Integer::new(x)),
        Value::I32(x) => Literal::I32(Integer::new(x)),
        Value::I64(x) => Literal::I64(Integer::new(x)),
        Value::I128(x) => Literal::I128(Integer::new(x)),
        Value::Group(x) => Literal::Group(x),
        Value::Field(x) => Literal::Field(x),
        Value::Scalar(x) => Literal::Scalar(x),
        Value::Address(x) => Literal::Address(x),
        Value::Array(_) | Value::Tuple(_) | Value::Unit | Value::Future(_) | Value::Struct(_) => tc_fail!(),
    }
}
