//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::Variable as R1CSVariable,
        utilities::{
            boolean::Boolean, uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64,
            uint8::UInt8,
        },
    },
};
use std::{collections::HashMap, marker::PhantomData};

/// An identifier in the constrained program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier<F: Field + PrimeField> {
    pub name: String,
    pub(crate) _engine: PhantomData<F>,
}

impl<F: Field + PrimeField> Identifier<F> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            _engine: PhantomData::<F>,
        }
    }

    pub fn is_self(&self) -> bool {
        self.name == "Self"
    }
}

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq)]
pub struct Variable<F: Field + PrimeField> {
    pub identifier: Identifier<F>,
    pub mutable: bool,
    pub _type: Option<Type<F>>,
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

/// A constant or allocated element in the field
#[derive(Clone, PartialEq, Eq)]
pub enum FieldElement<F: Field + PrimeField> {
    Constant(F),
    Allocated(Option<F>, R1CSVariable),
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
    // Identifier
    Identifier(Identifier<F>),

    // Values
    Integer(Integer),
    Field(String),
    Group(String),
    Boolean(Boolean),
    Implicit(String),

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

    // Circuits
    Circuit(Identifier<F>, Vec<CircuitFieldDefinition<F>>),
    CircuitMemberAccess(Box<Expression<F>>, Identifier<F>), // (declared circuit name, circuit member name)
    CircuitStaticFunctionAccess(Box<Expression<F>>, Identifier<F>), // (defined circuit name, circuit static member name)

    // Functions
    FunctionCall(Box<Expression<F>>, Vec<Expression<F>>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee<F: Field + PrimeField> {
    Identifier(Identifier<F>),
    Array(Box<Assignee<F>>, RangeOrExpression<F>),
    CircuitField(Box<Assignee<F>>, Identifier<F>), // (circuit name, circuit field name)
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
pub enum Type<F: Field + PrimeField> {
    IntegerType(IntegerType),
    Field,
    Group,
    Boolean,
    Array(Box<Type<F>>, Vec<usize>),
    Circuit(Identifier<F>),
    SelfType,
}

impl<F: Field + PrimeField> Type<F> {
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
    Return(Vec<Expression<F>>),
    Definition(Variable<F>, Expression<F>),
    Assign(Assignee<F>, Expression<F>),
    MultipleAssign(Vec<Variable<F>>, Expression<F>),
    Conditional(ConditionalStatement<F>),
    For(Identifier<F>, Integer, Integer, Vec<Statement<F>>),
    AssertEq(Expression<F>, Expression<F>),
    Expression(Expression<F>),
}

/// Circuits

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitFieldDefinition<F: Field + PrimeField> {
    pub identifier: Identifier<F>,
    pub expression: Expression<F>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CircuitMember<F: Field + PrimeField> {
    CircuitField(Identifier<F>, Type<F>),
    CircuitFunction(bool, Function<F>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Circuit<F: Field + PrimeField> {
    pub identifier: Identifier<F>,
    pub members: Vec<CircuitMember<F>>,
}

/// Function parameters

#[derive(Clone, PartialEq, Eq)]
pub struct InputModel<F: Field + PrimeField> {
    pub identifier: Identifier<F>,
    pub mutable: bool,
    pub private: bool,
    pub _type: Type<F>,
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
pub struct Function<F: Field + PrimeField> {
    pub function_name: Identifier<F>,
    pub inputs: Vec<InputModel<F>>,
    pub returns: Vec<Type<F>>,
    pub statements: Vec<Statement<F>>,
}

impl<F: Field + PrimeField> Function<F> {
    pub fn get_name(&self) -> String {
        self.function_name.name.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program<F: Field + PrimeField> {
    pub name: Identifier<F>,
    pub num_parameters: usize,
    pub imports: Vec<Import<F>>,
    pub circuits: HashMap<Identifier<F>, Circuit<F>>,
    pub functions: HashMap<Identifier<F>, Function<F>>,
}

impl<'ast, F: Field + PrimeField> Program<F> {
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
