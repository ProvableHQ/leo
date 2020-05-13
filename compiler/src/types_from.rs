//! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.

use crate::{ast, types, Import, ImportSymbol};

use snarkos_models::curves::{Field, Group, PrimeField};
use snarkos_models::gadgets::utilities::{
    boolean::Boolean, uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64,
    uint8::UInt8,
};
use std::{collections::HashMap, marker::PhantomData};

/// pest ast -> types::Variable

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Variable<'ast>> for types::Variable<F, G> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Variable {
            name: variable.value,
            _engine: PhantomData::<F>,
            _group: PhantomData::<G>,
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Variable<'ast>> for types::Expression<F, G> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Expression::Variable(types::Variable::from(variable))
    }
}
/// pest ast - types::Integer

impl<'ast> types::Integer {
    pub(crate) fn from(number: ast::Number<'ast>, _type: ast::IntegerType) -> Self {
        match _type {
            ast::IntegerType::U8Type(_u8) => types::Integer::U8(UInt8::constant(
                number.value.parse::<u8>().expect("unable to parse u8"),
            )),
            ast::IntegerType::U16Type(_u16) => types::Integer::U16(UInt16::constant(
                number.value.parse::<u16>().expect("unable to parse u16"),
            )),
            ast::IntegerType::U32Type(_u32) => types::Integer::U32(UInt32::constant(
                number.value.parse::<u32>().expect("unable to parse u32"),
            )),
            ast::IntegerType::U64Type(_u64) => types::Integer::U64(UInt64::constant(
                number.value.parse::<u64>().expect("unable to parse u64"),
            )),
            ast::IntegerType::U128Type(_u128) => types::Integer::U128(UInt128::constant(
                number.value.parse::<u128>().expect("unable to parse u128"),
            )),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Integer<'ast>> for types::Expression<F, G> {
    fn from(field: ast::Integer<'ast>) -> Self {
        types::Expression::Integer(match field._type {
            Some(_type) => types::Integer::from(field.number, _type),
            // default integer type is u32
            None => types::Integer::U32(UInt32::constant(
                field
                    .number
                    .value
                    .parse::<u32>()
                    .expect("unable to parse u32"),
            )),
        })
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::RangeOrExpression<'ast>>
    for types::RangeOrExpression<F, G>
{
    fn from(range_or_expression: ast::RangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            ast::RangeOrExpression::Range(range) => {
                let from = range
                    .from
                    .map(|from| match types::Expression::<F, G>::from(from.0) {
                        types::Expression::Integer(number) => number,
                        expression => {
                            unimplemented!("Range bounds should be integers, found {}", expression)
                        }
                    });
                let to = range
                    .to
                    .map(|to| match types::Expression::<F, G>::from(to.0) {
                        types::Expression::Integer(number) => number,
                        expression => {
                            unimplemented!("Range bounds should be integers, found {}", expression)
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

/// pest ast -> types::Field

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Field<'ast>> for types::Expression<F, G> {
    fn from(field: ast::Field<'ast>) -> Self {
        types::Expression::FieldElement(types::FieldElement::Constant(
            F::from_str(&field.number.value).unwrap_or_default(),
        ))
    }
}

/// pest ast -> types::Group

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Group<'ast>> for types::Expression<F, G> {
    fn from(_group: ast::Group<'ast>) -> Self {
        types::Expression::GroupElement(G::zero())
    }
}

/// pest ast -> types::Boolean

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Boolean<'ast>> for types::Expression<F, G> {
    fn from(boolean: ast::Boolean<'ast>) -> Self {
        types::Expression::Boolean(Boolean::Constant(
            boolean
                .value
                .parse::<bool>()
                .expect("unable to parse boolean"),
        ))
    }
}

/// pest ast -> types::Expression

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Value<'ast>> for types::Expression<F, G> {
    fn from(value: ast::Value<'ast>) -> Self {
        match value {
            ast::Value::Integer(num) => types::Expression::from(num),
            ast::Value::Field(field) => types::Expression::from(field),
            ast::Value::Group(group) => types::Expression::from(group),
            ast::Value::Boolean(bool) => types::Expression::from(bool),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::NotExpression<'ast>>
    for types::Expression<F, G>
{
    fn from(expression: ast::NotExpression<'ast>) -> Self {
        types::Expression::Not(Box::new(types::Expression::from(*expression.expression)))
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::SpreadOrExpression<'ast>>
    for types::SpreadOrExpression<F, G>
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::BinaryExpression<'ast>>
    for types::Expression<F, G>
{
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::TernaryExpression<'ast>>
    for types::Expression<F, G>
{
    fn from(expression: ast::TernaryExpression<'ast>) -> Self {
        types::Expression::IfElse(
            Box::new(types::Expression::from(*expression.first)),
            Box::new(types::Expression::from(*expression.second)),
            Box::new(types::Expression::from(*expression.third)),
        )
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ArrayInlineExpression<'ast>>
    for types::Expression<F, G>
{
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
impl<'ast, F: Field + PrimeField, G: Group> From<ast::ArrayInitializerExpression<'ast>>
    for types::Expression<F, G>
{
    fn from(array: ast::ArrayInitializerExpression<'ast>) -> Self {
        let count = types::Expression::<F, G>::get_count(array.count);
        let expression = Box::new(types::SpreadOrExpression::from(*array.expression));

        types::Expression::Array(vec![expression; count])
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::InlineCircuitMember<'ast>>
    for types::CircuitMember<F, G>
{
    fn from(member: ast::InlineCircuitMember<'ast>) -> Self {
        types::CircuitMember {
            variable: types::Variable::from(member.variable),
            expression: types::Expression::from(member.expression),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::CircuitInlineExpression<'ast>>
    for types::Expression<F, G>
{
    fn from(expression: ast::CircuitInlineExpression<'ast>) -> Self {
        let variable = types::Variable::from(expression.variable);
        let members = expression
            .members
            .into_iter()
            .map(|member| types::CircuitMember::from(member))
            .collect::<Vec<types::CircuitMember<F, G>>>();

        types::Expression::Circuit(variable, members)
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::PostfixExpression<'ast>>
    for types::Expression<F, G>
{
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
                ast::Access::Member(struct_member) => types::Expression::CircuitMemberAccess(
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Expression<'ast>>
    for types::Expression<F, G>
{
    fn from(expression: ast::Expression<'ast>) -> Self {
        match expression {
            ast::Expression::Value(value) => types::Expression::from(value),
            ast::Expression::Variable(variable) => types::Expression::from(variable),
            ast::Expression::Not(expression) => types::Expression::from(expression),
            ast::Expression::Binary(expression) => types::Expression::from(expression),
            ast::Expression::Ternary(expression) => types::Expression::from(expression),
            ast::Expression::ArrayInline(expression) => types::Expression::from(expression),
            ast::Expression::ArrayInitializer(expression) => types::Expression::from(expression),
            ast::Expression::CircuitInline(expression) => types::Expression::from(expression),
            ast::Expression::Postfix(expression) => types::Expression::from(expression),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> types::Expression<F, G> {
    fn get_count(count: ast::Value<'ast>) -> usize {
        match count {
            ast::Value::Integer(f) => f
                .number
                .value
                .parse::<usize>()
                .expect("Unable to read array size"),
            size => unimplemented!("Array size should be an integer {}", size),
        }
    }
}

// ast::Assignee -> types::Expression for operator assign statements
impl<'ast, F: Field + PrimeField, G: Group> From<ast::Assignee<'ast>> for types::Expression<F, G> {
    fn from(assignee: ast::Assignee<'ast>) -> Self {
        let variable = types::Expression::Variable(types::Variable::from(assignee.variable));

        // we start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                ast::AssigneeAccess::Member(struct_member) => {
                    types::Expression::CircuitMemberAccess(
                        Box::new(acc),
                        types::Variable::from(struct_member.variable),
                    )
                }
                ast::AssigneeAccess::Array(array) => types::Expression::ArrayAccess(
                    Box::new(acc),
                    Box::new(types::RangeOrExpression::from(array.expression)),
                ),
            })
    }
}

/// pest ast -> types::Assignee

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Variable<'ast>> for types::Assignee<F, G> {
    fn from(variable: ast::Variable<'ast>) -> Self {
        types::Assignee::Variable(types::Variable::from(variable))
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Assignee<'ast>> for types::Assignee<F, G> {
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
                ast::AssigneeAccess::Member(struct_member) => types::Assignee::CircuitMember(
                    Box::new(acc),
                    types::Variable::from(struct_member.variable),
                ),
            })
    }
}

/// pest ast -> types::Statement

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ReturnStatement<'ast>>
    for types::Statement<F, G>
{
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::DefinitionStatement<'ast>>
    for types::Statement<F, G>
{
    fn from(statement: ast::DefinitionStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Assignee::from(statement.variable),
            statement
                ._type
                .map(|_type| types::Type::<F, G>::from(_type)),
            types::Expression::from(statement.expression),
        )
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::AssignStatement<'ast>>
    for types::Statement<F, G>
{
    fn from(statement: ast::AssignStatement<'ast>) -> Self {
        match statement.assign {
            ast::OperationAssign::Assign(ref _assign) => types::Statement::Assign(
                types::Assignee::from(statement.assignee),
                types::Expression::from(statement.expression),
            ),
            operation_assign => {
                // convert assignee into postfix expression
                let converted = types::Expression::from(statement.assignee.clone());

                match operation_assign {
                    ast::OperationAssign::AddAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Add(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    ast::OperationAssign::SubAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Sub(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    ast::OperationAssign::MulAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Mul(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    ast::OperationAssign::DivAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Div(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    ast::OperationAssign::PowAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Pow(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    ast::OperationAssign::Assign(ref _assign) => {
                        unimplemented!("cannot assign twice to assign statement")
                    }
                }
            }
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::MultipleAssignmentStatement<'ast>>
    for types::Statement<F, G>
{
    fn from(statement: ast::MultipleAssignmentStatement<'ast>) -> Self {
        let assignees = statement
            .assignees
            .into_iter()
            .map(|i| types::Assignee::Variable(types::Variable::from(i.id)))
            .collect();

        types::Statement::MultipleAssign(
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ConditionalNestedOrEnd<'ast>>
    for types::ConditionalNestedOrEnd<F, G>
{
    fn from(statement: ast::ConditionalNestedOrEnd<'ast>) -> Self {
        match statement {
            ast::ConditionalNestedOrEnd::Nested(nested) => types::ConditionalNestedOrEnd::Nested(
                Box::new(types::ConditionalStatement::from(*nested)),
            ),
            ast::ConditionalNestedOrEnd::End(statements) => types::ConditionalNestedOrEnd::End(
                statements
                    .into_iter()
                    .map(|statement| types::Statement::from(statement))
                    .collect(),
            ),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ConditionalStatement<'ast>>
    for types::ConditionalStatement<F, G>
{
    fn from(statement: ast::ConditionalStatement<'ast>) -> Self {
        types::ConditionalStatement {
            condition: types::Expression::from(statement.condition),
            statements: statement
                .statements
                .into_iter()
                .map(|statement| types::Statement::from(statement))
                .collect(),
            next: statement
                .next
                .map(|n_or_e| Some(types::ConditionalNestedOrEnd::from(n_or_e)))
                .unwrap_or(None),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ForStatement<'ast>>
    for types::Statement<F, G>
{
    fn from(statement: ast::ForStatement<'ast>) -> Self {
        let from = match types::Expression::<F, G>::from(statement.start) {
            types::Expression::Integer(number) => number,
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };
        let to = match types::Expression::<F, G>::from(statement.stop) {
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::AssertStatement<'ast>>
    for types::Statement<F, G>
{
    fn from(statement: ast::AssertStatement<'ast>) -> Self {
        match statement {
            ast::AssertStatement::AssertEq(assert_eq) => types::Statement::AssertEq(
                types::Expression::from(assert_eq.left),
                types::Expression::from(assert_eq.right),
            ),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ExpressionStatement<'ast>>
    for types::Statement<F, G>
{
    fn from(statement: ast::ExpressionStatement<'ast>) -> Self {
        types::Statement::Expression(types::Expression::from(statement.expression))
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Statement<'ast>> for types::Statement<F, G> {
    fn from(statement: ast::Statement<'ast>) -> Self {
        match statement {
            ast::Statement::Return(statement) => types::Statement::from(statement),
            ast::Statement::Definition(statement) => types::Statement::from(statement),
            ast::Statement::Assign(statement) => types::Statement::from(statement),
            ast::Statement::MultipleAssignment(statement) => types::Statement::from(statement),
            ast::Statement::Conditional(statement) => {
                types::Statement::Conditional(types::ConditionalStatement::from(statement))
            }
            ast::Statement::Iteration(statement) => types::Statement::from(statement),
            ast::Statement::Assert(statement) => types::Statement::from(statement),
            ast::Statement::Expression(statement) => types::Statement::from(statement),
        }
    }
}

/// pest ast -> Explicit types::Type for defining struct members and function params

impl From<ast::IntegerType> for types::IntegerType {
    fn from(integer_type: ast::IntegerType) -> Self {
        match integer_type {
            ast::IntegerType::U8Type(_type) => types::IntegerType::U8,
            ast::IntegerType::U16Type(_type) => types::IntegerType::U16,
            ast::IntegerType::U32Type(_type) => types::IntegerType::U32,
            ast::IntegerType::U64Type(_type) => types::IntegerType::U64,
            ast::IntegerType::U128Type(_type) => types::IntegerType::U128,
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::BasicType<'ast>> for types::Type<F, G> {
    fn from(basic_type: ast::BasicType<'ast>) -> Self {
        match basic_type {
            ast::BasicType::Integer(_type) => {
                types::Type::IntegerType(types::IntegerType::from(_type))
            }
            ast::BasicType::Field(_type) => types::Type::FieldElement,
            ast::BasicType::Group(_type) => unimplemented!(),
            ast::BasicType::Boolean(_type) => types::Type::Boolean,
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ArrayType<'ast>> for types::Type<F, G> {
    fn from(array_type: ast::ArrayType<'ast>) -> Self {
        let element_type = Box::new(types::Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| types::Expression::<F, G>::get_count(row))
            .collect();

        types::Type::Array(element_type, dimensions)
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::CircuitType<'ast>> for types::Type<F, G> {
    fn from(struct_type: ast::CircuitType<'ast>) -> Self {
        types::Type::Circuit(types::Variable::from(struct_type.variable))
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Type<'ast>> for types::Type<F, G> {
    fn from(_type: ast::Type<'ast>) -> Self {
        match _type {
            ast::Type::Basic(_type) => types::Type::from(_type),
            ast::Type::Array(_type) => types::Type::from(_type),
            ast::Type::Circuit(_type) => types::Type::from(_type),
        }
    }
}

/// pest ast -> types::Struct

impl<'ast, F: Field + PrimeField, G: Group> From<ast::CircuitObject<'ast>>
    for types::CircuitObject<F, G>
{
    fn from(struct_field: ast::CircuitObject<'ast>) -> Self {
        types::CircuitObject {
            variable: types::Variable::from(struct_field.variable),
            _type: types::Type::from(struct_field._type),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Circuit<'ast>> for types::Circuit<F, G> {
    fn from(struct_definition: ast::Circuit<'ast>) -> Self {
        let variable = types::Variable::from(struct_definition.variable);
        let fields = struct_definition
            .fields
            .into_iter()
            .map(|struct_field| types::CircuitObject::from(struct_field))
            .collect();

        types::Circuit { variable, fields }
    }
}

/// pest ast -> function types::Parameters

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Parameter<'ast>> for types::InputModel<F, G> {
    fn from(parameter: ast::Parameter<'ast>) -> Self {
        let _type = types::Type::from(parameter._type);
        let variable = types::Variable::from(parameter.variable);

        if parameter.visibility.is_some() {
            let private = match parameter.visibility.unwrap() {
                ast::Visibility::Private(_) => true,
                ast::Visibility::Public(_) => false,
            };
            types::InputModel {
                private,
                _type,
                variable,
            }
        } else {
            types::InputModel {
                private: true,
                _type,
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

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Function<'ast>> for types::Function<F, G> {
    fn from(function_definition: ast::Function<'ast>) -> Self {
        let function_name = types::FunctionName::from(function_definition.function_name);
        let parameters = function_definition
            .parameters
            .into_iter()
            .map(|parameter| types::InputModel::from(parameter))
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
            inputs: parameters,
            returns,
            statements,
        }
    }
}

/// pest ast -> Import

impl<'ast, F: Field + PrimeField, G: Group> From<ast::ImportSymbol<'ast>> for ImportSymbol<F, G> {
    fn from(symbol: ast::ImportSymbol<'ast>) -> Self {
        ImportSymbol {
            symbol: types::Variable::from(symbol.value),
            alias: symbol.alias.map(|alias| types::Variable::from(alias)),
        }
    }
}

impl<'ast, F: Field + PrimeField, G: Group> From<ast::Import<'ast>> for Import<F, G> {
    fn from(import: ast::Import<'ast>) -> Self {
        Import {
            path_string: import.source.value,
            symbols: import
                .symbols
                .into_iter()
                .map(|symbol| ImportSymbol::from(symbol))
                .collect(),
        }
    }
}

/// pest ast -> types::Program

impl<'ast, F: Field + PrimeField, G: Group> types::Program<F, G> {
    pub fn from(file: ast::File<'ast>, name: String) -> Self {
        // Compiled ast -> aleo program representation
        let imports = file
            .imports
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import<F, G>>>();

        let mut structs = HashMap::new();
        let mut functions = HashMap::new();
        let mut num_parameters = 0usize;

        file.structs.into_iter().for_each(|struct_def| {
            structs.insert(
                types::Variable::from(struct_def.variable.clone()),
                types::Circuit::from(struct_def),
            );
        });
        file.functions.into_iter().for_each(|function_def| {
            functions.insert(
                types::FunctionName::from(function_def.function_name.clone()),
                types::Function::from(function_def),
            );
        });

        if let Some(main_function) = functions.get(&types::FunctionName("main".into())) {
            num_parameters = main_function.inputs.len();
        }

        types::Program {
            name: types::Variable {
                name,
                _group: PhantomData::<G>,
                _engine: PhantomData::<F>,
            },
            num_parameters,
            imports,
            circuits: structs,
            functions,
        }
    }
}
