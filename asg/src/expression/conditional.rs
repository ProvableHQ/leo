use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct ConditionalExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub condition: Arc<Expression>,
    pub if_true: Arc<Expression>,
    pub if_false: Arc<Expression>,
}

impl Node for ConditionalExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ConditionalExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.condition.set_parent(Arc::downgrade(expr));
        self.if_true.set_parent(Arc::downgrade(expr));
        self.if_false.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        self.if_true.get_type()
    }

    fn is_mut_ref(&self) -> bool {
        self.if_true.is_mut_ref() && self.if_false.is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        if let Some(ConstValue::Boolean(switch)) = self.condition.const_value() {
            if switch {
                self.if_true.const_value()
            } else {
                self.if_false.const_value()
            }
        } else {
            None
        }
    }
}

impl FromAst<leo_ast::ConditionalExpression> for ConditionalExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::ConditionalExpression, expected_type: Option<PartialType>) -> Result<ConditionalExpression, AsgConvertError> {
        Ok(ConditionalExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            condition: Arc::<Expression>::from_ast(scope, &*value.condition, Some(Type::Boolean.partial()))?,
            if_true: Arc::<Expression>::from_ast(scope, &*value.if_true, expected_type.clone())?,
            if_false: Arc::<Expression>::from_ast(scope, &*value.if_false, expected_type)?,
        })
    }
}

impl Into<leo_ast::ConditionalExpression> for &ConditionalExpression {
    fn into(self) -> leo_ast::ConditionalExpression {
        leo_ast::ConditionalExpression {
            condition: Box::new(self.condition.as_ref().into()),
            if_true: Box::new(self.if_true.as_ref().into()),
            if_false: Box::new(self.if_false.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}