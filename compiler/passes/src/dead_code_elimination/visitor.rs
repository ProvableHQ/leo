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

use crate::CompilerState;

use leo_ast::{BinaryOperation, Expression, Node as _, Type, UnaryOperation};
use leo_span::{Symbol, sym};

use indexmap::IndexSet;

pub struct DeadCodeEliminatingVisitor<'a> {
    pub state: &'a mut CompilerState,

    /// The set of used variables in the current function body.
    pub used_variables: IndexSet<Symbol>,

    /// The name of the program currently being processed.
    pub program_name: Symbol,

    /// How many statements were in the AST before DCE?
    pub statements_before: u32,

    /// How many statements were in the AST after DCE?
    pub statements_after: u32,
}

impl DeadCodeEliminatingVisitor<'_> {
    pub fn side_effect_free(&self, expr: &Expression) -> bool {
        use Expression::*;

        let sef = |expr| self.side_effect_free(expr);

        match expr {
            ArrayAccess(array) => sef(&array.array) && sef(&array.index),
            MemberAccess(mem) => sef(&mem.inner),
            Repeat(repeat) => sef(&repeat.expr) && sef(&repeat.count),
            TupleAccess(tuple) => sef(&tuple.tuple),
            Array(array) => array.elements.iter().all(sef),
            AssociatedConstant(_) => true,
            AssociatedFunction(func) => {
                // CheatCode, Mapping, and Future operations obviously have side effects.
                // Pedersen64 and Pedersen128 operations can fail for large inputs.
                func.arguments.iter().all(sef)
                    && !matches!(
                        func.variant.name,
                        sym::CheatCode | sym::Mapping | sym::Future | sym::Pedersen64 | sym::Pedersen128
                    )
            }
            Async(_) => false,
            Binary(bin) => {
                use BinaryOperation::*;
                let halting_op = match bin.op {
                    // These can halt for any of their operand types.
                    Div | Mod | Rem | Shl | Shr => true,
                    // These can only halt for integers.
                    Add | Mul | Pow => {
                        matches!(
                            self.state.type_table.get(&expr.id()).expect("Types should be assigned."),
                            Type::Integer(..)
                        )
                    }
                    _ => false,
                };
                !halting_op && sef(&bin.left) && sef(&bin.right)
            }
            Call(..) => {
                // Since calls may halt, be conservative and don't consider any call side effect free.
                false
            }
            Cast(..) => {
                // At least for now, be conservative and don't consider any cast side effect free.
                // Of course for some combinations of types, casts will never halt.
                false
            }
            Struct(struct_) => struct_.members.iter().all(|mem| mem.expression.as_ref().is_none_or(sef)),
            Ternary(tern) => [&tern.condition, &tern.if_true, &tern.if_false].into_iter().all(sef),
            Tuple(tuple) => tuple.elements.iter().all(sef),
            Unary(un) => {
                use UnaryOperation::*;
                let halting_op = match un.op {
                    // These can halt for any of their operand types.
                    Abs | Inverse | SquareRoot => true,
                    // Negate can only halt for integers.
                    Negate => {
                        matches!(
                            self.state.type_table.get(&expr.id()).expect("Type should be assigned."),
                            Type::Integer(..)
                        )
                    }
                    _ => false,
                };
                !halting_op && sef(&un.receiver)
            }
            Err(_) => false,
            Identifier(_) | Literal(_) | Locator(_) | Unit(_) => true,
        }
    }
}
