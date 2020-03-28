//! Logic to convert from an abstract syntax tree (ast) representation to a typed zokrates_program.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::{
    ast,
    zokrates_program::{program, types, NodeValue},
};

impl<'ast> From<ast::Boolean<'ast>> for types::ExpressionNode<'ast> {
    fn from(boolean: ast::Boolean<'ast>) -> Self {
        types::Expression::Boolean(
            boolean
                .value
                .parse::<bool>()
                .expect("unable to parse boolean"),
        )
        .span(boolean.span)
    }
}

impl<'ast> From<ast::Field<'ast>> for types::ExpressionNode<'ast> {
    fn from(field: ast::Field<'ast>) -> Self {
        types::Expression::Field(field.span.as_str()).span(field.span)
    }
}

impl<'ast> From<ast::Value<'ast>> for types::ExpressionNode<'ast> {
    fn from(value: ast::Value<'ast>) -> Self {
        match value {
            ast::Value::Boolean(boolean) => types::ExpressionNode::from(boolean),
            ast::Value::Field(field) => types::ExpressionNode::from(field),
        }
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::VariableNode<'ast> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Variable {
            id: variable.span.as_str(),
        }
        .span(variable.span)
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::ExpressionNode<'ast> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Expression::Variable(types::VariableNode::from(variable.clone())).span(variable.span)
    }
}

impl<'ast> From<ast::NotExpression<'ast>> for types::ExpressionNode<'ast> {
    fn from(expression: ast::NotExpression<'ast>) -> Self {
        types::Expression::Not(Box::new(types::ExpressionNode::from(
            *expression.expression,
        )))
        .span(expression.span)
    }
}

impl<'ast> From<ast::BinaryExpression<'ast>> for types::ExpressionNode<'ast> {
    fn from(expression: ast::BinaryExpression<'ast>) -> Self {
        match expression.operation {
            ast::BinaryOperator::Or => types::Expression::Or(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::And => types::Expression::And(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Eq => types::Expression::Eq(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Neq => types::Expression::Neq(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Geq => types::Expression::Geq(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Gt => types::Expression::Gt(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Leq => types::Expression::Leq(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Lt => types::Expression::Lt(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Add => types::Expression::Add(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Sub => types::Expression::Sub(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Mul => types::Expression::Mul(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Div => types::Expression::Div(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
            ast::BinaryOperator::Pow => types::Expression::Pow(
                Box::new(types::ExpressionNode::from(*expression.left)),
                Box::new(types::ExpressionNode::from(*expression.right)),
            ),
        }
        .span(expression.span)
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::ExpressionNode<'ast> {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match expression {
            ast::Expression::Value(expression) => types::ExpressionNode::from(expression),
            ast::Expression::Variable(expression) => types::ExpressionNode::from(expression),
            ast::Expression::Not(expression) => types::ExpressionNode::from(expression),
            ast::Expression::Binary(expression) => types::ExpressionNode::from(expression),
        }
    }
}

impl<'ast> From<ast::AssignStatement<'ast>> for types::StatementNode<'ast> {
    fn from(statement: ast::AssignStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::VariableNode::from(statement.variable),
            types::ExpressionNode::from(statement.expression),
        )
        .span(statement.span)
    }
}

impl<'ast> From<ast::ReturnStatement<'ast>> for types::StatementNode<'ast> {
    fn from(statement: ast::ReturnStatement<'ast>) -> Self {
        types::Statement::Return(
            types::ExpressionList {
                expressions: statement
                    .expressions
                    .into_iter()
                    .map(|expression| types::ExpressionNode::from(expression))
                    .collect(),
            }
            .span(statement.span.clone()),
        )
        .span(statement.span)
    }
}

impl<'ast> From<ast::Statement<'ast>> for types::StatementNode<'ast> {
    fn from(statement: ast::Statement<'ast>) -> Self {
        match statement {
            ast::Statement::Assign(statement) => types::StatementNode::from(statement),
            ast::Statement::Return(statement) => types::StatementNode::from(statement),
        }
    }
}

impl<'ast> From<ast::File<'ast>> for program::Program<'ast> {
    fn from(file: ast::File<'ast>) -> Self {
        program::Program {
            nodes: file
                .statement
                .iter()
                .map(|statement| types::StatementNode::from(statement.clone()))
                .collect(),
        }
    }
}
