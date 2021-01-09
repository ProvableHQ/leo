use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, AsgConvertError, FromAst, Scope, ConstValue, ConstInt, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct ArrayAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub array: Arc<Expression>,
    pub index: Arc<Expression>,
}

impl Node for ArrayAccessExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.array.set_parent(Arc::downgrade(expr));
        self.index.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        match self.array.get_type() {
            Some(Type::Array(element, _)) => Some(*element),
            _ => None,
        }
    }

    fn is_mut_ref(&self) -> bool {
        self.array.is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut array = match self.array.const_value()? {
            ConstValue::Array(values) => values,
            _ => return None,
        };
        let const_index = match self.index.const_value()? {
            ConstValue::Int(ConstInt::U32(x)) => x,
            _ => return None,
        };
        if const_index as usize >= array.len() {
            return None;
        }
        Some(array.remove(const_index as usize))
    }
}

impl FromAst<leo_ast::ArrayAccessExpression> for ArrayAccessExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::ArrayAccessExpression, expected_type: Option<PartialType>) -> Result<ArrayAccessExpression, AsgConvertError> {
        let array = Arc::<Expression>::from_ast(scope, &*value.array, Some(PartialType::Array(expected_type.clone().map(Box::new), None)))?;
        match array.get_type() {
            Some(Type::Array(..)) => (),
            type_ => return Err(AsgConvertError::unexpected_type("array", type_.map(|x| x.to_string()).as_deref(), &value.span)),
        }

        Ok(ArrayAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            array,
            index: Arc::<Expression>::from_ast(scope, &*value.index, Some(Type::Integer(leo_ast::IntegerType::U32).partial()))?,
        })
    }
}

impl Into<leo_ast::ArrayAccessExpression> for &ArrayAccessExpression {
    fn into(self) -> leo_ast::ArrayAccessExpression {
        leo_ast::ArrayAccessExpression {
            array: Box::new(self.array.as_ref().into()),
            index: Box::new(self.index.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}