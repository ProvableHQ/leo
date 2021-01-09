pub use leo_ast::UnaryOperation;
use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, Scope, AsgConvertError, FromAst, ConstValue, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct UnaryExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub operation: UnaryOperation,
    pub inner: Arc<Expression>,
}

impl Node for UnaryExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for UnaryExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.inner.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        self.inner.get_type()
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        if let Some(inner) = self.inner.const_value() {
            match self.operation {
                UnaryOperation::Not => {
                    match inner {
                        ConstValue::Boolean(value) => Some(ConstValue::Boolean(!value)),
                        _ => None,
                    }
                },
                UnaryOperation::Negate => {
                    match inner {
                        ConstValue::Int(value) => Some(ConstValue::Int(value.value_negate()?)),
                        // ConstValue::Group(value) => Some(ConstValue::Group(value)), TODO: groups
                        // ConstValue::Field(value) => Some(ConstValue::Field(-value)),
                        _ => None,
                    }
                },
            }
        } else {
            None
        }
    }
}

impl FromAst<leo_ast::UnaryExpression> for UnaryExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::UnaryExpression, expected_type: Option<PartialType>) -> Result<UnaryExpression, AsgConvertError> {
        let expected_type = match value.op {
            UnaryOperation::Not => match expected_type.map(|x| x.full()).flatten() {
                Some(Type::Boolean) | None => Some(Type::Boolean),
                Some(type_) => return Err(AsgConvertError::unexpected_type(&type_.to_string(), Some(&*Type::Boolean.to_string()), &value.span)),
            },
            UnaryOperation::Negate => match expected_type.map(|x| x.full()).flatten() {
                Some(type_ @ Type::Integer(_)) => Some(type_),
                Some(Type::Group) => Some(Type::Group),
                Some(Type::Field) => Some(Type::Field),
                None => None,
                Some(type_) => return Err(AsgConvertError::unexpected_type(&type_.to_string(), Some("integer, group, field"), &value.span)),
            },
        };
        Ok(UnaryExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            operation: value.op.clone(),
            inner: Arc::<Expression>::from_ast(scope, &*value.inner, expected_type.map(Into::into))?,
        })
    }
}

impl Into<leo_ast::UnaryExpression> for &UnaryExpression {
    fn into(self) -> leo_ast::UnaryExpression {
        leo_ast::UnaryExpression {
            op: self.operation.clone(),
            inner: Box::new(self.inner.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}