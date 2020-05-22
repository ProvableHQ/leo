//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;
use leo_gadgets::integers::{
    uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64, uint8::UInt8,
};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::utilities::boolean::Boolean;
use std::collections::HashMap;
use std::marker::PhantomData;

/// An identifier in the constrained program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier<NativeF: Field, F: Field + PrimeField> {
    pub name: String,
    _group: PhantomData<NativeF>,
    _engine: PhantomData<F>,
}

impl<NativeF: Field, F: Field + PrimeField> Identifier<NativeF, F> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            _group: PhantomData,
            _engine: PhantomData,
        }
    }

    pub fn is_self(&self) -> bool {
        self.name == "Self"
    }
}

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq)]
pub struct Variable<NativeF: Field, F: Field + PrimeField> {
    pub identifier: Identifier<NativeF, F>,
    pub mutable: bool,
    pub _type: Option<Type<NativeF, F>>,
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
pub enum RangeOrExpression<NativeF: Field, F: Field + PrimeField> {
    Range(Option<Integer>, Option<Integer>),
    Expression(Expression<NativeF, F>),
}

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadOrExpression<NativeF: Field, F: Field + PrimeField> {
    Spread(Expression<NativeF, F>),
    Expression(Expression<NativeF, F>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<NativeF: Field, F: Field + PrimeField> {
    // Identifier
    Identifier(Identifier<NativeF, F>),

    // Values
    Integer(Integer),
    FieldElement(F),
    GroupElement(NativeF, NativeF),
    Boolean(Boolean),
    Implicit(String),

    // Number operations
    Add(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Sub(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Mul(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Div(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Pow(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),

    // Boolean operations
    Not(Box<Expression<NativeF, F>>),
    Or(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    And(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Eq(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Ge(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Gt(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Le(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),
    Lt(Box<Expression<NativeF, F>>, Box<Expression<NativeF, F>>),

    // Conditionals
    IfElse(
        Box<Expression<NativeF, F>>,
        Box<Expression<NativeF, F>>,
        Box<Expression<NativeF, F>>,
    ),

    // Arrays
    Array(Vec<Box<SpreadOrExpression<NativeF, F>>>),
    ArrayAccess(
        Box<Expression<NativeF, F>>,
        Box<RangeOrExpression<NativeF, F>>,
    ), // (array name, range)

    // Circuits
    Circuit(
        Identifier<NativeF, F>,
        Vec<CircuitFieldDefinition<NativeF, F>>,
    ),
    CircuitMemberAccess(Box<Expression<NativeF, F>>, Identifier<NativeF, F>), // (declared circuit name, circuit member name)
    CircuitStaticFunctionAccess(Box<Expression<NativeF, F>>, Identifier<NativeF, F>), // (defined circuit name, circuit static member name)

    // Functions
    FunctionCall(Box<Expression<NativeF, F>>, Vec<Expression<NativeF, F>>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee<NativeF: Field, F: Field + PrimeField> {
    Identifier(Identifier<NativeF, F>),
    Array(Box<Assignee<NativeF, F>>, RangeOrExpression<NativeF, F>),
    CircuitField(Box<Assignee<NativeF, F>>, Identifier<NativeF, F>), // (circuit name, circuit field name)
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
pub enum Type<NativeF: Field, F: Field + PrimeField> {
    IntegerType(IntegerType),
    FieldElement,
    GroupElement,
    Boolean,
    Array(Box<Type<NativeF, F>>, Vec<usize>),
    Circuit(Identifier<NativeF, F>),
    SelfType,
}

impl<NativeF: Field, F: Field + PrimeField> Type<NativeF, F> {
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
pub enum ConditionalNestedOrEnd<NativeF: Field, F: Field + PrimeField> {
    Nested(Box<ConditionalStatement<NativeF, F>>),
    End(Vec<Statement<NativeF, F>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ConditionalStatement<NativeF: Field, F: Field + PrimeField> {
    pub condition: Expression<NativeF, F>,
    pub statements: Vec<Statement<NativeF, F>>,
    pub next: Option<ConditionalNestedOrEnd<NativeF, F>>,
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq)]
pub enum Statement<NativeF: Field, F: Field + PrimeField> {
    Return(Vec<Expression<NativeF, F>>),
    Definition(Variable<NativeF, F>, Expression<NativeF, F>),
    Assign(Assignee<NativeF, F>, Expression<NativeF, F>),
    MultipleAssign(Vec<Variable<NativeF, F>>, Expression<NativeF, F>),
    Conditional(ConditionalStatement<NativeF, F>),
    For(
        Identifier<NativeF, F>,
        Integer,
        Integer,
        Vec<Statement<NativeF, F>>,
    ),
    AssertEq(Expression<NativeF, F>, Expression<NativeF, F>),
    Expression(Expression<NativeF, F>),
}

/// Circuits

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitFieldDefinition<NativeF: Field, F: Field + PrimeField> {
    pub identifier: Identifier<NativeF, F>,
    pub expression: Expression<NativeF, F>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CircuitMember<NativeF: Field, F: Field + PrimeField> {
    CircuitField(Identifier<NativeF, F>, Type<NativeF, F>),
    CircuitFunction(bool, Function<NativeF, F>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Circuit<NativeF: Field, F: Field + PrimeField> {
    pub identifier: Identifier<NativeF, F>,
    pub members: Vec<CircuitMember<NativeF, F>>,
}

/// Function parameters

#[derive(Clone, PartialEq, Eq)]
pub struct InputModel<NativeF: Field, F: Field + PrimeField> {
    pub identifier: Identifier<NativeF, F>,
    pub mutable: bool,
    pub private: bool,
    pub _type: Type<NativeF, F>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue<NativeF: Field, F: Field + PrimeField> {
    Integer(usize),
    Field(F),
    Group(NativeF, NativeF),
    Boolean(bool),
    Array(Vec<InputValue<NativeF, F>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Function<NativeF: Field, F: Field + PrimeField> {
    pub function_name: Identifier<NativeF, F>,
    pub inputs: Vec<InputModel<NativeF, F>>,
    pub returns: Vec<Type<NativeF, F>>,
    pub statements: Vec<Statement<NativeF, F>>,
}

impl<NativeF: Field, F: Field + PrimeField> Function<NativeF, F> {
    pub fn get_name(&self) -> String {
        self.function_name.name.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program<NativeF: Field, F: Field + PrimeField> {
    pub name: Identifier<NativeF, F>,
    pub num_parameters: usize,
    pub imports: Vec<Import<NativeF, F>>,
    pub circuits: HashMap<Identifier<NativeF, F>, Circuit<NativeF, F>>,
    pub functions: HashMap<Identifier<NativeF, F>, Function<NativeF, F>>,
}

impl<'ast, NativeF: Field, F: Field + PrimeField> Program<NativeF, F> {
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
