//! Logic to convert from an abstract syntax tree (ast) representation to a typed zokrates_program.
//! We implement "unwrap" functions instead of the From trait to handle nested statements (flattening).
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::{aleo_program::types, ast};

impl<'ast> From<ast::Field<'ast>> for types::FieldExpression {
    fn from(field: ast::Field<'ast>) -> Self {
        let number = field.value.parse::<u32>().expect("unable to unwrap field");
        types::FieldExpression::Number(number)
    }
}

impl<'ast> From<ast::Boolean<'ast>> for types::BooleanExpression {
    fn from(boolean: ast::Boolean<'ast>) -> Self {
        let boolean = boolean
            .value
            .parse::<bool>()
            .expect("unable to unwrap boolean");
        types::BooleanExpression::Value(boolean)
    }
}

impl<'ast> From<ast::Value<'ast>> for types::Expression {
    fn from(value: ast::Value<'ast>) -> Self {
        match value {
            ast::Value::Boolean(value) => {
                types::Expression::Boolean(types::BooleanExpression::from(value))
            }
            ast::Value::Field(value) => {
                types::Expression::FieldElement(types::FieldExpression::from(value))
            }
        }
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::Variable {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Variable(variable.value)
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::FieldExpression {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::FieldExpression::Variable(types::Variable(variable.value))
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::BooleanExpression {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::BooleanExpression::Variable(types::Variable(variable.value))
    }
}

impl<'ast> From<ast::Variable<'ast>> for types::Expression {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Expression::Variable(types::Variable(variable.value))
    }
}

impl<'ast> From<ast::NotExpression<'ast>> for types::Expression {
    fn from(expression: ast::NotExpression<'ast>) -> Self {
        types::Expression::Boolean(types::BooleanExpression::Not(Box::new(
            types::BooleanExpression::from(*expression.expression),
        )))
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::BooleanExpression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match types::Expression::from(expression) {
            types::Expression::Boolean(boolean_expression) => boolean_expression,
            types::Expression::Variable(variable) => types::BooleanExpression::Variable(variable),
            types::Expression::FieldElement(field_expression) => unimplemented!(
                "cannot compare field expression {} in boolean expression",
                field_expression
            ),
        }
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::FieldExpression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match types::Expression::from(expression) {
            types::Expression::FieldElement(field_expression) => field_expression,
            types::Expression::Variable(variable) => types::FieldExpression::Variable(variable),
            types::Expression::Boolean(boolean_expression) => unimplemented!(
                "cannot compare boolean expression {} in field expression",
                boolean_expression
            ),
        }
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
            (types::Expression::Boolean(lhs), types::Expression::Boolean(rhs)) => {
                types::BooleanExpression::BoolEq(Box::new(lhs), Box::new(rhs))
            }
            (types::Expression::Boolean(lhs), types::Expression::Variable(rhs)) => {
                types::BooleanExpression::BoolEq(
                    Box::new(lhs),
                    Box::new(types::BooleanExpression::Variable(rhs)),
                )
            }
            (types::Expression::Variable(lhs), types::Expression::Boolean(rhs)) => {
                types::BooleanExpression::BoolEq(
                    Box::new(types::BooleanExpression::Variable(lhs)),
                    Box::new(rhs),
                )
            } //TODO: check case for two variables?
            // Field equality
            (types::Expression::FieldElement(lhs), types::Expression::FieldElement(rhs)) => {
                types::BooleanExpression::FieldEq(Box::new(lhs), Box::new(rhs))
            }
            (types::Expression::FieldElement(lhs), types::Expression::Variable(rhs)) => {
                types::BooleanExpression::FieldEq(
                    Box::new(lhs),
                    Box::new(types::FieldExpression::Variable(rhs)),
                )
            }
            (types::Expression::Variable(lhs), types::Expression::FieldElement(rhs)) => {
                types::BooleanExpression::FieldEq(
                    Box::new(types::FieldExpression::Variable(lhs)),
                    Box::new(rhs),
                )
            }

            (lhs, rhs) => unimplemented!("pattern {} == {} unimplemented", lhs, rhs),
        }
    }

    fn from_neq(expression: ast::BinaryExpression<'ast>) -> Self {
        types::BooleanExpression::Not(Box::new(Self::from_eq(expression)))
    }
}

impl<'ast> From<ast::BinaryExpression<'ast>> for types::Expression {
    fn from(expression: ast::BinaryExpression<'ast>) -> Self {
        match expression.operation {
            // Boolean operations
            ast::BinaryOperator::Or => types::Expression::Boolean(types::BooleanExpression::Or(
                Box::new(types::BooleanExpression::from(*expression.left)),
                Box::new(types::BooleanExpression::from(*expression.right)),
            )),
            ast::BinaryOperator::And => types::Expression::Boolean(types::BooleanExpression::And(
                Box::new(types::BooleanExpression::from(*expression.left)),
                Box::new(types::BooleanExpression::from(*expression.right)),
            )),
            ast::BinaryOperator::Eq => {
                types::Expression::Boolean(types::BooleanExpression::from_eq(expression))
            }
            ast::BinaryOperator::Neq => {
                types::Expression::Boolean(types::BooleanExpression::from_neq(expression))
            }
            ast::BinaryOperator::Geq => types::Expression::Boolean(types::BooleanExpression::Geq(
                Box::new(types::FieldExpression::from(*expression.left)),
                Box::new(types::FieldExpression::from(*expression.right)),
            )),
            ast::BinaryOperator::Gt => types::Expression::Boolean(types::BooleanExpression::Gt(
                Box::new(types::FieldExpression::from(*expression.left)),
                Box::new(types::FieldExpression::from(*expression.right)),
            )),
            ast::BinaryOperator::Leq => types::Expression::Boolean(types::BooleanExpression::Leq(
                Box::new(types::FieldExpression::from(*expression.left)),
                Box::new(types::FieldExpression::from(*expression.right)),
            )),
            ast::BinaryOperator::Lt => types::Expression::Boolean(types::BooleanExpression::Lt(
                Box::new(types::FieldExpression::from(*expression.left)),
                Box::new(types::FieldExpression::from(*expression.right)),
            )),
            // Field operations
            ast::BinaryOperator::Add => {
                types::Expression::FieldElement(types::FieldExpression::Add(
                    Box::new(types::FieldExpression::from(*expression.left)),
                    Box::new(types::FieldExpression::from(*expression.right)),
                ))
            }
            ast::BinaryOperator::Sub => {
                types::Expression::FieldElement(types::FieldExpression::Sub(
                    Box::new(types::FieldExpression::from(*expression.left)),
                    Box::new(types::FieldExpression::from(*expression.right)),
                ))
            }
            ast::BinaryOperator::Mul => {
                types::Expression::FieldElement(types::FieldExpression::Mul(
                    Box::new(types::FieldExpression::from(*expression.left)),
                    Box::new(types::FieldExpression::from(*expression.right)),
                ))
            }
            ast::BinaryOperator::Div => {
                types::Expression::FieldElement(types::FieldExpression::Div(
                    Box::new(types::FieldExpression::from(*expression.left)),
                    Box::new(types::FieldExpression::from(*expression.right)),
                ))
            }
            ast::BinaryOperator::Pow => {
                types::Expression::FieldElement(types::FieldExpression::Pow(
                    Box::new(types::FieldExpression::from(*expression.left)),
                    Box::new(types::FieldExpression::from(*expression.right)),
                ))
            }
        }
    }
}

impl<'ast> From<ast::TernaryExpression<'ast>> for types::Expression {
    fn from(expression: ast::TernaryExpression<'ast>) -> Self {
        // Evaluate expressions to find out result type
        let first = types::BooleanExpression::from(*expression.first);
        let second = types::Expression::from(*expression.second);
        let third = types::Expression::from(*expression.third);

        match (second, third) {
            // Boolean Result
            (types::Expression::Boolean(second), types::Expression::Boolean(third)) => {
                types::Expression::Boolean(types::BooleanExpression::IfElse(
                    Box::new(first),
                    Box::new(second),
                    Box::new(third),
                ))
            }
            (types::Expression::Boolean(second), types::Expression::Variable(third)) => {
                types::Expression::Boolean(types::BooleanExpression::IfElse(
                    Box::new(first),
                    Box::new(second),
                    Box::new(types::BooleanExpression::Variable(third)),
                ))
            }
            (types::Expression::Variable(second), types::Expression::Boolean(third)) => {
                types::Expression::Boolean(types::BooleanExpression::IfElse(
                    Box::new(first),
                    Box::new(types::BooleanExpression::Variable(second)),
                    Box::new(third),
                ))
            }
            // Field Result
            (types::Expression::FieldElement(second), types::Expression::FieldElement(third)) => {
                types::Expression::FieldElement(types::FieldExpression::IfElse(
                    Box::new(first),
                    Box::new(second),
                    Box::new(third),
                ))
            }
            (types::Expression::FieldElement(second), types::Expression::Variable(third)) => {
                types::Expression::FieldElement(types::FieldExpression::IfElse(
                    Box::new(first),
                    Box::new(second),
                    Box::new(types::FieldExpression::Variable(third)),
                ))
            }
            (types::Expression::Variable(second), types::Expression::FieldElement(third)) => {
                types::Expression::FieldElement(types::FieldExpression::IfElse(
                    Box::new(first),
                    Box::new(types::FieldExpression::Variable(second)),
                    Box::new(third),
                ))
            }

            (second, third) => unimplemented!(
                "pattern if {} then {} else {} unimplemented",
                first,
                second,
                third
            ),
        }
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::Expression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match expression {
            ast::Expression::Value(value) => types::Expression::from(value),
            ast::Expression::Variable(variable) => types::Expression::from(variable),
            ast::Expression::Not(expression) => types::Expression::from(expression),
            ast::Expression::Binary(expression) => types::Expression::from(expression),
            ast::Expression::Ternary(expression) => types::Expression::from(expression),
            _ => unimplemented!(),
        }
    }
}

impl<'ast> From<ast::AssignStatement<'ast>> for types::Statement {
    fn from(statement: ast::AssignStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Variable::from(statement.variable),
            types::Expression::from(statement.expression),
        )
    }
}

impl<'ast> From<ast::DefinitionStatement<'ast>> for types::Statement {
    fn from(statement: ast::DefinitionStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Variable::from(statement.variable),
            types::Expression::from(statement.expression),
        )
    }
}

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
            ast::Statement::Assign(statement) => types::Statement::from(statement),
            ast::Statement::Definition(statement) => types::Statement::from(statement),
            ast::Statement::Return(statement) => types::Statement::from(statement),
        }
    }
}

impl<'ast> From<ast::File<'ast>> for types::Program {
    fn from(file: ast::File<'ast>) -> Self {
        // 1. compile ast -> aleo program representation
        file.structs
            .into_iter()
            .for_each(|struct_def| println!("{:#?}", struct_def));
        file.functions
            .into_iter()
            .for_each(|function_def| println!("{:#?}", function_def));
        let statements: Vec<types::Statement> = file
            .statements
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
