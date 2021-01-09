use crate::Span;
use crate::{ Statement, FromAst, Scope, AsgConvertError, InnerScope, PartialType, Node };
use std::sync::{ Weak, Arc };

pub struct BlockStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub statements: Vec<Arc<Statement>>,
    pub scope: Scope,
}

impl Node for BlockStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::Block> for BlockStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::Block, _expected_type: Option<PartialType>) -> Result<Self, AsgConvertError> {
        let new_scope = InnerScope::make_subscope(scope);

        let mut output = vec![];
        for item in statement.statements.iter() {
            output.push(Arc::<Statement>::from_ast(&new_scope, item, None)?);
        }
        Ok(BlockStatement {
            parent: None,
            span: Some(statement.span.clone()),
            statements: output,
            scope: new_scope,
        })
    }
}

impl Into<leo_ast::Block> for &BlockStatement {
    fn into(self) -> leo_ast::Block {
        leo_ast::Block {
            statements: self.statements.iter().map(|statement| statement.as_ref().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}