//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;

use snarkos_models::gadgets::utilities::{
    boolean::Boolean, uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64,
    uint8::UInt8,
};
use std::collections::HashMap;

/// An identifier in the constrained program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn is_self(&self) -> bool {
        self.name == "Self"
    }
}

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq)]
pub struct Variable {
    pub identifier: Identifier,
    pub mutable: bool,
    pub _type: Option<Type>,
}

/// An integer type enum wrapping the integer value. Used only in expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Integer {
    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),
}

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOrExpression {
    Range(Option<Integer>, Option<Integer>),
    Expression(Expression),
}

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadOrExpression {
    Spread(Expression),
    Expression(Expression),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // Identifier
    Identifier(Identifier),

    // Values
    Integer(Integer),
    Field(String),
    Group(String),
    Boolean(Boolean),
    Implicit(String),

    // Number operations
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),

    // Boolean operations
    Not(Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    Geq(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Leq(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),

    // Conditionals
    IfElse(Box<Expression>, Box<Expression>, Box<Expression>),

    // Arrays
    Array(Vec<Box<SpreadOrExpression>>),
    ArrayAccess(Box<Expression>, Box<RangeOrExpression>), // (array name, range)

    // Circuits
    Circuit(Identifier, Vec<CircuitFieldDefinition>),
    CircuitMemberAccess(Box<Expression>, Identifier), // (declared circuit name, circuit member name)
    CircuitStaticFunctionAccess(Box<Expression>, Identifier), // (defined circuit name, circuit static member name)

    // Functions
    FunctionCall(Box<Expression>, Vec<Expression>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee {
    Identifier(Identifier),
    Array(Box<Assignee>, RangeOrExpression),
    CircuitField(Box<Assignee>, Identifier), // (circuit name, circuit field name)
}

/// Explicit integer type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    IntegerType(IntegerType),
    Field,
    Group,
    Boolean,
    Array(Box<Type>, Vec<usize>),
    Circuit(Identifier),
    SelfType,
}

impl Type {
    pub fn outer_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let _type = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[1..]);

            return Type::Array(Box::new(_type), next);
        }

        _type
    }

    pub fn inner_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let _type = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[..dimensions.len() - 1]);

            return Type::Array(Box::new(_type), next);
        }

        _type
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConditionalNestedOrEnd {
    Nested(Box<ConditionalStatement>),
    End(Vec<Statement>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub next: Option<ConditionalNestedOrEnd>,
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Vec<Expression>),
    Definition(Variable, Expression),
    Assign(Assignee, Expression),
    MultipleAssign(Vec<Variable>, Expression),
    Conditional(ConditionalStatement),
    For(Identifier, Integer, Integer, Vec<Statement>),
    AssertEq(Expression, Expression),
    Expression(Expression),
}

/// Circuits

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitFieldDefinition {
    pub identifier: Identifier,
    pub expression: Expression,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CircuitMember {
    CircuitField(Identifier, Type),
    CircuitFunction(bool, Function),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Circuit {
    pub identifier: Identifier,
    pub members: Vec<CircuitMember>,
}

/// Function parameters

#[derive(Clone, PartialEq, Eq)]
pub struct InputModel {
    pub identifier: Identifier,
    pub mutable: bool,
    pub private: bool,
    pub _type: Type,
}

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue {
    Integer(usize),
    Field(String),
    Group(String),
    Boolean(bool),
    Array(Vec<InputValue>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Function {
    pub function_name: Identifier,
    pub inputs: Vec<InputModel>,
    pub returns: Vec<Type>,
    pub statements: Vec<Statement>,
}

impl Function {
    pub fn get_name(&self) -> String {
        self.function_name.name.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub name: Identifier,
    pub num_parameters: usize,
    pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
}

impl<'ast> Program {
    pub fn new() -> Self {
        Self {
            name: Identifier::new("".into()),
            num_parameters: 0,
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.name.clone()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Identifier::new(name);
        self
    }
}
