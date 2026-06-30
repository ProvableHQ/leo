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

use super::StorageReadForwardingVisitor;
use crate::CompilerState;

use leo_ast::*;
use leo_span::{Symbol, create_session_if_not_set_then, sym};

use std::rc::Rc;

fn ident(state: &mut CompilerState, name: &str) -> Identifier {
    Identifier::new(Symbol::intern(name), state.node_builder.next_id())
}

fn local(state: &mut CompilerState, name: &str) -> Expression {
    Path::from(ident(state, name)).to_local().into()
}

fn u8_lit(state: &mut CompilerState, value: &str) -> Expression {
    Literal::integer(IntegerType::U8, value.into(), Default::default(), state.node_builder.next_id()).into()
}

fn definition(state: &mut CompilerState, name: &str, value: Expression) -> Statement {
    DefinitionStatement {
        place: DefinitionPlace::Single(ident(state, name)),
        type_: None,
        value,
        span: Default::default(),
        id: state.node_builder.next_id(),
    }
    .into()
}

fn storage_read(state: &mut CompilerState) -> Expression {
    storage_read_with_key(state, "key")
}

fn storage_read_with_key(state: &mut CompilerState, key: &str) -> Expression {
    let arguments = vec![local(state, "data"), local(state, key), u8_lit(state, "0")];
    IntrinsicExpression {
        name: sym::_mapping_get_or_use,
        type_parameters: Vec::new(),
        input_types: Vec::new(),
        return_types: Vec::new(),
        arguments,
        span: Default::default(),
        id: state.node_builder.next_id(),
    }
    .into()
}

fn storage_contains(state: &mut CompilerState) -> Expression {
    let arguments = vec![local(state, "data"), local(state, "key")];
    IntrinsicExpression {
        name: sym::_mapping_contains,
        type_parameters: Vec::new(),
        input_types: Vec::new(),
        return_types: Vec::new(),
        arguments,
        span: Default::default(),
        id: state.node_builder.next_id(),
    }
    .into()
}

fn dynamic_storage_read(state: &mut CompilerState) -> Expression {
    DynamicOpExpression {
        interface: Type::Ident(ident(state, "External")),
        target_program: local(state, "target"),
        network: None,
        kind: DynamicOpKind::Read { storage: ident(state, "external_value") },
        span: Default::default(),
        id: state.node_builder.next_id(),
    }
    .into()
}

fn final_run(state: &mut CompilerState) -> Expression {
    IntrinsicExpression {
        name: sym::_final_run,
        type_parameters: Vec::new(),
        input_types: Vec::new(),
        return_types: Vec::new(),
        arguments: vec![local(state, "future")],
        span: Default::default(),
        id: state.node_builder.next_id(),
    }
    .into()
}

fn expression_statement(state: &mut CompilerState, expression: Expression) -> Statement {
    ExpressionStatement { expression, span: Default::default(), id: state.node_builder.next_id() }.into()
}

fn output_for(state: &mut CompilerState, block: Block) -> String {
    let mut visitor = StorageReadForwardingVisitor {
        state,
        reads: Default::default(),
        aliases: Default::default(),
        then_join_aliases: Default::default(),
        otherwise_join_aliases: Default::default(),
        join_condition: None,
        in_finalize_context: true,
    };
    visitor.reconstruct_block(block).0.to_string()
}

#[test]
fn preserves_branch_alias_for_ssa_join_operand() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let condition = local(&mut state, "flag");
        let true_operand = local(&mut state, "x2");
        let false_operand = local(&mut state, "x0");
        let join = TernaryExpression {
            condition: local(&mut state, "flag"),
            if_true: true_operand,
            if_false: false_operand,
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let initial_value = u8_lit(&mut state, "0");
        let first_read = storage_read(&mut state);
        let second_read = storage_read(&mut state);
        let block = Block {
            statements: vec![
                definition(&mut state, "x0", initial_value),
                ConditionalStatement {
                    condition,
                    then: Block {
                        statements: vec![
                            definition(&mut state, "x1", first_read),
                            definition(&mut state, "x2", second_read),
                        ],
                        span: Default::default(),
                        id: state.node_builder.next_id(),
                    },
                    otherwise: None,
                    span: Default::default(),
                    id: state.node_builder.next_id(),
                }
                .into(),
                definition(&mut state, "x3", join.into()),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert_eq!(
            output.matches("_mapping_get_or_use").count(),
            1,
            "expected the repeated branch read to be forwarded:\n{output}"
        );
        assert!(
            output.contains("let x3 = flag ? x1 : x0"),
            "expected the SSA join to use the surviving branch definition:\n{output}"
        );
        assert!(output.contains("let x2 = x1"), "expected the repeated read definition to remain as a copy:\n{output}");
    });
}

#[test]
fn does_not_apply_branch_alias_to_different_condition() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let condition = local(&mut state, "flag");
        let unrelated_join = TernaryExpression {
            condition: local(&mut state, "other"),
            if_true: local(&mut state, "x2"),
            if_false: local(&mut state, "x0"),
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let initial_value = u8_lit(&mut state, "0");
        let first_read = storage_read(&mut state);
        let second_read = storage_read(&mut state);
        let block = Block {
            statements: vec![
                definition(&mut state, "x0", initial_value),
                ConditionalStatement {
                    condition,
                    then: Block {
                        statements: vec![
                            definition(&mut state, "x1", first_read),
                            definition(&mut state, "x2", second_read),
                        ],
                        span: Default::default(),
                        id: state.node_builder.next_id(),
                    },
                    otherwise: None,
                    span: Default::default(),
                    id: state.node_builder.next_id(),
                }
                .into(),
                definition(&mut state, "x3", unrelated_join.into()),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert!(
            output.contains("let x3 = other ? x2 : x0"),
            "branch-local alias was applied outside the matching SSA join:\n{output}"
        );
        assert!(
            output.contains("let x2 = x1"),
            "non-matching join kept a reference to x2 without preserving its definition:\n{output}"
        );
    });
}

#[test]
fn does_not_apply_branch_alias_after_intervening_definition() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let condition = local(&mut state, "flag");
        let join = TernaryExpression {
            condition: local(&mut state, "flag"),
            if_true: local(&mut state, "x2"),
            if_false: local(&mut state, "x0"),
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let initial_value = u8_lit(&mut state, "0");
        let first_read = storage_read(&mut state);
        let second_read = storage_read(&mut state);
        let intervening_value = u8_lit(&mut state, "1");
        let block = Block {
            statements: vec![
                definition(&mut state, "x0", initial_value),
                ConditionalStatement {
                    condition,
                    then: Block {
                        statements: vec![
                            definition(&mut state, "x1", first_read),
                            definition(&mut state, "x2", second_read),
                        ],
                        span: Default::default(),
                        id: state.node_builder.next_id(),
                    },
                    otherwise: None,
                    span: Default::default(),
                    id: state.node_builder.next_id(),
                }
                .into(),
                definition(&mut state, "tmp", intervening_value),
                definition(&mut state, "x3", join.into()),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert!(
            output.contains("let x3 = flag ? x2 : x0"),
            "branch-local alias was applied after a non-join definition:\n{output}"
        );
        assert!(
            output.contains("let x2 = x1"),
            "later same-condition join kept a reference to x2 without preserving its definition:\n{output}"
        );
    });
}

#[test]
fn does_not_apply_then_branch_alias_to_otherwise_join_operand() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let condition = local(&mut state, "flag");
        let join = TernaryExpression {
            condition: local(&mut state, "flag"),
            if_true: local(&mut state, "x0"),
            if_false: local(&mut state, "x2"),
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let initial_value = u8_lit(&mut state, "0");
        let first_read = storage_read(&mut state);
        let second_read = storage_read(&mut state);
        let block = Block {
            statements: vec![
                definition(&mut state, "x0", initial_value),
                ConditionalStatement {
                    condition,
                    then: Block {
                        statements: vec![
                            definition(&mut state, "x1", first_read),
                            definition(&mut state, "x2", second_read),
                        ],
                        span: Default::default(),
                        id: state.node_builder.next_id(),
                    },
                    otherwise: None,
                    span: Default::default(),
                    id: state.node_builder.next_id(),
                }
                .into(),
                definition(&mut state, "x3", join.into()),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert!(
            output.contains("let x3 = flag ? x0 : x2"),
            "then-branch alias was applied to the opposite ternary arm:\n{output}"
        );
        assert!(
            output.contains("let x2 = x1"),
            "opposite-arm join kept a reference to x2 without preserving its definition:\n{output}"
        );
    });
}

#[test]
fn canonicalizes_aliased_storage_read_keys() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let key_alias = local(&mut state, "key");
        let first_read = storage_read_with_key(&mut state, "key");
        let second_read = storage_read_with_key(&mut state, "key2");
        let block = Block {
            statements: vec![
                definition(&mut state, "key2", key_alias),
                definition(&mut state, "x1", first_read),
                definition(&mut state, "x2", second_read),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert_eq!(
            output.matches("_mapping_get_or_use").count(),
            1,
            "expected aliased storage-read keys to share the same read fact:\n{output}"
        );
        assert!(
            output.contains("let x2 = x1"),
            "expected the aliased-key duplicate read to remain as a copy:\n{output}"
        );
    });
}

#[test]
fn dynamic_storage_read_clears_static_read_facts() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let first_read = storage_read(&mut state);
        let dynamic_read = dynamic_storage_read(&mut state);
        let second_read = storage_read(&mut state);
        let block = Block {
            statements: vec![
                definition(&mut state, "x1", first_read),
                definition(&mut state, "external", dynamic_read),
                definition(&mut state, "x2", second_read),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert_eq!(
            output.matches("_mapping_get_or_use").count(),
            2,
            "dynamic storage reads must prevent forwarding across the boundary:\n{output}"
        );
        assert!(
            output.contains("let external = External@(target)::external_value"),
            "expected the dynamic storage read to remain in the output:\n{output}"
        );
    });
}

#[test]
fn final_run_clears_static_read_facts() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let first_read = storage_read(&mut state);
        let run = final_run(&mut state);
        let run_statement = expression_statement(&mut state, run);
        let second_read = storage_read(&mut state);
        let block = Block {
            statements: vec![
                definition(&mut state, "x1", first_read),
                run_statement,
                definition(&mut state, "x2", second_read),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert_eq!(
            output.matches("_mapping_get_or_use").count(),
            2,
            "`Final::run` must prevent forwarding across the boundary:\n{output}"
        );
        assert!(output.contains("_final_run(future)"), "expected the final run to remain in the output:\n{output}");
    });
}

#[test]
fn canonicalizes_aliased_join_conditions() {
    create_session_if_not_set_then(|_| {
        let mut state = CompilerState { node_builder: Rc::new(NodeBuilder::default()), ..Default::default() };

        let join = TernaryExpression {
            condition: local(&mut state, "cond2"),
            if_true: local(&mut state, "x2"),
            if_false: local(&mut state, "x0"),
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let initial_value = u8_lit(&mut state, "0");
        let first_condition = storage_contains(&mut state);
        let second_condition = storage_contains(&mut state);
        let first_read = storage_read(&mut state);
        let second_read = storage_read(&mut state);
        let block = Block {
            statements: vec![
                definition(&mut state, "x0", initial_value),
                definition(&mut state, "cond1", first_condition),
                definition(&mut state, "cond2", second_condition),
                ConditionalStatement {
                    condition: local(&mut state, "cond2"),
                    then: Block {
                        statements: vec![
                            definition(&mut state, "x1", first_read),
                            definition(&mut state, "x2", second_read),
                        ],
                        span: Default::default(),
                        id: state.node_builder.next_id(),
                    },
                    otherwise: None,
                    span: Default::default(),
                    id: state.node_builder.next_id(),
                }
                .into(),
                definition(&mut state, "x3", join.into()),
            ],
            span: Default::default(),
            id: state.node_builder.next_id(),
        };

        let output = output_for(&mut state, block);

        assert!(
            output.contains("let cond2 = cond1"),
            "expected the repeated branch condition read to remain as a copy:\n{output}"
        );
        assert!(
            output.contains("let x3 = cond1 ? x1 : x0"),
            "expected same-condition SSA join matching to use canonical condition aliases:\n{output}"
        );
        assert!(
            output.contains("let x2 = x1"),
            "expected the branch-local repeated read to remain as a copy:\n{output}"
        );
    });
}
