//! A typed Leo program consists of import, struct, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;

use snarkos_models::curves::{Field, PrimeField};
use std::collections::HashMap;
use std::marker::PhantomData;

/// A variable in a constraint system.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variable<F: Field + PrimeField> {
    pub name: String,
    pub(crate) _field: PhantomData<F>,
}

/// An integer type enum wrapping the integer value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Integer {
    U8(u8),
    U32(u32),
    // U64(u64),
}

impl Integer {
    pub fn to_usize(&self) -> usize {
        match *self {
            Integer::U8(num) => num as usize,
            Integer::U32(num) => num as usize,
            // U64(u64)
        }
    }
}

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOrExpression<F: Field + PrimeField> {
    Range(Option<Integer>, Option<Integer>),
    Expression(Expression<F>),
}

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadOrExpression<F: Field + PrimeField> {
    Spread(Expression<F>),
    Expression(Expression<F>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<F: Field + PrimeField> {
    // Variable
    Variable(Variable<F>),

    // Values
    Integer(Integer),
    FieldElement(F),
    Boolean(bool),

    // Number operations
    Add(Box<Expression<F>>, Box<Expression<F>>),
    Sub(Box<Expression<F>>, Box<Expression<F>>),
    Mul(Box<Expression<F>>, Box<Expression<F>>),
    Div(Box<Expression<F>>, Box<Expression<F>>),
    Pow(Box<Expression<F>>, Box<Expression<F>>),

    // Boolean operations
    Not(Box<Expression<F>>),
    Or(Box<Expression<F>>, Box<Expression<F>>),
    And(Box<Expression<F>>, Box<Expression<F>>),
    Eq(Box<Expression<F>>, Box<Expression<F>>),
    Geq(Box<Expression<F>>, Box<Expression<F>>),
    Gt(Box<Expression<F>>, Box<Expression<F>>),
    Leq(Box<Expression<F>>, Box<Expression<F>>),
    Lt(Box<Expression<F>>, Box<Expression<F>>),

    // Conditionals
    IfElse(Box<Expression<F>>, Box<Expression<F>>, Box<Expression<F>>),

    // Arrays
    Array(Vec<Box<SpreadOrExpression<F>>>),
    ArrayAccess(Box<Expression<F>>, Box<RangeOrExpression<F>>), // (array name, range)

    // Structs
    Struct(Variable<F>, Vec<StructMember<F>>),
    StructMemberAccess(Box<Expression<F>>, Variable<F>), // (struct name, struct member name)

    // Functions
    FunctionCall(Variable<F>, Vec<Expression<F>>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee<F: Field + PrimeField> {
    Variable(Variable<F>),
    Array(Box<Assignee<F>>, RangeOrExpression<F>),
    StructMember(Box<Assignee<F>>, Variable<F>),
}

/// Explicit integer type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntegerType {
    U8,
    U32,
}

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<F: Field + PrimeField> {
    IntegerType(IntegerType),
    FieldElement,
    Boolean,
    Array(Box<Type<F>>, usize),
    Struct(Variable<F>),
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConditionalNestedOrEnd<F: Field + PrimeField> {
    Nested(Box<ConditionalStatement<F>>),
    End(Vec<Statement<F>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ConditionalStatement<F: Field + PrimeField> {
    pub condition: Expression<F>,
    pub statements: Vec<Statement<F>>,
    pub next: Option<ConditionalNestedOrEnd<F>>,
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq)]
pub enum Statement<F: Field + PrimeField> {
    // Declaration(Variable),
    Return(Vec<Expression<F>>),
    Definition(Assignee<F>, Option<Type<F>>, Expression<F>),
    Assign(Assignee<F>, Expression<F>),
    MultipleAssign(Vec<Assignee<F>>, Expression<F>),
    Conditional(ConditionalStatement<F>),
    For(Variable<F>, Integer, Integer, Vec<Statement<F>>),
    AssertEq(Expression<F>, Expression<F>),
    Expression(Expression<F>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructMember<F: Field + PrimeField> {
    pub variable: Variable<F>,
    pub expression: Expression<F>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct StructField<F: Field + PrimeField> {
    pub variable: Variable<F>,
    pub _type: Type<F>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Struct<F: Field + PrimeField> {
    pub variable: Variable<F>,
    pub fields: Vec<StructField<F>>,
}

/// Function parameters

#[derive(Clone, PartialEq, Eq)]
pub struct ParameterModel<F: Field + PrimeField> {
    pub private: bool,
    pub _type: Type<F>,
    pub variable: Variable<F>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ParameterValue<F: Field + PrimeField> {
    Integer(usize),
    Field(F),
    Boolean(bool),
}

/// The given name for a defined function in the program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionName(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct Function<F: Field + PrimeField> {
    pub function_name: FunctionName,
    pub parameters: Vec<ParameterModel<F>>,
    pub returns: Vec<Type<F>>,
    pub statements: Vec<Statement<F>>,
}

impl<F: Field + PrimeField> Function<F> {
    pub fn get_name(&self) -> String {
        self.function_name.0.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program<F: Field + PrimeField> {
    pub name: Variable<F>,
    pub num_parameters: usize,
    pub imports: Vec<Import<F>>,
    pub structs: HashMap<Variable<F>, Struct<F>>,
    pub functions: HashMap<FunctionName, Function<F>>,
}

impl<'ast, F: Field + PrimeField> Program<F> {
    pub fn new() -> Self {
        Self {
            name: Variable {
                name: "".into(),
                _field: PhantomData::<F>,
            },
            num_parameters: 0,
            imports: vec![],
            structs: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.name.clone()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Variable {
            name,
            _field: PhantomData::<F>,
        };
        self
    }
}
