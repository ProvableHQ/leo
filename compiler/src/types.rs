//! A typed Leo program consists of import, struct, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::{errors::IntegerError, Import};

use crate::errors::ValueError;
use snarkos_models::curves::{Field, PrimeField, Group};
use snarkos_models::gadgets::{
    r1cs::Variable as R1CSVariable,
    utilities::{
        boolean::Boolean, uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64,
        uint8::UInt8,
    },
};
use std::collections::HashMap;
use std::marker::PhantomData;

/// A variable in a constraint system.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variable<G: Group, F: Field + PrimeField> {
    pub name: String,
    pub(crate) _group: PhantomData<G>,
    pub(crate) _engine: PhantomData<F>,
}

impl<G: Group, F: Field + PrimeField> Variable<G, F> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            _group: PhantomData::<G>,
            _engine: PhantomData::<F>,
        }
    }
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

impl Integer {
    pub fn to_usize(&self) -> usize {
        match self {
            Integer::U8(u8) => u8.value.unwrap() as usize,
            Integer::U16(u16) => u16.value.unwrap() as usize,
            Integer::U32(u32) => u32.value.unwrap() as usize,
            Integer::U64(u64) => u64.value.unwrap() as usize,
            Integer::U128(u128) => u128.value.unwrap() as usize,
        }
    }

    pub(crate) fn get_type(&self) -> IntegerType {
        match self {
            Integer::U8(_u8) => IntegerType::U8,
            Integer::U16(_u16) => IntegerType::U16,
            Integer::U32(_u32) => IntegerType::U32,
            Integer::U64(_u64) => IntegerType::U64,
            Integer::U128(_u128) => IntegerType::U128,
        }
    }

    pub(crate) fn expect_type(&self, integer_type: &IntegerType) -> Result<(), IntegerError> {
        if self.get_type() != *integer_type {
            unimplemented!(
                "expected integer type {}, got {}",
                self.get_type(),
                integer_type
            )
        }

        Ok(())
    }
}

/// A constant or allocated element in the field
#[derive(Clone, PartialEq, Eq)]
pub enum FieldElement<F: Field + PrimeField> {
    Constant(F),
    Allocated(Option<F>, R1CSVariable),
}

// /// A constant or allocated element in the field
// #[derive(Clone, PartialEq, Eq)]
// pub enum GroupElement<G: Field + PrimeField> {
//     Constant(G),
//     Allocated(Option<G>, R1CSVariable),
// }

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOrExpression<G: Group, F: Field + PrimeField> {
    Range(Option<Integer>, Option<Integer>),
    Expression(Expression<G, F>),
}

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadOrExpression<G: Group, F: Field + PrimeField> {
    Spread(Expression<G, F>),
    Expression(Expression<G, F>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<G: Group, F: Field + PrimeField> {
    // Variable
    Variable(Variable<G, F>),

    // Values
    Integer(Integer),
    FieldElement(FieldElement<F>),
    GroupElement(G),
    Boolean(Boolean),

    // Number operations
    Add(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Sub(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Mul(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Div(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Pow(Box<Expression<G, F>>, Box<Expression<G, F>>),

    // Boolean operations
    Not(Box<Expression<G, F>>),
    Or(Box<Expression<G, F>>, Box<Expression<G, F>>),
    And(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Eq(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Geq(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Gt(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Leq(Box<Expression<G, F>>, Box<Expression<G, F>>),
    Lt(Box<Expression<G, F>>, Box<Expression<G, F>>),

    // Conditionals
    IfElse(Box<Expression<G, F>>, Box<Expression<G, F>>, Box<Expression<G, F>>),

    // Arrays
    Array(Vec<Box<SpreadOrExpression<G, F>>>),
    ArrayAccess(Box<Expression<G, F>>, Box<RangeOrExpression<G, F>>), // (array name, range)

    // Structs
    Struct(Variable<G, F>, Vec<StructMember<G, F>>),
    StructMemberAccess(Box<Expression<G, F>>, Variable<G, F>), // (struct name, struct member name)

    // Functions
    FunctionCall(Variable<G, F>, Vec<Expression<G, F>>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee<G: Group, F: Field + PrimeField> {
    Variable(Variable<G, F>),
    Array(Box<Assignee<G, F>>, RangeOrExpression<G, F>),
    StructMember(Box<Assignee<G, F>>, Variable<G, F>),
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
pub enum Type<G: Group, F: Field + PrimeField> {
    IntegerType(IntegerType),
    FieldElement,
    GroupElement,
    Boolean,
    Array(Box<Type<G, F>>, Vec<usize>),
    Struct(Variable<G, F>),
}

impl<G: Group, F: Field + PrimeField> Type<G, F> {
    pub fn next_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let _type = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[1..]);

            return Type::Array(Box::new(_type), next);
        }

        _type
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConditionalNestedOrEnd<G: Group, F: Field + PrimeField> {
    Nested(Box<ConditionalStatement<G, F>>),
    End(Vec<Statement<G, F>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ConditionalStatement<G: Group, F: Field + PrimeField> {
    pub condition: Expression<G, F>,
    pub statements: Vec<Statement<G, F>>,
    pub next: Option<ConditionalNestedOrEnd<G, F>>,
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq)]
pub enum Statement<G: Group, F: Field + PrimeField> {
    // Declaration(Variable),
    Return(Vec<Expression<G, F>>),
    Definition(Assignee<G, F>, Option<Type<G, F>>, Expression<G, F>),
    Assign(Assignee<G, F>, Expression<G, F>),
    MultipleAssign(Vec<Assignee<G, F>>, Expression<G, F>),
    Conditional(ConditionalStatement<G, F>),
    For(Variable<G, F>, Integer, Integer, Vec<Statement<G, F>>),
    AssertEq(Expression<G, F>, Expression<G, F>),
    Expression(Expression<G, F>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructMember<G: Group, F: Field + PrimeField> {
    pub variable: Variable<G, F>,
    pub expression: Expression<G, F>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct StructField<G: Group, F: Field + PrimeField> {
    pub variable: Variable<G, F>,
    pub _type: Type<G, F>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Struct<G: Group, F: Field + PrimeField> {
    pub variable: Variable<G, F>,
    pub fields: Vec<StructField<G, F>>,
}

/// Function parameters

#[derive(Clone, PartialEq, Eq)]
pub struct InputModel<G: Group, F: Field + PrimeField> {
    pub private: bool,
    pub _type: Type<G, F>,
    pub variable: Variable<G, F>,
}

impl<G: Group, F: Field + PrimeField> InputModel<G, F> {
    pub fn inner_type(&self) -> Result<Type<G, F>, ValueError> {
        match &self._type {
            Type::Array(ref _type, _length) => Ok(*_type.clone()),
            ref _type => Err(ValueError::ArrayModel(_type.to_string())),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue<G: Group, F: Field + PrimeField> {
    Integer(usize),
    Field(F),
    Boolean(bool),
    Array(Vec<InputValue<G, F>>),
}

/// The given name for a defined function in the program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionName(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct Function<G: Group, F: Field + PrimeField> {
    pub function_name: FunctionName,
    pub inputs: Vec<InputModel<G, F>>,
    pub returns: Vec<Type<G, F>>,
    pub statements: Vec<Statement<G, F>>,
}

impl<G: Group, F: Field + PrimeField> Function<G, F> {
    pub fn get_name(&self) -> String {
        self.function_name.0.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program<G: Group, F: Field + PrimeField> {
    pub name: Variable<G, F>,
    pub num_parameters: usize,
    pub imports: Vec<Import<G, F>>,
    pub structs: HashMap<Variable<G, F>, Struct<G, F>>,
    pub functions: HashMap<FunctionName, Function<G, F>>,
}

impl<'ast, G: Group, F: Field + PrimeField> Program<G, F> {
    pub fn new() -> Self {
        Self {
            name: Variable {
                name: "".into(),
                _group: PhantomData::<G>,
                _engine: PhantomData::<F>,
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
            _group: PhantomData::<G>,
            _engine: PhantomData::<F>,
        };
        self
    }
}
