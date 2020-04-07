//! A zokrates_program consists of nodes that keep track of position and wrap zokrates_program types.
//!
//! @file types.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

// id == 0 for field values
// id < 0 for boolean values
/// A variable in a constraint system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable(pub String);
//
// /// Linear combination of variables in a program. (a + b + c)
// #[derive(Debug, Clone)]
// pub struct LinearCombination (pub Vec<Variable>);
//
// impl LinearCombination {
//     pub fn one() -> Self {
//         LinearCombination(vec![Variable{ id: 0, value: "1".into() }])
//     }
//
//     pub fn value(&self) -> String {
//         self.0[0].value.clone()
//     }
// }
//
// /// Quadratic combination of variables in a program (a * b)
// #[derive(Debug, Clone)]
// pub struct QuadraticCombination (pub LinearCombination, pub LinearCombination);

/// Expression that evaluates to a field value
#[derive(Debug, Clone)]
pub enum FieldExpression {
    Variable(Variable),
    Number(u32),
    Add(Box<FieldExpression>, Box<FieldExpression>),
    Sub(Box<FieldExpression>, Box<FieldExpression>),
    Mul(Box<FieldExpression>, Box<FieldExpression>),
    Div(Box<FieldExpression>, Box<FieldExpression>),
    Pow(Box<FieldExpression>, Box<FieldExpression>),
    IfElse(
        Box<BooleanExpression>,
        Box<FieldExpression>,
        Box<FieldExpression>,
    ),
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

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub id: String,
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
    #[test]
    fn test_basic_prog() {
        // return 1 == 1
        let prog = Program {
            id: "main".into(),
            statements: vec![Statement::Return(vec![Expression::Boolean(
                BooleanExpression::FieldEq(
                    Box::new(FieldExpression::Number(1)),
                    Box::new(FieldExpression::Number(1)),
                ),
            )])],
            arguments: vec![],
            returns: vec![],
        };

        println!("{:#?}", prog);
    }
}
