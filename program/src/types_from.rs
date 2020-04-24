//! Logic to convert from an abstract syntax tree (ast) representation to a typed aleo program.

use crate::ast;
use crate::{types, Import, PathString};

use snarkos_models::curves::{Field, PrimeField};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

/// pest ast -> types::Variable

impl<'ast, F: Field + PrimeField> From<ast::Variable<'ast>> for types::Variable<F> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Variable {
            name: variable.value,
            _field: PhantomData::<F>,
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Variable<'ast>> for types::Expression<F> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Expression::Variable(types::Variable::from(variable))
    }
}
/// pest ast - types::Integer

impl<'ast, F: Field + PrimeField> From<ast::U32<'ast>> for types::Expression<F> {
    fn from(field: ast::U32<'ast>) -> Self {
        types::Expression::Integer(types::Integer::U32(
            field
                .number
                .value
                .parse::<u32>()
                .expect("unable to parse u32"),
        ))
    }
}

impl<'ast, F: Field + PrimeField> From<ast::RangeOrExpression<'ast>>
    for types::RangeOrExpression<F>
{
    fn from(range_or_expression: ast::RangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            ast::RangeOrExpression::Range(range) => {
                let from = range
                    .from
                    .map(|from| match types::Expression::<F>::from(from.0) {
                        types::Expression::Integer(number) => number,
                        expression => {
                            unimplemented!("Range bounds should be integers, found {}", expression)
                        }
                    });
                let to = range.to.map(|to| match types::Expression::<F>::from(to.0) {
                    types::Expression::Integer(number) => number,
                    expression => {
                        unimplemented!("Range bounds should be intgers, found {}", expression)
                    }
                });

                types::RangeOrExpression::Range(from, to)
            }
            ast::RangeOrExpression::Expression(expression) => {
                types::RangeOrExpression::Expression(types::Expression::from(expression))
            }
        }
    }
}

/// pest ast -> types::FieldExpression

impl<'ast, F: Field + PrimeField> From<ast::Field<'ast>> for types::Expression<F> {
    fn from(field: ast::Field<'ast>) -> Self {
        types::Expression::FieldElement(F::from_str(&field.number.value).unwrap_or_default())
    }
}

/// pest ast -> types::Boolean

impl<'ast, F: Field + PrimeField> From<ast::Boolean<'ast>> for types::Expression<F> {
    fn from(boolean: ast::Boolean<'ast>) -> Self {
        types::Expression::Boolean(
            boolean
                .value
                .parse::<bool>()
                .expect("unable to parse boolean"),
        )
    }
}

/// pest ast -> types::Expression

impl<'ast, F: Field + PrimeField> From<ast::Value<'ast>> for types::Expression<F> {
    fn from(value: ast::Value<'ast>) -> Self {
        match value {
            ast::Value::U32(num) => types::Expression::from(num),
            ast::Value::Field(fe) => types::Expression::from(fe),
            ast::Value::Boolean(bool) => types::Expression::from(bool),
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::NotExpression<'ast>> for types::Expression<F> {
    fn from(expression: ast::NotExpression<'ast>) -> Self {
        types::Expression::Not(Box::new(types::Expression::from(*expression.expression)))
    }
}

impl<'ast, F: Field + PrimeField> From<ast::SpreadOrExpression<'ast>>
    for types::SpreadOrExpression<F>
{
    fn from(s_or_e: ast::SpreadOrExpression<'ast>) -> Self {
        match s_or_e {
            ast::SpreadOrExpression::Spread(spread) => {
                types::SpreadOrExpression::Spread(types::Expression::from(spread.expression))
            }
            ast::SpreadOrExpression::Expression(expression) => {
                types::SpreadOrExpression::Expression(types::Expression::from(expression))
            }
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::BinaryExpression<'ast>> for types::Expression<F> {
    fn from(expression: ast::BinaryExpression<'ast>) -> Self {
        match expression.operation {
            // Boolean operations
            ast::BinaryOperator::Or => types::Expression::Or(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::And => types::Expression::And(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Eq => types::Expression::Eq(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Neq => {
                types::Expression::Not(Box::new(types::Expression::from(expression)))
            }
            ast::BinaryOperator::Geq => types::Expression::Geq(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Gt => types::Expression::Gt(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Leq => types::Expression::Leq(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Lt => types::Expression::Lt(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            // Number operations
            ast::BinaryOperator::Add => types::Expression::Add(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Sub => types::Expression::Sub(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Mul => types::Expression::Mul(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Div => types::Expression::Div(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            ast::BinaryOperator::Pow => types::Expression::Pow(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::TernaryExpression<'ast>> for types::Expression<F> {
    fn from(expression: ast::TernaryExpression<'ast>) -> Self {
        types::Expression::IfElse(
            Box::new(types::Expression::from(*expression.first)),
            Box::new(types::Expression::from(*expression.second)),
            Box::new(types::Expression::from(*expression.third)),
        )
    }
}

impl<'ast, F: Field + PrimeField> From<ast::PostfixExpression<'ast>> for types::Expression<F> {
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
                    types::Expression::Variable(variable) => types::Expression::FunctionCall(
                        variable,
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
                ast::Access::Array(array) => types::Expression::ArrayAccess(
                    Box::new(acc),
                    Box::new(types::RangeOrExpression::from(array.expression)),
                ),
            })
    }
}

impl<'ast, F: Field + PrimeField> From<ast::ArrayInlineExpression<'ast>> for types::Expression<F> {
    fn from(array: ast::ArrayInlineExpression<'ast>) -> Self {
        types::Expression::Array(
            array
                .expressions
                .into_iter()
                .map(|s_or_e| Box::new(types::SpreadOrExpression::from(s_or_e)))
                .collect(),
        )
    }
}
impl<'ast, F: Field + PrimeField> From<ast::ArrayInitializerExpression<'ast>>
    for types::Expression<F>
{
    fn from(array: ast::ArrayInitializerExpression<'ast>) -> Self {
        let count = types::Expression::<F>::get_count(array.count);
        let expression = Box::new(types::SpreadOrExpression::from(*array.expression));

        types::Expression::Array(vec![expression; count])
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Expression<'ast>> for types::Expression<F> {
    fn from(expression: ast::Expression<'ast>) -> Self {
        match expression {
            ast::Expression::Value(value) => types::Expression::from(value),
            ast::Expression::Variable(variable) => types::Expression::from(variable),
            ast::Expression::Not(expression) => types::Expression::from(expression),
            ast::Expression::Binary(expression) => types::Expression::from(expression),
            ast::Expression::Ternary(expression) => types::Expression::from(expression),
            ast::Expression::ArrayInline(expression) => types::Expression::from(expression),
            ast::Expression::ArrayInitializer(expression) => types::Expression::from(expression),
            ast::Expression::StructInline(_expression) => {
                unimplemented!("unknown type for inline struct expression")
            }
            ast::Expression::Postfix(expression) => types::Expression::from(expression),
            // _ => unimplemented!(),
        }
    }
}

/// pest ast -> typed types::Expression
/// For defined types (ex: u32[4]) we manually construct the expression instead of implementing the From trait.
/// This saves us from having to resolve things at a later point in time.
impl<'ast, F: Field + PrimeField> types::Expression<F> {
    fn get_count(count: ast::Value<'ast>) -> usize {
        match count {
            ast::Value::U32(f) => f
                .number
                .value
                .parse::<usize>()
                .expect("Unable to read array size"),
            size => unimplemented!("Array size should be an integer {}", size),
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
                    .collect::<Vec<types::StructMember<F>>>();

                types::Expression::Struct(variable, members)
            }
            _ => unimplemented!("Struct declaration must be followed by inline struct"),
        }
    }

    fn from_type(ty: ast::Type<'ast>, expression: ast::Expression<'ast>) -> Self {
        match ty {
            ast::Type::Basic(_ty) => Self::from(expression),
            ast::Type::Array(_ty) => Self::from(expression),
            ast::Type::Struct(ty) => Self::from_struct(ty, expression),
        }
    }
}

/// pest ast -> types::Assignee

impl<'ast, F: Field + PrimeField> From<ast::Variable<'ast>> for types::Assignee<F> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Assignee::Variable(types::Variable::from(variable))
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Assignee<'ast>> for types::Assignee<F> {
    fn from(assignee: ast::Assignee<'ast>) -> Self {
        let variable = types::Assignee::from(assignee.variable);

        // we start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                ast::AssigneeAccess::Array(array) => types::Assignee::Array(
                    Box::new(acc),
                    types::RangeOrExpression::from(array.expression),
                ),
                ast::AssigneeAccess::Member(struct_member) => types::Assignee::StructMember(
                    Box::new(acc),
                    types::Variable::from(struct_member.variable),
                ),
            })
    }
}

/// pest ast -> types::Statement

impl<'ast, F: Field + PrimeField> From<ast::ReturnStatement<'ast>> for types::Statement<F> {
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

impl<'ast, F: Field + PrimeField> From<ast::ForStatement<'ast>> for types::Statement<F> {
    fn from(statement: ast::ForStatement<'ast>) -> Self {
        let from = match types::Expression::<F>::from(statement.start) {
            types::Expression::Integer(number) => number,
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };
        let to = match types::Expression::<F>::from(statement.stop) {
            types::Expression::Integer(number) => number,
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };

        types::Statement::For(
            types::Variable::from(statement.index),
            from,
            to,
            statement
                .statements
                .into_iter()
                .map(|statement| types::Statement::from(statement))
                .collect(),
        )
    }
}

impl<'ast, F: Field + PrimeField> From<ast::MultipleAssignmentStatement<'ast>>
    for types::Statement<F>
{
    fn from(statement: ast::MultipleAssignmentStatement<'ast>) -> Self {
        let assignees = statement
            .assignees
            .into_iter()
            .map(|i| types::Assignee::Variable(types::Variable::from(i.id)))
            .collect();

        types::Statement::MultipleDefinition(
            assignees,
            types::Expression::FunctionCall(
                types::Variable::from(statement.function_name),
                statement
                    .arguments
                    .into_iter()
                    .map(|e| types::Expression::from(e))
                    .collect(),
            ),
        )
    }
}

impl<'ast, F: Field + PrimeField> From<ast::AssignStatement<'ast>> for types::Statement<F> {
    fn from(statement: ast::AssignStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Assignee::from(statement.assignee),
            types::Expression::from(statement.expression),
        )
    }
}

impl<'ast, F: Field + PrimeField> From<ast::DefinitionStatement<'ast>> for types::Statement<F> {
    fn from(statement: ast::DefinitionStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Assignee::from(statement.variable),
            types::Expression::from_type(statement.ty, statement.expression),
        )
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Statement<'ast>> for types::Statement<F> {
    fn from(statement: ast::Statement<'ast>) -> Self {
        match statement {
            ast::Statement::Return(statement) => types::Statement::from(statement),
            ast::Statement::Iteration(statement) => types::Statement::from(statement),
            ast::Statement::MultipleAssignment(statement) => types::Statement::from(statement),
            ast::Statement::Assign(statement) => types::Statement::from(statement),
            ast::Statement::Definition(statement) => types::Statement::from(statement),
        }
    }
}

/// pest ast -> Explicit types::Type for defining struct members and function params

impl<'ast, F: Field + PrimeField> From<ast::BasicType<'ast>> for types::Type<F> {
    fn from(basic_type: ast::BasicType<'ast>) -> Self {
        match basic_type {
            ast::BasicType::U32(_ty) => types::Type::U32,
            ast::BasicType::Field(_ty) => types::Type::FieldElement,
            ast::BasicType::Boolean(_ty) => types::Type::Boolean,
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::ArrayType<'ast>> for types::Type<F> {
    fn from(array_type: ast::ArrayType<'ast>) -> Self {
        let element_type = Box::new(types::Type::from(array_type.ty));
        let count = types::Expression::<F>::get_count(array_type.count);

        types::Type::Array(element_type, count)
    }
}

impl<'ast, F: Field + PrimeField> From<ast::StructType<'ast>> for types::Type<F> {
    fn from(struct_type: ast::StructType<'ast>) -> Self {
        types::Type::Struct(types::Variable::from(struct_type.variable))
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Type<'ast>> for types::Type<F> {
    fn from(ty: ast::Type<'ast>) -> Self {
        match ty {
            ast::Type::Basic(ty) => types::Type::from(ty),
            ast::Type::Array(ty) => types::Type::from(ty),
            ast::Type::Struct(ty) => types::Type::from(ty),
        }
    }
}

/// pest ast -> types::Struct

impl<'ast, F: Field + PrimeField> From<ast::InlineStructMember<'ast>> for types::StructMember<F> {
    fn from(member: ast::InlineStructMember<'ast>) -> Self {
        types::StructMember {
            variable: types::Variable::from(member.variable),
            expression: types::Expression::from(member.expression),
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::StructField<'ast>> for types::StructField<F> {
    fn from(struct_field: ast::StructField<'ast>) -> Self {
        types::StructField {
            variable: types::Variable::from(struct_field.variable),
            ty: types::Type::from(struct_field.ty),
        }
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Struct<'ast>> for types::Struct<F> {
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

/// pest ast -> function types::Parameters

impl<'ast, F: Field + PrimeField> From<ast::Parameter<'ast>> for types::Parameter<F> {
    fn from(parameter: ast::Parameter<'ast>) -> Self {
        let ty = types::Type::from(parameter.ty);
        println!("type {}", ty);
        let variable = types::Variable::from(parameter.variable);

        if parameter.visibility.is_some() {
            let private = match parameter.visibility.unwrap() {
                ast::Visibility::Private(_) => true,
                ast::Visibility::Public(_) => false,
            };
            types::Parameter {
                private,
                ty,
                variable,
            }
        } else {
            types::Parameter {
                private: true,
                ty,
                variable,
            }
        }
    }
}

/// pest ast -> types::Function

impl<'ast> From<ast::FunctionName<'ast>> for types::FunctionName {
    fn from(name: ast::FunctionName<'ast>) -> Self {
        types::FunctionName(name.value)
    }
}

impl<'ast, F: Field + PrimeField> From<ast::Function<'ast>> for types::Function<F> {
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

/// pest ast -> Import

impl<'ast> From<ast::Variable<'ast>> for PathString<'ast> {
    fn from(import: ast::Variable<'ast>) -> Self {
        import.span.as_str()
    }
}

impl<'ast> From<ast::Import<'ast>> for Import<'ast> {
    fn from(import: ast::Import<'ast>) -> Self {
        match import {
            ast::Import::Main(import) => Import::new(None, Path::new(import.source.span.as_str()))
                .alias(import.alias.map(|alias| PathString::from(alias))),
            ast::Import::From(import) => Import::new(
                Some(PathString::from(import.symbol)),
                Path::new(import.source.span.as_str()),
            )
            .alias(import.alias.map(|alias| PathString::from(alias))),
        }
    }
}

/// pest ast -> types::Program

impl<'ast, F: Field + PrimeField> From<ast::File<'ast>> for types::Program<'ast, F> {
    fn from(file: ast::File<'ast>) -> Self {
        // Compiled ast -> aleo program representation
        let imports = file
            .imports
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import>>();

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
            name: types::Variable {
                name: "".into(),
                _field: PhantomData::<F>,
            },
            imports,
            structs,
            functions,
        }
    }
}
