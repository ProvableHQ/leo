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
    ArrayAccess,
    ArrayExpression,
    AssociatedConstantExpression,
    AssociatedFunctionExpression,
    AsyncExpression,
    BinaryExpression,
    BinaryOperation,
    CallExpression,
    CastExpression,
    CoreFunction,
    ErrExpression,
    Expression,
    Literal,
    LiteralVariant,
    Location,
    LocatorExpression,
    MemberAccess,
    NetworkName,
    Node,
    Path,
    ProgramId,
    RepeatExpression,
    StructExpression,
    TernaryExpression,
    TupleExpression,
    Type,
    UnaryExpression,
    UnaryOperation,
    UnitExpression,
    Variant,
};
use leo_span::sym;
use snarkvm::{
    prelude::{CanaryV0, MainnetV0, TestnetV0},
    synthesizer::program::{CommitVariant, DeserializeVariant, SerializeVariant},
};

use anyhow::bail;
use std::{borrow::Borrow, fmt::Write as _};

/// Implement the necessary methods to visit nodes in the AST.
impl CodeGeneratingVisitor<'_> {
    pub fn visit_expression(&mut self, input: &Expression) -> (String, String) {
        let is_empty_type = self.state.type_table.get(&input.id()).map(|ty| ty.is_empty()).unwrap_or(false);
        let is_pure = input.is_pure();

        if is_empty_type && is_pure {
            // ignore expresssion
            return (String::new(), String::new());
        }

        match input {
            Expression::Array(expr) => self.visit_array(expr),
            Expression::ArrayAccess(expr) => self.visit_array_access(expr),
            Expression::AssociatedConstant(expr) => self.visit_associated_constant(expr),
            Expression::AssociatedFunction(expr) => self.visit_associated_function(expr),
            Expression::Async(expr) => self.visit_async(expr),
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Call(expr) => self.visit_call(expr),
            Expression::Cast(expr) => self.visit_cast(expr),
            Expression::Struct(expr) => self.visit_struct_init(expr),
            Expression::Err(expr) => self.visit_err(expr),
            Expression::Path(expr) => self.visit_path(expr),
            Expression::Literal(expr) => self.visit_value(expr),
            Expression::Locator(expr) => self.visit_locator(expr),
            Expression::MemberAccess(expr) => self.visit_member_access(expr),
            Expression::Repeat(expr) => self.visit_repeat(expr),
            Expression::Ternary(expr) => self.visit_ternary(expr),
            Expression::Tuple(expr) => self.visit_tuple(expr),
            Expression::TupleAccess(_) => panic!("Tuple accesses should not appear in the AST at this point."),
            Expression::Unary(expr) => self.visit_unary(expr),
            Expression::Unit(expr) => self.visit_unit(expr),
        }
    }

    fn visit_path(&mut self, input: &Path) -> (String, String) {
        // The only relevant paths here are paths to local variable or to mappings, so we really only care about their
        // names since mappings are only allowed in the top level program scope
        let var_name = input.identifier().name;
        (
            self.variable_mapping.get(&var_name).or_else(|| self.global_mapping.get(&var_name)).unwrap().clone(),
            String::new(),
        )
    }

    fn visit_err(&mut self, _input: &ErrExpression) -> (String, String) {
        panic!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_value(&mut self, input: &Literal) -> (String, String) {
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
        (format!("{}", literal.display_decimal()), String::new())
    }

    fn visit_locator(&mut self, input: &LocatorExpression) -> (String, String) {
        if input.program.name.name == self.program_id.expect("Locators only appear within programs.").name.name {
            // This locator refers to the current program, so we only output the name, not the program.
            (format!("{}", input.name), String::new())
        } else {
            (format!("{input}"), String::new())
        }
    }

    fn visit_binary(&mut self, input: &BinaryExpression) -> (String, String) {
        let (left_operand, left_instructions) = self.visit_expression(&input.left);
        let (right_operand, right_instructions) = self.visit_expression(&input.right);

        let opcode = match input.op {
            BinaryOperation::Add => String::from("add"),
            BinaryOperation::AddWrapped => String::from("add.w"),
            BinaryOperation::And => String::from("and"),
            BinaryOperation::BitwiseAnd => String::from("and"),
            BinaryOperation::Div => String::from("div"),
            BinaryOperation::DivWrapped => String::from("div.w"),
            BinaryOperation::Eq => String::from("is.eq"),
            BinaryOperation::Gte => String::from("gte"),
            BinaryOperation::Gt => String::from("gt"),
            BinaryOperation::Lte => String::from("lte"),
            BinaryOperation::Lt => String::from("lt"),
            BinaryOperation::Mod => String::from("mod"),
            BinaryOperation::Mul => String::from("mul"),
            BinaryOperation::MulWrapped => String::from("mul.w"),
            BinaryOperation::Nand => String::from("nand"),
            BinaryOperation::Neq => String::from("is.neq"),
            BinaryOperation::Nor => String::from("nor"),
            BinaryOperation::Or => String::from("or"),
            BinaryOperation::BitwiseOr => String::from("or"),
            BinaryOperation::Pow => String::from("pow"),
            BinaryOperation::PowWrapped => String::from("pow.w"),
            BinaryOperation::Rem => String::from("rem"),
            BinaryOperation::RemWrapped => String::from("rem.w"),
            BinaryOperation::Shl => String::from("shl"),
            BinaryOperation::ShlWrapped => String::from("shl.w"),
            BinaryOperation::Shr => String::from("shr"),
            BinaryOperation::ShrWrapped => String::from("shr.w"),
            BinaryOperation::Sub => String::from("sub"),
            BinaryOperation::SubWrapped => String::from("sub.w"),
            BinaryOperation::Xor => String::from("xor"),
        };

        let destination_register = self.next_register();
        let binary_instruction = format!("    {opcode} {left_operand} {right_operand} into {destination_register};\n",);

        // Concatenate the instructions.
        let mut instructions = left_instructions;
        instructions.push_str(&right_instructions);
        instructions.push_str(&binary_instruction);

        (destination_register, instructions)
    }

    fn visit_cast(&mut self, input: &CastExpression) -> (String, String) {
        let (expression_operand, mut instructions) = self.visit_expression(&input.expression);

        // Construct the destination register.
        let destination_register = self.next_register();

        let cast_instruction = format!(
            "    cast {expression_operand} into {destination_register} as {};\n",
            Self::visit_type(&input.type_)
        );

        // Concatenate the instructions.
        instructions.push_str(&cast_instruction);

        (destination_register, instructions)
    }

    fn visit_array(&mut self, input: &ArrayExpression) -> (String, String) {
        let mut expression_operands = String::new();
        let mut instructions = String::new();

        for (operand, operand_instructions) in input.elements.iter().map(|expr| self.visit_expression(expr)) {
            let space = if expression_operands.is_empty() { "" } else { " " };
            write!(&mut expression_operands, "{space}{operand}").unwrap();
            instructions.push_str(&operand_instructions);
        }

        // Construct the destination register.
        let destination_register = self.next_register();

        // Get the array type.
        let Some(array_type @ Type::Array(..)) = self.state.type_table.get(&input.id) else {
            panic!("All types should be known at this phase of compilation");
        };
        let array_type: String = Self::visit_type(&array_type);

        let array_instruction =
            format!("    cast {expression_operands} into {destination_register} as {array_type};\n");

        // Concatenate the instructions.
        instructions.push_str(&array_instruction);

        (destination_register, instructions)
    }

    fn visit_unary(&mut self, input: &UnaryExpression) -> (String, String) {
        let (expression_operand, expression_instructions) = self.visit_expression(&input.receiver);

        // Note that non-empty suffixes must be preceded by a space.
        let (opcode, suffix) = match input.op {
            UnaryOperation::Abs => ("abs", ""),
            UnaryOperation::AbsWrapped => ("abs.w", ""),
            UnaryOperation::Double => ("double", ""),
            UnaryOperation::Inverse => ("inv", ""),
            UnaryOperation::Not => ("not", ""),
            UnaryOperation::Negate => ("neg", ""),
            UnaryOperation::Square => ("square", ""),
            UnaryOperation::SquareRoot => ("sqrt", ""),
            UnaryOperation::ToXCoordinate => ("cast", " as group.x"),
            UnaryOperation::ToYCoordinate => ("cast", " as group.y"),
        };

        let destination_register = self.next_register();
        let unary_instruction = format!("    {opcode} {expression_operand} into {destination_register}{suffix};\n");

        // Concatenate the instructions.
        let mut instructions = expression_instructions;
        instructions.push_str(&unary_instruction);

        (destination_register, instructions)
    }

    fn visit_ternary(&mut self, input: &TernaryExpression) -> (String, String) {
        let (condition_operand, condition_instructions) = self.visit_expression(&input.condition);
        let (if_true_operand, if_true_instructions) = self.visit_expression(&input.if_true);
        let (if_false_operand, if_false_instructions) = self.visit_expression(&input.if_false);

        let destination_register = self.next_register();
        let ternary_instruction = format!(
            "    ternary {condition_operand} {if_true_operand} {if_false_operand} into {destination_register};\n",
        );

        // Concatenate the instructions.
        let mut instructions = condition_instructions;
        instructions.push_str(&if_true_instructions);
        instructions.push_str(&if_false_instructions);
        instructions.push_str(&ternary_instruction);

        (destination_register, instructions)
    }

    fn visit_struct_init(&mut self, input: &StructExpression) -> (String, String) {
        // Lookup struct or record.
        let name = if let Some((is_record, type_)) = self.composite_mapping.get(&input.path.absolute_path()) {
            if *is_record {
                // record.private;
                let [record_name] = &input.path.absolute_path()[..] else {
                    panic!("Absolute paths to records can only have a single segment at this stage.")
                };
                format!("{record_name}.{type_}")
            } else {
                // foo; // no visibility for structs
                Self::legalize_path(&input.path.absolute_path()).expect("path format cannot be legalized at this point")
            }
        } else {
            panic!("All composite types should be known at this phase of compilation")
        };

        // Initialize instruction builder strings.
        let mut instructions = String::new();
        let mut struct_init_instruction = String::from("    cast ");

        // Visit each struct member and accumulate instructions from expressions.
        for member in input.members.iter() {
            let operand = if let Some(expr) = member.expression.as_ref() {
                // Visit variable expression.
                let (variable_operand, variable_instructions) = self.visit_expression(expr);
                instructions.push_str(&variable_instructions);

                variable_operand
            } else {
                // Push operand identifier.
                let (ident_operand, ident_instructions) =
                    self.visit_path(&Path::from(member.identifier).into_absolute());
                instructions.push_str(&ident_instructions);

                ident_operand
            };

            // Push operand name to struct init instruction.
            write!(struct_init_instruction, "{operand} ").expect("failed to write to string");
        }

        // Push destination register to struct init instruction.
        let destination_register = self.next_register();
        writeln!(struct_init_instruction, "into {destination_register} as {name};",)
            .expect("failed to write to string");

        instructions.push_str(&struct_init_instruction);

        (destination_register, instructions)
    }

    fn visit_array_access(&mut self, input: &ArrayAccess) -> (String, String) {
        let (array_operand, _) = self.visit_expression(&input.array);

        assert!(
            matches!(self.state.type_table.get(&input.index.id()), Some(Type::Integer(_))),
            "unexpected type for for array index. This should have been caught by the type checker."
        );

        let index_operand = match &input.index {
            Expression::Literal(Literal {
                variant: LiteralVariant::Integer(_, s) | LiteralVariant::Unsuffixed(s),
                ..
            }) => format!("{s}u32"),
            _ => panic!("Array indices must be integer literals"),
        };

        (format!("{array_operand}[{index_operand}]"), String::new())
    }

    fn visit_member_access(&mut self, input: &MemberAccess) -> (String, String) {
        // Handle `self.address`, `self.caller`, `self.checksum`, `self.edition`, `self.id`, `self.program_owner`, `self.signer`.
        if let Expression::Path(path) = input.inner.borrow()
            && matches!(path.try_absolute_path().as_deref(), Some([sym::SelfLower]))
        {
            // Get the current program ID.
            let program_id = self.program_id.expect("Program ID should be set before traversing the program");

            match input.name.name {
                // Return the program ID directly.
                sym::address | sym::id => {
                    return (program_id.to_string(), String::new());
                }
                // Return the appropriate snarkVM operand.
                name @ (sym::checksum | sym::edition | sym::program_owner) => {
                    return (name.to_string(), String::new());
                }
                _ => {} // Do nothing as `self.signer` and `self.caller` are handled below.
            }
        }

        let (inner_expr, _) = self.visit_expression(&input.inner);
        let member_access = format!("{}.{}", inner_expr, input.name);

        (member_access, String::new())
    }

    fn visit_repeat(&mut self, input: &RepeatExpression) -> (String, String) {
        let (operand, mut operand_instructions) = self.visit_expression(&input.expr);
        let count = input.count.as_u32().expect("repeat count should be known at this point");

        let expression_operands = std::iter::repeat_n(operand, count as usize).collect::<Vec<_>>().join(" ");

        // Construct the destination register.
        let destination_register = self.next_register();

        // Get the array type.
        let Some(array_type @ Type::Array(..)) = self.state.type_table.get(&input.id) else {
            panic!("All types should be known at this phase of compilation");
        };
        let array_type: String = Self::visit_type(&array_type);

        let array_instruction =
            format!("    cast {expression_operands} into {destination_register} as {array_type};\n");

        // Concatenate the instructions.
        operand_instructions.push_str(&array_instruction);

        (destination_register, operand_instructions)
    }

    // group::GEN -> group::GEN
    fn visit_associated_constant(&mut self, input: &AssociatedConstantExpression) -> (String, String) {
        (format!("{input}"), String::new())
    }

    // Pedersen64::hash() -> hash.ped64
    fn visit_associated_function(&mut self, input: &AssociatedFunctionExpression) -> (String, String) {
        let mut instructions = String::new();

        // Visit each function argument and accumulate instructions from expressions.
        let arguments = input
            .arguments
            .iter()
            .map(|argument| {
                let (arg_string, arg_instructions) = self.visit_expression(argument);
                instructions.push_str(&arg_instructions);
                arg_string
            })
            .collect::<Vec<_>>();

        // A helper function to help with `Program::checksum`, `Program::edition`, and `Program::program_owner`.
        let generate_program_core = |program: &str, name: &str| {
            // Get the program ID from the first argument.
            let program_id = ProgramId::from_str_with_network(&program.replace("\"", ""), self.state.network)
                .expect("Type checking guarantees that the program name is valid");
            // If the program name matches the current program ID, then use the operand directly, otherwise fully qualify the operand.
            let operand = match program_id.to_string()
                == self.program_id.expect("The program ID is set before traversing the program").to_string()
            {
                true => name.to_string(),
                false => format!("{program_id}/{name}"),
            };
            (operand, String::new())
        };

        // Construct the instruction.
        let (destination, instruction) = match CoreFunction::try_from(input).ok() {
            Some(CoreFunction::Commit(variant, ref type_)) => {
                let mut instruction = format!("    {}", CommitVariant::opcode(variant as u8));
                let destination_register = self.next_register();
                // Write the arguments and the destination register.
                writeln!(instruction, " {} {} into {destination_register} as {type_};", arguments[0], arguments[1])
                    .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::Hash(variant, ref type_)) => {
                let mut instruction = format!("    {}", variant.opcode());
                let destination_register = self.next_register();
                let type_ = match self.state.network {
                    NetworkName::TestnetV0 => {
                        type_.to_snarkvm::<TestnetV0>().expect("TYC guarantees that the type is valid").to_string()
                    }
                    NetworkName::CanaryV0 => {
                        type_.to_snarkvm::<CanaryV0>().expect("TYC guarantees that the type is valid").to_string()
                    }
                    NetworkName::MainnetV0 => {
                        type_.to_snarkvm::<MainnetV0>().expect("TYC guarantees that the type is valid").to_string()
                    }
                };
                // Write the arguments and the destination register.
                writeln!(instruction, " {} into {destination_register} as {type_};", arguments[0])
                    .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::Get) => {
                let mut instruction = "    get".to_string();
                let destination_register = self.next_register();
                // Write the mapping name and the key.
                writeln!(instruction, " {}[{}] into {destination_register};", arguments[0], arguments[1])
                    .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::MappingGetOrUse) => {
                let mut instruction = "    get.or_use".to_string();
                let destination_register = self.next_register();
                // Write the mapping name, the key, and the default value.
                writeln!(
                    instruction,
                    " {}[{}] {} into {destination_register};",
                    arguments[0], arguments[1], arguments[2]
                )
                .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::Set) => {
                let mut instruction = "    set".to_string();
                // Write the value, mapping name, and the key.
                writeln!(instruction, " {} into {}[{}];", arguments[2], arguments[0], arguments[1])
                    .expect("failed to write to string");
                (String::new(), instruction)
            }
            Some(CoreFunction::MappingRemove) => {
                let mut instruction = "    remove".to_string();
                // Write the mapping name and the key.
                writeln!(instruction, " {}[{}];", arguments[0], arguments[1]).expect("failed to write to string");
                (String::new(), instruction)
            }
            Some(CoreFunction::MappingContains) => {
                let mut instruction = "    contains".to_string();
                let destination_register = self.next_register();
                // Write the mapping name and the key.
                writeln!(instruction, " {}[{}] into {destination_register};", arguments[0], arguments[1])
                    .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::GroupToXCoordinate) => {
                let mut instruction = "    cast".to_string();
                let destination_register = self.next_register();
                // Write the argument and the destination register.
                writeln!(instruction, " {} into {destination_register} as group.x;", arguments[0],)
                    .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::GroupToYCoordinate) => {
                let mut instruction = "    cast".to_string();
                let destination_register = self.next_register();
                // Write the argument and the destination register.
                writeln!(instruction, " {} into {destination_register} as group.y;", arguments[0],)
                    .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::ChaChaRand(type_)) => {
                // Get the destination register.
                let destination_register = self.next_register();
                // Construct the instruction template.
                let instruction = format!("    rand.chacha into {destination_register} as {type_};\n");

                (destination_register, instruction)
            }
            Some(CoreFunction::SignatureVerify) => {
                let mut instruction = "    sign.verify".to_string();
                let destination_register = self.next_register();
                // Write the arguments and the destination register.
                writeln!(
                    instruction,
                    " {} {} {} into {destination_register};",
                    arguments[0], arguments[1], arguments[2]
                )
                .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::ECDSAVerify(variant)) => {
                let mut instruction = format!("    {}", variant.opcode());
                let destination_register = self.next_register();
                // Write the arguments and the destination register.
                writeln!(
                    instruction,
                    " {} {} {} into {destination_register};",
                    arguments[0], arguments[1], arguments[2]
                )
                .expect("failed to write to string");
                (destination_register, instruction)
            }
            Some(CoreFunction::FutureAwait) => {
                let mut instruction = "    await".to_string();
                writeln!(instruction, " {};", arguments[0]).expect("failed to write to string");
                (String::new(), instruction)
            }
            Some(CoreFunction::ProgramChecksum) => generate_program_core(&arguments[0], "checksum"),
            Some(CoreFunction::ProgramEdition) => generate_program_core(&arguments[0], "edition"),
            Some(CoreFunction::ProgramOwner) => generate_program_core(&arguments[0], "program_owner"),
            Some(CoreFunction::CheatCodePrintMapping)
            | Some(CoreFunction::CheatCodeSetBlockHeight)
            | Some(CoreFunction::CheatCodeSetSigner) => {
                (String::new(), String::new())
                // Do nothing. Cheat codes do not generate instructions.
            }
            Some(CoreFunction::Serialize(variant)) => {
                // Get the input type.
                let Some(input_type) = self.state.type_table.get(&input.arguments[0].id()) else {
                    panic!("All types should be known at this phase of compilation");
                };
                // Get the instruction variant.
                let (is_raw, variant) = match variant {
                    SerializeVariant::ToBits => (false, "bits"),
                    SerializeVariant::ToBitsRaw => (true, "bits.raw"),
                };
                // Get the size in bits of the input type.
                let size_in_bits = match self.state.network {
                    NetworkName::TestnetV0 => {
                        input_type.size_in_bits::<TestnetV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                    NetworkName::MainnetV0 => {
                        input_type.size_in_bits::<MainnetV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                    NetworkName::CanaryV0 => {
                        input_type.size_in_bits::<CanaryV0, _>(is_raw, |_| bail!("structs are not supported"))
                    }
                }
                .expect("TYC guarantees that all types have a valid size in bits");

                // Construct the output array type.
                let output_array_type = format!("[boolean; {size_in_bits}u32]");
                // Construct the destination register.
                let destination_register = self.next_register();
                // Construct the instruction template.
                let instruction = format!(
                    "    serialize.{variant} {} ({}) into {destination_register} ({output_array_type});\n",
                    arguments[0],
                    Self::visit_type(&input_type)
                );

                (destination_register, instruction)
            }
            Some(CoreFunction::Deserialize(variant, output_type)) => {
                // Get the instruction variant.
                let variant = match variant {
                    DeserializeVariant::FromBits => "bits",
                    DeserializeVariant::FromBitsRaw => "bits.raw",
                };
                // Get the input type.
                let Some(input_type) = self.state.type_table.get(&input.arguments[0].id()) else {
                    panic!("All types should be known at this phase of compilation");
                };
                // Construct the destination register.
                let destination_register = self.next_register();
                // Construct the instruction template.
                let instruction = format!(
                    "    deserialize.{variant} {} ({}) into {destination_register} ({});\n",
                    arguments[0],
                    Self::visit_type(&input_type),
                    Self::visit_type(&output_type)
                );

                (destination_register, instruction)
            }
            Some(CoreFunction::OptionalUnwrap) | Some(CoreFunction::OptionalUnwrapOr) => {
                panic!("`Optional` core functions should have been lowered before code generation")
            }
            Some(CoreFunction::VectorPush)
            | Some(CoreFunction::VectorPop)
            | Some(CoreFunction::VectorLen)
            | Some(CoreFunction::VectorClear)
            | Some(CoreFunction::VectorSwapRemove) => {
                panic!("`Vector` core functions should have been lowered before code generation")
            }
            None => {
                panic!("All core functions should be known at this phase of compilation")
            }
        };
        // Add the instruction to the list of instructions.
        instructions.push_str(&instruction);

        (destination, instructions)
    }

    fn visit_async(&mut self, _input: &AsyncExpression) -> (String, String) {
        panic!("`AsyncExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_call(&mut self, input: &CallExpression) -> (String, String) {
        let caller_program = self.program_id.expect("Calls only appear within programs.").name.name;
        let callee_program = input.program.unwrap_or(caller_program);
        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(&Location::new(callee_program, input.function.absolute_path().to_vec()))
            .expect("Type checking guarantees functions exist");

        // Need to determine the program the function originated from as well as if the function has a finalize block.
        let mut call_instruction = if caller_program != callee_program {
            // All external functions must be defined as stubs.
            assert!(
                self.program.stubs.get(&callee_program).is_some(),
                "Type checking guarantees that imported and stub programs are present."
            );
            format!("    call {}.aleo/{}", callee_program, input.function)
        } else if func_symbol.function.variant.is_async() {
            format!("    async {}", self.current_function.unwrap().identifier)
        } else {
            format!("    call {}", input.function)
        };

        let mut instructions = String::new();

        for argument in input.arguments.iter() {
            let (argument, argument_instructions) = self.visit_expression(argument);
            write!(call_instruction, " {argument}").expect("failed to write to string");
            instructions.push_str(&argument_instructions);
        }

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

        // Construct the output operands. These are the destination registers **without** the future.
        let output_operands = destinations.join(" ");

        // If destination registers were created, write them to the call instruction.
        if !destinations.is_empty() {
            write!(call_instruction, " into").expect("failed to write to string");
            for destination in &destinations {
                write!(call_instruction, " {destination}").expect("failed to write to string");
            }
        }

        // Write the closing semicolon.
        writeln!(call_instruction, ";").expect("failed to write to string");

        // Push the call instruction to the list of instructions.
        instructions.push_str(&call_instruction);

        // Return the output operands and the instructions.
        (output_operands, instructions)
    }

    fn visit_tuple(&mut self, input: &TupleExpression) -> (String, String) {
        // Need to return a single string here so we will join the tuple elements with ' '
        // and split them after this method is called.
        let mut tuple_elements = Vec::with_capacity(input.elements.len());
        let mut instructions = String::new();

        // Visit each tuple element and accumulate instructions from expressions.
        for element in input.elements.iter() {
            let (element, element_instructions) = self.visit_expression(element);
            tuple_elements.push(element);
            instructions.push_str(&element_instructions);
        }

        // CAUTION: does not return the destination_register.
        (tuple_elements.join(" "), instructions)
    }

    fn visit_unit(&mut self, _input: &UnitExpression) -> (String, String) {
        panic!("`UnitExpression`s should not be visited during code generation.")
    }

    pub fn clone_register(&mut self, register: &str, typ: &Type) -> (String, String) {
        if typ.is_empty() {
            return (String::new(), String::new());
        }
        let new_reg = self.next_register();
        match typ {
            Type::Address
            | Type::Boolean
            | Type::Field
            | Type::Group
            | Type::Scalar
            | Type::Signature
            | Type::Integer(_) => {
                // These types can be cloned just by casting them to themselves.
                let instruction = format!("    cast {register} into {new_reg} as {typ};\n");
                (new_reg, instruction)
            }

            Type::Array(array_type) => {
                // We need to cast the old array's members into the new array.
                let mut instruction = "    cast ".to_string();
                for i in 0..array_type.length.as_u32().expect("length should be known at this point") as usize {
                    write!(&mut instruction, "{register}[{i}u32] ").unwrap();
                }
                writeln!(&mut instruction, "into {new_reg} as {};", Self::visit_type(typ)).unwrap();
                (new_reg, instruction)
            }

            Type::Composite(comp_ty) => {
                // We need to cast the old struct or record's members into the new one.
                let program = comp_ty.program.unwrap_or(self.program_id.unwrap().name.name);
                let location = Location::new(program, comp_ty.path.absolute_path().to_vec());
                let comp = self
                    .state
                    .symbol_table
                    .lookup_record(&location)
                    .or_else(|| self.state.symbol_table.lookup_struct(&comp_ty.path.absolute_path()))
                    .unwrap();
                let mut instruction = "    cast ".to_string();
                for member in &comp.members {
                    write!(&mut instruction, "{register}.{} ", member.identifier.name).unwrap();
                }
                writeln!(
                    &mut instruction,
                    "into {new_reg} as {};",
                    // We call `..with_visibility` just so we get the `.record` appended if it's a record.
                    self.visit_type_with_visibility(typ, leo_ast::Mode::None)
                )
                .unwrap();
                (new_reg, instruction)
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
