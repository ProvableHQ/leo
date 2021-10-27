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

use dotgraph::LabelType;
use petgraph::graph::NodeIndex;

type M = Fixed<NodeIndex>;

impl<'a, 'b> MonoidalReducerStatement<'a, M> for Dotifier<'a, 'b> {
    fn reduce_statement(&mut self, input: &'a Statement<'a>, value: M) -> M {
        if !input.is_empty() {
            if let Some(parent) = input.get_parent() {
                if !parent.is_empty() {
                    self.edges.push((
                        input.get_id(),
                        parent.get_id(),
                        ("parent".to_string(), LabelType::Label),
                        Some(("red".to_string(), LabelType::Label)),
                    ))
                }
            }
        }
        value
    }

    fn reduce_assign_access(&mut self, input: &AssignAccess<'a>, left: Option<M>, right: Option<M>) -> M {
        let id = self.context.get_id();
        let label = format!(
            "Assign Access: {:}\nNode ID: {:}",
            match input {
                AssignAccess::ArrayRange(_, _) => "Array Range",
                AssignAccess::ArrayIndex(_) => "Array Index",
                AssignAccess::Tuple(_) => "Tuple",
                AssignAccess::Member(_) => "Member",
            },
            id,
        );
        let start_idx = self.add_or_get_node(id, label, LabelType::Esc);

        if let Some(Fixed(end_idx)) = left {
            self.add_edge(
                start_idx,
                end_idx,
                ("left".to_string(), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        if let Some(Fixed(end_idx)) = right {
            self.add_edge(
                start_idx,
                end_idx,
                ("right".to_string(), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            )
        }

        Fixed(start_idx)
    }

    fn reduce_assign(&mut self, input: &AssignStatement<'a>, variable: M, accesses: Vec<M>, value: M) -> M {
        let label = format!(
            "Assign Statement\nNode ID: {:}\n\nOperation: {:}\n\n{:}",
            input.id,
            input.operation.as_ref(),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = variable;
        self.add_edge(
            start_idx,
            end_idx,
            ("variable".to_string(), LabelType::Label),
            Some(("olive".to_string(), LabelType::Label)),
        );

        for (i, Fixed(end_idx)) in accesses.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("access_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        let Fixed(end_idx) = value;
        self.add_edge(
            start_idx,
            end_idx,
            ("value".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_block(&mut self, input: &BlockStatement<'a>, statements: Vec<M>) -> M {
        let label = format!(
            "Block Statement\nNode ID: {:}\n\n{:}",
            input.id,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in statements.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("statement_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
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
        let label = format!(
            "Console Statement\nNode ID: {:}\n\n{:}",
            input.id,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = condition;
        self.add_edge(
            start_idx,
            end_idx,
            ("condition".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        let Fixed(end_idx) = if_true;
        self.add_edge(
            start_idx,
            end_idx,
            ("if_true".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        if let Some(Fixed(end_idx)) = if_false {
            self.add_edge(
                start_idx,
                end_idx,
                ("if_false".to_string(), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_formatted_string(&mut self, input: &ConsoleArgs<'a>, parameters: Vec<M>) -> M {
        //TODO: Implement Display for CharValue, don't use debug
        let label = format!("Console Args\nNode ID: {:}\n\nString: {:?}", input.id, input.string);
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        for (i, Fixed(end_idx)) in parameters.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("parameter_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_console(&mut self, input: &ConsoleStatement<'a>, argument: M) -> M {
        let label = format!(
            "Console Statement\nNode ID: {:}\n\nFunction Type: {:}\n\n{:}",
            input.id,
            match input.function {
                ConsoleFunction::Assert(_) => "Assert",
                ConsoleFunction::Error(_) => "Error",
                ConsoleFunction::Log(_) => "Log",
            },
            Dotifier::generate_span_info(&input.span)
        );

        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = argument;
        self.add_edge(
            start_idx,
            end_idx,
            ("argument".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement<'a>, variables: Vec<M>, value: M) -> M {
        let label = format!(
            "Definition Statement\nNode ID: {:}\n\n{:}",
            input.id,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in variables.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("variable_{:}", i), LabelType::Label),
                Some(("olive".to_string(), LabelType::Label)),
            );
        }

        let Fixed(end_idx) = value;
        self.add_edge(
            start_idx,
            end_idx,
            ("value".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement<'a>, expression: M) -> M {
        let label = format!(
            "Expression Statement\nNode ID: {:}\n\n{:}",
            input.id,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = expression;
        self.add_edge(
            start_idx,
            end_idx,
            ("expression".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_iteration(&mut self, input: &IterationStatement<'a>, variable: M, start: M, stop: M, body: M) -> M {
        let label = format!(
            "Iteration Statement\nNode ID: {:}\n\nInclusive: {:}\n\n{:}",
            input.id,
            input.inclusive,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = variable;
        self.add_edge(
            start_idx,
            end_idx,
            ("variable".to_string(), LabelType::Label),
            Some(("olive".to_string(), LabelType::Label)),
        );

        let Fixed(end_idx) = start;
        self.add_edge(
            start_idx,
            end_idx,
            ("start".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        let Fixed(end_idx) = stop;
        self.add_edge(
            start_idx,
            end_idx,
            ("stop".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        let Fixed(end_idx) = body;
        self.add_edge(
            start_idx,
            end_idx,
            ("body".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_return(&mut self, input: &ReturnStatement<'a>, value: M) -> M {
        let label = format!(
            "Return Statement\nNode ID: {:}\n\n{:}",
            input.id,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = value;
        self.add_edge(
            start_idx,
            end_idx,
            ("value".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }
}
