use crate::Span;
use crate::{ Statement, Expression, BlockStatement, FromAst, Scope, AsgConvertError, Type, PartialType, Node };
use std::sync::{ Weak, Arc };

pub struct ConditionalStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub condition: Arc<Expression>,
    pub result: Arc<Statement>,
    pub next: Option<Arc<Statement>>,
}

impl Node for ConditionalStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::ConditionalStatement> for ConditionalStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::ConditionalStatement, _expected_type: Option<PartialType>) -> Result<Self, AsgConvertError> {
        let condition = Arc::<Expression>::from_ast(scope, &statement.condition, Some(Type::Boolean.into()))?;
        let result = Arc::new(Statement::Block(BlockStatement::from_ast(scope, &statement.block, None)?));
        let next = statement.next.as_deref().map(|next| -> Result<Arc<Statement>, AsgConvertError> { Ok(Arc::<Statement>::from_ast(scope, next, None)?) }).transpose()?;
        
        Ok(ConditionalStatement {
            parent: None,
            span: Some(statement.span.clone()),
            condition,
            result,
            next,
        })
    }
}

impl Into<leo_ast::ConditionalStatement> for &ConditionalStatement {
    fn into(self) -> leo_ast::ConditionalStatement {
        leo_ast::ConditionalStatement {
            condition: self.condition.as_ref().into(),
            block: match self.result.as_ref() {
                Statement::Block(block) => block.into(),
                _ => unimplemented!(),
            },
            next: self.next.as_deref().map(|e| Box::new(e.into())),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}