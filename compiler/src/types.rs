//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;
use leo_types::{Assignee, Expression, Identifier, Integer, Type, Variable};

use std::collections::HashMap;


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

/// Tests

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Test(pub Function);

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub name: Identifier,
    pub num_parameters: usize,
    pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, Test>,
}

impl<'ast> Program {
    pub fn new() -> Self {
        Self {
            name: Identifier::new("".into()),
            num_parameters: 0,
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
            tests: HashMap::new(),
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
