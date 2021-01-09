use crate::Span;
use crate::{ Statement, Expression, FromAst, Scope, AsgConvertError, Type, PartialType };
use std::sync::{ Weak, Arc };

pub struct ExpressionStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub expression: Arc<Expression>,
}

impl FromAst<leo_ast::ExpressionStatement> for ExpressionStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::ExpressionStatement, _expected_type: Option<PartialType>) -> Result<Self, AsgConvertError> {
        let expression = Arc::<Expression>::from_ast(scope, &statement.expression, None)?;
        
        Ok(ExpressionStatement {
            parent: None,
            span: Some(statement.span.clone()),
            expression,
        })
    }
}

impl Into<leo_ast::ExpressionStatement> for &ExpressionStatement {
    fn into(self) -> leo_ast::ExpressionStatement {
        leo_ast::ExpressionStatement {
            expression: self.expression.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}