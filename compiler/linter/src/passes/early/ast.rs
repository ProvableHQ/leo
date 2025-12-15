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

use leo_parser_lossless::{ExpressionKind, StatementKind, SyntaxKind, SyntaxNode, TypeKind};

use crate::{
    check_node,
    passes::early::{EarlyLintingVisitor, match_expression, match_statement, match_type},
};

impl EarlyLintingVisitor<'_> {
    pub(super) fn visit_expression(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::Expression(..));
        let SyntaxKind::Expression(kind) = node.kind else { panic!("Can't happen") };
        match kind {
            ExpressionKind::ArrayAccess => {
                let [array, _left, index, _right] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(array);
                self.visit_expression(index);
            }

            ExpressionKind::AssociatedFunctionCall => {
                self.visit_nodes(node, match_expression, Self::visit_expression);
            }

            ExpressionKind::Async => {
                let [_a, block] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_statement(block);
            }

            ExpressionKind::Array => self.visit_nodes(node, match_expression, Self::visit_expression),

            ExpressionKind::Binary => {
                let [lhs, _op, rhs] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(lhs);
                self.visit_expression(rhs);
            }

            ExpressionKind::Call => {
                if let Some(argument_list) =
                    node.children.iter().find(|child| matches!(child.kind, SyntaxKind::ConstArgumentList))
                {
                    self.visit_const_list(argument_list);
                }

                self.visit_nodes(node, match_expression, Self::visit_expression);
            }

            ExpressionKind::Cast => {
                let [expression, _as, type_] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(expression);
                self.visit_type(type_);
            }

            ExpressionKind::MemberAccess => {
                let [struct_, _dot, _name] = &node.children[..] else {
                    panic!("Can't happen.");
                };

                self.visit_expression(struct_);
            }

            ExpressionKind::MethodCall => {
                let [expr, _dot, _name, ..] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(expr);
                node.children[3..]
                    .iter()
                    .filter(|child| matches!(child.kind, SyntaxKind::Expression(..)))
                    .for_each(|ch| self.visit_expression(ch));
            }

            ExpressionKind::Parenthesized => {
                let [_left, expr, _right] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(expr);
            }

            ExpressionKind::Repeat => {
                let [_left, expr, _s, count, _right] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(expr);
                self.visit_expression(count);
            }

            ExpressionKind::Struct => {
                self.visit_nodes(
                    node,
                    |n| matches!(n.kind, SyntaxKind::StructMemberInitializer),
                    |slf, node| match &node.children[..] {
                        [_] => {}
                        [_, _, expr] => slf.visit_expression(expr),
                        _ => panic!("Can't happen"),
                    },
                );

                let maybe_const_params = &node.children[1];
                if maybe_const_params.kind == SyntaxKind::ConstArgumentList {
                    self.visit_nodes(maybe_const_params, match_expression, Self::visit_expression);
                }
            }

            ExpressionKind::Ternary => {
                let [cond, _q, if_, _c, then] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(cond);
                self.visit_expression(if_);
                self.visit_expression(then);
            }

            ExpressionKind::Tuple => self.visit_nodes(node, match_expression, Self::visit_expression),

            ExpressionKind::TupleAccess => {
                let [expr, _dot, _integer] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(expr);
            }

            ExpressionKind::Unary => {
                let [_op, operand] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(operand);
            }

            _ => {}
        }
    }

    fn visit_statement(&mut self, node: &SyntaxNode) {
        self.check_node(node, match_statement);
        let SyntaxKind::Statement(statement_kind) = node.kind else { panic!() };
        match statement_kind {
            StatementKind::Assert => {
                let [_a, _left, expr, _right, _s] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(expr);
            }

            StatementKind::AssertEq | StatementKind::AssertNeq => {
                let [_a, _left, e0, _c, e1, _right, _s] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(e0);
                self.visit_expression(e1);
            }

            StatementKind::Assign => {
                let [lhs, _a, rhs, _s] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_expression(lhs);
                self.visit_expression(rhs);
            }

            StatementKind::Block => self.visit_block(node),

            StatementKind::Conditional => match &node.children[..] {
                [_if, c, block] => {
                    self.visit_expression(c);
                    self.visit_block(block);
                }

                [_if, c, block, _else, otherwise] => {
                    self.visit_expression(c);
                    self.visit_block(block);
                    self.visit_statement(otherwise);
                }

                _ => panic!("Can't happen"),
            },

            StatementKind::Const => {
                let [_const, _name, _c, type_, _a, rhs, _s] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_type(type_);
                self.visit_expression(rhs);
            }

            StatementKind::Definition => match &node.children[..] {
                [_let, _name, _c, type_, _assign, e, _s] => {
                    self.visit_expression(e);
                    self.visit_type(type_);
                }
                [_let, _name, _assign, e, _s] => {
                    self.visit_expression(e);
                }
                children => {
                    self.visit_nodes(node, match_type, Self::visit_type);

                    let expr = &children[children.len() - 2];
                    self.visit_expression(expr);
                }
            },

            StatementKind::Expression => self.visit_expression(&node.children[0]),

            StatementKind::Iteration => match &node.children[..] {
                [_f, _i, _n, low, _d, hi, block] => {
                    self.visit_expression(low);
                    self.visit_expression(hi);
                    self.visit_block(block);
                }
                [_f, _i, _c, type_, _n, low, _d, hi, block] => {
                    self.visit_type(type_);
                    self.visit_expression(low);
                    self.visit_expression(hi);
                    self.visit_block(block);
                }
                _ => panic!("Can't happen"),
            },

            StatementKind::Return => match &node.children[..] {
                [_r, e, _s] => {
                    self.visit_expression(e);
                }
                [_r, _s] => {}
                _ => panic!("Can't happen"),
            },
        }
    }

    pub(super) fn visit_block(&mut self, node: &SyntaxNode) {
        self.check_node(node, |n| n.kind == SyntaxKind::Statement(StatementKind::Block));
        self.visit_nodes(node, match_statement, Self::visit_statement);
    }

    pub(super) fn visit_type(&mut self, node: &SyntaxNode) {
        self.check_node(node, match_type);
        let SyntaxKind::Type(type_kind) = node.kind else { panic!("Can't happen") };
        match type_kind {
            TypeKind::Array => {
                let [_l, type_, _s, length, _r] = &node.children[..] else {
                    panic!("Can't happen");
                };

                self.visit_type(type_);
                self.visit_expression(length);
            }
            TypeKind::Composite => {
                let name = &node.children[0];
                if name.text.split_once(".aleo/").is_none()
                    && let Some(arg_list) = node.children.get(1)
                {
                    self.visit_nodes(arg_list, match_expression, Self::visit_expression);
                }
            }
            TypeKind::Future => {
                if node.children.len() != 1 {
                    self.visit_nodes(node, match_type, Self::visit_type);
                }
            }
            TypeKind::Tuple => {
                self.visit_nodes(node, match_type, Self::visit_type);
            }
            _ => {}
        }
    }
}
