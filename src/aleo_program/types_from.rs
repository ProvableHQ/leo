//! Logic to convert from an abstract syntax tree (ast) representation to a typed zokrates_program.
//! We implement "unwrap" functions instead of the From trait to handle nested statements (flattening).
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::aleo_program::StructMember;
use crate::{aleo_program::types, ast};
use std::collections::HashMap;

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
            _ => unimplemented!("expected boolean in boolean expression"),
        }
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::FieldExpression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match types::Expression::from(expression) {
            types::Expression::FieldElement(field_expression) => field_expression,
            types::Expression::Variable(variable) => types::FieldExpression::Variable(variable),
            _ => unimplemented!("expected field in field expression"),
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

impl<'ast> From<ast::RangeOrExpression<'ast>> for types::FieldRangeOrExpression {
    fn from(range_or_expression: ast::RangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            ast::RangeOrExpression::Range(range) => {
                let from = range
                    .from
                    .map(|from| match types::Expression::from(from.0) {
                        types::Expression::FieldElement(field) => field,
                        expression => {
                            unimplemented!("Range bounds should be numbers, found {}", expression)
                        }
                    });
                let to = range.to.map(|to| match types::Expression::from(to.0) {
                    types::Expression::FieldElement(field) => field,
                    expression => {
                        unimplemented!("Range bounds should be numbers, found {}", expression)
                    }
                });

                types::FieldRangeOrExpression::Range(from, to)
            }
            ast::RangeOrExpression::Expression(expression) => {
                match types::Expression::from(expression) {
                    types::Expression::FieldElement(field_expression) => {
                        types::FieldRangeOrExpression::FieldExpression(field_expression)
                    }
                    // types::Expression::ArrayAccess(expression, field), // recursive array access
                    expression => unimplemented!("expression must be field, found {}", expression),
                }
            }
        }
    }
}

impl<'ast> From<ast::PostfixExpression<'ast>> for types::Expression {
    fn from(expression: ast::PostfixExpression<'ast>) -> Self {
        let variable = types::Expression::Variable(types::Variable::from(expression.variable));

        // ast::PostFixExpression contains an array of "accesses": `a(34)[42]` is represented as `[a, [Call(34), Select(42)]]`, but Access call expressions
        // are recursive, so it is `Select(Call(a, 34), 42)`. We apply this transformation here

        // we start with the id, and we fold the array of accesses by wrapping the current value
        expression
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                ast::Access::Call(function) => match acc {
                    types::Expression::Variable(_) => types::Expression::FunctionCall(
                        Box::new(acc),
                        function
                            .expressions
                            .into_iter()
                            .map(|expression| types::Expression::from(expression))
                            .collect(),
                    ),
                    expression => {
                        unimplemented!("only function names are callable, found \"{}\"", expression)
                    }
                },
                ast::Access::Member(struct_member) => types::Expression::StructMemberAccess(
                    Box::new(acc),
                    types::Variable::from(struct_member.variable),
                ),
                ast::Access::Select(array) => types::Expression::ArrayAccess(
                    Box::new(acc),
                    types::FieldRangeOrExpression::from(array.expression),
                ),
            })
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
            ast::Expression::ArrayInline(_expression) => {
                unimplemented!("unknown type for inline array expression")
            }
            ast::Expression::ArrayInitializer(_expression) => {
                unimplemented!("unknown type for array initializer expression")
            }
            ast::Expression::Postfix(expression) => types::Expression::from(expression),
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

impl<'ast> From<ast::Spread<'ast>> for types::BooleanSpread {
    fn from(spread: ast::Spread<'ast>) -> Self {
        let boolean_expression = types::Expression::from(spread.expression);
        match boolean_expression {
            types::Expression::Boolean(expression) => types::BooleanSpread(expression),
            _ => unimplemented!("cannot create boolean spread from field type"),
        }
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::BooleanSpreadOrExpression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match types::Expression::from(expression) {
            types::Expression::Boolean(expression) => {
                types::BooleanSpreadOrExpression::BooleanExpression(expression)
            }
            _ => unimplemented!("cannot create boolean expression from field type"),
        }
    }
}

impl<'ast> From<ast::SpreadOrExpression<'ast>> for types::BooleanSpreadOrExpression {
    fn from(s_or_e: ast::SpreadOrExpression<'ast>) -> Self {
        match s_or_e {
            ast::SpreadOrExpression::Spread(spread) => {
                types::BooleanSpreadOrExpression::Spread(types::BooleanSpread::from(spread))
            }
            ast::SpreadOrExpression::Expression(expression) => {
                match types::Expression::from(expression) {
                    types::Expression::Boolean(expression) => {
                        types::BooleanSpreadOrExpression::BooleanExpression(expression)
                    }
                    _ => unimplemented!("cannot create boolean expression from field type"),
                }
            }
        }
    }
}

impl<'ast> From<ast::Spread<'ast>> for types::FieldSpread {
    fn from(spread: ast::Spread<'ast>) -> Self {
        let field_expression = types::Expression::from(spread.expression);
        match field_expression {
            types::Expression::FieldElement(expression) => types::FieldSpread(expression),
            _ => unimplemented!("cannot create field spread from boolean type"),
        }
    }
}

impl<'ast> From<ast::Expression<'ast>> for types::FieldSpreadOrExpression {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match types::Expression::from(expression) {
            types::Expression::FieldElement(expression) => {
                types::FieldSpreadOrExpression::FieldExpression(expression)
            }
            _ => unimplemented!("cannot create field expression from boolean type"),
        }
    }
}

impl<'ast> From<ast::SpreadOrExpression<'ast>> for types::FieldSpreadOrExpression {
    fn from(s_or_e: ast::SpreadOrExpression<'ast>) -> Self {
        match s_or_e {
            ast::SpreadOrExpression::Spread(spread) => {
                types::FieldSpreadOrExpression::Spread(types::FieldSpread::from(spread))
            }
            ast::SpreadOrExpression::Expression(expression) => {
                types::FieldSpreadOrExpression::from(expression)
            }
        }
    }
}

impl<'ast> From<ast::InlineStructMember<'ast>> for types::StructMember {
    fn from(member: ast::InlineStructMember<'ast>) -> Self {
        types::StructMember {
            variable: types::Variable::from(member.variable),
            expression: types::Expression::from(member.expression),
        }
    }
}

impl<'ast> types::Expression {
    fn from_basic(_ty: ast::BasicType<'ast>, _expression: ast::Expression<'ast>) -> Self {
        unimplemented!("from basic not impl");
    }

    fn from_array(ty: ast::ArrayType<'ast>, expression: ast::Expression<'ast>) -> Self {
        match ty.ty {
            ast::BasicType::Boolean(_ty) => {
                let elements: Vec<Box<types::BooleanSpreadOrExpression>> = match expression {
                    ast::Expression::ArrayInline(array) => array
                        .expressions
                        .into_iter()
                        .map(|s_or_e| Box::new(types::BooleanSpreadOrExpression::from(s_or_e)))
                        .collect(),
                    ast::Expression::ArrayInitializer(array) => {
                        let count = match array.count {
                            ast::Value::Field(f) => {
                                f.value.parse::<usize>().expect("Unable to read array size")
                            }
                            _ => unimplemented!("Array size should be an integer"),
                        };
                        let expression =
                            Box::new(types::BooleanSpreadOrExpression::from(*array.expression));

                        vec![expression; count]
                    }
                    _ => unimplemented!("expected array after array type"),
                };
                types::Expression::Boolean(types::BooleanExpression::Array(elements))
            }
            ast::BasicType::Field(_ty) => {
                let elements: Vec<Box<types::FieldSpreadOrExpression>> = match expression {
                    ast::Expression::ArrayInline(array) => array
                        .expressions
                        .into_iter()
                        .map(|s_or_e| Box::new(types::FieldSpreadOrExpression::from(s_or_e)))
                        .collect(),
                    ast::Expression::ArrayInitializer(array) => {
                        let count = match array.count {
                            ast::Value::Field(f) => {
                                f.value.parse::<usize>().expect("Unable to read array size")
                            }
                            _ => unimplemented!("Array size should be an integer"),
                        };
                        let expression =
                            Box::new(types::FieldSpreadOrExpression::from(*array.expression));

                        vec![expression; count]
                    }
                    _ => unimplemented!("expected array after array type"),
                };
                types::Expression::FieldElement(types::FieldExpression::Array(elements))
            }
        }
    }

    fn from_struct(ty: ast::StructType<'ast>, expression: ast::Expression<'ast>) -> Self {
        let declaration_struct = ty.variable.value;
        match expression {
            ast::Expression::StructInline(inline_struct) => {
                if inline_struct.variable.value != declaration_struct {
                    unimplemented!("Declared struct type must match inline struct type")
                }
                let variable = types::Variable::from(inline_struct.variable);
                let members = inline_struct
                    .members
                    .into_iter()
                    .map(|member| types::StructMember::from(member))
                    .collect::<Vec<StructMember>>();

                types::Expression::Struct(variable, members)
            }
            _ => unimplemented!("Struct declaration must be followed by inline struct"),
        }
    }

    fn from_type(ty: ast::Type<'ast>, expression: ast::Expression<'ast>) -> Self {
        match ty {
            ast::Type::Basic(ty) => Self::from_basic(ty, expression),
            ast::Type::Array(ty) => Self::from_array(ty, expression),
            ast::Type::Struct(ty) => Self::from_struct(ty, expression),
        }
    }
}

impl<'ast> From<ast::DefinitionStatement<'ast>> for types::Statement {
    fn from(statement: ast::DefinitionStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Variable::from(statement.variable),
            types::Expression::from_type(statement.ty, statement.expression),
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

impl<'ast> From<ast::ForStatement<'ast>> for types::Statement {
    fn from(statement: ast::ForStatement<'ast>) -> Self {
        types::Statement::For(
            types::Variable::from(statement.index),
            types::FieldExpression::from(statement.start),
            types::FieldExpression::from(statement.stop),
            statement
                .statements
                .into_iter()
                .map(|statement| types::Statement::from(statement))
                .collect(),
        )
    }
}

impl<'ast> From<ast::Statement<'ast>> for types::Statement {
    fn from(statement: ast::Statement<'ast>) -> Self {
        match statement {
            ast::Statement::Assign(statement) => types::Statement::from(statement),
            ast::Statement::Definition(statement) => types::Statement::from(statement),
            ast::Statement::Iteration(statement) => types::Statement::from(statement),
            ast::Statement::Return(statement) => types::Statement::from(statement),
        }
    }
}

impl<'ast> From<ast::BasicType<'ast>> for types::Type {
    fn from(basic_type: ast::BasicType<'ast>) -> Self {
        match basic_type {
            ast::BasicType::Field(_ty) => types::Type::FieldElement,
            ast::BasicType::Boolean(_ty) => types::Type::Boolean,
        }
    }
}

impl<'ast> From<ast::ArrayType<'ast>> for types::Type {
    fn from(array_type: ast::ArrayType<'ast>) -> Self {
        let element_type = Box::new(types::Type::from(array_type.ty));
        let count = match array_type.count {
            ast::Value::Field(f) => f.value.parse::<usize>().expect("Unable to read array size"),
            _ => unimplemented!("Array size should be an integer"),
        };
        types::Type::Array(element_type, count)
    }
}

impl<'ast> From<ast::StructType<'ast>> for types::Type {
    fn from(struct_type: ast::StructType<'ast>) -> Self {
        types::Type::Struct(types::Variable::from(struct_type.variable))
    }
}

impl<'ast> From<ast::Type<'ast>> for types::Type {
    fn from(ty: ast::Type<'ast>) -> Self {
        match ty {
            ast::Type::Basic(ty) => types::Type::from(ty),
            ast::Type::Array(ty) => types::Type::from(ty),
            ast::Type::Struct(ty) => types::Type::from(ty),
        }
    }
}

impl<'ast> From<ast::StructField<'ast>> for types::StructField {
    fn from(struct_field: ast::StructField<'ast>) -> Self {
        types::StructField {
            variable: types::Variable::from(struct_field.variable),
            ty: types::Type::from(struct_field.ty),
        }
    }
}

impl<'ast> From<ast::Struct<'ast>> for types::Struct {
    fn from(struct_definition: ast::Struct<'ast>) -> Self {
        let variable = types::Variable::from(struct_definition.variable);
        let fields = struct_definition
            .fields
            .into_iter()
            .map(|struct_field| types::StructField::from(struct_field))
            .collect();

        types::Struct { variable, fields }
    }
}

impl From<ast::Visibility> for types::Visibility {
    fn from(visibility: ast::Visibility) -> Self {
        match visibility {
            ast::Visibility::Private(_private) => types::Visibility::Private,
            ast::Visibility::Public(_public) => types::Visibility::Public,
        }
    }
}

impl<'ast> From<ast::Parameter<'ast>> for types::Parameter {
    fn from(parameter: ast::Parameter<'ast>) -> Self {
        let ty = types::Type::from(parameter.ty);
        let variable = types::Variable::from(parameter.variable);

        if parameter.visibility.is_some() {
            let visibility = Some(types::Visibility::from(parameter.visibility.unwrap()));
            types::Parameter {
                visibility,
                ty,
                variable,
            }
        } else {
            types::Parameter {
                visibility: None,
                ty,
                variable,
            }
        }
    }
}

impl<'ast> From<ast::FunctionName<'ast>> for types::FunctionName {
    fn from(name: ast::FunctionName<'ast>) -> Self {
        types::FunctionName(name.value)
    }
}

impl<'ast> From<ast::Function<'ast>> for types::Function {
    fn from(function_definition: ast::Function<'ast>) -> Self {
        let function_name = types::FunctionName::from(function_definition.function_name);
        let parameters = function_definition
            .parameters
            .into_iter()
            .map(|parameter| types::Parameter::from(parameter))
            .collect();
        let returns = function_definition
            .returns
            .into_iter()
            .map(|return_type| types::Type::from(return_type))
            .collect();
        let statements = function_definition
            .statements
            .into_iter()
            .map(|statement| types::Statement::from(statement))
            .collect();

        types::Function {
            function_name,
            parameters,
            returns,
            statements,
        }
    }
}

impl<'ast> From<ast::File<'ast>> for types::Program {
    fn from(file: ast::File<'ast>) -> Self {
        // Compiled ast -> aleo program representation

        let mut structs = HashMap::new();
        let mut functions = HashMap::new();

        file.structs.into_iter().for_each(|struct_def| {
            structs.insert(
                types::Variable::from(struct_def.variable.clone()),
                types::Struct::from(struct_def),
            );
        });
        file.functions.into_iter().for_each(|function_def| {
            functions.insert(
                types::FunctionName::from(function_def.function_name.clone()),
                types::Function::from(function_def),
            );
        });

        types::Program {
            id: "main".into(),
            structs,
            functions,
            arguments: vec![],
            returns: vec![],
        }
    }
}
