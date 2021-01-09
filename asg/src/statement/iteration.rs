use crate::Span;
use crate::{ Statement, Expression, Variable, InnerVariable, Scope, AsgConvertError, FromAst, ExpressionNode, Type, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct IterationStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub variable: Variable,
    pub start: Arc<Expression>,
    pub stop: Arc<Expression>,
    pub body: Arc<Statement>,
}

impl FromAst<leo_ast::IterationStatement> for IterationStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::IterationStatement, _expected_type: Option<PartialType>) -> Result<Self, AsgConvertError> {
        // todo: is u32 the right type to enforce
        let expected_index_type = Some(Type::Integer(leo_ast::IntegerType::U32).into());
        let start = Arc::<Expression>::from_ast(scope, &statement.start, expected_index_type.clone())?;
        let stop = Arc::<Expression>::from_ast(scope, &statement.stop, expected_index_type)?;
        let variable = Arc::new(RefCell::new(InnerVariable {
            name: statement.variable.clone(),
            type_: start.get_type().ok_or_else(|| AsgConvertError::unresolved_type(&statement.variable.name, &statement.span))?,
            mutable: false,
            declaration: crate::VariableDeclaration::IterationDefinition,
            const_value: None,
            references: vec![],
        }));
        scope.borrow_mut().variables.insert(statement.variable.name.clone(), variable.clone());

        Ok(IterationStatement {
            parent: None,
            span: Some(statement.span.clone()),
            variable,
            stop,
            start,
            body: Arc::new(Statement::Block(crate::BlockStatement::from_ast(scope, &statement.block, None)?)),
        })
    }
}

impl Into<leo_ast::IterationStatement> for &IterationStatement {
    fn into(self) -> leo_ast::IterationStatement {
        leo_ast::IterationStatement {
            variable: self.variable.borrow().name.clone(),
            start: self.start.as_ref().into(),
            stop: self.stop.as_ref().into(),
            block: match self.body.as_ref() {
                Statement::Block(block) => block.into(),
                _ => unimplemented!(),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}