// Copyright (C) 2019-2022 Aleo Systems Inc.
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
    AccessExpression, AssociatedFunction, BinaryExpression, BinaryOperation, CallExpression, CircuitExpression,
    ErrExpression, Expression, Identifier, Literal, MemberAccess, TernaryExpression, TupleExpression, Type,
    UnaryExpression, UnaryOperation,
};
use leo_span::sym;

use std::fmt::Write as _;

/// Implement the necessary methods to visit nodes in the AST.
// Note: We opt for this option instead of using `Visitor` and `Director` because this pass requires
// a post-order traversal of the AST. This is sufficient since this implementation is intended to be
// a prototype. The production implementation will require a redesign of `Director`.
impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_expression(&mut self, input: &'a Expression) -> (String, String) {
        match input {
            Expression::Access(expr) => self.visit_access(expr),
            Expression::Binary(expr) => self.visit_binary(expr),
            Expression::Call(expr) => self.visit_call(expr),
            Expression::Circuit(expr) => self.visit_circuit_init(expr),
            Expression::Err(expr) => self.visit_err(expr),
            Expression::Identifier(expr) => self.visit_identifier(expr),
            Expression::Literal(expr) => self.visit_value(expr),
            Expression::Ternary(expr) => self.visit_ternary(expr),
            Expression::Tuple(expr) => self.visit_tuple(expr),
            Expression::Unary(expr) => self.visit_unary(expr),
        }
    }

    fn visit_identifier(&mut self, input: &'a Identifier) -> (String, String) {
        (self.variable_mapping.get(&input.name).unwrap().clone(), String::new())
    }

    fn visit_value(&mut self, input: &'a Literal) -> (String, String) {
        (format!("{}", input), String::new())
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
        let binary_instruction = format!(
            "    {} {} {} into {};\n",
            opcode, left_operand, right_operand, destination_register
        );

        // Increment the register counter.
        self.next_register += 1;

        // Concatenate the instructions.
        let mut instructions = left_instructions;
        instructions.push_str(&right_instructions);
        instructions.push_str(&binary_instruction);

        (destination_register, instructions)
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression) -> (String, String) {
        let (expression_operand, expression_instructions) = self.visit_expression(&input.receiver);

        let opcode = match input.op {
            UnaryOperation::Abs => String::from("abs"),
            UnaryOperation::AbsWrapped => String::from("abs.w"),
            UnaryOperation::Double => String::from("double"),
            UnaryOperation::Inverse => String::from("inv"),
            UnaryOperation::Not => String::from("not"),
            UnaryOperation::Negate => String::from("neg"),
            UnaryOperation::Square => String::from("square"),
            UnaryOperation::SquareRoot => String::from("sqrt"),
        };

        let destination_register = format!("r{}", self.next_register);
        let unary_instruction = format!("    {} {} into {};\n", opcode, expression_operand, destination_register);

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
            "    ternary {} {} {} into {};\n",
            condition_operand, if_true_operand, if_false_operand, destination_register
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

    fn visit_circuit_init(&mut self, input: &'a CircuitExpression) -> (String, String) {
        // Lookup circuit or record.
        let name = if let Some((is_record, type_)) = self.composite_mapping.get(&input.name.name) {
            let name = input.name.to_string().to_lowercase();
            if *is_record {
                // record.private;
                format!("{}.{}", name, type_)
            } else {
                // foo; // no visibility for interfaces
                name
            }
        } else {
            unreachable!("All composite types should be known at this phase of compilation")
        };

        // Initialize instruction builder strings.
        let mut instructions = String::new();
        let mut circuit_init_instruction = String::from("    cast ");

        // Visit each circuit member and accumulate instructions from expressions.
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

            // Push operand name to circuit init instruction.
            write!(circuit_init_instruction, "{} ", operand).expect("failed to write to string");
        }

        // Push destination register to circuit init instruction.
        let destination_register = format!("r{}", self.next_register);
        writeln!(
            circuit_init_instruction,
            "into {dest} as {name};",
            dest = destination_register,
            name = name,
        )
        .expect("failed to write to string");

        instructions.push_str(&circuit_init_instruction);

        // Increment the register counter.
        self.next_register += 1;

        (destination_register, instructions)
    }

    fn visit_member_access(&mut self, input: &'a MemberAccess) -> (String, String) {
        let (inner_circuit, _inner_instructions) = self.visit_expression(&input.inner);
        let member_access_instruction = format!("{}.{}", inner_circuit, input.name);

        (member_access_instruction, String::new())
    }

    // Pedersen64::hash() -> hash.ped64
    fn visit_associated_function(&mut self, input: &'a AssociatedFunction) -> (String, String) {
        // Write identifier as opcode. `Pedersen64` -> `ped64`.
        let symbol: &str = if let Type::Identifier(identifier) = input.ty {
            match identifier.name {
                sym::BHP256 => "bhp256",
                sym::BHP512 => "bhp512",
                sym::BHP768 => "bhp768",
                sym::BHP1024 => "bhp1024",
                sym::Pedersen64 => "ped64",
                sym::Pedersen128 => "ped128",
                sym::Poseidon2 => "psd2",
                sym::Poseidon4 => "psd4",
                sym::Poseidon8 => "psd8",
                _ => unreachable!("All core circuit function calls should be known at this time."),
            }
        } else {
            unreachable!("All core circuits should be known at this time.")
        };

        // Construct associated function call.
        let mut associated_function_call = format!("    {}.{} ", input.name, symbol);
        let mut instructions = String::new();

        // Visit each function argument and accumulate instructions from expressions.
        for arg in input.args.iter() {
            let (arg_string, arg_instructions) = self.visit_expression(arg);
            write!(associated_function_call, "{} ", arg_string).expect("failed to write associated function argument");
            instructions.push_str(&arg_instructions);
        }

        // Push destination register to associated function call instruction.
        let destination_register = format!("r{}", self.next_register);
        writeln!(associated_function_call, "into {};", destination_register)
            .expect("failed to write dest register for associated function");
        instructions.push_str(&associated_function_call);

        // Increment the register counter.
        self.next_register += 1;

        (destination_register, instructions)
    }

    fn visit_access(&mut self, input: &'a AccessExpression) -> (String, String) {
        match input {
            AccessExpression::Member(access) => self.visit_member_access(access),
            AccessExpression::AssociatedConstant(_) => todo!(), // Associated constants are not supported in AVM yet.
            AccessExpression::AssociatedFunction(function) => self.visit_associated_function(function),
            AccessExpression::Tuple(_) => todo!(), // Tuples are not supported in AVM yet.
        }
    }

    fn visit_call(&mut self, input: &'a CallExpression) -> (String, String) {
        let mut call_instruction = format!("    call {} ", input.function);
        let mut instructions = String::new();

        for argument in input.arguments.iter() {
            let (argument, argument_instructions) = self.visit_expression(argument);
            write!(call_instruction, "{} ", argument).expect("failed to write to string");
            instructions.push_str(&argument_instructions);
        }

        // Push destination register to call instruction.
        let destination_register = format!("r{}", self.next_register);
        writeln!(call_instruction, "into {};", destination_register).expect("failed to write to string");
        instructions.push_str(&call_instruction);

        // Increment the register counter.
        self.next_register += 1;

        (destination_register, instructions)
    }

    fn visit_tuple(&mut self, input: &'a TupleExpression) -> (String, String) {
        // Need to return a single string here so we will join the tuple elements with '\n'
        // and split them after this method is called.
        let mut tuple_elements = String::new();
        let mut instructions = String::new();

        // Visit each tuple element and accumulate instructions from expressions.
        for element in input.elements.iter() {
            let (element, element_instructions) = self.visit_expression(element);
            writeln!(tuple_elements, "{}", element).expect("failed to write tuple to string");
            instructions.push_str(&element_instructions);
        }

        // CAUTION: does not return the destination_register.
        (tuple_elements, instructions)
    }

    fn visit_err(&mut self, _input: &'a ErrExpression) -> (String, String) {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }
}
