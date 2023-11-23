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

use crate::CodeGenerator;
use leo_ast::{
    AccessExpression,
    ArrayAccess,
    ArrayExpression,
    AssociatedConstant,
    AssociatedFunction,
    BinaryExpression,
    BinaryOperation,
    CallExpression,
    CastExpression,
    ErrExpression,
    Expression,
    Identifier,
    Literal,
    MemberAccess,
    StructExpression,
    TernaryExpression,
    TupleExpression,
    Type,
    UnaryExpression,
    UnaryOperation,
    UnitExpression,
};
use leo_span::sym;
use std::borrow::Borrow;

use std::fmt::Write as _;

/// Implement the necessary methods to visit nodes in the AST.
// Note: We opt for this option instead of using `Visitor` and `Director` because this pass requires
// a post-order traversal of the AST. This is sufficient since this implementation is intended to be
// a prototype. The production implementation will require a redesign of `Director`.
impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_expression(&mut self, input: &'a Expression) -> (String, String) {
        match input {
            Expression::Access(expr) => self.visit_access(expr),
            Expression::Array(expr) => self.visit_array(expr),
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Call(expr) => self.visit_call(expr),
            Expression::Cast(expr) => self.visit_cast(expr),
            Expression::Struct(expr) => self.visit_struct_init(expr),
            Expression::Err(expr) => self.visit_err(expr),
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Literal(expr) => self.visit_value(expr),
            Expression::Ternary(expr) => self.visit_ternary(expr),
            Expression::Tuple(expr) => self.visit_tuple(expr),
            Expression::Unary(expr) => self.visit_unary(expr),
            Expression::Unit(expr) => self.visit_unit(expr),
        }
    }

    fn visit_identifier(&mut self, input: &'a Identifier) -> (String, String) {
        (
            self.variable_mapping.get(&input.name).or_else(|| self.global_mapping.get(&input.name)).unwrap().clone(),
            String::new(),
        )
    }

    fn visit_err(&mut self, _input: &'a ErrExpression) -> (String, String) {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn visit_value(&mut self, input: &'a Literal) -> (String, String) {
        (format!("{input}"), String::new())
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression) -> (String, String) {
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

        let destination_register = format!("r{}", self.next_register);
        let binary_instruction = format!("    {opcode} {left_operand} {right_operand} into {destination_register};\n",);

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = left_instructions;
        instructions.push_str(&right_instructions);
        instructions.push_str(&binary_instruction);

        (destination_register, instructions)
    }

    fn visit_cast(&mut self, input: &'a CastExpression) -> (String, String) {
        let (expression_operand, mut instructions) = self.visit_expression(&input.expression);

        // Construct the destination register.
        let destination_register = format!("r{}", self.next_register);
        // Increment the register counter.
        self.next_register += 1;

        let cast_instruction =
            format!("    cast {expression_operand} into {destination_register} as {};\n", input.type_);

        // Concatenate the instructions.
        instructions.push_str(&cast_instruction);

        (destination_register, instructions)
    }

    fn visit_array(&mut self, input: &'a ArrayExpression) -> (String, String) {
        let (expression_operands, mut instructions) =
            input.elements.iter().map(|expr| self.visit_expression(expr)).fold(
                (String::new(), String::new()),
                |(mut operands, mut instructions), (operand, operand_instructions)| {
                    operands.push_str(&format!(" {operand}"));
                    instructions.push_str(&operand_instructions);
                    (operands, instructions)
                },
            );

        // Construct the destination register.
        let destination_register = format!("r{}", self.next_register);
        // Increment the register counter.
        self.next_register += 1;

        // Get the array type.
        let array_type = match self.type_table.get(&input.id) {
            Some(Type::Array(array_type)) => Type::Array(array_type),
            _ => unreachable!("All types should be known at this phase of compilation"),
        };
        let array_type: String = Self::visit_type(&array_type);

        let array_instruction =
            format!("    cast {expression_operands} into {destination_register} as {};\n", array_type);

        // Concatenate the instructions.
        instructions.push_str(&array_instruction);

        (destination_register, instructions)
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression) -> (String, String) {
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

        let destination_register = format!("r{}", self.next_register);
        let unary_instruction = format!("    {opcode} {expression_operand} into {destination_register}{suffix};\n");

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = expression_instructions;
        instructions.push_str(&unary_instruction);

        (destination_register, instructions)
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression) -> (String, String) {
        let (condition_operand, condition_instructions) = self.visit_expression(&input.condition);
        let (if_true_operand, if_true_instructions) = self.visit_expression(&input.if_true);
        let (if_false_operand, if_false_instructions) = self.visit_expression(&input.if_false);

        let destination_register = format!("r{}", self.next_register);
        let ternary_instruction = format!(
            "    ternary {condition_operand} {if_true_operand} {if_false_operand} into {destination_register};\n",
        );

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = condition_instructions;
        instructions.push_str(&if_true_instructions);
        instructions.push_str(&if_false_instructions);
        instructions.push_str(&ternary_instruction);

        (destination_register, instructions)
    }

    fn visit_struct_init(&mut self, input: &'a StructExpression) -> (String, String) {
        // Lookup struct or record.
        let name = if let Some((is_record, type_)) = self.composite_mapping.get(&input.name.name) {
            if *is_record {
                // record.private;
                format!("{}.{type_}", input.name)
            } else {
                // foo; // no visibility for structs
                input.name.to_string()
            }
        } else {
            unreachable!("All composite types should be known at this phase of compilation")
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
                let (ident_operand, ident_instructions) = self.visit_identifier(&member.identifier);
                instructions.push_str(&ident_instructions);

                ident_operand
            };

            // Push operand name to struct init instruction.
            write!(struct_init_instruction, "{operand} ").expect("failed to write to string");
        }

        // Push destination register to struct init instruction.
        let destination_register = format!("r{}", self.next_register);
        writeln!(struct_init_instruction, "into {destination_register} as {name};",)
            .expect("failed to write to string");

        instructions.push_str(&struct_init_instruction);

        // Increment the register counter.
        self.next_register += 1;

        (destination_register, instructions)
    }

    fn visit_array_access(&mut self, input: &'a ArrayAccess) -> (String, String) {
        let (array_operand, _) = self.visit_expression(&input.array);
        let index_operand = match input.index.as_ref() {
            Expression::Literal(Literal::Integer(_, string, _, _)) => format!("{}u32", string),
            _ => unreachable!("Array indices must be integer literals"),
        };
        let array_access = format!("{}[{}]", array_operand, index_operand);

        (array_access, String::new())
    }

    fn visit_member_access(&mut self, input: &'a MemberAccess) -> (String, String) {
        let (inner_struct, _) = self.visit_expression(&input.inner);
        let member_access = format!("{inner_struct}.{}", input.name);

        (member_access, String::new())
    }

    // group::GEN -> group::GEN
    fn visit_associated_constant(&mut self, input: &'a AssociatedConstant) -> (String, String) {
        (format!("{input}"), String::new())
    }

    // Pedersen64::hash() -> hash.ped64
    fn visit_associated_function(&mut self, input: &'a AssociatedFunction) -> (String, String) {
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

        // Helper function to get a destination register for a function call.
        let mut get_destination_register = || {
            let destination_register = format!("r{}", self.next_register);
            self.next_register += 1;
            destination_register
        };

        // Helper function to construct the instruction associated with a simple function call.
        // This assumes that the function call has one output.
        let mut construct_simple_function_call = |function: &Identifier, variant: &str, arguments: Vec<String>| {
            // Split function into [opcode, return type] e.g. hash_to_field -> [hash, field]
            let function_name = function.name.to_string();
            let mut names = function_name.split("_to_");
            let opcode = names.next().expect("failed to get opcode");
            let return_type = names.next().expect("failed to get type");

            let mut instruction = format!("    {opcode}.{variant}");
            for argument in arguments {
                write!(instruction, " {argument}").expect("failed to write to string");
            }
            let destination_register = get_destination_register();
            writeln!(instruction, " into {destination_register} as {return_type};").expect("failed to write to string");
            (destination_register, instruction)
        };

        // Construct the instruction.
        let (destination, instruction) = match &input.ty {
            Type::Identifier(Identifier { name: sym::BHP256, .. }) => {
                construct_simple_function_call(&input.name, "bhp256", arguments)
            }
            Type::Identifier(Identifier { name: sym::BHP512, .. }) => {
                construct_simple_function_call(&input.name, "bhp512", arguments)
            }
            Type::Identifier(Identifier { name: sym::BHP768, .. }) => {
                construct_simple_function_call(&input.name, "bhp768", arguments)
            }
            Type::Identifier(Identifier { name: sym::BHP1024, .. }) => {
                construct_simple_function_call(&input.name, "bhp1024", arguments)
            }
            Type::Identifier(Identifier { name: sym::Keccak256, .. }) => {
                construct_simple_function_call(&input.name, "keccak256", arguments)
            }
            Type::Identifier(Identifier { name: sym::Keccak384, .. }) => {
                construct_simple_function_call(&input.name, "keccak384", arguments)
            }
            Type::Identifier(Identifier { name: sym::Keccak512, .. }) => {
                construct_simple_function_call(&input.name, "keccak512", arguments)
            }
            Type::Identifier(Identifier { name: sym::Pedersen64, .. }) => {
                construct_simple_function_call(&input.name, "ped64", arguments)
            }
            Type::Identifier(Identifier { name: sym::Pedersen128, .. }) => {
                construct_simple_function_call(&input.name, "ped128", arguments)
            }
            Type::Identifier(Identifier { name: sym::Poseidon2, .. }) => {
                construct_simple_function_call(&input.name, "psd2", arguments)
            }
            Type::Identifier(Identifier { name: sym::Poseidon4, .. }) => {
                construct_simple_function_call(&input.name, "psd4", arguments)
            }
            Type::Identifier(Identifier { name: sym::Poseidon8, .. }) => {
                construct_simple_function_call(&input.name, "psd8", arguments)
            }
            Type::Identifier(Identifier { name: sym::SHA3_256, .. }) => {
                construct_simple_function_call(&input.name, "sha3_256", arguments)
            }
            Type::Identifier(Identifier { name: sym::SHA3_384, .. }) => {
                construct_simple_function_call(&input.name, "sha3_384", arguments)
            }
            Type::Identifier(Identifier { name: sym::SHA3_512, .. }) => {
                construct_simple_function_call(&input.name, "sha3_512", arguments)
            }
            Type::Identifier(Identifier { name: sym::Mapping, .. }) => match input.name.name {
                sym::get => {
                    let mut instruction = "    get".to_string();
                    let destination_register = get_destination_register();
                    // Write the mapping name and the key.
                    writeln!(instruction, " {}[{}] into {destination_register};", arguments[0], arguments[1])
                        .expect("failed to write to string");
                    (destination_register, instruction)
                }
                sym::get_or_use => {
                    let mut instruction = "    get.or_use".to_string();
                    let destination_register = get_destination_register();
                    // Write the mapping name, the key, and the default value.
                    writeln!(
                        instruction,
                        " {}[{}] {} into {destination_register};",
                        arguments[0], arguments[1], arguments[2]
                    )
                    .expect("failed to write to string");
                    (destination_register, instruction)
                }
                sym::set => {
                    let mut instruction = "    set".to_string();
                    // Write the value, mapping name, and the key.
                    writeln!(instruction, " {} into {}[{}];", arguments[2], arguments[0], arguments[1])
                        .expect("failed to write to string");
                    (String::new(), instruction)
                }
                sym::remove => {
                    let mut instruction = "    remove".to_string();
                    // Write the mapping name and the key.
                    writeln!(instruction, " {}[{}];", arguments[0], arguments[1]).expect("failed to write to string");
                    (String::new(), instruction)
                }
                sym::contains => {
                    let mut instruction = "    contains".to_string();
                    let destination_register = get_destination_register();
                    // Write the mapping name and the key.
                    writeln!(instruction, " {}[{}] into {destination_register};", arguments[0], arguments[1])
                        .expect("failed to write to string");
                    (destination_register, instruction)
                }
                _ => unreachable!("The only variants of Mapping are get, get_or, and set"),
            },
            Type::Identifier(Identifier { name: sym::group, .. }) => {
                match input.name {
                    Identifier { name: sym::to_x_coordinate, .. } => {
                        let mut instruction = "    cast".to_string();
                        let destination_register = get_destination_register();
                        // Write the argument and the destination register.
                        writeln!(instruction, " {} into {destination_register} as group.x;", arguments[0],)
                            .expect("failed to write to string");
                        (destination_register, instruction)
                    }
                    Identifier { name: sym::to_y_coordinate, .. } => {
                        let mut instruction = "    cast".to_string();
                        let destination_register = get_destination_register();
                        // Write the argument and the destination register.
                        writeln!(instruction, " {} into {destination_register} as group.y;", arguments[0],)
                            .expect("failed to write to string");
                        (destination_register, instruction)
                    }
                    _ => unreachable!("The only associated methods of group are to_x_coordinate and to_y_coordinate"),
                }
            }
            Type::Identifier(Identifier { name: sym::ChaCha, .. }) => {
                // Get the destination register.
                let destination_register = get_destination_register();
                // Construct the instruction template.
                let mut instruction = format!("    rand.chacha into {destination_register} as ");
                // Write the return type.
                match input.name {
                    Identifier { name: sym::rand_address, .. } => writeln!(instruction, "address;"),
                    Identifier { name: sym::rand_bool, .. } => writeln!(instruction, "boolean;"),
                    Identifier { name: sym::rand_field, .. } => writeln!(instruction, "field;"),
                    Identifier { name: sym::rand_group, .. } => writeln!(instruction, "group;"),
                    Identifier { name: sym::rand_i8, .. } => writeln!(instruction, "i8;"),
                    Identifier { name: sym::rand_i16, .. } => writeln!(instruction, "i16;"),
                    Identifier { name: sym::rand_i32, .. } => writeln!(instruction, "i32;"),
                    Identifier { name: sym::rand_i64, .. } => writeln!(instruction, "i64;"),
                    Identifier { name: sym::rand_i128, .. } => writeln!(instruction, "i128;"),
                    Identifier { name: sym::rand_scalar, .. } => writeln!(instruction, "scalar;"),
                    Identifier { name: sym::rand_u8, .. } => writeln!(instruction, "u8;"),
                    Identifier { name: sym::rand_u16, .. } => writeln!(instruction, "u16;"),
                    Identifier { name: sym::rand_u32, .. } => writeln!(instruction, "u32;"),
                    Identifier { name: sym::rand_u64, .. } => writeln!(instruction, "u64;"),
                    Identifier { name: sym::rand_u128, .. } => writeln!(instruction, "u128;"),
                    _ => unreachable!("The only associated methods of ChaCha are `rand_*`"),
                }
                .expect("failed to write to string");
                (destination_register, instruction)
            }
            Type::Identifier(Identifier { name: sym::signature, .. }) => {
                let mut instruction = "    sign.verify".to_string();
                let destination_register = get_destination_register();
                // Write the arguments and the destination register.
                writeln!(
                    instruction,
                    " {} {} {} into {destination_register};",
                    arguments[0], arguments[1], arguments[2]
                )
                .expect("failed to write to string");
                (destination_register, instruction)
            }
            _ => unreachable!("All core functions should be known at this phase of compilation"),
        };
        // Add the instruction to the list of instructions.
        instructions.push_str(&instruction);

        (destination, instructions)
    }

    fn visit_access(&mut self, input: &'a AccessExpression) -> (String, String) {
        match input {
            AccessExpression::Array(array) => self.visit_array_access(array),
            AccessExpression::Member(access) => self.visit_member_access(access),
            AccessExpression::AssociatedConstant(constant) => self.visit_associated_constant(constant),
            AccessExpression::AssociatedFunction(function) => self.visit_associated_function(function),
            AccessExpression::Tuple(_) => {
                unreachable!("Tuple access should not be in the AST at this phase of compilation.")
            }
        }
    }

    // TODO: Cleanup
    fn visit_call(&mut self, input: &'a CallExpression) -> (String, String) {
        let (mut call_instruction, has_finalize) = match &input.external {
            Some(external) => {
                // If the function is an external call, then check whether or not it has an associated finalize block.
                // Extract the program name from the external call.
                let program_name = match **external {
                    Expression::Identifier(identifier) => identifier.name,
                    _ => unreachable!("Parsing guarantees that a program name is always an identifier."),
                };
                // Lookup the imported program scope.
                let imported_program_scope = match self
                    .program
                    .imports
                    .get(&program_name)
                    .and_then(|(program, _)| program.program_scopes.get(&program_name))
                {
                    Some(program) => program,
                    None => unreachable!("Type checking guarantees that imported programs are well defined."),
                };
                // Check if the external function has a finalize block.
                let function_name = match *input.function {
                    Expression::Identifier(identifier) => identifier.name,
                    _ => unreachable!("Parsing guarantees that a function name is always an identifier."),
                };
                let has_finalize = match imported_program_scope.functions.iter().find(|(sym, _)| *sym == function_name)
                {
                    Some((_, function)) => function.finalize.is_some(),
                    None => unreachable!("Type checking guarantees that imported functions are well defined."),
                };
                (format!("    call {external}.aleo/{}", input.function), has_finalize)
            }
            None => (format!("    call {}", input.function), false),
        };
        let mut instructions = String::new();

        for argument in input.arguments.iter() {
            let (argument, argument_instructions) = self.visit_expression(argument);
            write!(call_instruction, " {argument}").expect("failed to write to string");
            instructions.push_str(&argument_instructions);
        }

        // Lookup the function return type.
        let function_name = match input.function.borrow() {
            Expression::Identifier(identifier) => identifier.name,
            _ => unreachable!("Parsing guarantees that a function name is always an identifier."),
        };

        // Initialize storage for the destination registers.
        let mut destinations = Vec::new();

        let return_type = &self.symbol_table.lookup_fn_symbol(function_name).unwrap().output_type;
        match return_type {
            Type::Unit => {} // Do nothing
            Type::Tuple(tuple) => match tuple.length() {
                0 | 1 => unreachable!("Parsing guarantees that a tuple type has at least two elements"),
                len => {
                    for _ in 0..len {
                        let destination_register = format!("r{}", self.next_register);
                        destinations.push(destination_register);
                        self.next_register += 1;
                    }
                }
            },
            _ => {
                let destination_register = format!("r{}", self.next_register);
                destinations.push(destination_register);
                self.next_register += 1;
            }
        }

        // Construct the output operands. These are the destination registers **without** the future.
        let output_operands = destinations.join(" ");

        // If `has_finalize`, create another destination register for the future.
        if has_finalize {
            // Construct the future register.
            let future_register = format!("r{}", self.next_register);
            self.next_register += 1;

            // Construct the future type.
            let program_id = match input.external.as_deref() {
                Some(Expression::Identifier(identifier)) => identifier,
                _ => unreachable!("If `has_finalize` is true, then the external call must be an identifier."),
            };

            // Add the futures register to the list of futures.
            self.futures.push((future_register.clone(), format!("{program_id}.aleo/{function_name}")));

            // Add the future register to the list of destinations.
            destinations.push(future_register);
        }

        // If destination registers were created, write them to the call instruction.
        if !destinations.is_empty() {
            write!(call_instruction, " into").expect("failed to write to string");
            for destination in &destinations {
                write!(call_instruction, " {}", destination).expect("failed to write to string");
            }
        }

        // Write the closing semicolon.
        writeln!(call_instruction, ";").expect("failed to write to string");

        // Push the call instruction to the list of instructions.
        instructions.push_str(&call_instruction);

        // Return the output operands and the instructions.
        (output_operands, instructions)
    }

    fn visit_tuple(&mut self, input: &'a TupleExpression) -> (String, String) {
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

    fn visit_unit(&mut self, _input: &'a UnitExpression) -> (String, String) {
        unreachable!("`UnitExpression`s should not be visited during code generation.")
    }
}
