use crate::program::Node;

// Program Nodes - Wrappers for different types in a program.
pub type ExpressionNode<'ast> = Node<Expression<'ast>>;
pub type ExpressionListNode<'ast> = Node<ExpressionList<'ast>>;
pub type StatementNode<'ast> = Node<Statement<'ast>>;
pub type VariableNode<'ast> = Node<Variable<'ast>>;

/// Identifier string
pub type Identifier<'ast> = &'ast str;

/// Program variable
#[derive(Debug, Clone, PartialEq)]
pub struct Variable<'ast> {
    pub id: Identifier<'ast>,
}

/// Program expression that evaluates to a value
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'ast> {
    // Expression identifier
    Identifier(Identifier<'ast>),
    // Values
    Boolean(bool),
    Field(Identifier<'ast>),
    // Variable
    Variable(VariableNode<'ast>),
    // Not expression
    Not(Box<ExpressionNode<'ast>>),
    // Binary expression
    Or(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    And(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Eq(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Neq(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Geq(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Gt(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Leq(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Lt(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Add(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Sub(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Mul(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Div(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
    Pow(Box<ExpressionNode<'ast>>, Box<ExpressionNode<'ast>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionList<'ast> {
    pub expressions: Vec<ExpressionNode<'ast>>,
}

/// Program statement that defines some action (or expression) to be carried out
#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'ast> {
    Declaration(VariableNode<'ast>),
    Definition(VariableNode<'ast>, ExpressionNode<'ast>),
    Return(ExpressionListNode<'ast>),
}
