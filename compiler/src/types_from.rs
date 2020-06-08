//! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.

use crate::{types, Import, ImportSymbol};
use leo_ast::{
    access::{
        Access,
        AssigneeAccess,
    },
    circuits::{
        Circuit,
        CircuitField,
        CircuitFieldDefinition,
        CircuitFunction,
        CircuitMember
    },
    common::{
        Assignee,
        Identifier as AstIdentifier,
        RangeOrExpression as AstRangeOrExpression,
        SpreadOrExpression as AstSpreadOrExpression,
        Variable as AstVariable,
        Visibility,
        Private,
    },
    expressions::{
        ArrayInitializerExpression,
        ArrayInlineExpression,
        BinaryExpression,
        CircuitInlineExpression,
        Expression,
        NotExpression,
        PostfixExpression,
        TernaryExpression
    },
    files::File,
    functions::{
        Function,
        FunctionInput,
        TestFunction
    },
    imports::{
        Import as AstImport,
        ImportSymbol as AstImportSymbol,
    },
    operations::{
        AssignOperation,
        BinaryOperation,
    },
    statements::{
        AssertStatement,
        AssignStatement,
        ConditionalStatement,
        ConditionalNestedOrEndStatement,
        DefinitionStatement,
        ExpressionStatement,
        ForStatement,
        MultipleAssignmentStatement,
        ReturnStatement,
        Statement,
    },
    types::{
        ArrayType,
        CircuitType,
        DataType,
        Type as AstType
    },
    values::{
        BooleanValue,
        FieldValue,
        GroupValue,
        IntegerValue,
        NumberImplicitValue,
        Value
    }
};
use leo_types::{Identifier, Integer, IntegerType};

use snarkos_models::gadgets::utilities::{
    boolean::Boolean,
};
use std::collections::HashMap;

impl<'ast> From<AstIdentifier<'ast>> for types::Expression {
    fn from(identifier: AstIdentifier<'ast>) -> Self {
        types::Expression::Identifier(Identifier::from(identifier))
    }
}

/// pest ast -> types::Variable

impl<'ast> From<AstVariable<'ast>> for types::Variable {
    fn from(variable: AstVariable<'ast>) -> Self {
        types::Variable {
            identifier: Identifier::from(variable.identifier),
            mutable: variable.mutable.is_some(),
            _type: variable._type.map(|_type| types::Type::from(_type)),
        }
    }
}

/// pest ast - Integer

impl<'ast> From<IntegerValue<'ast>> for types::Expression {
    fn from(field: IntegerValue<'ast>) -> Self {
        types::Expression::Integer(Integer::from(field.number, field._type))
    }
}

impl<'ast> From<AstRangeOrExpression<'ast>> for types::RangeOrExpression {
    fn from(range_or_expression: AstRangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            AstRangeOrExpression::Range(range) => {
                let from = range
                    .from
                    .map(|from| match types::Expression::from(from.0) {
                        types::Expression::Integer(number) => number,
                        types::Expression::Implicit(string) => {
                            Integer::from_implicit(string)
                        }
                        expression => {
                            unimplemented!("Range bounds should be integers, found {}", expression)
                        }
                    });
                let to = range.to.map(|to| match types::Expression::from(to.0) {
                    types::Expression::Integer(number) => number,
                    types::Expression::Implicit(string) => Integer::from_implicit(string),
                    expression => {
                        unimplemented!("Range bounds should be integers, found {}", expression)
                    }
                });

                types::RangeOrExpression::Range(from, to)
            }
            AstRangeOrExpression::Expression(expression) => {
                types::RangeOrExpression::Expression(types::Expression::from(expression))
            }
        }
    }
}

/// pest ast -> types::Field

impl<'ast> From<FieldValue<'ast>> for types::Expression {
    fn from(field: FieldValue<'ast>) -> Self {
        types::Expression::Field(field.number.value)
    }
}

/// pest ast -> types::Group

impl<'ast> From<GroupValue<'ast>> for types::Expression {
    fn from(group: GroupValue<'ast>) -> Self {
        types::Expression::Group(group.to_string())
    }
}

/// pest ast -> types::Boolean

impl<'ast> From<BooleanValue<'ast>> for types::Expression {
    fn from(boolean: BooleanValue<'ast>) -> Self {
        types::Expression::Boolean(Boolean::Constant(
            boolean
                .value
                .parse::<bool>()
                .expect("unable to parse boolean"),
        ))
    }
}

/// pest ast -> types::NumberImplicit

impl<'ast> From<NumberImplicitValue<'ast>> for types::Expression {
    fn from(number: NumberImplicitValue<'ast>) -> Self {
        types::Expression::Implicit(number.number.value)
    }
}

/// pest ast -> types::Expression

impl<'ast> From<Value<'ast>> for types::Expression {
    fn from(value: Value<'ast>) -> Self {
        match value {
            Value::Integer(num) => types::Expression::from(num),
            Value::Field(field) => types::Expression::from(field),
            Value::Group(group) => types::Expression::from(group),
            Value::Boolean(bool) => types::Expression::from(bool),
            Value::Implicit(value) => types::Expression::from(value),
        }
    }
}

impl<'ast> From<NotExpression<'ast>> for types::Expression {
    fn from(expression: NotExpression<'ast>) -> Self {
        types::Expression::Not(Box::new(types::Expression::from(*expression.expression)))
    }
}

impl<'ast> From<AstSpreadOrExpression<'ast>> for types::SpreadOrExpression {
    fn from(s_or_e: AstSpreadOrExpression<'ast>) -> Self {
        match s_or_e {
            AstSpreadOrExpression::Spread(spread) => {
                types::SpreadOrExpression::Spread(types::Expression::from(spread.expression))
            }
            AstSpreadOrExpression::Expression(expression) => {
                types::SpreadOrExpression::Expression(types::Expression::from(expression))
            }
        }
    }
}

impl<'ast> From<BinaryExpression<'ast>> for types::Expression {
    fn from(expression: BinaryExpression<'ast>) -> Self {
        match expression.operation {
            // Boolean operations
            BinaryOperation::Or => types::Expression::Or(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::And => types::Expression::And(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Eq => types::Expression::Eq(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Ne => {
                types::Expression::Not(Box::new(types::Expression::from(expression)))
            }
            BinaryOperation::Ge => types::Expression::Ge(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Gt => types::Expression::Gt(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Le => types::Expression::Le(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Lt => types::Expression::Lt(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            // Number operations
            BinaryOperation::Add => types::Expression::Add(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Sub => types::Expression::Sub(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Mul => types::Expression::Mul(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Div => types::Expression::Div(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
            BinaryOperation::Pow => types::Expression::Pow(
                Box::new(types::Expression::from(*expression.left)),
                Box::new(types::Expression::from(*expression.right)),
            ),
        }
    }
}

impl<'ast> From<TernaryExpression<'ast>> for types::Expression {
    fn from(expression: TernaryExpression<'ast>) -> Self {
        types::Expression::IfElse(
            Box::new(types::Expression::from(*expression.first)),
            Box::new(types::Expression::from(*expression.second)),
            Box::new(types::Expression::from(*expression.third)),
        )
    }
}

impl<'ast> From<ArrayInlineExpression<'ast>> for types::Expression {
    fn from(array: ArrayInlineExpression<'ast>) -> Self {
        types::Expression::Array(
            array
                .expressions
                .into_iter()
                .map(|s_or_e| Box::new(types::SpreadOrExpression::from(s_or_e)))
                .collect(),
        )
    }
}
impl<'ast> From<ArrayInitializerExpression<'ast>> for types::Expression {
    fn from(array: ArrayInitializerExpression<'ast>) -> Self {
        let count = types::Expression::get_count(array.count);
        let expression = Box::new(types::SpreadOrExpression::from(*array.expression));

        types::Expression::Array(vec![expression; count])
    }
}

impl<'ast> From<CircuitField<'ast>> for types::CircuitFieldDefinition {
    fn from(member: CircuitField<'ast>) -> Self {
        types::CircuitFieldDefinition {
            identifier: Identifier::from(member.identifier),
            expression: types::Expression::from(member.expression),
        }
    }
}

impl<'ast> From<CircuitInlineExpression<'ast>> for types::Expression {
    fn from(expression: CircuitInlineExpression<'ast>) -> Self {
        let variable = Identifier::from(expression.identifier);
        let members = expression
            .members
            .into_iter()
            .map(|member| types::CircuitFieldDefinition::from(member))
            .collect::<Vec<types::CircuitFieldDefinition>>();

        types::Expression::Circuit(variable, members)
    }
}

impl<'ast> From<PostfixExpression<'ast>> for types::Expression {
    fn from(expression: PostfixExpression<'ast>) -> Self {
        let variable =
            types::Expression::Identifier(Identifier::from(expression.identifier));

        // ast::PostFixExpression contains an array of "accesses": `a(34)[42]` is represented as `[a, [Call(34), Select(42)]]`, but Access call expressions
        // are recursive, so it is `Select(Call(a, 34), 42)`. We apply this transformation here

        // we start with the id, and we fold the array of accesses by wrapping the current value
        expression
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                // Handle array accesses
                Access::Array(array) => types::Expression::ArrayAccess(
                    Box::new(acc),
                    Box::new(types::RangeOrExpression::from(array.expression)),
                ),

                // Handle function calls
                Access::Call(function) => types::Expression::FunctionCall(
                    Box::new(acc),
                    function
                        .expressions
                        .into_iter()
                        .map(|expression| types::Expression::from(expression))
                        .collect(),
                ),

                // Handle circuit member accesses
                Access::Object(circuit_object) => types::Expression::CircuitMemberAccess(
                    Box::new(acc),
                    Identifier::from(circuit_object.identifier),
                ),
                Access::StaticObject(circuit_object) => {
                    types::Expression::CircuitStaticFunctionAccess(
                        Box::new(acc),
                        Identifier::from(circuit_object.identifier),
                    )
                }
            })
    }
}

impl<'ast> From<Expression<'ast>> for types::Expression {
    fn from(expression: Expression<'ast>) -> Self {
        match expression {
            Expression::Value(value) => types::Expression::from(value),
            Expression::Identifier(variable) => types::Expression::from(variable),
            Expression::Not(expression) => types::Expression::from(expression),
            Expression::Binary(expression) => types::Expression::from(expression),
            Expression::Ternary(expression) => types::Expression::from(expression),
            Expression::ArrayInline(expression) => types::Expression::from(expression),
            Expression::ArrayInitializer(expression) => types::Expression::from(expression),
            Expression::CircuitInline(expression) => types::Expression::from(expression),
            Expression::Postfix(expression) => types::Expression::from(expression),
        }
    }
}

impl<'ast> types::Expression {
    fn get_count(count: Value<'ast>) -> usize {
        match count {
            Value::Integer(integer) => integer
                .number
                .value
                .parse::<usize>()
                .expect("Unable to read array size"),
            Value::Implicit(number) => number
                .number
                .value
                .parse::<usize>()
                .expect("Unable to read array size"),
            size => unimplemented!("Array size should be an integer {}", size),
        }
    }
}

// Assignee -> types::Expression for operator assign statements
impl<'ast> From<Assignee<'ast>> for types::Expression {
    fn from(assignee: Assignee<'ast>) -> Self {
        let variable = types::Expression::Identifier(Identifier::from(assignee.identifier));

        // we start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                AssigneeAccess::Member(circuit_member) => {
                    types::Expression::CircuitMemberAccess(
                        Box::new(acc),
                        Identifier::from(circuit_member.identifier),
                    )
                }
                AssigneeAccess::Array(array) => types::Expression::ArrayAccess(
                    Box::new(acc),
                    Box::new(types::RangeOrExpression::from(array.expression)),
                ),
            })
    }
}

/// pest ast -> types::Assignee

impl<'ast> From<AstIdentifier<'ast>> for types::Assignee {
    fn from(variable: AstIdentifier<'ast>) -> Self {
        types::Assignee::Identifier(Identifier::from(variable))
    }
}

impl<'ast> From<Assignee<'ast>> for types::Assignee {
    fn from(assignee: Assignee<'ast>) -> Self {
        let variable = types::Assignee::from(assignee.identifier);

        // we start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                AssigneeAccess::Array(array) => types::Assignee::Array(
                    Box::new(acc),
                    types::RangeOrExpression::from(array.expression),
                ),
                AssigneeAccess::Member(circuit_field) => types::Assignee::CircuitField(
                    Box::new(acc),
                    Identifier::from(circuit_field.identifier),
                ),
            })
    }
}

/// pest ast -> types::Statement

impl<'ast> From<ReturnStatement<'ast>> for types::Statement {
    fn from(statement: ReturnStatement<'ast>) -> Self {
        types::Statement::Return(
            statement
                .expressions
                .into_iter()
                .map(|expression| types::Expression::from(expression))
                .collect(),
        )
    }
}

impl<'ast> From<DefinitionStatement<'ast>> for types::Statement {
    fn from(statement: DefinitionStatement<'ast>) -> Self {
        types::Statement::Definition(
            types::Variable::from(statement.variable),
            types::Expression::from(statement.expression),
        )
    }
}

impl<'ast> From<AssignStatement<'ast>> for types::Statement {
    fn from(statement: AssignStatement<'ast>) -> Self {
        match statement.assign {
            AssignOperation::Assign(ref _assign) => types::Statement::Assign(
                types::Assignee::from(statement.assignee),
                types::Expression::from(statement.expression),
            ),
            operation_assign => {
                // convert assignee into postfix expression
                let converted = types::Expression::from(statement.assignee.clone());

                match operation_assign {
                    AssignOperation::AddAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Add(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::SubAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Sub(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::MulAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Mul(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::DivAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Div(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::PowAssign(ref _assign) => types::Statement::Assign(
                        types::Assignee::from(statement.assignee),
                        types::Expression::Pow(
                            Box::new(converted),
                            Box::new(types::Expression::from(statement.expression)),
                        ),
                    ),
                    AssignOperation::Assign(ref _assign) => {
                        unimplemented!("cannot assign twice to assign statement")
                    }
                }
            }
        }
    }
}

impl<'ast> From<MultipleAssignmentStatement<'ast>> for types::Statement {
    fn from(statement: MultipleAssignmentStatement<'ast>) -> Self {
        let variables = statement
            .variables
            .into_iter()
            .map(|typed_variable| types::Variable::from(typed_variable))
            .collect();

        types::Statement::MultipleAssign(
            variables,
            types::Expression::FunctionCall(
                Box::new(types::Expression::from(statement.function_name)),
                statement
                    .arguments
                    .into_iter()
                    .map(|e| types::Expression::from(e))
                    .collect(),
            ),
        )
    }
}

impl<'ast> From<ConditionalNestedOrEndStatement<'ast>> for types::ConditionalNestedOrEnd {
    fn from(statement: ConditionalNestedOrEndStatement<'ast>) -> Self {
        match statement {
            ConditionalNestedOrEndStatement::Nested(nested) => types::ConditionalNestedOrEnd::Nested(
                Box::new(types::ConditionalStatement::from(*nested)),
            ),
            ConditionalNestedOrEndStatement::End(statements) => types::ConditionalNestedOrEnd::End(
                statements
                    .into_iter()
                    .map(|statement| types::Statement::from(statement))
                    .collect(),
            ),
        }
    }
}

impl<'ast> From<ConditionalStatement<'ast>> for types::ConditionalStatement {
    fn from(statement: ConditionalStatement<'ast>) -> Self {
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

impl<'ast> From<ForStatement<'ast>> for types::Statement {
    fn from(statement: ForStatement<'ast>) -> Self {
        let from = match types::Expression::from(statement.start) {
            types::Expression::Integer(number) => number,
            types::Expression::Implicit(string) => Integer::from_implicit(string),
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };
        let to = match types::Expression::from(statement.stop) {
            types::Expression::Integer(number) => number,
            types::Expression::Implicit(string) => Integer::from_implicit(string),
            expression => unimplemented!("Range bounds should be integers, found {}", expression),
        };

        types::Statement::For(
            Identifier::from(statement.index),
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

impl<'ast> From<AssertStatement<'ast>> for types::Statement {
    fn from(statement: AssertStatement<'ast>) -> Self {
        match statement {
            AssertStatement::AssertEq(assert_eq) => types::Statement::AssertEq(
                types::Expression::from(assert_eq.left),
                types::Expression::from(assert_eq.right),
            ),
        }
    }
}

impl<'ast> From<ExpressionStatement<'ast>> for types::Statement {
    fn from(statement: ExpressionStatement<'ast>) -> Self {
        types::Statement::Expression(types::Expression::from(statement.expression))
    }
}

impl<'ast> From<Statement<'ast>> for types::Statement {
    fn from(statement: Statement<'ast>) -> Self {
        match statement {
            Statement::Return(statement) => types::Statement::from(statement),
            Statement::Definition(statement) => types::Statement::from(statement),
            Statement::Assign(statement) => types::Statement::from(statement),
            Statement::MultipleAssignment(statement) => types::Statement::from(statement),
            Statement::Conditional(statement) => {
                types::Statement::Conditional(types::ConditionalStatement::from(statement))
            }
            Statement::Iteration(statement) => types::Statement::from(statement),
            Statement::Assert(statement) => types::Statement::from(statement),
            Statement::Expression(statement) => types::Statement::from(statement),
        }
    }
}

/// pest ast -> Explicit types::Type for defining circuit members and function params

impl From<DataType> for types::Type {
    fn from(basic_type: DataType) -> Self {
        match basic_type {
            DataType::Integer(_type) => {
                types::Type::IntegerType(IntegerType::from(_type))
            }
            DataType::Field(_type) => types::Type::Field,
            DataType::Group(_type) => types::Type::Group,
            DataType::Boolean(_type) => types::Type::Boolean,
        }
    }
}

impl<'ast> From<ArrayType<'ast>> for types::Type {
    fn from(array_type: ArrayType<'ast>) -> Self {
        let element_type = Box::new(types::Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| types::Expression::get_count(row))
            .collect();

        types::Type::Array(element_type, dimensions)
    }
}

impl<'ast> From<CircuitType<'ast>> for types::Type {
    fn from(circuit_type: CircuitType<'ast>) -> Self {
        types::Type::Circuit(Identifier::from(circuit_type.identifier))
    }
}

impl<'ast> From<AstType<'ast>> for types::Type {
    fn from(_type: AstType<'ast>) -> Self {
        match _type {
            AstType::Basic(_type) => types::Type::from(_type),
            AstType::Array(_type) => types::Type::from(_type),
            AstType::Circuit(_type) => types::Type::from(_type),
            AstType::SelfType(_type) => types::Type::SelfType,
        }
    }
}

/// pest ast -> types::Circuit

impl<'ast> From<CircuitFieldDefinition<'ast>> for types::CircuitMember {
    fn from(circuit_value: CircuitFieldDefinition<'ast>) -> Self {
        types::CircuitMember::CircuitField(
            Identifier::from(circuit_value.identifier),
            types::Type::from(circuit_value._type),
        )
    }
}

impl<'ast> From<CircuitFunction<'ast>> for types::CircuitMember {
    fn from(circuit_function: CircuitFunction<'ast>) -> Self {
        types::CircuitMember::CircuitFunction(
            circuit_function._static.is_some(),
            types::Function::from(circuit_function.function),
        )
    }
}

impl<'ast> From<CircuitMember<'ast>> for types::CircuitMember {
    fn from(object: CircuitMember<'ast>) -> Self {
        match object {
            CircuitMember::CircuitFieldDefinition(circuit_value) => {
                types::CircuitMember::from(circuit_value)
            }
            CircuitMember::CircuitFunction(circuit_function) => {
                types::CircuitMember::from(circuit_function)
            }
        }
    }
}

impl<'ast> From<Circuit<'ast>> for types::Circuit {
    fn from(circuit: Circuit<'ast>) -> Self {
        let variable = Identifier::from(circuit.identifier);
        let members = circuit
            .members
            .into_iter()
            .map(|member| types::CircuitMember::from(member))
            .collect();

        types::Circuit {
            identifier: variable,
            members,
        }
    }
}

/// pest ast -> function types::Parameters

impl<'ast> From<FunctionInput<'ast>> for types::InputModel {
    fn from(parameter: FunctionInput<'ast>) -> Self {
        types::InputModel {
            identifier: Identifier::from(parameter.identifier),
            mutable: parameter.mutable.is_some(),
            // private by default
            private: parameter.visibility.map_or(true, |visibility| {
                visibility.eq(&Visibility::Private(Private {}))
            }),
            _type: types::Type::from(parameter._type),
        }
    }
}

/// pest ast -> types::Function

impl<'ast> From<Function<'ast>> for types::Function {
    fn from(function_definition: Function<'ast>) -> Self {
        let function_name = Identifier::from(function_definition.function_name);
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

impl<'ast> From<AstImportSymbol<'ast>> for ImportSymbol {
    fn from(symbol: AstImportSymbol<'ast>) -> Self {
        ImportSymbol {
            symbol: Identifier::from(symbol.value),
            alias: symbol.alias.map(|alias| Identifier::from(alias)),
        }
    }
}

impl<'ast> From<AstImport<'ast>> for Import {
    fn from(import: AstImport<'ast>) -> Self {
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

/// pest ast -> Test
impl<'ast> From<TestFunction<'ast>> for types::Test {
    fn from(test: TestFunction) -> Self {
        types::Test(types::Function::from(test.function))
    }
}

/// pest ast -> types::Program

impl<'ast> types::Program {
    pub fn from(file: File<'ast>, name: String) -> Self {
        // Compiled ast -> aleo program representation
        let imports = file
            .imports
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import>>();

        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();
        let mut num_parameters = 0usize;

        file.circuits.into_iter().for_each(|circuit| {
            circuits.insert(
                Identifier::from(circuit.identifier.clone()),
                types::Circuit::from(circuit),
            );
        });
        file.functions.into_iter().for_each(|function_def| {
            functions.insert(
                Identifier::from(function_def.function_name.clone()),
                types::Function::from(function_def),
            );
        });
        file.tests.into_iter().for_each(|test_def| {
            tests.insert(
                Identifier::from(test_def.function.function_name.clone()),
                types::Test::from(test_def),
            );
        });

        if let Some(main_function) = functions.get(&Identifier::new("main".into())) {
            num_parameters = main_function.inputs.len();
        }

        types::Program {
            name: Identifier::new(name),
            num_parameters,
            imports,
            circuits,
            functions,
            tests,
        }
    }
}
