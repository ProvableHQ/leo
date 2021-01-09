use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct TupleInitExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub elements: Vec<Arc<Expression>>,
}

impl Node for TupleInitExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for TupleInitExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.elements.iter().for_each(|element| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        let mut output = vec![];
        for element in self.elements.iter() {
            output.push(element.get_type()?);
        }
        Some(Type::Tuple(output))
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut consts = vec![];
        for element in self.elements.iter() {
            if let Some(const_value) = element.const_value() {
                consts.push(const_value);
            } else {
                return None;
            }
        }
        Some(ConstValue::Tuple(consts))
    }
}

impl FromAst<leo_ast::TupleInitExpression> for TupleInitExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::TupleInitExpression, expected_type: Option<PartialType>) -> Result<TupleInitExpression, AsgConvertError> {
        let tuple_types = match expected_type {
            Some(PartialType::Tuple(sub_types)) => Some(sub_types),
            None => None,
            x => return Err(AsgConvertError::unexpected_type("tuple", x.map(|x| x.to_string()).as_deref(), &value.span)),
        };

        if let Some(tuple_types) = tuple_types.as_ref() {
            if tuple_types.len() != value.elements.len() {
                return Err(AsgConvertError::unexpected_type(&*format!("tuple of length {}", tuple_types.len()), Some(&*format!("tuple of length {}", value.elements.len())), &value.span));
            }
        }

        let elements = value.elements.iter().enumerate()
            .map(|(i, e)| Arc::<Expression>::from_ast(scope, e, tuple_types.as_ref().map(|x| x.get(i)).flatten().cloned().flatten()))
            .collect::<Result<Vec<_>, AsgConvertError>>()?;
        
        Ok(TupleInitExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            elements,
        })
    }
}

impl Into<leo_ast::TupleInitExpression> for &TupleInitExpression {
    fn into(self) -> leo_ast::TupleInitExpression {
        leo_ast::TupleInitExpression {
            elements: self.elements.iter().map(|e| e.as_ref().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}