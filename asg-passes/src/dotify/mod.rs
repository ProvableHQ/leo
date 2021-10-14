// Copyright (C) 2019-2021 Aleo Systems Inc.
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
mod dotgraph;

use leo_asg::*;
use leo_errors::Result;

use dot::{ArrowShape, Style};
use dotgraph::{DotEdge, DotGraph, DotNode, LabelType};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

pub struct Dotify<'a, 'b> {
    graph: DotGraph,
    context: &'b AsgContext<'a>,
    id_map: HashMap<u32, NodeIndex>,
}

impl<'a, 'b> Dotify<'a, 'b> {
    fn new(graph: DotGraph, context: &'b AsgContext<'a>) -> Self {
        Dotify {
            graph,
            context,
            id_map: HashMap::new(),
        }
    }

    // Helper functions to make it easier to construct graphs
    fn add_or_get_node(&mut self, id: u32, label: String) -> NodeIndex {
        let &mut Dotify {
            ref mut id_map,
            ref mut graph,
            context: _,
        } = self;
        *id_map.entry(id).or_insert_with(|| {
            let node = DotNode {
                id: format!("N{:?}", id),
                shape: None,
                label: (label, LabelType::Label),
                style: Style::None,
                color: None,
            };
            graph.add_node(node)
        })
    }

    fn add_edge(&mut self, start_idx: NodeIndex, end_idx: NodeIndex, label: String) {
        let edge = DotEdge {
            start_idx,
            end_idx,
            label: (label, LabelType::Label),
            end_arrow: ArrowShape::NoArrow,
            start_arrow: ArrowShape::NoArrow,
            style: Style::None,
            color: None,
        };
        self.graph.add_edge(edge);
    }
}

type M = Fixed<NodeIndex>;

impl<'a, 'b> MonoidalReducerExpression<'a, M> for Dotify<'a, 'b> {
    fn reduce_expression(&mut self, _input: &'a Expression<'a>, value: M) -> M {
        // Bubble up value since `Expression` is an enum
        value
    }

    fn reduce_array_access(&mut self, input: &ArrayAccessExpression<'a>, array: M, index: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "array_access_expression".to_string());

        let Fixed(end_idx) = array;
        self.add_edge(start_idx, end_idx, "array".to_string());

        let Fixed(end_idx) = index;
        self.add_edge(start_idx, end_idx, "index".to_string());

        Fixed(start_idx)
    }

    fn reduce_array_init(&mut self, input: &ArrayInitExpression<'a>, element: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "array_init_expression".to_string());

        let Fixed(end_idx) = element;
        self.add_edge(start_idx, end_idx, "element".to_string());

        Fixed(start_idx)
    }

    fn reduce_array_inline(&mut self, input: &ArrayInlineExpression<'a>, elements: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "array_inline_expression".to_string());

        for (i, Fixed(end_idx)) in elements.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("element_{:?}", i));
        }

        Fixed(start_idx)
    }

    fn reduce_array_range_access(
        &mut self,
        input: &ArrayRangeAccessExpression<'a>,
        array: M,
        left: Option<M>,
        right: Option<M>,
    ) -> M {
        let start_idx = self.add_or_get_node(input.id, "array_range_access_expression".to_string());

        let Fixed(end_idx) = array;
        self.add_edge(start_idx, end_idx, "array".to_string());

        if let Some(Fixed(end_idx)) = left {
            self.add_edge(start_idx, end_idx, "left".to_string());
        }

        if let Some(Fixed(end_idx)) = right {
            self.add_edge(start_idx, end_idx, "right".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_binary(&mut self, input: &BinaryExpression<'a>, left: M, right: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "binary_expression".to_string());

        let Fixed(end_idx) = left;
        self.add_edge(start_idx, end_idx, "left".to_string());

        let Fixed(end_idx) = right;
        self.add_edge(start_idx, end_idx, "right".to_string());

        Fixed(start_idx)
    }

    fn reduce_call(&mut self, input: &CallExpression<'a>, target: Option<M>, arguments: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "call_expression".to_string());

        for (i, Fixed(end_idx)) in arguments.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("argument_{:?}", i));
        }

        if let Some(Fixed(end_idx)) = target {
            self.add_edge(start_idx, end_idx, "target".to_string())
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_access(&mut self, input: &CircuitAccessExpression<'a>, target: Option<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "circuit_access_expression".to_string());

        if let Some(Fixed(end_idx)) = target {
            self.add_edge(start_idx, end_idx, "target".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_init(&mut self, input: &CircuitInitExpression<'a>, values: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "circuit_init_expression".to_string());

        for (i, Fixed(end_idx)) in values.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("value_{:?}", i));
        }

        Fixed(start_idx)
    }

    fn reduce_ternary_expression(&mut self, input: &TernaryExpression<'a>, condition: M, if_true: M, if_false: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "ternary_expression".to_string());

        let Fixed(end_idx) = condition;
        self.add_edge(start_idx, end_idx, "condition".to_string());

        let Fixed(end_idx) = if_true;
        self.add_edge(start_idx, end_idx, "if_true".to_string());

        let Fixed(end_idx) = if_false;
        self.add_edge(start_idx, end_idx, "if_false".to_string());

        Fixed(start_idx)
    }

    fn reduce_cast_expression(&mut self, input: &CastExpression<'a>, inner: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "cast_expression".to_string());
        let Fixed(end_idx) = inner;
        self.add_edge(start_idx, end_idx, "inner".to_string());

        Fixed(start_idx)
    }

    fn reduce_lengthof_expression(&mut self, input: &LengthOfExpression<'a>, inner: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "lengthof_expression".to_string());
        let Fixed(end_idx) = inner;
        self.add_edge(start_idx, end_idx, "inner".to_string());

        Fixed(start_idx)
    }

    fn reduce_constant(&mut self, input: &Constant<'a>) -> M {
        //TODO: Are there children to reduce?
        let start_idx = self.add_or_get_node(input.id, "constant".to_string());
        Fixed(start_idx)
    }

    fn reduce_tuple_access(&mut self, input: &TupleAccessExpression<'a>, tuple_ref: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "tuple_access_expression".to_string());
        let Fixed(end_idx) = tuple_ref;
        self.add_edge(start_idx, end_idx, "tuple_ref".to_string());

        Fixed(start_idx)
    }

    fn reduce_tuple_init(&mut self, input: &TupleInitExpression<'a>, values: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "tuple_init_expression".to_string());

        for (i, Fixed(end_idx)) in values.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("value_{:?}", i));
        }

        Fixed(start_idx)
    }

    fn reduce_unary(&mut self, input: &UnaryExpression<'a>, inner: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "unary_expression".to_string());
        let Fixed(end_idx) = inner;
        self.add_edge(start_idx, end_idx, "inner".to_string());

        Fixed(start_idx)
    }

    fn reduce_variable_ref(&mut self, input: &VariableRef<'a>) -> M {
        //TODO: Are there children to visit here?
        let start_idx = self.add_or_get_node(input.id, "variable_ref".to_string());
        Fixed(start_idx)
    }
}

impl<'a, 'b> MonoidalReducerStatement<'a, M> for Dotify<'a, 'b> {
    fn reduce_statement(&mut self, _input: &'a Statement<'a>, value: M) -> M {
        // Just bubble up value as `Statement` is an enum
        value
    }

    fn reduce_assign_access(&mut self, input: &AssignAccess<'a>, left: Option<M>, right: Option<M>) -> M {
        // TODO: Monoidal reducer might need to be rewritter for this
        let start_idx = self.add_or_get_node(self.context.get_id(), "assign_access".to_string());

        if let Some(Fixed(end_idx)) = left {
            self.add_edge(start_idx, end_idx, "left".to_string());
        }

        if let Some(Fixed(end_idx)) = right {
            self.add_edge(start_idx, end_idx, "right".to_string())
        }

        Fixed(start_idx)
    }

    fn reduce_assign(&mut self, input: &AssignStatement<'a>, accesses: Vec<M>, value: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "assign_statement".to_string());

        for (i, Fixed(end_idx)) in accesses.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("access_{:?}", i));
        }

        let Fixed(end_idx) = value;
        self.add_edge(start_idx, end_idx, "value".to_string());

        Fixed(start_idx)
    }

    fn reduce_block(&mut self, input: &BlockStatement<'a>, statements: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "block_statement".to_string());

        for (i, Fixed(end_idx)) in statements.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("statement_{:?}", i));
        }

        Fixed(start_idx)
    }

    fn reduce_conditional_statement(
        &mut self,
        input: &ConditionalStatement<'a>,
        condition: M,
        if_true: M,
        if_false: Option<M>,
    ) -> M {
        let start_idx = self.add_or_get_node(input.id, "conditional_statement".to_string());

        let Fixed(end_idx) = condition;
        self.add_edge(start_idx, end_idx, "condition".to_string());

        let Fixed(end_idx) = if_true;
        self.add_edge(start_idx, end_idx, "if_true".to_string());

        if let Some(Fixed(end_idx)) = if_false {
            self.add_edge(start_idx, end_idx, "condition".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_formatted_string(&mut self, input: &ConsoleArgs<'a>, parameters: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "console_args".to_string());

        for (i, Fixed(end_idx)) in parameters.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("parameter_{:?}", i));
        }

        Fixed(start_idx)
    }

    fn reduce_console(&mut self, input: &ConsoleStatement<'a>, argument: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "console_statement".to_string());
        let Fixed(end_idx) = argument;
        self.add_edge(start_idx, end_idx, "argument".to_string());

        Fixed(start_idx)
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement<'a>, value: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "definition_statement".to_string());
        let Fixed(end_idx) = value;
        self.add_edge(start_idx, end_idx, "value".to_string());

        Fixed(start_idx)
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement<'a>, expression: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "expression_statement".to_string());
        let Fixed(end_idx) = expression;
        self.add_edge(start_idx, end_idx, "expression".to_string());

        Fixed(start_idx)
    }

    fn reduce_iteration(&mut self, input: &IterationStatement<'a>, start: M, stop: M, body: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "iteration_statement".to_string());
        let Fixed(end_idx) = start;
        self.add_edge(start_idx, end_idx, "start".to_string());

        let Fixed(end_idx) = stop;
        self.add_edge(start_idx, end_idx, "stop".to_string());

        let Fixed(end_idx) = body;
        self.add_edge(start_idx, end_idx, "body".to_string());

        Fixed(start_idx)
    }

    fn reduce_return(&mut self, input: &ReturnStatement<'a>, value: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "return_statement".to_string());
        let Fixed(end_idx) = value;
        self.add_edge(start_idx, end_idx, "value".to_string());

        Fixed(start_idx)
    }
}

impl<'a, 'b> MonoidalReducerProgram<'a, M> for Dotify<'a, 'b> {
    fn reduce_function(&mut self, input: &'a Function<'a>, body: M) -> M {
        let start_idx = self.add_or_get_node(input.id, "function".to_string());
        let Fixed(end_idx) = body;

        self.add_edge(start_idx, end_idx, "body".to_string());

        Fixed(start_idx)
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMember<'a>, function: Option<M>) -> M {
        //TODO: Need to figure out how to reduce types
        //TODO: Might need to fix monoidal director
        let start_idx = self.add_or_get_node(self.context.get_id(), "circuit_member".to_string());
        Fixed(start_idx)
    }

    fn reduce_circuit(&mut self, input: &'a Circuit<'a>, members: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "circuit".to_string());

        for (i, Fixed(end_idx)) in members.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("member_{:?}", i));
        }

        Fixed(start_idx)
    }

    fn reduce_program(&mut self, input: &Program, imported_modules: Vec<M>, functions: Vec<M>, circuits: Vec<M>) -> M {
        let start_idx = self.add_or_get_node(input.id, "program".to_string());

        for (i, Fixed(end_idx)) in functions.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("function_{:?}", i));
        }

        for (i, Fixed(end_idx)) in circuits.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("circuit_{:?}", i));
        }

        Fixed(start_idx)
    }
}

impl<'a, 'b> AsgPass<'a> for Dotify<'a, 'b> {
    type Input = (Program<'a>, &'b AsgContext<'a>, String, PathBuf);
    type Output = Result<Program<'a>>;

    fn do_pass((asg, ctx, id, path): Self::Input) -> Self::Output {
        let graph = DotGraph::new(id);
        let mut director = MonoidalDirector::new(Dotify::new(graph, ctx));
        director.reduce_program(&asg);

        let mut file = File::create(path).unwrap();
        dot::render(&director.reducer().graph, &mut file).unwrap();
        Ok(asg)
    }
}
