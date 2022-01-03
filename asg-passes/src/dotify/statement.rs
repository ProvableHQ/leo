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
                        .push((input.asg_id(), parent.asg_id(), "parent".to_string(), DotColor::Red))
                }
            }
        }
        value
    }

    fn reduce_assign_access(&mut self, input: &AssignAccess<'a>, left: Option<M>, right: Option<M>) -> M {
        let id = self.context.get_id();

        let labels = vec![
            ("NodeID", id.to_string()),
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
            self.graph.add_default_edge(start_idx, end_idx, "left".to_string());
        }

        if let Some(Fixed(end_idx)) = right {
            self.graph.add_default_edge(start_idx, end_idx, "right".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_assign(&mut self, input: &AssignStatement<'a>, variable: M, accesses: Vec<M>, value: M) -> M {
        let mut labels = Dotifier::generate_default_stmt_labels(input);

        labels.push(("Operation", input.operation.as_ref().to_string()));

        let start_idx = self.add_or_get_node(input.id, "AssignStatement".to_string(), labels);

        let Fixed(end_idx) = variable;
        self.graph
            .add_edge(start_idx, end_idx, "variable".to_string(), DotColor::Olive);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "access_", accesses);

        let Fixed(end_idx) = value;
        self.graph.add_default_edge(start_idx, end_idx, "value".to_string());

        Fixed(start_idx)
    }

    fn reduce_block(&mut self, input: &BlockStatement<'a>, statements: Vec<M>) -> M {
        let labels = Dotifier::generate_default_stmt_labels(input);

        let start_idx = self.add_or_get_node(input.id, "BlockStatement".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "statement_", statements);

        Fixed(start_idx)
    }

    fn reduce_conditional_statement(
        &mut self,
        input: &ConditionalStatement<'a>,
        condition: M,
        if_true: M,
        if_false: Option<M>,
    ) -> M {
        let labels = Dotifier::generate_default_stmt_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ConditionalStatement".to_string(), labels);

        let Fixed(end_idx) = condition;
        self.graph.add_default_edge(start_idx, end_idx, "condition".to_string());

        let Fixed(end_idx) = if_true;
        self.graph.add_default_edge(start_idx, end_idx, "if_true".to_string());

        if let Some(Fixed(end_idx)) = if_false {
            self.graph.add_default_edge(start_idx, end_idx, "if_false".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_formatted_string(&mut self, input: &ConsoleArgs<'a>, parameters: Vec<M>) -> M {
        let mut labels = Dotifier::generate_default_stmt_labels(input);

        labels.push(("String", format!("{:?}", input.string))); //Note: Debug seems to work, revisit if needed

        let start_idx = self.add_or_get_node(input.id, "ConsoleArgs".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "parameter_", parameters);

        Fixed(start_idx)
    }

    fn reduce_console(&mut self, input: &ConsoleStatement<'a>, argument: M) -> M {
        let mut labels = Dotifier::generate_default_stmt_labels(input);
        labels.push((
            "FunctionType",
            match input.function {
                ConsoleFunction::Assert(_) => "Assert",
                ConsoleFunction::Error(_) => "Error",
                ConsoleFunction::Log(_) => "Log",
            }
            .to_string(),
        ));

        let start_idx = self.add_or_get_node(input.id, "ConsoleStatement".to_string(), labels);
        let Fixed(end_idx) = argument;
        self.graph.add_default_edge(start_idx, end_idx, "argument".to_string());

        Fixed(start_idx)
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement<'a>, variables: Vec<M>, value: M) -> M {
        let labels = Dotifier::generate_default_stmt_labels(input);

        let start_idx = self.add_or_get_node(input.id, "DefinitionStatement".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Olive, "variable_", variables);

        let Fixed(end_idx) = value;
        self.graph.add_default_edge(start_idx, end_idx, "value".to_string());

        Fixed(start_idx)
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement<'a>, expression: M) -> M {
        let labels = Dotifier::generate_default_stmt_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ExpressionStatement".to_string(), labels);
        let Fixed(end_idx) = expression;
        self.graph
            .add_default_edge(start_idx, end_idx, "expression".to_string());

        Fixed(start_idx)
    }

    fn reduce_iteration(&mut self, input: &IterationStatement<'a>, variable: M, start: M, stop: M, body: M) -> M {
        let mut labels = Dotifier::generate_default_stmt_labels(input);

        labels.push(("Inclusive", input.inclusive.to_string()));

        let start_idx = self.add_or_get_node(input.id, "IterationStatement".to_string(), labels);
        let Fixed(end_idx) = variable;
        self.graph.add_default_edge(start_idx, end_idx, "variable".to_string());

        let Fixed(end_idx) = start;
        self.graph.add_default_edge(start_idx, end_idx, "start".to_string());

        let Fixed(end_idx) = stop;
        self.graph.add_default_edge(start_idx, end_idx, "stop".to_string());

        let Fixed(end_idx) = body;
        self.graph.add_default_edge(start_idx, end_idx, "body".to_string());

        Fixed(start_idx)
    }

    fn reduce_return(&mut self, input: &ReturnStatement<'a>, value: M) -> M {
        let labels = Dotifier::generate_default_stmt_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ReturnStatement".to_string(), labels);
        let Fixed(end_idx) = value;
        self.graph.add_default_edge(start_idx, end_idx, "value".to_string());

        Fixed(start_idx)
    }
}
