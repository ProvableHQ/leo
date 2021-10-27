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

impl<'a, 'b> MonoidalReducerExpression<'a, M> for Dotifier<'a, 'b> {
    fn reduce_expression(&mut self, input: &'a Expression<'a>, value: M) -> M {
        if let Some(parent) = input.get_parent() {
            self.edges.push((
                input.get_id(),
                parent.get_id(),
                ("parent".to_string(), LabelType::Label),
                Some(("red".to_string(), LabelType::Label)),
            ))
        }
        value
    }

    fn reduce_array_access(&mut self, input: &'a ArrayAccessExpression<'a>, array: M, index: M) -> M {
        let label = format!(
            "Array Access Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = array;
        self.add_edge(
            start_idx,
            end_idx,
            ("array".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        let Fixed(end_idx) = index;
        self.add_edge(
            start_idx,
            end_idx,
            ("index".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_array_init(&mut self, input: &'a ArrayInitExpression<'a>, element: M) -> M {
        let label = format!(
            "Array Init Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = element;
        self.add_edge(
            start_idx,
            end_idx,
            ("element".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_array_inline(&mut self, input: &'a ArrayInlineExpression<'a>, elements: Vec<M>) -> M {
        let label = format!(
            "Array Inline Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in elements.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("element_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_array_range_access(
        &mut self,
        input: &'a ArrayRangeAccessExpression<'a>,
        array: M,
        left: Option<M>,
        right: Option<M>,
    ) -> M {
        let label = format!(
            "Array Range Access Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = array;
        self.add_edge(
            start_idx,
            end_idx,
            ("array".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

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
            );
        }

        Fixed(start_idx)
    }

    fn reduce_binary(&mut self, input: &'a BinaryExpression<'a>, left: M, right: M) -> M {
        let label = format!(
            "Binary Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = left;
        self.add_edge(
            start_idx,
            end_idx,
            ("left".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        let Fixed(end_idx) = right;
        self.add_edge(
            start_idx,
            end_idx,
            ("right".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_call(&mut self, input: &'a CallExpression<'a>, target: Option<M>, arguments: Vec<M>) -> M {
        let label = format!(
            "Call Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in arguments.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("argument_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        if let Some(Fixed(end_idx)) = target {
            self.add_edge(
                start_idx,
                end_idx,
                ("target".to_string(), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            )
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_access(&mut self, input: &'a CircuitAccessExpression<'a>, target: Option<M>) -> M {
        let label = format!(
            "Circuit Access Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        if let Some(Fixed(end_idx)) = target {
            self.add_edge(
                start_idx,
                end_idx,
                ("target".to_string(), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_init(&mut self, input: &'a CircuitInitExpression<'a>, values: Vec<M>) -> M {
        let label = format!(
            "Circuit Init Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in values.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("value_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_ternary_expression(
        &mut self,
        input: &'a TernaryExpression<'a>,
        condition: M,
        if_true: M,
        if_false: M,
    ) -> M {
        let label = format!(
            "Ternary Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
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

        let Fixed(end_idx) = if_false;
        self.add_edge(
            start_idx,
            end_idx,
            ("if_false".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_cast_expression(&mut self, input: &'a CastExpression<'a>, inner: M) -> M {
        let label = format!(
            "Cast Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = inner;
        self.add_edge(
            start_idx,
            end_idx,
            ("inner".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_lengthof_expression(&mut self, input: &'a LengthOfExpression<'a>, inner: M) -> M {
        let label = format!(
            "LengthOf Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = inner;
        self.add_edge(
            start_idx,
            end_idx,
            ("inner".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_constant(&mut self, input: &'a Constant<'a>) -> M {
        let label = format!(
            "Constant\nNode ID: {:}\nType: {:}\nValue: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            input.value,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        Fixed(start_idx)
    }

    fn reduce_tuple_access(&mut self, input: &'a TupleAccessExpression<'a>, tuple_ref: M) -> M {
        let label = format!(
            "Tuple Access Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = tuple_ref;
        self.add_edge(
            start_idx,
            end_idx,
            ("tuple_ref".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_tuple_init(&mut self, input: &'a TupleInitExpression<'a>, values: Vec<M>) -> M {
        let label = format!(
            "Tuple Init Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in values.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("value_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_unary(&mut self, input: &'a UnaryExpression<'a>, inner: M) -> M {
        let label = format!(
            "Unary Expression\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = inner;
        self.add_edge(
            start_idx,
            end_idx,
            ("inner".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_variable(&mut self, input: &'a Variable<'a>) -> M {
        let inner_var = input.borrow();
        let label = format!(
            "Variable\nNode ID: {:}\n\nName: {:}\nType: {:}\nMutable: {:}\nConst: {:}\nDeclaration: {:}",
            inner_var.id,
            inner_var.name,
            inner_var.type_,
            inner_var.mutable,
            inner_var.const_,
            match inner_var.declaration {
                VariableDeclaration::Definition => "Definition",
                VariableDeclaration::IterationDefinition => "IterationDefinition",
                VariableDeclaration::Parameter => "Parameter",
                VariableDeclaration::Input => "Input",
            },
        );
        let start_idx = self.add_or_get_node(inner_var.id, label, LabelType::Esc);

        for reference in &inner_var.references {
            self.edges.push((
                inner_var.id,
                reference.get_id(),
                ("reference".to_string(), LabelType::Label),
                Some(("navy".to_string(), LabelType::Label)),
            ));
        }

        for assignment in &inner_var.assignments {
            self.edges.push((
                inner_var.id,
                assignment.get_id(),
                ("assignment".to_string(), LabelType::Label),
                Some(("goldenrod".to_string(), LabelType::Label)),
            ));
        }

        Fixed(start_idx)
    }

    fn reduce_variable_ref(&mut self, input: &'a VariableRef<'a>, variable: M) -> M {
        let label = format!(
            "Variable Ref\nNode ID: {:}\nType: {:}\n\n{:}",
            input.id,
            Dotifier::generate_type_info(input.get_type()),
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        let Fixed(end_idx) = variable;
        self.add_edge(
            start_idx,
            end_idx,
            ("variable".to_string(), LabelType::Label),
            Some(("brown".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }
}
