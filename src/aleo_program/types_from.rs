//! Logic to convert from an abstract syntax tree (ast) representation to a typed zokrates_program.
//! We implement "unwrap" functions instead of the From trait to handle nested statements (flattening).
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::{BooleanExpression, Expression, FieldExpression};
use crate::{aleo_program::types, ast};

// use crate::{
//     ast,
//     aleo_program::{program, types, NodeValue},
// };
//
// impl<'ast> From<ast::Boolean<'ast>> for types::ExpressionNode<'ast> {
//     fn from(boolean: ast::Boolean<'ast>) -> Self {
//         types::Expression::Boolean(
//             boolean
//                 .value
//                 .parse::<bool>()
//                 .expect("unable to parse boolean"),
//         )
//         .span(boolean.span)
//     }
// }
//
// impl<'ast> From<ast::Field<'ast>> for types::ExpressionNode<'ast> {
//     fn from(field: ast::Field<'ast>) -> Self {
//         types::Expression::Field(field.span.as_str()).span(field.span)
//     }
// }
//
// impl<'ast> From<ast::Value<'ast>> for types::ExpressionNode<'ast> {
//     fn from(value: ast::Value<'ast>) -> Self {
//         match value {
//             ast::Value::Boolean(boolean) => types::ExpressionNode::from(boolean),
//             ast::Value::Field(field) => types::ExpressionNode::from(field),
//         }
//     }
// }
//
// impl<'ast> From<ast::Variable<'ast>> for types::VariableNode<'ast> {
//     fn from(variable: ast::Variable<'ast>) -> Self {
//         types::Variable {
//             id: variable.span.as_str(),
//         }
//         .span(variable.span)
//     }
// }
//
// impl<'ast> From<ast::Variable<'ast>> for types::ExpressionNode<'ast> {
//     fn from(variable: ast::Variable<'ast>) -> Self {
//         types::Expression::Variable(types::VariableNode::from(variable.clone())).span(variable.span)
//     }
// }
//
// impl<'ast> From<ast::NotExpression<'ast>> for types::ExpressionNode<'ast> {
//     fn from(expression: ast::NotExpression<'ast>) -> Self {
//         types::Expression::Not(Box::new(types::ExpressionNode::from(
//             *expression.expression,
//         )))
//         .span(expression.span)
//     }
// }

impl<'ast> From<ast::Field<'ast>> for types::FieldExpression {
    fn from(field: ast::Field<'ast>) -> Self {
        let number = field.value.parse::<u32>().expect("unable to unwrap field");
        FieldExpression::Number(number)
    }
}

impl<'ast> From<ast::Boolean<'ast>> for types::BooleanExpression {
    fn from(boolean: ast::Boolean<'ast>) -> Self {
        let boolean = boolean
            .value
            .parse::<bool>()
            .expect("unable to unwrap boolean");
        BooleanExpression::Value(boolean)
    }
}

impl<'ast> From<ast::Value<'ast>> for types::Expression {
    fn from(value: ast::Value<'ast>) -> Self {
        match value {
            ast::Value::Boolean(value) => Expression::Boolean(BooleanExpression::from(value)),
            ast::Value::Field(value) => Expression::FieldElement(FieldExpression::from(value)),
        }
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::FieldExpression {
    fn from(variable: ast::Variable<'ast>) -> Self {
        FieldExpression::Variable(types::Variable(variable.value))
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::BooleanExpression {
    fn from(variable: ast::Variable<'ast>) -> Self {
        BooleanExpression::Variable(types::Variable(variable.value))
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::Expression {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Expression::Variable(types::Variable(variable.value))
    }
}

impl<'ast> types::BooleanExpression {
    /// Find out which types we are comparing and output the corresponding expression.
    fn from_eq(expression: ast::BinaryExpression<'ast>) -> Self {
        let left = types::Expression::from(*expression.left);
        let right = types::Expression::from(*expression.right);

        // When matching a variable, look at the opposite side to see what we are comparing to and assume that variable type
        match (left, right) {
            // Boolean equality
            (Expression::Boolean(lhs), Expression::Boolean(rhs)) => {
                BooleanExpression::BoolEq(Box::new(lhs), Box::new(rhs))
            }
            (Expression::Boolean(lhs), Expression::Variable(rhs)) => {
                BooleanExpression::BoolEq(Box::new(lhs), Box::new(BooleanExpression::Variable(rhs)))
            }
            (Expression::Variable(lhs), Expression::Boolean(rhs)) => {
                BooleanExpression::BoolEq(Box::new(BooleanExpression::Variable(lhs)), Box::new(rhs))
            }
            // Field equality
            (Expression::FieldElement(lhs), Expression::FieldElement(rhs)) => {
                BooleanExpression::FieldEq(Box::new(lhs), Box::new(rhs))
            }
            (Expression::FieldElement(lhs), Expression::Variable(rhs)) => {
                BooleanExpression::FieldEq(Box::new(lhs), Box::new(FieldExpression::Variable(rhs)))
            }
            (Expression::Variable(lhs), Expression::FieldElement(rhs)) => {
                BooleanExpression::FieldEq(Box::new(FieldExpression::Variable(lhs)), Box::new(rhs))
            }

            (_, _) => unimplemented!(),
        }
    }
}

impl<'ast> From<ast::BinaryExpression<'ast>> for types::Expression {
    fn from(expression: ast::BinaryExpression<'ast>) -> Self {
        match expression.operation {
            // Boolean operations
            ast::BinaryOperator::Or => unimplemented!(),
            ast::BinaryOperator::And => unimplemented!(),
            ast::BinaryOperator::Eq => {
                types::Expression::Boolean(BooleanExpression::from_eq(expression))
            }
            ast::BinaryOperator::Neq => unimplemented!(),
            ast::BinaryOperator::Geq => unimplemented!(),
            ast::BinaryOperator::Gt => unimplemented!(),
            ast::BinaryOperator::Leq => unimplemented!(),
            ast::BinaryOperator::Lt => unimplemented!(),
            // Field operations
            ast::BinaryOperator::Add => unimplemented!(),
            ast::BinaryOperator::Sub => unimplemented!(),
            ast::BinaryOperator::Mul => unimplemented!(),
            ast::BinaryOperator::Div => unimplemented!(),
            ast::BinaryOperator::Pow => unimplemented!(),
        }
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::Expression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match expression {
            ast::Expression::Value(value) => types::Expression::from(value),
            ast::Expression::Variable(variable) => types::Expression::from(variable),
            ast::Expression::Not(_expression) => unimplemented!(),
            ast::Expression::Binary(expression) => types::Expression::from(expression),
        }
    }
}

// impl<'ast> From<ast::AssignStatement<'ast>> for types::StatementNode<'ast> {
//     fn from(statement: ast::AssignStatement<'ast>) -> Self {
//         types::Statement::Definition(
//             types::VariableNode::from(statement.variable),
//             types::ExpressionNode::from(statement.expression),
//         )
//         .span(statement.span)
//     }
// }
//
impl<'ast> From<ast::ReturnStatement<'ast>> for types::Statement {
    fn from(statement: ast::ReturnStatement<'ast>) -> Self {
        types::Statement::Return(
            statement
                .expressions
                .into_iter()
                .map(|expression| types::Expression::from(expression))
                .collect(),
        )
    }
}

impl<'ast> From<ast::Statement<'ast>> for types::Statement {
    fn from(statement: ast::Statement<'ast>) -> Self {
        match statement {
            ast::Statement::Assign(_statement) => unimplemented!(),
            ast::Statement::Return(statement) => types::Statement::from(statement),
        }
    }
}

impl<'ast> From<ast::File<'ast>> for types::Program {
    fn from(file: ast::File<'ast>) -> Self {
        let statements = file
            .statement
            .into_iter()
            .map(|statement| types::Statement::from(statement))
            .collect();

        types::Program {
            id: "main".into(),
            statements,
            arguments: vec![],
            returns: vec![],
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_file() {
//
//     }
// }

// impl<'ast> From<ast::Variable<'ast>> for types::LinearCombination {
//     fn from(variable: ast::Variable<'ast>) -> Self {
//         LinearCombination(vec![Variable { id: 1, value: variable.value }])
//     }
// }
//
// impl<'ast> From<ast::Boolean<'ast>> for types::LinearCombination {
//     fn from(boolean: ast::Boolean<'ast>) -> Self {
//         LinearCombination(vec![Variable { id: -1, value: boolean.value }])
//     }
// }
//
// impl<'ast> From<ast::Field<'ast>> for types::LinearCombination {
//     fn from(field: ast::Field<'ast>) -> Self {
//         LinearCombination(vec![Variable { id: 0, value: field.value }])
//     }
// }
// impl<'ast> From<ast::Value<'ast>> for types::LinearCombination {
//     fn from(value: ast::Value<'ast>) -> Self {
//         match value {
//             ast::Value::Boolean(boolean) => types::LinearCombination::from(boolean),
//             ast::Value::Field(field) => types::LinearCombination::from(field),
//         }
//     }
// }
//
// impl<'ast> From<ast::Expression<'ast>> for types::LinearCombination {
//     fn from(expression: ast::Expression<'ast>) -> Self {
//         match expression {
//             ast::Expression::Value(value) => types::LinearCombination::from(value),
//             ast::Expression::Variable(variable) => types::LinearCombination::from(variable),
//             ast::Expression::Not(_) => unimplemented!(),
//             ast::Expression::Binary(_) => unimplemented!(),
//         }
//     }
// }
//
// impl<'ast> types::Expression {
//     fn unwrap_expression(expression: ast::Expression<'ast>) -> Vec<Self> {
//         match expression {
//             ast::Expression::Value(value) => unimplemented!(),
//             ast::Expression::Variable(variable) => unimplemented!(),
//             ast::Expression::Not(expression) => unimplemented!(),
//             ast::Expression::Binary(expression) => Self::unwrap_binary(expression),
//         }
//     }
//
//     fn unwrap_binary(expression: ast::BinaryExpression<'ast>) -> Vec<Self> {
//         match expression.operation {
//             ast::BinaryOperator::Eq => ,
//             _ => unimplemented!()
//         }
//     }
//
//     fn unwrap_eq(expression: ast::BinaryExpression<'ast>) -> Vec<Self> {
//
//     }
// }
//
// impl<'ast> types::Statement {
//     fn unwrap_statement(statement: ast::Statement<'ast>) -> Self {
//         match statement {
//             ast::Statement::Assign(statement) => unimplemented!(),
//             ast::Statement::Return(statement) => Self::unwrap_return(statement),
//         }
//     }
//
//     fn unwrap_return(statement: ast::ReturnStatement<'ast>) -> Self {
//         let mut expressions: Vec<types::Expression> = vec![];
//
//         statement
//             .expressions
//             .into_iter()
//             .map(|expression| {
//                 expressions.extend_from_slice(&types::Expression::unwrap_expression(expression))
//             });
//
//         types::Statement::Return(expressions)
//     }
// }
//
// impl<'ast> From<ast::Statement<'ast>> for types::Statement {
//     fn from(statement: ast::Statement<'ast>) -> Self {
//         match statement {
//             ast::Statement::Assign(statement) => unimplemented!()
//         }
//     }
// }
