use crate::Span;
use crate::{ Statement, Expression, Scope, AsgConvertError, FromAst, Type, PartialType };
use std::sync::{ Weak, Arc };

pub struct ReturnStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub expression: Arc<Expression>,
}


impl FromAst<leo_ast::ReturnStatement> for ReturnStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::ReturnStatement, _expected_type: Option<PartialType>) -> Result<Self, AsgConvertError> {
        let return_type: Option<Type> = scope.borrow().resolve_current_function().map(|x| x.output.clone()).map(Into::into);
        Ok(ReturnStatement {
            parent: None,
            span: Some(statement.span.clone()),
            expression: Arc::<Expression>::from_ast(scope, &statement.expression, return_type.map(Into::into))?,
        })
    }
}

impl Into<leo_ast::ReturnStatement> for &ReturnStatement {
    fn into(self) -> leo_ast::ReturnStatement {
        leo_ast::ReturnStatement {
            expression: self.expression.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}