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

impl<'a, 'b> MonoidalReducerExpression<'a, M> for Dotifier<'a, 'b> {
    fn reduce_expression(&mut self, input: &'a Expression<'a>, value: M) -> M {
        if let Some(parent) = input.get_parent() {
            self.edges
                .push((input.get_id(), parent.get_id(), "parent".to_string(), DotColor::Red))
        }
        value
    }

    fn reduce_err(&mut self, input: &'a ErrExpression<'a>) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ErrExpression".to_string(), labels);

        Fixed(start_idx)
    }

    fn reduce_array_init(&mut self, input: &'a ArrayInitExpression<'a>, element: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ArrayInitExpression".to_string(), labels);

        let Fixed(end_idx) = element;
        self.graph.add_default_edge(start_idx, end_idx, "element".to_string());

        Fixed(start_idx)
    }

    fn reduce_array_inline(&mut self, input: &'a ArrayInlineExpression<'a>, elements: Vec<M>) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ArrayInlineExpression".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "element_", elements);

        Fixed(start_idx)
    }

    fn reduce_binary(&mut self, input: &'a BinaryExpression<'a>, left: M, right: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "BinaryExpression".to_string(), labels);

        let Fixed(end_idx) = left;
        self.graph.add_default_edge(start_idx, end_idx, "left".to_string());

        let Fixed(end_idx) = right;
        self.graph.add_default_edge(start_idx, end_idx, "right".to_string());

        Fixed(start_idx)
    }

    fn reduce_call(&mut self, input: &'a CallExpression<'a>, target: Option<M>, arguments: Vec<M>) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "CallExpression".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "argument_", arguments);

        if let Some(Fixed(end_idx)) = target {
            self.graph.add_default_edge(start_idx, end_idx, "target".to_string());
        }

        self.edges.push((
            input.id,
            input.function.get().id,
            "function".to_string(),
            DotColor::Magenta,
        ));

        Fixed(start_idx)
    }

    fn reduce_circuit_init(&mut self, input: &'a CircuitInitExpression<'a>, values: Vec<M>) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "CircuitInitExpression".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "value_", values);

        Fixed(start_idx)
    }

    fn reduce_ternary_expression(
        &mut self,
        input: &'a TernaryExpression<'a>,
        condition: M,
        if_true: M,
        if_false: M,
    ) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "TernaryExpression".to_string(), labels);

        let Fixed(end_idx) = condition;
        self.graph.add_default_edge(start_idx, end_idx, "condition".to_string());

        let Fixed(end_idx) = if_true;
        self.graph.add_default_edge(start_idx, end_idx, "if_true".to_string());

        let Fixed(end_idx) = if_false;
        self.graph.add_default_edge(start_idx, end_idx, "if_false".to_string());

        Fixed(start_idx)
    }

    fn reduce_cast_expression(&mut self, input: &'a CastExpression<'a>, inner: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "CastExpression".to_string(), labels);
        let Fixed(end_idx) = inner;
        self.graph.add_default_edge(start_idx, end_idx, "inner".to_string());

        Fixed(start_idx)
    }

    fn reduce_array_access(&mut self, input: &'a ArrayAccess<'a>, array: M, index: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "ArrayAccess".to_string(), labels);

        let Fixed(end_idx) = array;
        self.graph.add_default_edge(start_idx, end_idx, "array".to_string());

        let Fixed(end_idx) = index;
        self.graph.add_default_edge(start_idx, end_idx, "index".to_string());

        Fixed(start_idx)
    }

    fn reduce_constant(&mut self, input: &'a Constant<'a>) -> M {
        let mut labels = Dotifier::generate_default_expr_labels(input);

        labels.push(("Value", format!("{:}", input.value)));

        let start_idx = self.add_or_get_node(input.id, "Constant".to_string(), labels);

        Fixed(start_idx)
    }

    fn reduce_array_range_access(
        &mut self,
        input: &'a ArrayRangeAccess<'a>,
        array: M,
        left: Option<M>,
        right: Option<M>,
    ) -> M {
        let mut labels = Dotifier::generate_default_expr_labels(input);

        labels.push(("Length", input.length.to_string()));

        let start_idx = self.add_or_get_node(input.id, "ArrayRangeAccess".to_string(), labels);

        let Fixed(end_idx) = array;
        self.graph.add_default_edge(start_idx, end_idx, "array".to_string());

        if let Some(Fixed(end_idx)) = left {
            self.graph.add_default_edge(start_idx, end_idx, "left".to_string());
        }

        if let Some(Fixed(end_idx)) = right {
            self.graph.add_default_edge(start_idx, end_idx, "right".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_access(&mut self, input: &'a CircuitAccess<'a>, target: Option<M>) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "CircuitAccess".to_string(), labels);

        if let Some(Fixed(end_idx)) = target {
            self.graph.add_default_edge(start_idx, end_idx, "target".to_string());
        }

        Fixed(start_idx)
    }

    fn reduce_tuple_access(&mut self, input: &'a TupleAccess<'a>, tuple_ref: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "TupleAccessExpression".to_string(), labels);
        let Fixed(end_idx) = tuple_ref;
        self.graph.add_default_edge(start_idx, end_idx, "tuple_ref".to_string());

        Fixed(start_idx)
    }

    fn reduce_tuple_init(&mut self, input: &'a TupleInitExpression<'a>, values: Vec<M>) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "TupleInitExpression".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Black, "value_", values);

        Fixed(start_idx)
    }

    fn reduce_unary(&mut self, input: &'a UnaryExpression<'a>, inner: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "UnaryExpresssion".to_string(), labels);
        let Fixed(end_idx) = inner;
        self.graph.add_default_edge(start_idx, end_idx, "inner".to_string());

        Fixed(start_idx)
    }

    fn reduce_variable(&mut self, input: &'a Variable<'a>) -> M {
        let inner_var = input.borrow();

        let labels = vec![
            ("NodeID", inner_var.id.to_string()),
            ("Name", inner_var.name.to_string()),
            ("Type", Dotifier::generate_type_info(Some(inner_var.type_.clone()))),
            ("Mutable", format!("{:}", inner_var.mutable)),
            ("Const", format!("{:}", inner_var.const_)),
            (
                "Declaration",
                match inner_var.declaration {
                    VariableDeclaration::Definition => "Definition",
                    VariableDeclaration::IterationDefinition => "IterationDefinition",
                    VariableDeclaration::Parameter => "Parameter",
                    VariableDeclaration::Input => "Input",
                }
                .to_string(),
            ),
        ];

        let start_idx = self.add_or_get_node(inner_var.id, "Variable".to_string(), labels);

        for reference in &inner_var.references {
            self.edges.push((
                inner_var.id,
                reference.get_id(),
                "reference".to_string(),
                DotColor::Navy,
            ));
        }

        for assignment in &inner_var.assignments {
            self.edges.push((
                inner_var.id,
                assignment.get_id(),
                "assignment".to_string(),
                DotColor::Goldenrod,
            ));
        }

        Fixed(start_idx)
    }

    fn reduce_variable_ref(&mut self, input: &'a VariableRef<'a>, variable: M) -> M {
        let labels = Dotifier::generate_default_expr_labels(input);

        let start_idx = self.add_or_get_node(input.id, "VariableRef".to_string(), labels);

        let Fixed(end_idx) = variable;
        self.graph
            .add_edge(start_idx, end_idx, "variable".to_string(), DotColor::Brown);

        Fixed(start_idx)
    }
}
