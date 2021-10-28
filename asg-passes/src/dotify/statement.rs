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
use super::*;

use leo_asg::*;

use petgraph::graph::NodeIndex;

type M = Fixed<NodeIndex>;

impl<'a, 'b> MonoidalReducerStatement<'a, M> for Dotifier<'a, 'b> {
    fn reduce_statement(&mut self, input: &'a Statement<'a>, value: M) -> M {
        if !input.is_empty() {
            if let Some(parent) = input.get_parent() {
                if !parent.is_empty() {
                    self.edges
                        .push((input.get_id(), parent.get_id(), "parent".to_string(), "red"))
                }
            }
        }
        value
    }

    fn reduce_assign_access(&mut self, input: &AssignAccess<'a>, left: Option<M>, right: Option<M>) -> M {
        let id = self.context.get_id();

        let labels = vec![
            ("Node ID", id.to_string()),
            (
                "Access Type",
                match input {
                    AssignAccess::ArrayRange(_, _) => "Array Range",
                    AssignAccess::ArrayIndex(_) => "Array Index",
                    AssignAccess::Tuple(_) => "Tuple",
                    AssignAccess::Member(_) => "Member",
                }
                .to_string(),
            ),
        ];

        let start_idx = self.add_or_get_node(id, "AssignAccess".to_string(), labels);

        if let Some(Fixed(end_idx)) = left {
            self.add_edge(start_idx, end_idx, "left".to_string(), "black");
        }

        if let Some(Fixed(end_idx)) = right {
            self.add_edge(start_idx, end_idx, "right".to_string(), "black")
        }

        Fixed(start_idx)
    }

    fn reduce_assign(&mut self, input: &AssignStatement<'a>, variable: M, accesses: Vec<M>, value: M) -> M {
        let mut labels = vec![
            ("Node ID", input.id.to_string()),
            ("Operation", input.operation.as_ref().to_string()),
        ];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "AssignStatement".to_string(), labels);

        let Fixed(end_idx) = variable;
        self.add_edge(start_idx, end_idx, "variable".to_string(), "olive");

        for (i, Fixed(end_idx)) in accesses.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("access_{:}", i), "black");
        }

        let Fixed(end_idx) = value;
        self.add_edge(start_idx, end_idx, "value".to_string(), "black");

        Fixed(start_idx)
    }

    fn reduce_block(&mut self, input: &BlockStatement<'a>, statements: Vec<M>) -> M {
        let mut labels = vec![("Node ID", input.id.to_string())];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "BlockStatement".to_string(), labels);

        for (i, Fixed(end_idx)) in statements.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("statement_{:}", i), "black");
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
        let mut labels = vec![("Node ID", input.id.to_string())];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "ConsoleStatement".to_string(), labels);

        let Fixed(end_idx) = condition;
        self.add_edge(start_idx, end_idx, "condition".to_string(), "black");

        let Fixed(end_idx) = if_true;
        self.add_edge(start_idx, end_idx, "if_true".to_string(), "black");

        if let Some(Fixed(end_idx)) = if_false {
            self.add_edge(start_idx, end_idx, "if_false".to_string(), "black");
        }

        Fixed(start_idx)
    }

    fn reduce_formatted_string(&mut self, input: &ConsoleArgs<'a>, parameters: Vec<M>) -> M {
        //TODO: Implement Display for CharValue, don't use debug
        let mut labels = vec![
            ("Node ID", input.id.to_string()),
            ("String", format!("{:?}", input.string)),
        ];

        Dotifier::add_span_info(&mut labels, &Some(input.span.clone()));

        let start_idx = self.add_or_get_node(input.id, "ConsoleArgs".to_string(), labels);
        for (i, Fixed(end_idx)) in parameters.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("parameter_{:}", i), "black");
        }

        Fixed(start_idx)
    }

    fn reduce_console(&mut self, input: &ConsoleStatement<'a>, argument: M) -> M {
        let mut labels = vec![
            ("Node ID", input.id.to_string()),
            (
                "FunctionType",
                match input.function {
                    ConsoleFunction::Assert(_) => "Assert",
                    ConsoleFunction::Error(_) => "Error",
                    ConsoleFunction::Log(_) => "Log",
                }
                .to_string(),
            ),
        ];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "ConsoleStatement".to_string(), labels);
        let Fixed(end_idx) = argument;
        self.add_edge(start_idx, end_idx, "argument".to_string(), "black");

        Fixed(start_idx)
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement<'a>, variables: Vec<M>, value: M) -> M {
        let mut labels = vec![("Node ID", input.id.to_string())];
        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "DefinitionStatement".to_string(), labels);

        for (i, Fixed(end_idx)) in variables.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("variable_{:}", i), "olive");
        }

        let Fixed(end_idx) = value;
        self.add_edge(start_idx, end_idx, "value".to_string(), "black");

        Fixed(start_idx)
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement<'a>, expression: M) -> M {
        let mut labels = vec![("Node ID", input.id.to_string())];
        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "ExpressionStatement".to_string(), labels);
        let Fixed(end_idx) = expression;
        self.add_edge(start_idx, end_idx, "expression".to_string(), "black");

        Fixed(start_idx)
    }

    fn reduce_iteration(&mut self, input: &IterationStatement<'a>, variable: M, start: M, stop: M, body: M) -> M {
        let mut labels = vec![
            ("Node ID", input.id.to_string()),
            ("Inclusive", input.inclusive.to_string()),
        ];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "IterationStatement".to_string(), labels);
        let Fixed(end_idx) = variable;
        self.add_edge(start_idx, end_idx, "variable".to_string(), "olive");

        let Fixed(end_idx) = start;
        self.add_edge(start_idx, end_idx, "start".to_string(), "black");

        let Fixed(end_idx) = stop;
        self.add_edge(start_idx, end_idx, "stop".to_string(), "black");

        let Fixed(end_idx) = body;
        self.add_edge(start_idx, end_idx, "body".to_string(), "black");

        Fixed(start_idx)
    }

    fn reduce_return(&mut self, input: &ReturnStatement<'a>, value: M) -> M {
        let mut labels = vec![("Node ID", input.id.to_string())];
        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "ReturnStatement".to_string(), labels);
        let Fixed(end_idx) = value;
        self.add_edge(start_idx, end_idx, "value".to_string(), "black");

        Fixed(start_idx)
    }
}
