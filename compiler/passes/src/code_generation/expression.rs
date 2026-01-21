// Copyright (C) 2019-2026 Provable Inc.
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
    ArrayAccess,
    ArrayExpression,
    BinaryExpression,
    BinaryOperation,
    CallExpression,
    CastExpression,
    CompositeExpression,
    Expression,
    FromStrRadix,
    IntegerType,
    Intrinsic,
    IntrinsicExpression,
    Literal,
    LiteralVariant,
    LocatorExpression,
    MemberAccess,
    NetworkName,
    Node,
    NodeID,
    Path,
    ProgramId,
    RepeatExpression,
    TernaryExpression,
    TupleExpression,
    Type,
    UnaryExpression,
    UnaryOperation,
    Variant,
};
use snarkvm::{
    prelude::{CanaryV0, MainnetV0, TestnetV0},
    synthesizer::program::SerializeVariant,
};

use anyhow::bail;

/// Implement the necessary methods to visit nodes in the AST.
impl CodeGeneratingVisitor<'_> {
    pub fn visit_expression(&mut self, input: &Expression) -> (Option<AleoExpr>, Vec<AleoStmt>) {
        let is_empty_type = self.state.type_table.get(&input.id()).map(|ty| ty.is_empty()).unwrap_or(false);
        let is_pure = input.is_pure(&|id| self.state.type_table.get(&id).expect("Types should be resolved by now."));

        if is_empty_type && is_pure {
            // ignore expresssion
            return (None, vec![]);
        }

        let some_expr = |(expr, stmts): (AleoExpr, Vec<AleoStmt>)| (Some(expr), stmts);

        match input {
            Expression::ArrayAccess(expr) => (Some(self.visit_array_access(expr)), vec![]),
            Expression::MemberAccess(expr) => (Some(self.visit_member_access(expr)), vec![]),
            Expression::Path(expr) => (Some(self.visit_path(expr)), vec![]),
            Expression::Literal(expr) => (Some(self.visit_value(expr)), vec![]),
            Expression::Locator(expr) => (Some(self.visit_locator(expr)), vec![]),

            Expression::Array(expr) => some_expr(self.visit_array(expr)),
            Expression::Binary(expr) => some_expr(self.visit_binary(expr)),
            Expression::Call(expr) => some_expr(self.visit_call(expr)),
            Expression::Cast(expr) => some_expr(self.visit_cast(expr)),
            Expression::Composite(expr) => some_expr(self.visit_composite_init(expr)),
            Expression::Repeat(expr) => some_expr(self.visit_repeat(expr)),
            Expression::Ternary(expr) => some_expr(self.visit_ternary(expr)),
            Expression::Tuple(expr) => some_expr(self.visit_tuple(expr)),
            Expression::Unary(expr) => some_expr(self.visit_unary(expr)),

            Expression::Intrinsic(expr) => self.visit_intrinsic(expr),

            Expression::Async(..) => {
                panic!("`AsyncExpression`s should not be in the AST at this phase of compilation.")
            }
            Expression::Err(..) => panic!("`ErrExpression`s should not be in the AST at this phase of compilation."),
            Expression::TupleAccess(..) => panic!("Tuple accesses should not appear in the AST at this point."),
            Expression::Unit(..) => panic!("`UnitExpression`s should not be visited during code generation."),
        }
    }

    fn visit_path(&mut self, input: &Path) -> AleoExpr {
        // The only relevant paths here are paths to local variable or to mappings, so we really only care about their
        // names since mappings are only allowed in the top level program scope
        let var_name = input.identifier().name;
        self.variable_mapping.get(&var_name).or_else(|| self.global_mapping.get(&var_name)).unwrap().clone()
    }

    fn visit_value(&mut self, input: &Literal) -> AleoExpr {
        // AVM can only parse decimal numbers.
        let literal = if let LiteralVariant::Unsuffixed(value) = &input.variant {
            // For unsuffixed lierals, consult the `type_table` for their types. The type checker
            // ensures that their type can only be `Integer`, `Field`, `Group`, or `Scalar`.
            match self.state.type_table.get(&input.id) {
                Some(Type::Integer(int_ty)) => Literal {
                    variant: LiteralVariant::Integer(int_ty, value.clone()),
                    id: self.state.node_builder.next_id(),
                    span: input.span,
                },
                Some(Type::Field) => Literal {
                    variant: LiteralVariant::Field(value.clone()),
                    id: self.state.node_builder.next_id(),
                    span: input.span,
                },
                Some(Type::Group) => Literal {
                    variant: LiteralVariant::Group(value.clone()),
                    id: self.state.node_builder.next_id(),
                    span: input.span,
                },
                Some(Type::Scalar) => Literal {
                    variant: LiteralVariant::Scalar(value.clone()),
                    id: self.state.node_builder.next_id(),
                    span: input.span,
                },
                _ => panic!(
                    "Unexpected type for unsuffixed integer literal. This should have been caught by the type checker"
                ),
            }
        } else {
            input.clone()
        };

        // This function is duplicated in `interpreter/src/cursor.rs`,
        // but there's not really a great place to put a common implementation
        // right now.
        fn prepare_literal(s: &str) -> String {
            // If there's a `-`, separate it from the rest of the string.
            let (neg, rest) = s.strip_prefix("-").map(|rest| ("-", rest)).unwrap_or(("", s));
            // Remove leading zeros.
            let mut rest = rest.trim_start_matches('0');
            if rest.is_empty() {
                rest = "0";
            }
            format!("{neg}{rest}")
        }

        match literal.variant.clone() {
            LiteralVariant::None | LiteralVariant::Unsuffixed(..) => {
                panic!("This literal variant should no longer exist at code generation")
            }
            LiteralVariant::Address(val) => AleoExpr::Address(prepare_literal(&val)),
            LiteralVariant::Boolean(val) => AleoExpr::Bool(val),
            LiteralVariant::Field(val) => AleoExpr::Field(prepare_literal(&val)),
            LiteralVariant::Group(val) => AleoExpr::Group(prepare_literal(&val)),
            LiteralVariant::Scalar(val) => AleoExpr::Scalar(prepare_literal(&val)),
            LiteralVariant::String(val) => AleoExpr::String(val),
            LiteralVariant::Integer(itype, val) => {
                let val = val.replace('_', "");

                match itype {
                    IntegerType::U8 => AleoExpr::U8(u8::from_str_by_radix(&val).unwrap()),
                    IntegerType::U16 => AleoExpr::U16(u16::from_str_by_radix(&val).unwrap()),
                    IntegerType::U32 => AleoExpr::U32(u32::from_str_by_radix(&val).unwrap()),
                    IntegerType::U64 => AleoExpr::U64(u64::from_str_by_radix(&val).unwrap()),
                    IntegerType::U128 => AleoExpr::U128(u128::from_str_by_radix(&val).unwrap()),
                    IntegerType::I8 => AleoExpr::I8(i8::from_str_by_radix(&val).unwrap()),
                    IntegerType::I16 => AleoExpr::I16(i16::from_str_by_radix(&val).unwrap()),
                    IntegerType::I32 => AleoExpr::I32(i32::from_str_by_radix(&val).unwrap()),
                    IntegerType::I64 => AleoExpr::I64(i64::from_str_by_radix(&val).unwrap()),
                    IntegerType::I128 => AleoExpr::I128(i128::from_str_by_radix(&val).unwrap()),
                }
            }
        }
    }

    fn visit_locator(&mut self, input: &LocatorExpression) -> AleoExpr {
        if input.program.name.name == self.program_id.expect("Locators only appear within programs.").name.name {
            // This locator refers to the current program, so we only output the name, not the program.
            AleoExpr::RawName(input.name.to_string())
        } else {
            AleoExpr::RawName(input.to_string())
        }
    }

    fn visit_binary(&mut self, input: &BinaryExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let (left, left_instructions) = self.visit_expression(&input.left);
        let (right, right_instructions) = self.visit_expression(&input.right);
        let left = left.expect("Trying to operate on an empty expression");
        let right = right.expect("Trying to operate on an empty expression");

        let dest_reg = self.next_register();

        let binary_instruction = match input.op {
            BinaryOperation::Add => AleoStmt::Add(left, right, dest_reg.clone()),
            BinaryOperation::AddWrapped => AleoStmt::AddWrapped(left, right, dest_reg.clone()),
            BinaryOperation::And | BinaryOperation::BitwiseAnd => AleoStmt::And(left, right, dest_reg.clone()),
            BinaryOperation::Div => AleoStmt::Div(left, right, dest_reg.clone()),
            BinaryOperation::DivWrapped => AleoStmt::DivWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Eq => AleoStmt::Eq(left, right, dest_reg.clone()),
            BinaryOperation::Gte => AleoStmt::Gte(left, right, dest_reg.clone()),
            BinaryOperation::Gt => AleoStmt::Gt(left, right, dest_reg.clone()),
            BinaryOperation::Lte => AleoStmt::Lte(left, right, dest_reg.clone()),
            BinaryOperation::Lt => AleoStmt::Lt(left, right, dest_reg.clone()),
            BinaryOperation::Mod => AleoStmt::Mod(left, right, dest_reg.clone()),
            BinaryOperation::Mul => AleoStmt::Mul(left, right, dest_reg.clone()),
            BinaryOperation::MulWrapped => AleoStmt::MulWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Nand => AleoStmt::Nand(left, right, dest_reg.clone()),
            BinaryOperation::Neq => AleoStmt::Neq(left, right, dest_reg.clone()),
            BinaryOperation::Nor => AleoStmt::Nor(left, right, dest_reg.clone()),
            BinaryOperation::Or | BinaryOperation::BitwiseOr => AleoStmt::Or(left, right, dest_reg.clone()),
            BinaryOperation::Pow => AleoStmt::Pow(left, right, dest_reg.clone()),
            BinaryOperation::PowWrapped => AleoStmt::PowWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Rem => AleoStmt::Rem(left, right, dest_reg.clone()),
            BinaryOperation::RemWrapped => AleoStmt::RemWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Shl => AleoStmt::Shl(left, right, dest_reg.clone()),
            BinaryOperation::ShlWrapped => AleoStmt::ShlWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Shr => AleoStmt::Shr(left, right, dest_reg.clone()),
            BinaryOperation::ShrWrapped => AleoStmt::ShrWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Sub => AleoStmt::Sub(left, right, dest_reg.clone()),
            BinaryOperation::SubWrapped => AleoStmt::SubWrapped(left, right, dest_reg.clone()),
            BinaryOperation::Xor => AleoStmt::Xor(left, right, dest_reg.clone()),
        };

        // Concatenate the instructions.
        let mut instructions = left_instructions;
        instructions.extend(right_instructions);
        instructions.push(binary_instruction);

        (AleoExpr::Reg(dest_reg), instructions)
    }

    fn visit_cast(&mut self, input: &CastExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let (operand, mut instructions) = self.visit_expression(&input.expression);
        let operand = operand.expect("Trying to cast an empty expression");

        // Construct the destination register.
        let dest_reg = self.next_register();

        let cast_instruction = AleoStmt::Cast(operand, dest_reg.clone(), self.visit_type(&input.type_));

        // Concatenate the instructions.
        instructions.push(cast_instruction);

        (AleoExpr::Reg(dest_reg), instructions)
    }

    fn visit_array(&mut self, input: &ArrayExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let mut instructions = vec![];
        let operands = input
            .elements
            .iter()
            .map(|expr| self.visit_expression(expr))
            .filter_map(|(operand, operand_instructions)| {
                instructions.extend(operand_instructions);
                operand
            })
            .collect();

        // Construct the destination register.
        let destination_register = self.next_register();

        // Get the array type.
        let Some(array_type @ Type::Array(..)) = self.state.type_table.get(&input.id) else {
            panic!("All types should be known at this phase of compilation");
        };
        let array_type: AleoType = self.visit_type(&array_type);

        let array_instruction = AleoStmt::Cast(AleoExpr::Tuple(operands), destination_register.clone(), array_type);

        // Concatenate the instructions.
        instructions.push(array_instruction);

        (AleoExpr::Reg(destination_register), instructions)
    }

    fn visit_unary(&mut self, input: &UnaryExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let (operand, stmts) = self.visit_expression(&input.receiver);
        let operand = operand.expect("Trying to operate on an empty value");

        let dest_reg = self.next_register();

        // Note that non-empty suffixes must be preceded by a space.
        let unary_instruction = match input.op {
            UnaryOperation::Abs => AleoStmt::Abs(operand, dest_reg.clone()),
            UnaryOperation::AbsWrapped => AleoStmt::AbsW(operand, dest_reg.clone()),
            UnaryOperation::Double => AleoStmt::Double(operand, dest_reg.clone()),
            UnaryOperation::Inverse => AleoStmt::Inv(operand, dest_reg.clone()),
            UnaryOperation::Not => AleoStmt::Not(operand, dest_reg.clone()),
            UnaryOperation::Negate => AleoStmt::Neg(operand, dest_reg.clone()),
            UnaryOperation::Square => AleoStmt::Square(operand, dest_reg.clone()),
            UnaryOperation::SquareRoot => AleoStmt::Sqrt(operand, dest_reg.clone()),
            UnaryOperation::ToXCoordinate => AleoStmt::Cast(operand, dest_reg.clone(), AleoType::GroupX),
            UnaryOperation::ToYCoordinate => AleoStmt::Cast(operand, dest_reg.clone(), AleoType::GroupY),
        };

        // Concatenate the instructions.
        let mut instructions = stmts;
        instructions.push(unary_instruction);

        (AleoExpr::Reg(dest_reg), instructions)
    }

    fn visit_ternary(&mut self, input: &TernaryExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let (cond, cond_stmts) = self.visit_expression(&input.condition);
        let (if_true, if_true_stmts) = self.visit_expression(&input.if_true);
        let (if_false, if_false_stmts) = self.visit_expression(&input.if_false);
        let cond = cond.expect("Trying to build a ternary with an empty expression.");
        let if_true = if_true.expect("Trying to build a ternary with an empty expression.");
        let if_false = if_false.expect("Trying to build a ternary with an empty expression.");

        let dest_reg = self.next_register();
        let ternary_instruction = AleoStmt::Ternary(cond, if_true, if_false, dest_reg.clone());

        // Concatenate the instructions.
        let mut stmts = cond_stmts;
        stmts.extend(if_true_stmts);
        stmts.extend(if_false_stmts);
        stmts.push(ternary_instruction);

        (AleoExpr::Reg(dest_reg), stmts)
    }

    fn visit_composite_init(&mut self, input: &CompositeExpression) -> (AleoExpr, Vec<AleoStmt>) {
        // Lookup struct or record.
        let composite_location = input.path.expect_global_location();
        let program = composite_location.program;
        let this_program_name = self.program_id.unwrap().name.name;
        let composite_type = if let Some(is_record) = self.composite_mapping.get(composite_location) {
            if *is_record {
                // record.private;
                let [record_name] = &composite_location.path[..] else {
                    panic!("Absolute paths to records can only have a single segment at this stage.")
                };
                AleoType::Record { name: record_name.to_string(), program: None }
            } else {
                // foo; // no visibility for structs
                let struct_name = Self::legalize_path(&composite_location.path)
                    .expect("path format cannot be legalized at this point");
                if program == this_program_name {
                    AleoType::Ident { name: struct_name.to_string() }
                } else {
                    AleoType::Location { program: program.to_string(), name: struct_name.to_string() }
                }
            }
        } else {
            panic!("All composite types should be known at this phase of compilation")
        };

        // Initialize instruction builder strings.
        let mut instructions = vec![];

        // Visit each composite member and accumulate instructions from expressions.
        let operands: Vec<AleoExpr> = input
            .members
            .iter()
            .filter_map(|member| {
                if let Some(expr) = member.expression.as_ref() {
                    // Visit variable expression.
                    let (variable_operand, variable_instructions) = self.visit_expression(expr);
                    instructions.extend(variable_instructions);

                    variable_operand
                } else {
                    Some(self.visit_path(&Path::from(member.identifier).to_local()))
                }
            })
            .collect();

        // Push destination register to composite init instruction.
        let dest_reg = self.next_register();

        let composite_init_instruction = AleoStmt::Cast(AleoExpr::Tuple(operands), dest_reg.clone(), composite_type);

        instructions.push(composite_init_instruction);

        (AleoExpr::Reg(dest_reg), instructions)
    }

    fn visit_array_access(&mut self, input: &ArrayAccess) -> AleoExpr {
        let (array_operand, _) = self.visit_expression(&input.array);
        let array_operand = array_operand.expect("Trying to access an element of an empty expression.");

        assert!(
            matches!(self.state.type_table.get(&input.index.id()), Some(Type::Integer(_))),
            "unexpected type for for array index. This should have been caught by the type checker."
        );

        let index_operand = match &input.index {
            Expression::Literal(Literal {
                variant: LiteralVariant::Integer(_, s) | LiteralVariant::Unsuffixed(s),
                ..
            }) => AleoExpr::U32(s.parse().unwrap()),
            _ => panic!("Array indices must be integer literals"),
        };

        AleoExpr::ArrayAccess(Box::new(array_operand), Box::new(index_operand))
    }

    fn visit_member_access(&mut self, input: &MemberAccess) -> AleoExpr {
        let (inner_expr, _) = self.visit_expression(&input.inner);
        let inner_expr = inner_expr.expect("Trying to access a member of an empty expression.");

        AleoExpr::MemberAccess(Box::new(inner_expr), input.name.to_string())
    }

    fn visit_repeat(&mut self, input: &RepeatExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let (operand, mut operand_instructions) = self.visit_expression(&input.expr);
        let operand = operand.expect("Trying to repeat an empty expression");

        let count = input.count.as_u32().expect("repeat count should be known at this point");

        let expression_operands = std::iter::repeat_n(operand, count as usize).collect::<Vec<_>>();

        // Construct the destination register.
        let dest_reg = self.next_register();

        // Get the array type.
        let Some(array_type @ Type::Array(..)) = self.state.type_table.get(&input.id) else {
            panic!("All types should be known at this phase of compilation");
        };
        let array_type = self.visit_type(&array_type);

        let array_instruction = AleoStmt::Cast(AleoExpr::Tuple(expression_operands), dest_reg.clone(), array_type);

        // Concatenate the instructions.
        operand_instructions.push(array_instruction);

        (AleoExpr::Reg(dest_reg), operand_instructions)
    }

    fn visit_intrinsic(&mut self, input: &IntrinsicExpression) -> (Option<AleoExpr>, Vec<AleoStmt>) {
        let mut stmts = vec![];

        // Visit each function argument and accumulate instructions from expressions.
        let arguments = input
            .arguments
            .iter()
            .filter_map(|argument| {
                let (arg_string, arg_instructions) = self.visit_expression(argument);
                stmts.extend(arg_instructions);
                arg_string.map(|arg| (arg, argument.id()))
            })
            .collect::<Vec<_>>();

        let (intr_dest, intr_stmts) = self.generate_intrinsic(
            Intrinsic::from_symbol(input.name, &input.type_parameters)
                .expect("All core functions should be known at this phase of compilation"),
            &arguments,
        );

        // Add the instruction to the list of instructions.
        stmts.extend(intr_stmts);

        (intr_dest, stmts)
    }

    fn visit_call(&mut self, input: &CallExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let function_location = input.function.expect_global_location();
        let caller_program = self.program_id.expect("Calls only appear within programs.").name.name;
        let callee_program = function_location.program;
        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(caller_program, function_location)
            .expect("Type checking guarantees functions exist");

        let mut instructions = vec![];

        let arguments = input
            .arguments
            .iter()
            .filter_map(|argument| {
                let (argument, argument_instructions) = self.visit_expression(argument);
                instructions.extend(argument_instructions);
                argument
            })
            .collect();

        // Initialize storage for the destination registers.
        let mut destinations = Vec::new();

        // Create operands for the output registers.
        match func_symbol.function.output_type.clone() {
            t if t.is_empty() => {} // Do nothing
            Type::Tuple(tuple) => match tuple.length() {
                0 | 1 => panic!("Parsing guarantees that a tuple type has at least two elements"),
                len => {
                    for _ in 0..len {
                        destinations.push(self.next_register());
                    }
                }
            },
            _ => {
                destinations.push(self.next_register());
            }
        }

        // Add a register for async functions to represent the future created.
        if func_symbol.function.variant == Variant::AsyncFunction {
            destinations.push(self.next_register());
        }

        // Need to determine the program the function originated from as well as if the function has a finalize block.
        let call_instruction = if caller_program != callee_program {
            // All external functions must be defined as stubs.
            assert!(
                self.program.stubs.get(&callee_program).is_some(),
                "Type checking guarantees that imported and stub programs are present."
            );

            let [function_name] = &function_location.path[..] else {
                panic!("paths to external functions can only have a single segment at this stage.")
            };
            AleoStmt::Call(format!("{}.aleo/{}", callee_program, function_name), arguments, destinations.clone())
        } else if func_symbol.function.variant.is_async() {
            AleoStmt::Async(self.current_function.unwrap().identifier.to_string(), arguments, destinations.clone())
        } else {
            AleoStmt::Call(input.function.identifier().to_string(), arguments, destinations.clone())
        };

        // Push the call instruction to the list of instructions.
        instructions.push(call_instruction);

        // Return the output operands and the instructions.
        (AleoExpr::Tuple(destinations.into_iter().map(AleoExpr::Reg).collect()), instructions)
    }

    fn visit_tuple(&mut self, input: &TupleExpression) -> (AleoExpr, Vec<AleoStmt>) {
        let mut instructions = vec![];

        // Visit each tuple element and accumulate instructions from expressions.
        let tuple_elements = input
            .elements
            .iter()
            .filter_map(|element| {
                let (element, element_instructions) = self.visit_expression(element);
                instructions.extend(element_instructions);
                element
            })
            .collect();

        // CAUTION: does not return the destination_register.
        (AleoExpr::Tuple(tuple_elements), instructions)
    }

    fn generate_intrinsic(
        &mut self,
        intrinsic: Intrinsic,
        arguments: &[(AleoExpr, NodeID)],
    ) -> (Option<AleoExpr>, Vec<AleoStmt>) {
        {
            let args = arguments.iter().map(|(arg, _)| arg).collect_vec();

            let mut instructions = vec![];

            // A helper function to help with `Program::checksum`, `Program::edition`, and `Program::program_owner`.
            let generate_program_core = |program: &str, name: &str| {
                // Get the program ID from the first argument.
                let program_id = ProgramId::from_str_with_network(&program.replace("\"", ""), self.state.network)
                    .expect("Type checking guarantees that the program name is valid");
                // If the program name matches the current program ID, then use the operand directly, otherwise fully qualify the operand.
                match program_id.to_string()
                    == self.program_id.expect("The program ID is set before traversing the program").to_string()
                {
                    true => name.to_string(),
                    false => format!("{program_id}/{name}"),
                }
            };

            // Construct the instruction.
            let (destination, instruction) = match intrinsic {
                Intrinsic::SelfId | Intrinsic::SelfAddress => (
                    Some(AleoExpr::RawName(
                        self.program_id.expect("The program ID is set before traversing the program").to_string(),
                    )),
                    vec![],
                ),
                Intrinsic::SelfChecksum => (Some(AleoExpr::RawName("checksum".into())), vec![]),
                Intrinsic::SelfEdition => (Some(AleoExpr::RawName("edition".into())), vec![]),
                Intrinsic::SelfProgramOwner => (Some(AleoExpr::RawName("program_owner".into())), vec![]),
                Intrinsic::SelfCaller => (Some(AleoExpr::RawName("self.caller".into())), vec![]),
                Intrinsic::SelfSigner => (Some(AleoExpr::RawName("self.signer".into())), vec![]),
                Intrinsic::BlockHeight => (Some(AleoExpr::RawName("block.height".into())), vec![]),
                Intrinsic::BlockTimestamp => (Some(AleoExpr::RawName("block.timestamp".into())), vec![]),
                Intrinsic::NetworkId => (Some(AleoExpr::RawName("network.id".into())), vec![]),
                Intrinsic::Commit(variant, ref type_) => {
                    let type_ = AleoType::from(*type_);
                    let dest_reg = self.next_register();
                    let instruction =
                        AleoStmt::Commit(variant, args[0].clone(), args[1].clone(), dest_reg.clone(), type_);
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::Hash(variant, ref type_) => {
                    let dest_reg = self.next_register();
                    let type_ = match self.state.network {
                        NetworkName::TestnetV0 => AleoType::from(
                            type_.to_snarkvm::<TestnetV0>().expect("TYC guarantees that the type is valid"),
                        ),
                        NetworkName::CanaryV0 => AleoType::from(
                            type_.to_snarkvm::<CanaryV0>().expect("TYC guarantees that the type is valid"),
                        ),
                        NetworkName::MainnetV0 => AleoType::from(
                            type_.to_snarkvm::<MainnetV0>().expect("TYC guarantees that the type is valid"),
                        ),
                    };
                    let instruction = AleoStmt::Hash(variant, args[0].clone(), dest_reg.clone(), type_);
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::MappingGet => {
                    let dest_reg = self.next_register();
                    let instruction = AleoStmt::Get(args[0].clone(), args[1].clone(), dest_reg.clone());
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::MappingGetOrUse => {
                    let dest_reg = self.next_register();
                    let instruction =
                        AleoStmt::GetOrUse(args[0].clone(), args[1].clone(), args[2].clone(), dest_reg.clone());
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::MappingSet => {
                    let instruction = AleoStmt::Set(args[2].clone(), args[0].clone(), args[1].clone());
                    (None, vec![instruction])
                }
                Intrinsic::MappingRemove => {
                    let instruction = AleoStmt::Remove(args[0].clone(), args[1].clone());
                    (None, vec![instruction])
                }
                Intrinsic::MappingContains => {
                    let dest_reg = self.next_register();
                    let instruction = AleoStmt::Contains(args[0].clone(), args[1].clone(), dest_reg.clone());
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::GroupToXCoordinate => {
                    let dest_reg = self.next_register();
                    let instruction = AleoStmt::Cast(args[0].clone(), dest_reg.clone(), AleoType::GroupX);
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::GroupToYCoordinate => {
                    let dest_reg = self.next_register();
                    let instruction = AleoStmt::Cast(args[0].clone(), dest_reg.clone(), AleoType::GroupY);
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::GroupGen => (Some(AleoExpr::RawName("group::GEN".into())), vec![]),
                Intrinsic::ChaChaRand(type_) => {
                    let dest_reg = self.next_register();
                    let instruction = AleoStmt::RandChacha(dest_reg.clone(), type_.into());
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::SignatureVerify => {
                    let dest_reg = self.next_register();
                    let instruction =
                        AleoStmt::SignVerify(args[0].clone(), args[1].clone(), args[2].clone(), dest_reg.clone());
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::ECDSAVerify(variant) => {
                    let dest_reg = self.next_register();
                    let instruction = AleoStmt::EcdsaVerify(
                        variant,
                        args[0].clone(),
                        args[1].clone(),
                        args[2].clone(),
                        dest_reg.clone(),
                    );
                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::FutureAwait => {
                    let instruction = AleoStmt::Await(args[0].clone());
                    (None, vec![instruction])
                }
                Intrinsic::ProgramChecksum => {
                    (Some(AleoExpr::RawName(generate_program_core(&args[0].to_string(), "checksum"))), vec![])
                }
                Intrinsic::ProgramEdition => {
                    (Some(AleoExpr::RawName(generate_program_core(&args[0].to_string(), "edition"))), vec![])
                }
                Intrinsic::ProgramOwner => {
                    (Some(AleoExpr::RawName(generate_program_core(&args[0].to_string(), "program_owner"))), vec![])
                }
                Intrinsic::CheatCodePrintMapping
                | Intrinsic::CheatCodeSetBlockHeight
                | Intrinsic::CheatCodeSetBlockTimestamp
                | Intrinsic::CheatCodeSetSigner => {
                    (None, vec![])
                    // Do nothing. Cheat codes do not generate instructions.
                }
                Intrinsic::Serialize(variant) => {
                    // Get the input type.
                    let Some(input_type) = self.state.type_table.get(&arguments[0].1) else {
                        panic!("All types should be known at this phase of compilation");
                    };
                    // Get the instruction variant.
                    let is_raw = matches!(variant, SerializeVariant::ToBitsRaw);
                    // Get the size in bits of the input type.
                    fn struct_not_supported<T, U>(_: &T) -> anyhow::Result<U> {
                        bail!("structs are not supported")
                    }
                    let size_in_bits = match self.state.network {
                        NetworkName::TestnetV0 => input_type.size_in_bits::<TestnetV0, _, _>(
                            is_raw,
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                        NetworkName::MainnetV0 => input_type.size_in_bits::<MainnetV0, _, _>(
                            is_raw,
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                        NetworkName::CanaryV0 => input_type.size_in_bits::<CanaryV0, _, _>(
                            is_raw,
                            &struct_not_supported,
                            &struct_not_supported,
                        ),
                    }
                    .expect("TYC guarantees that all types have a valid size in bits");

                    let dest_reg = self.next_register();
                    let output_type = AleoType::Array { inner: Box::new(AleoType::Boolean), len: size_in_bits as u32 };
                    let input_type = self.visit_type(&input_type);
                    let instruction =
                        AleoStmt::Serialize(variant, args[0].clone(), input_type, dest_reg.clone(), output_type);

                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::Deserialize(variant, output_type) => {
                    // Get the input type.
                    let Some(input_type) = self.state.type_table.get(&arguments[0].1) else {
                        panic!("All types should be known at this phase of compilation");
                    };

                    let dest_reg = self.next_register();
                    let input_type = self.visit_type(&input_type);
                    let output_type = self.visit_type(&output_type);
                    let instruction =
                        AleoStmt::Deserialize(variant, args[0].clone(), input_type, dest_reg.clone(), output_type);

                    (Some(AleoExpr::Reg(dest_reg)), vec![instruction])
                }
                Intrinsic::OptionalUnwrap | Intrinsic::OptionalUnwrapOr => {
                    panic!("`Optional` intrinsics should have been lowered before code generation")
                }
                Intrinsic::VectorPush
                | Intrinsic::VectorPop
                | Intrinsic::VectorGet
                | Intrinsic::VectorSet
                | Intrinsic::VectorLen
                | Intrinsic::VectorClear
                | Intrinsic::VectorSwapRemove => {
                    panic!("`Vector` intrinsics should have been lowered before code generation")
                }
            };
            // Add the instruction to the list of instructions.
            instructions.extend(instruction);

            (destination, instructions)
        }
    }

    pub fn clone_register(&mut self, register: &AleoExpr, typ: &Type) -> (AleoExpr, Vec<AleoStmt>) {
        let new_reg = self.next_register();
        match typ {
            Type::Address => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), AleoType::Address);
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }
            Type::Boolean => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), AleoType::Boolean);
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }
            Type::Field => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), AleoType::Field);
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }
            Type::Group => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), AleoType::Group);
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }
            Type::Scalar => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), AleoType::Scalar);
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }
            Type::Signature => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), AleoType::Signature);
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }
            Type::Integer(int) => {
                let ins = AleoStmt::Cast(register.clone(), new_reg.clone(), match int {
                    IntegerType::U8 => AleoType::U8,
                    IntegerType::U16 => AleoType::U16,
                    IntegerType::U32 => AleoType::U32,
                    IntegerType::U64 => AleoType::U64,
                    IntegerType::U128 => AleoType::U128,
                    IntegerType::I8 => AleoType::I8,
                    IntegerType::I16 => AleoType::I16,
                    IntegerType::I32 => AleoType::I32,
                    IntegerType::I64 => AleoType::I64,
                    IntegerType::I128 => AleoType::I128,
                });
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }

            Type::Array(array_type) => {
                // We need to cast the old array's members into the new array.
                let elems = (0..array_type.length.as_u32().expect("length should be known at this point"))
                    .map(|i| AleoExpr::ArrayAccess(Box::new(register.clone()), Box::new(AleoExpr::U32(i))))
                    .collect::<Vec<_>>();

                let ins = AleoStmt::Cast(AleoExpr::Tuple(elems), new_reg.clone(), self.visit_type(typ));
                ((AleoExpr::Reg(new_reg)), vec![ins])
            }

            Type::Composite(comp_ty) => {
                let current_program = self.program_id.unwrap().name.name;
                // We need to cast the old struct or record's members into the new one.
                let composite_location = comp_ty.path.expect_global_location();
                let comp = self
                    .state
                    .symbol_table
                    .lookup_record(current_program, composite_location)
                    .or_else(|| self.state.symbol_table.lookup_struct(current_program, composite_location))
                    .unwrap();
                let elems = comp
                    .members
                    .iter()
                    .map(|member| {
                        AleoExpr::MemberAccess(Box::new(register.clone()), member.identifier.name.to_string())
                    })
                    .collect();
                let instruction = AleoStmt::Cast(
                    AleoExpr::Tuple(elems),
                    new_reg.clone(),
                    self.visit_type_with_visibility(typ, None).0,
                );
                ((AleoExpr::Reg(new_reg)), vec![instruction])
            }

            Type::Optional(_) => panic!("All optional types should have been lowered by now."),

            Type::Vector(_) => panic!("All vector types should have been lowered by now."),

            Type::Mapping(..)
            | Type::Future(..)
            | Type::Tuple(..)
            | Type::Identifier(..)
            | Type::String
            | Type::Unit
            | Type::Numeric
            | Type::Err => panic!("Objects of type {typ} cannot be cloned."),
        }
    }
}
