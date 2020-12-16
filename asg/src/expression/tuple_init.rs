use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue };
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
    fn from_ast(scope: &Scope, value: &leo_ast::TupleInitExpression, expected_type: Option<Type>) -> Result<TupleInitExpression, AsgConvertError> {
        let tuple_types = match expected_type {
            Some(Type::Tuple(sub_types)) => Some(sub_types),
            None => None,
            x => return Err(AsgConvertError::unexpected_type("tuple", x.map(|x| x.to_string()).as_deref(), &value.span)),
        };

        let elements = value.elements.iter().enumerate()
            .map(|(i, e)| Arc::<Expression>::from_ast(scope, e, tuple_types.as_ref().map(|x| x.get(i)).flatten().cloned()))
            .collect::<Result<Vec<_>, AsgConvertError>>()?;
        
        if let Some(tuple_types) = tuple_types.as_ref() {
            if tuple_types.len() != elements.len() {
                return Err(AsgConvertError::unexpected_type(&*format!("tuple of length {}", tuple_types.len()), Some(&*format!("tuple of length {}", elements.len())), &value.span));
            }
            for (expected_type, element) in tuple_types.iter().zip(elements.iter()) {
                let concrete_type = element.get_type();
                if Some(expected_type) != concrete_type.as_ref() {
                    return Err(AsgConvertError::unexpected_type(&expected_type.to_string(), concrete_type.map(|x| x.to_string()).as_deref(), &value.span));
                }
            }
        }

        Ok(TupleInitExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            elements: value.elements.iter().enumerate().map(|(i, e)| Arc::<Expression>::from_ast(scope, e, tuple_types.as_ref().map(|x| x.get(i)).flatten().cloned())).collect::<Result<Vec<_>, AsgConvertError>>()?,
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