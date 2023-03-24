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

use crate::*;

/// A Consumer trait for instructions in the AST.
pub trait InstructionConsumer {
    type Output;

    fn consume_instruction(&mut self, input: Instruction) -> Self::Output {
        match input {
            Instruction::Abs(inst) => self.consume_unary_instruction(inst),
            Instruction::AbsWrapped(inst) => self.consume_unary_instruction(inst),
            Instruction::Add(inst) => self.consume_binary_instruction(inst),
            Instruction::AddWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::And(inst) => self.consume_binary_instruction(inst),
            Instruction::AssertEq(inst) => self.consume_binary_instruction(inst),
            Instruction::AssertNeq(inst) => self.consume_binary_instruction(inst),
            Instruction::Call(inst) => self.consume_call_instruction(inst),
            Instruction::Cast(inst) => self.consume_cast_instruction(inst),
            Instruction::CommitBHP256(inst) => self.consume_binary_instruction(inst),
            Instruction::CommitBHP512(inst) => self.consume_binary_instruction(inst),
            Instruction::CommitBHP768(inst) => self.consume_binary_instruction(inst),
            Instruction::CommitBHP1024(inst) => self.consume_binary_instruction(inst),
            Instruction::CommitPED64(inst) => self.consume_binary_instruction(inst),
            Instruction::CommitPED128(inst) => self.consume_binary_instruction(inst),
            Instruction::Decrement(inst) => self.consume_decrement_instruction(inst),
            Instruction::Div(inst) => self.consume_binary_instruction(inst),
            Instruction::DivWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Double(inst) => self.consume_unary_instruction(inst),
            Instruction::GreaterThan(inst) => self.consume_binary_instruction(inst),
            Instruction::GreaterThanOrEqual(inst) => self.consume_binary_instruction(inst),
            Instruction::HashBHP256(inst) => self.consume_unary_instruction(inst),
            Instruction::HashBHP512(inst) => self.consume_unary_instruction(inst),
            Instruction::HashBHP768(inst) => self.consume_unary_instruction(inst),
            Instruction::HashBHP1024(inst) => self.consume_unary_instruction(inst),
            Instruction::HashPED64(inst) => self.consume_unary_instruction(inst),
            Instruction::HashPED128(inst) => self.consume_unary_instruction(inst),
            Instruction::HashPSD2(inst) => self.consume_unary_instruction(inst),
            Instruction::HashPSD4(inst) => self.consume_unary_instruction(inst),
            Instruction::HashPSD8(inst) => self.consume_unary_instruction(inst),
            Instruction::Increment(inst) => self.consume_increment_instruction(inst),
            Instruction::Inv(inst) => self.consume_unary_instruction(inst),
            Instruction::IsEq(inst) => self.consume_binary_instruction(inst),
            Instruction::IsNeq(inst) => self.consume_binary_instruction(inst),
            Instruction::LessThan(inst) => self.consume_binary_instruction(inst),
            Instruction::LessThanOrEqual(inst) => self.consume_binary_instruction(inst),
            Instruction::Modulo(inst) => self.consume_binary_instruction(inst),
            Instruction::Mul(inst) => self.consume_binary_instruction(inst),
            Instruction::MulWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Nand(inst) => self.consume_binary_instruction(inst),
            Instruction::Neg(inst) => self.consume_unary_instruction(inst),
            Instruction::Nor(inst) => self.consume_binary_instruction(inst),
            Instruction::Not(inst) => self.consume_unary_instruction(inst),
            Instruction::Or(inst) => self.consume_binary_instruction(inst),
            Instruction::Pow(inst) => self.consume_binary_instruction(inst),
            Instruction::PowWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Rem(inst) => self.consume_binary_instruction(inst),
            Instruction::RemWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Shl(inst) => self.consume_binary_instruction(inst),
            Instruction::ShlWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Shr(inst) => self.consume_binary_instruction(inst),
            Instruction::ShrWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Square(inst) => self.consume_unary_instruction(inst),
            Instruction::SquareRoot(inst) => self.consume_unary_instruction(inst),
            Instruction::Sub(inst) => self.consume_binary_instruction(inst),
            Instruction::SubWrapped(inst) => self.consume_binary_instruction(inst),
            Instruction::Ternary(inst) => self.consume_ternary_instruction(inst),
            Instruction::Xor(inst) => self.consume_binary_instruction(inst),
        }
    }

    fn consume_unary_instruction(&mut self, _: impl Unary) -> Self::Output;

    fn consume_binary_instruction(&mut self, _: impl Binary) -> Self::Output;

    fn consume_ternary_instruction(&mut self, _: Ternary) -> Self::Output;

    fn consume_call_instruction(&mut self, _: Call) -> Self::Output;

    fn consume_cast_instruction(&mut self, _: Cast) -> Self::Output;

    fn consume_increment_instruction(&mut self, _: Increment) -> Self::Output;

    fn consume_decrement_instruction(&mut self, _: Decrement) -> Self::Output;
}
