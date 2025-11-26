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
    BinaryOperation,
    CoreFunction,
    IntegerType,
    Location,
    Type,
    UnaryOperation,
    interpreter_value::{self, AsyncExecution, Value},
};

use snarkvm::{
    prelude::{Identifier, LiteralType, PlaintextType, Register, TestnetV0},
    synthesizer::{
        Command,
        Instruction,
        program::{
            CallOperator,
            CastType,
            CommitVariant,
            DeserializeVariant,
            ECDSAVerifyVariant,
            HashVariant,
            Operand,
            SerializeVariant,
        },
    },
};

use std::mem;

impl Cursor {
    fn mapping_by_call_operator(&self, operator: &CallOperator<TestnetV0>) -> Option<&HashMap<Value, Value>> {
        let (program, name) = match operator {
            CallOperator::Locator(locator) => {
                (Some(snarkvm_identifier_to_symbol(locator.name())), snarkvm_identifier_to_symbol(locator.resource()))
            }
            CallOperator::Resource(id) => (None, snarkvm_identifier_to_symbol(id)),
        };
        self.lookup_mapping(program, name)
    }

    fn get_register(&self, reg: &Register<TestnetV0>) -> Value {
        let Some(Frame { element: Element::AleoExecution { registers, .. }, .. }) = self.frames.last() else {
            panic!();
        };
        match reg {
            Register::Locator(index) => {
                registers.get(index).expect("valid .aleo code doesn't access undefined registers").clone()
            }
            Register::Access(index, accesses) => {
                let value = registers.get(index).expect("valid .aleo code doesn't access undefined registers");
                value.accesses(accesses.iter().cloned()).expect("Accesses should work.")
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
            Register::Access(_, _) => panic!("Can't happen"),
        }
    }

    fn instructions_len(&self) -> usize {
        let Some(Frame { element: Element::AleoExecution { context, .. }, .. }) = self.frames.last() else {
            panic!();
        };
        match &**context {
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
        match &**context {
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
        match &**context {
            AleoContext::Closure(_) | AleoContext::Function(_) => None,
            AleoContext::Finalize(finalize) => finalize.commands().get(*instruction_index),
        }
    }

    fn operand_value(&self, operand: &Operand<TestnetV0>) -> Value {
        match operand {
            Operand::Literal(literal) => literal.clone().into(),
            Operand::Register(register) => self.get_register(register).clone(),
            Operand::ProgramID(_) => todo!(),
            Operand::Signer => self.signer.clone(),
            Operand::Caller => {
                if let Some(function_context) = self.contexts.last() {
                    function_context.caller.clone()
                } else {
                    self.signer.clone()
                }
            }
            Operand::BlockHeight => self.block_height.into(),
            Operand::NetworkID => todo!(),
            Operand::Checksum(_) => todo!(),
            Operand::Edition(_) => todo!(),
            Operand::ProgramOwner(_) => todo!(),
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
                let value =
                    interpreter_value::evaluate_unary(Default::default(), UnaryOperation::$op, &operand, &None)?;
                self.increment_instruction_index();
                (value, $svm_op.destinations()[0].clone())
            }};
        }

        macro_rules! binary {
            ($svm_op: expr, $op: ident) => {{
                let operand0 = self.operand_value(&$svm_op.operands()[0]);
                let operand1 = self.operand_value(&$svm_op.operands()[1]);
                let value = interpreter_value::evaluate_binary(
                    Default::default(),
                    BinaryOperation::$op,
                    &operand0,
                    &operand1,
                    &None,
                )?;
                self.increment_instruction_index();
                (value, $svm_op.destinations()[0].clone())
            }};
        }

        macro_rules! commit_function {
            ($commit: expr, $variant: expr) => {{
                let core_function = CoreFunction::Commit($variant, $commit.destination_type());
                let randomizer_value = self.operand_value(&$commit.operands()[0]);
                let operand_value = self.operand_value(&$commit.operands()[1]);
                self.values.push(randomizer_value);
                self.values.push(operand_value);
                let value = interpreter_value::evaluate_core_function(self, core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value.expect("Evaluation should work"), $commit.destinations()[0].clone())
            }};
        }

        macro_rules! hash_function {
            ($hash: expr, $variant: expr) => {{
                // Note. The only supported output types of a `hash` function are literals or bit arrays.
                let core_function =
                    CoreFunction::Hash($variant, Type::from_snarkvm::<TestnetV0>($hash.destination_type(), None));
                let operand_value = self.operand_value(&$hash.operands()[0]);
                self.values.push(operand_value);
                let value = interpreter_value::evaluate_core_function(self, core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value.expect("Evaluation should work"), $hash.destinations()[0].clone())
            }};
        }

        macro_rules! ecdsa_function {
            ($ecdsa: expr, $variant: expr) => {{
                let core_function = CoreFunction::ECDSAVerify($variant);
                let signature = self.operand_value(&$ecdsa.operands()[0]);
                let public_key = self.operand_value(&$ecdsa.operands()[1]);
                let message = self.operand_value(&$ecdsa.operands()[2]);
                self.values.push(signature);
                self.values.push(public_key);
                self.values.push(message);
                let value = interpreter_value::evaluate_core_function(self, core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value.expect("Evaluation should work"), $ecdsa.destinations()[0].clone())
            }};
        }

        macro_rules! schnorr_function {
            ($schnorr: expr, $variant: expr) => {{
                let core_function = CoreFunction::SignatureVerify;
                let signature = self.operand_value(&$schnorr.operands()[0]);
                let public_key = self.operand_value(&$schnorr.operands()[1]);
                let message = self.operand_value(&$schnorr.operands()[2]);
                self.values.push(signature);
                self.values.push(public_key);
                self.values.push(message);
                let value = interpreter_value::evaluate_core_function(self, core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value.expect("Evaluation should work"), $schnorr.destinations()[0].clone())
            }};
        }

        macro_rules! serialize_function {
            ($serialize: expr, $variant: expr) => {{
                let core_function = CoreFunction::Serialize($variant);
                let operand_value = self.operand_value(&$serialize.operands()[0]);
                self.values.push(operand_value);
                let value = interpreter_value::evaluate_core_function(self, core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value.expect("Evaluation should work"), $serialize.destinations()[0].clone())
            }};
        }

        macro_rules! deserialize_function {
            ($deserialize: expr, $variant: expr) => {{
                let core_function = CoreFunction::Deserialize(
                    $variant,
                    Type::from_snarkvm::<TestnetV0>($deserialize.destination_type(), None),
                );
                let operand_value = self.operand_value(&$deserialize.operands()[0]);
                self.values.push(operand_value);
                let value = interpreter_value::evaluate_core_function(self, core_function, &[], Span::default())?;
                self.increment_instruction_index();
                (value.expect("Evaluation should work"), $deserialize.destinations()[0].clone())
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
                if !operand0.eq(&operand1)? {
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
                    let async_ex =
                        AsyncExecution::AsyncFunctionCall { function: Location::new(program, vec![name]), arguments };
                    (vec![async_ex].into(), async_.destinations()[0].clone())
                } else {
                    self.do_call(
                        program,
                        &[name],
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
                    &[name],
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
                self.set_register(async_.destinations()[0].clone(), Vec::<AsyncExecution>::new().into());
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
                    for (i, dest) in call.destinations().iter().enumerate() {
                        self.set_register(
                            dest.clone(),
                            result.tuple_index(i).expect("Function returning multiple values should create a tuple."),
                        );
                    }
                }
                self.increment_instruction_index();
                return Ok(());
            }
            Call(_) | Async(_) => unreachable!("all cases covered above"),
            Cast(cast) => {
                let destination = cast.destinations()[0].clone();

                self.increment_instruction_index();

                match cast.cast_type() {
                    CastType::GroupXCoordinate => {
                        let value = self.operand_value(&cast.operands()[0]);
                        let mut values = vec![value];
                        let Some(result_value) = interpreter_value::evaluate_core_function(
                            &mut values,
                            CoreFunction::GroupToXCoordinate,
                            &[],
                            Default::default(),
                        )?
                        else {
                            halt_no_span!("GroupToXCoordinate didn't produce a value.");
                        };
                        (result_value, destination)
                    }
                    CastType::GroupYCoordinate => {
                        let value = self.operand_value(&cast.operands()[0]);
                        let mut values = vec![value];
                        let Some(result_value) = interpreter_value::evaluate_core_function(
                            &mut values,
                            CoreFunction::GroupToYCoordinate,
                            &[],
                            Default::default(),
                        )?
                        else {
                            halt_no_span!("GroupToYCoordinate didn't produce a value.");
                        };
                        (result_value, destination)
                    }
                    CastType::Plaintext(PlaintextType::Array(_array)) => {
                        let value = Value::make_array(cast.operands().iter().map(|op| self.operand_value(op)));
                        (value, destination)
                    }
                    CastType::Plaintext(PlaintextType::Literal(literal_type)) => {
                        let operand = self.operand_value(&cast.operands()[0]);
                        let Some(value) = operand.cast(&snarkvm_literal_type_to_type(*literal_type)) else {
                            halt_no_span!("cast failure");
                        };
                        (value, destination)
                    }
                    CastType::Plaintext(PlaintextType::Struct(struct_name)) => {
                        let name = Symbol::intern(&struct_name.to_string());
                        let struct_type = self.structs.get([name].as_slice()).expect("struct type should exist");
                        let operands = cast.operands().iter().map(|op| self.operand_value(op));
                        let value = Value::make_struct(
                            struct_type.keys().cloned().zip(operands),
                            self.current_program().unwrap(),
                            vec![name],
                        );
                        (value, destination)
                    }
                    CastType::Record(record_name) => {
                        let program = self.current_program().unwrap();
                        let name = Symbol::intern(&record_name.to_string());
                        let path = vec![name];
                        let record_type = self.records.get(&(program, path.clone())).expect("record type should exist");
                        let operands = cast.operands().iter().map(|op| self.operand_value(op));
                        let value =
                            Value::make_record(record_type.keys().cloned().zip(operands), program, path.clone());
                        (value, destination)
                    }
                    CastType::ExternalRecord(locator) => {
                        let program = Symbol::intern(&locator.program_id().name().to_string());
                        let name = Symbol::intern(&locator.resource().to_string());
                        let path = vec![name];
                        let record_type = self.records.get(&(program, path.clone())).expect("record type should exist");
                        let operands = cast.operands().iter().map(|op| self.operand_value(op));
                        let value =
                            Value::make_record(record_type.keys().cloned().zip(operands), program, path.clone());
                        (value, destination)
                    }
                }
            }
            CastLossy(cast_lossy) => {
                match cast_lossy.cast_type() {
                    CastType::Plaintext(PlaintextType::Literal(literal_type)) => {
                        // This is the only variant supported for lossy casts.
                        let operand = self.operand_value(&cast_lossy.operands()[0]);
                        let Some(value) = operand.cast_lossy(literal_type) else {
                            halt_no_span!("Cast failure");
                        };
                        let destination = cast_lossy.destinations()[0].clone();
                        (value, destination)
                    }
                    _ => tc_fail!(),
                }
            }
            CommitBHP256(commit) => commit_function!(commit, CommitVariant::CommitBHP256),
            CommitBHP512(commit) => commit_function!(commit, CommitVariant::CommitBHP512),
            CommitBHP768(commit) => commit_function!(commit, CommitVariant::CommitBHP768),
            CommitBHP1024(commit) => commit_function!(commit, CommitVariant::CommitBHP1024),
            CommitPED64(commit) => commit_function!(commit, CommitVariant::CommitPED64),
            CommitPED128(commit) => commit_function!(commit, CommitVariant::CommitPED128),
            Div(div) => binary!(div, Div),
            DivWrapped(div_wrapped) => binary!(div_wrapped, DivWrapped),
            Double(double) => unary!(double, Double),
            GreaterThan(gt) => binary!(gt, Gt),
            GreaterThanOrEqual(gte) => binary!(gte, Gte),
            HashBHP256(hash) => hash_function!(hash, HashVariant::HashBHP256),
            HashBHP512(hash) => hash_function!(hash, HashVariant::HashBHP512),
            HashBHP768(hash) => hash_function!(hash, HashVariant::HashBHP768),
            HashBHP1024(hash) => hash_function!(hash, HashVariant::HashBHP1024),
            HashKeccak256(hash) => hash_function!(hash, HashVariant::HashKeccak256),
            HashKeccak384(hash) => hash_function!(hash, HashVariant::HashKeccak384),
            HashKeccak512(hash) => hash_function!(hash, HashVariant::HashKeccak512),
            HashPED64(hash) => hash_function!(hash, HashVariant::HashPED64),
            HashPED128(hash) => hash_function!(hash, HashVariant::HashPED128),
            HashPSD2(hash) => hash_function!(hash, HashVariant::HashPSD2),
            HashPSD4(hash) => hash_function!(hash, HashVariant::HashPSD4),
            HashPSD8(hash) => hash_function!(hash, HashVariant::HashPSD8),
            HashSha3_256(hash) => hash_function!(hash, HashVariant::HashSha3_256),
            HashSha3_384(hash) => hash_function!(hash, HashVariant::HashSha3_384),
            HashSha3_512(hash) => hash_function!(hash, HashVariant::HashSha3_512),
            HashBHP256Raw(hash) => hash_function!(hash, HashVariant::HashBHP256Raw),
            HashBHP512Raw(hash) => hash_function!(hash, HashVariant::HashBHP512Raw),
            HashBHP768Raw(hash) => hash_function!(hash, HashVariant::HashBHP768Raw),
            HashBHP1024Raw(hash) => hash_function!(hash, HashVariant::HashBHP1024Raw),
            HashKeccak256Raw(hash) => hash_function!(hash, HashVariant::HashKeccak256Raw),
            HashKeccak384Raw(hash) => hash_function!(hash, HashVariant::HashKeccak384Raw),
            HashKeccak512Raw(hash) => hash_function!(hash, HashVariant::HashKeccak512Raw),
            HashPED64Raw(hash) => hash_function!(hash, HashVariant::HashPED64Raw),
            HashPED128Raw(hash) => hash_function!(hash, HashVariant::HashPED128Raw),
            HashPSD2Raw(hash) => hash_function!(hash, HashVariant::HashPSD2Raw),
            HashPSD4Raw(hash) => hash_function!(hash, HashVariant::HashPSD4Raw),
            HashPSD8Raw(hash) => hash_function!(hash, HashVariant::HashPSD8Raw),
            HashSha3_256Raw(hash) => hash_function!(hash, HashVariant::HashSha3_256Raw),
            HashSha3_384Raw(hash) => hash_function!(hash, HashVariant::HashSha3_384Raw),
            HashSha3_512Raw(hash) => hash_function!(hash, HashVariant::HashSha3_512Raw),
            HashKeccak256Native(hash) => hash_function!(hash, HashVariant::HashKeccak256Native),
            HashKeccak384Native(hash) => hash_function!(hash, HashVariant::HashKeccak384Native),
            HashKeccak512Native(hash) => hash_function!(hash, HashVariant::HashKeccak512Native),
            HashSha3_256Native(hash) => hash_function!(hash, HashVariant::HashSha3_256Native),
            HashSha3_384Native(hash) => hash_function!(hash, HashVariant::HashSha3_384Native),
            HashSha3_512Native(hash) => hash_function!(hash, HashVariant::HashSha3_512Native),
            HashKeccak256NativeRaw(hash) => hash_function!(hash, HashVariant::HashKeccak256NativeRaw),
            HashKeccak384NativeRaw(hash) => hash_function!(hash, HashVariant::HashKeccak384NativeRaw),
            HashKeccak512NativeRaw(hash) => hash_function!(hash, HashVariant::HashKeccak512NativeRaw),
            HashSha3_256NativeRaw(hash) => hash_function!(hash, HashVariant::HashSha3_256NativeRaw),
            HashSha3_384NativeRaw(hash) => hash_function!(hash, HashVariant::HashSha3_384NativeRaw),
            HashSha3_512NativeRaw(hash) => hash_function!(hash, HashVariant::HashSha3_512NativeRaw),
            ECDSAVerifyDigest(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::Digest),
            ECDSAVerifyDigestEth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::DigestEth),
            ECDSAVerifyKeccak256(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak256),
            ECDSAVerifyKeccak256Raw(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak256Raw),
            ECDSAVerifyKeccak256Eth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak256Eth),
            ECDSAVerifyKeccak384(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak384),
            ECDSAVerifyKeccak384Raw(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak384Raw),
            ECDSAVerifyKeccak384Eth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak384Eth),
            ECDSAVerifyKeccak512(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak512),
            ECDSAVerifyKeccak512Raw(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak512Raw),
            ECDSAVerifyKeccak512Eth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashKeccak512Eth),
            ECDSAVerifySha3_256(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_256),
            ECDSAVerifySha3_256Raw(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_256Raw),
            ECDSAVerifySha3_256Eth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_256Eth),
            ECDSAVerifySha3_384(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_384),
            ECDSAVerifySha3_384Raw(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_384Raw),
            ECDSAVerifySha3_384Eth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_384Eth),
            ECDSAVerifySha3_512(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_512),
            ECDSAVerifySha3_512Raw(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_512Raw),
            ECDSAVerifySha3_512Eth(ecdsa) => ecdsa_function!(ecdsa, ECDSAVerifyVariant::HashSha3_512Eth),
            HashManyPSD2(_) | HashManyPSD4(_) | HashManyPSD8(_) => panic!("these functions don't exist yet"),
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
            SignVerify(schnorr) => schnorr_function!(schnorr, false),
            Square(square) => unary!(square, Square),
            SquareRoot(sqrt) => unary!(sqrt, SquareRoot),
            Sub(sub) => binary!(sub, Sub),
            SubWrapped(sub_wrapped) => binary!(sub_wrapped, SubWrapped),
            Ternary(ternary) => {
                let condition = self.operand_value(&ternary.operands()[0]);
                let result = match condition.try_into() {
                    Ok(true) => &ternary.operands()[1],
                    Ok(false) => &ternary.operands()[2],
                    _ => tc_fail!(),
                };
                self.increment_instruction_index();
                (self.operand_value(result), ternary.destinations()[0].clone())
            }
            Xor(xor) => binary!(xor, Xor),
            SerializeBits(serialize_bits) => serialize_function!(serialize_bits, SerializeVariant::ToBits),
            SerializeBitsRaw(serialize_bits_raw) => {
                serialize_function!(serialize_bits_raw, SerializeVariant::ToBitsRaw)
            }
            DeserializeBits(deserialize_bits) => deserialize_function!(deserialize_bits, DeserializeVariant::FromBits),
            DeserializeBitsRaw(deserialize_bits_raw) => {
                deserialize_function!(deserialize_bits_raw, DeserializeVariant::FromBitsRaw)
            }
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

        let mut result = match &**context {
            AleoContext::Closure(closure) => {
                closure.outputs().iter().map(|output| self.operand_value(output.operand())).collect()
            }
            AleoContext::Function(function) => {
                function.outputs().iter().map(|output| self.operand_value(output.operand())).collect()
            }
            AleoContext::Finalize(_finalize) => Vec::new(),
        };

        if result.is_empty() {
            result.push(Value::make_unit());
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
                let value = self.get_register(await_.register());
                let Some(asyncs) = value.as_future() else {
                    halt_no_span!("attempted to await a non-future");
                };
                self.contexts.add_future(asyncs.to_vec());
                self.increment_instruction_index();
                return Ok(());
            }
            Contains(contains) => {
                // Value has interior mutability, since SnarkVM's value does, but this is okay - it's just the OnceCell which houses its bits.
                #[allow(clippy::mutable_key_type)]
                let mapping = self.mapping_by_call_operator(contains.mapping()).expect("mapping should be present");
                let key = self.operand_value(contains.key());
                let result = mapping.contains_key(&key).into();
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
                // TODO - this is not using the other operands which are supposed to seed the RNG.
                use CoreFunction::*;
                let function = ChaChaRand(rand.destination_type());
                let value =
                    interpreter_value::evaluate_core_function(self, function, &[], Default::default())?.unwrap();
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
                if !first.eq(&second)? {
                    self.branch(branch_neq.position());
                } else {
                    self.increment_instruction_index();
                }
                return Ok(());
            }
            Position(_) => {
                self.increment_instruction_index();
                return Ok(());
            }
        };

        self.set_register(destination, value);

        Ok(())
    }

    fn branch(&mut self, label: &Identifier<TestnetV0>) {
        let Some(Frame { element: Element::AleoExecution { instruction_index, context, .. }, .. }) =
            self.frames.last_mut()
        else {
            panic!();
        };
        let AleoContext::Finalize(finalize) = &mut **context else {
            panic!();
        };
        for (i, cmd) in finalize.commands().iter().enumerate() {
            if let Command::Position(position) = cmd
                && position.name() == label
            {
                *instruction_index = i;
                return;
            }
        }
        panic!("branch to nonexistent label {label}");
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
                self.values.push(Value::make_tuple(outputs));
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
