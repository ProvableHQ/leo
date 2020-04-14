//! A zokrates_program consists of nodes that keep track of position and wrap zokrates_program types.
//!
//! @file types.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use std::collections::HashMap;

/// A variable in a constraint system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

/// Spread operator
#[derive(Debug, Clone)]
pub struct FieldSpread(pub FieldExpression);

/// Spread or field expression enum
#[derive(Debug, Clone)]
pub enum FieldSpreadOrExpression {
    Spread(FieldSpread),
    FieldExpression(FieldExpression),
}

/// Expression that evaluates to a field value
#[derive(Debug, Clone)]
pub enum FieldExpression {
    Variable(Variable),
    Number(u32),
    // Operators
    Add(Box<FieldExpression>, Box<FieldExpression>),
    Sub(Box<FieldExpression>, Box<FieldExpression>),
    Mul(Box<FieldExpression>, Box<FieldExpression>),
    Div(Box<FieldExpression>, Box<FieldExpression>),
    Pow(Box<FieldExpression>, Box<FieldExpression>),
    // Conditionals
    IfElse(
        Box<BooleanExpression>,
        Box<FieldExpression>,
        Box<FieldExpression>,
    ),
    // Arrays
    Array(Vec<Box<FieldSpreadOrExpression>>),
}

/// Spread operator
#[derive(Debug, Clone)]
pub struct BooleanSpread(pub BooleanExpression);

/// Spread or field expression enum
#[derive(Debug, Clone)]
pub enum BooleanSpreadOrExpression {
    Spread(BooleanSpread),
    BooleanExpression(BooleanExpression),
}

/// Expression that evaluates to a boolean value
#[derive(Debug, Clone)]
pub enum BooleanExpression {
    Variable(Variable),
    Value(bool),
    // Boolean operators
    Not(Box<BooleanExpression>),
    Or(Box<BooleanExpression>, Box<BooleanExpression>),
    And(Box<BooleanExpression>, Box<BooleanExpression>),
    BoolEq(Box<BooleanExpression>, Box<BooleanExpression>),
    // Field operators
    FieldEq(Box<FieldExpression>, Box<FieldExpression>),
    Geq(Box<FieldExpression>, Box<FieldExpression>),
    Gt(Box<FieldExpression>, Box<FieldExpression>),
    Leq(Box<FieldExpression>, Box<FieldExpression>),
    Lt(Box<FieldExpression>, Box<FieldExpression>),
    // Conditionals
    IfElse(
        Box<BooleanExpression>,
        Box<BooleanExpression>,
        Box<BooleanExpression>,
    ),
    // Arrays
    Array(Vec<Box<BooleanSpreadOrExpression>>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone)]
pub enum Expression {
    Boolean(BooleanExpression),
    FieldElement(FieldExpression),
    Variable(Variable),
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone)]
pub enum Statement {
    /// A statement that could be directly translated to a R1CS constraint a * b = c to be enforced
    // Constraint(QuadraticCombination, LinearCombination),
    // Declaration(Variable),
    Definition(Variable, Expression),
    Return(Vec<Expression>),
}

#[derive(Clone, Debug)]
pub enum Type {
    FieldElement,
    Boolean,
    Array(Box<Type>, usize),
    Struct(Variable),
}

#[derive(Clone)]
pub struct StructField {
    pub variable: Variable,
    pub ty: Type,
}

#[derive(Clone)]
pub struct Struct {
    pub variable: Variable,
    pub fields: Vec<StructField>,
}

#[derive(Clone, Debug)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub visibility: Option<Visibility>,
    pub ty: Type,
    pub variable: Variable,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub variable: Variable,
    pub parameters: Vec<Parameter>,
    pub returns: Vec<Type>,
    pub statements: Vec<Statement>,
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub id: String,
    pub structs: HashMap<Variable, Struct>,
    pub functions: HashMap<Variable, Function>,
    pub statements: Vec<Statement>,
    pub arguments: Vec<Variable>,
    pub returns: Vec<Variable>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable() {
        let variable = Variable("1".into());

        println!("{:#?}", variable);
    }

    // #[test]
    // fn test_linear_combination() {
    //     let variable_0 = Variable { id: 0, value: "1".into() };
    //     let variable_1 = Variable { id: 0, value: "1".into() };
    //     let linear_combination = LinearCombination(vec![variable_0, variable_1]);
    //
    //     println!("{:#?}", linear_combination);
    // }

    // #[test]
    // fn test_statement_linear() {
    //     let linear_combination = LinearCombination(vec![Variable { id: 0 }, Variable { id: 1 }]);
    //     let statement_linear = Statement::Linear(linear_combination);
    //
    //     println!("{:#?}", statement_linear);
    // }
    //
    // #[test]
    // fn test_statement_quadratic() {
    //     let linear_combination_0 = LinearCombination(vec![Variable { id: 0 }]);
    //     let linear_combination_1 = LinearCombination(vec![Variable { id: 1 }]);
    //     let statement_quadratic = Statement::Quadratic(linear_combination_0, linear_combination_1);
    //
    //     println!("{:#?}", statement_quadratic);
    // }
    //
    // #[test]
    // fn test_program() {
    //     let variable_0 = Variable{ id: 0};
    //     let linear_combination = LinearCombination(vec![variable_0.clone()]);
    //     let statement_linear = Statement::Linear(linear_combination.clone());
    //     let statement_quadratic = Statement::Quadratic(linear_combination.clone(), linear_combination);
    //     let program = Program{
    //         id: "main".into(),
    //         statements: vec![statement_linear, statement_quadratic],
    //         arguments: vec![variable_0.clone()],
    //         returns: vec![variable_0.clone()]
    //     };
    //
    //     println!("{:#?}", program);
    // }
    // #[test]
    // fn test_basic_prog() {
    //     // return 1 == 1
    //     let prog = Program {
    //         id: "main".into(),
    //         statements: vec![Statement::Return(vec![Expression::Boolean(
    //             BooleanExpression::FieldEq(
    //                 Box::new(FieldExpression::Number(1)),
    //                 Box::new(FieldExpression::Number(1)),
    //             ),
    //         )])],
    //         arguments: vec![],
    //         returns: vec![],
    //     };
    //
    //     println!("{:#?}", prog);
    // }
}
