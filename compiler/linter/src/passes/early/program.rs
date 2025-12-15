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

use leo_parser_lossless::{StatementKind, SyntaxKind, SyntaxNode};

use crate::{
    check_node,
    passes::early::{EarlyLintingVisitor, match_expression, match_kind},
};

impl EarlyLintingVisitor<'_> {
    pub(super) fn visit_module(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::ModuleContents);
        self.visit_nodes(
            node,
            |_| true,
            |slf, node| match node.kind {
                SyntaxKind::GlobalConst => slf.visit_const(node),
                SyntaxKind::Function => slf.visit_function(node),
                SyntaxKind::StructDeclaration => slf.visit_composite(node),
                _ => {}
            },
        );
    }

    pub(super) fn visit_main(&mut self, node: &SyntaxNode) {
        self.check_node(node, |n| n.kind == SyntaxKind::MainContents);
        node.children
            .iter()
            .filter(|child| matches!(child.kind, SyntaxKind::Import))
            .for_each(|import| self.visit_import(import));
        let program_node = node.children.last().unwrap();
        self.visit_program(program_node);
    }

    fn visit_program(&mut self, node: &SyntaxNode) {
        self.check_node(node, match_kind(SyntaxKind::ProgramDeclaration));
        self.visit_nodes(
            node,
            |_| true,
            |slf, node| match node.kind {
                SyntaxKind::GlobalConst => slf.visit_const(node),
                SyntaxKind::Function => slf.visit_function(node),
                SyntaxKind::StructDeclaration => slf.visit_composite(node),
                SyntaxKind::Constructor => slf.visit_constructor(node),
                SyntaxKind::Mapping => slf.visit_mapping(node),
                _ => {}
            },
        );
    }

    fn visit_mapping(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::Mapping);
        let [_mapping, _name, _colon, key_type, _arrow, value_type, _s] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.visit_type(key_type);
        self.visit_type(value_type);
    }

    fn visit_constructor(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::Constructor);
        self.visit_nodes(node, match_kind(SyntaxKind::Annotation), Self::visit_annotation);
        self.visit_block(node.children.last().unwrap());
    }

    fn visit_composite(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::StructDeclaration);
        self.visit_nodes(node, match_kind(SyntaxKind::StructMemberDeclaration), |slf, node| {
            check_node!(slf, node, SyntaxKind::StructMemberDeclaration);
            slf.visit_type(node.children.last().unwrap());
        });
    }

    fn visit_import(&mut self, node: &SyntaxNode) {
        self.check_node(node, match_kind(SyntaxKind::Import));
    }

    fn visit_const(&mut self, node: &SyntaxNode) {
        self.check_node(node, match_kind(SyntaxKind::GlobalConst));
        let [_l, _ident, _colon, type_, _a, expr, _s] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.visit_type(type_);
        self.visit_expression(expr);
    }

    fn visit_function(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::Function);
        self.visit_nodes(
            node,
            |_| true,
            |slf, node| match node.kind {
                SyntaxKind::Annotation => slf.visit_annotation(node),
                SyntaxKind::ConstParameterList => slf.visit_const_list(node),
                SyntaxKind::ParameterList => slf.visit_nodes(node, match_kind(SyntaxKind::Parameter), |slf, node| {
                    check_node!(slf, node, SyntaxKind::Parameter);
                    slf.visit_type(node.children.last().unwrap());
                }),
                SyntaxKind::FunctionOutput => slf.visit_type(node.children.last().unwrap()),
                SyntaxKind::FunctionOutputs => {
                    slf.visit_nodes(node, match_kind(SyntaxKind::FunctionOutput), Self::visit_type)
                }
                SyntaxKind::Statement(StatementKind::Block) => slf.visit_block(node),
                _ => {}
            },
        );
    }

    pub(super) fn visit_const_list(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::ConstParameterList | SyntaxKind::ConstArgumentList);
        match node.kind {
            SyntaxKind::ConstParameter => {
                self.visit_nodes(node, match_kind(SyntaxKind::ConstParameter), Self::visit_const_param)
            }
            SyntaxKind::ConstArgumentList => self.visit_nodes(node, match_expression, Self::visit_expression),
            _ => panic!(),
        }
    }

    fn visit_const_param(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::ConstParameter);
        let [_id, _c, type_] = &node.children[..] else {
            panic!("Can't happen");
        };

        self.visit_type(type_);
    }

    fn visit_annotation(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::Annotation);
        let [_at, _name, list @ ..] = &node.children[..] else { panic!("Can't happen") };
        if let Some(l) = list.first() {
            self.visit_annotation_list(l);
        }
    }

    fn visit_annotation_list(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::AnnotationList);
        let [_l, member_list @ .., _r] = &node.children[..] else { panic!("Can't happen") };
        for member in member_list.iter().filter(|m| matches!(m.kind, SyntaxKind::AnnotationMember)) {
            self.visit_annotation_member(member);
        }
    }

    fn visit_annotation_member(&mut self, node: &SyntaxNode) {
        check_node!(self, node, SyntaxKind::AnnotationMember)
    }
}
