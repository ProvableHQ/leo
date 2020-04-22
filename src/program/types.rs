//! A typed program in aleo consists of import, struct, and function definitions.
//! Each defined type consists of typed statements and expressions.
//!
//! @file types.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::Import;

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
#[derive(Debug, Clone)]
pub enum Integer {
    // U8(u8),
    U32(u32),
    // U64(u64),
}

/// Spread operator or u32 expression enum
#[derive(Debug, Clone)]
pub enum IntegerSpreadOrExpression<F: Field + PrimeField> {
    Spread(IntegerExpression<F>),
    Expression(IntegerExpression<F>),
}

/// Range or integer expression enum
#[derive(Debug, Clone)]
pub enum IntegerRangeOrExpression<F: Field + PrimeField> {
    Range(Option<IntegerExpression<F>>, Option<IntegerExpression<F>>),
    Expression(IntegerExpression<F>),
}

/// Expression that evaluates to a u32 value
#[derive(Debug, Clone)]
pub enum IntegerExpression<F: Field + PrimeField> {
    Variable(Variable<F>),
    Number(Integer),
    // Operators
    Add(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Sub(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Mul(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Div(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Pow(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    // Conditionals
    IfElse(
        Box<BooleanExpression<F>>,
        Box<IntegerExpression<F>>,
        Box<IntegerExpression<F>>,
    ),
    // Arrays
    Array(Vec<Box<IntegerSpreadOrExpression<F>>>),
}

/// Spread or field expression enum
#[derive(Debug, Clone)]
pub enum FieldSpreadOrExpression<F: Field + PrimeField> {
    Spread(FieldExpression<F>),
    Expression(FieldExpression<F>),
}

/// Expression that evaluates to a field value
#[derive(Debug, Clone)]
pub enum FieldExpression<F: Field + PrimeField> {
    Variable(Variable<F>),
    Number(F),
    // Operators
    Add(Box<FieldExpression<F>>, Box<FieldExpression<F>>),
    Sub(Box<FieldExpression<F>>, Box<FieldExpression<F>>),
    Mul(Box<FieldExpression<F>>, Box<FieldExpression<F>>),
    Div(Box<FieldExpression<F>>, Box<FieldExpression<F>>),
    Pow(Box<FieldExpression<F>>, Box<FieldExpression<F>>),
    // Conditionals
    IfElse(
        Box<BooleanExpression<F>>,
        Box<FieldExpression<F>>,
        Box<FieldExpression<F>>,
    ),
    // Arrays
    Array(Vec<Box<FieldSpreadOrExpression<F>>>),
}

/// Spread or field expression enum
#[derive(Debug, Clone)]
pub enum BooleanSpreadOrExpression<F: Field + PrimeField> {
    Spread(BooleanExpression<F>),
    Expression(BooleanExpression<F>),
}

/// Expression that evaluates to a boolean value
#[derive(Debug, Clone)]
pub enum BooleanExpression<F: Field + PrimeField> {
    Variable(Variable<F>),
    Value(bool),
    // Boolean operators
    Not(Box<BooleanExpression<F>>),
    Or(Box<BooleanExpression<F>>, Box<BooleanExpression<F>>),
    And(Box<BooleanExpression<F>>, Box<BooleanExpression<F>>),
    BoolEq(Box<BooleanExpression<F>>, Box<BooleanExpression<F>>),
    // Integer operators
    IntegerEq(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Geq(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Gt(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Leq(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    Lt(Box<IntegerExpression<F>>, Box<IntegerExpression<F>>),
    // Field operators
    FieldEq(Box<FieldExpression<F>>, Box<FieldExpression<F>>),
    // Conditionals
    IfElse(
        Box<BooleanExpression<F>>,
        Box<BooleanExpression<F>>,
        Box<BooleanExpression<F>>,
    ),
    // Arrays
    Array(Vec<Box<BooleanSpreadOrExpression<F>>>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone)]
pub enum Expression<F: Field + PrimeField> {
    Integer(IntegerExpression<F>),
    FieldElement(FieldExpression<F>),
    Boolean(BooleanExpression<F>),
    Variable(Variable<F>),
    Struct(Variable<F>, Vec<StructMember<F>>),
    ArrayAccess(Box<Expression<F>>, IntegerRangeOrExpression<F>),
    StructMemberAccess(Box<Expression<F>>, Variable<F>), // (struct name, struct member name)
    FunctionCall(Box<Expression<F>>, Vec<Expression<F>>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone)]
pub enum Assignee<F: Field + PrimeField> {
    Variable(Variable<F>),
    Array(Box<Assignee<F>>, IntegerRangeOrExpression<F>),
    StructMember(Box<Assignee<F>>, Variable<F>),
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone)]
pub enum Statement<F: Field + PrimeField> {
    // Declaration(Variable),
    Definition(Assignee<F>, Expression<F>),
    For(
        Variable<F>,
        IntegerExpression<F>,
        IntegerExpression<F>,
        Vec<Statement<F>>,
    ),
    Return(Vec<Expression<F>>),
}

/// Explicit type used for defining struct members and function parameters
#[derive(Clone, Debug)]
pub enum Type<F: Field + PrimeField> {
    U32,
    FieldElement,
    Boolean,
    Array(Box<Type<F>>, usize),
    Struct(Variable<F>),
}

#[derive(Clone, Debug)]
pub struct StructMember<F: Field + PrimeField> {
    pub variable: Variable<F>,
    pub expression: Expression<F>,
}

#[derive(Clone)]
pub struct StructField<F: Field + PrimeField> {
    pub variable: Variable<F>,
    pub ty: Type<F>,
}

#[derive(Clone)]
pub struct Struct<F: Field + PrimeField> {
    pub variable: Variable<F>,
    pub fields: Vec<StructField<F>>,
}

/// Function parameters

#[derive(Clone)]
pub struct Parameter<F: Field + PrimeField> {
    pub private: bool,
    pub ty: Type<F>,
    pub variable: Variable<F>,
}

/// The given name for a defined function in the program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionName(pub String);

#[derive(Clone)]
pub struct Function<F: Field + PrimeField> {
    pub function_name: FunctionName,
    pub parameters: Vec<Parameter<F>>,
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
pub struct Program<'ast, F: Field + PrimeField> {
    pub name: Variable<F>,
    pub imports: Vec<Import<'ast>>,
    pub structs: HashMap<Variable<F>, Struct<F>>,
    pub functions: HashMap<FunctionName, Function<F>>,
}

impl<'ast, F: Field + PrimeField> Program<'ast, F> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Variable {
            name,
            _field: PhantomData::<F>,
        };
        self
    }
}
