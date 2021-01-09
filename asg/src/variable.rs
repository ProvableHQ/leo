use leo_ast::Identifier;
use crate::{ Type, Expression, ConstValue, Statement };
use std::sync::{ Arc, Weak };
use std::cell::RefCell;

//todo: fill out
pub enum VariableDeclaration {
    Definition,
    IterationDefinition,
    Parameter,
    Input,
    //...
}

pub struct InnerVariable {
    pub name: Identifier,
    pub type_: Type,
    pub mutable: bool,
    pub declaration: VariableDeclaration,
    pub const_value: Option<ConstValue>,
    pub references: Vec<Weak<Expression>>, // all Expression::VariableRef or panic
    pub assignments: Vec<Weak<Statement>>, // all Statement::Assign or panic
}

pub type Variable = Arc<RefCell<InnerVariable>>;
pub type WeakVariable = Weak<RefCell<InnerVariable>>;
