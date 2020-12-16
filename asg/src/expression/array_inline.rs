use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue };
use std::sync::{ Weak, Arc };
use leo_ast::SpreadOrExpression;
use std::cell::RefCell;

pub struct ArrayInlineExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub elements: Vec<(Arc<Expression>, bool)>, // bool = if spread
}

impl ArrayInlineExpression {
    pub fn len(&self) -> usize {
        self.elements.iter().map(|(expr, is_spread)| if *is_spread {
            match expr.get_type() {
                Some(Type::Array(item, len)) => len,
                _ => 0,
            }
        } else {
            1
        }).sum()
    }
}

impl Node for ArrayInlineExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayInlineExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }
    
    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.elements.iter().for_each(|(element, _)| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        Some(Type::Array(Box::new(self.elements.first()?.0.get_type()?), self.len()))
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut const_values = vec![];
        for (expr, spread) in self.elements.iter() {
            if *spread {
                match expr.const_value()? {
                    ConstValue::Array(items) => const_values.extend(items),
                    _ => return None,
                }
            } else {
                const_values.push(expr.const_value()?);
            }
        }
        Some(ConstValue::Array(const_values))
    }
}

impl FromAst<leo_ast::ArrayInlineExpression> for ArrayInlineExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::ArrayInlineExpression, expected_type: Option<Type>) -> Result<ArrayInlineExpression, AsgConvertError> {
        let (mut expected_item, expected_len) = match expected_type {
            Some(Type::Array(item, dims)) => (Some(*item), Some(dims)),
            None => (None, None),
            Some(type_) => return Err(AsgConvertError::unexpected_type(&type_.to_string(), Some("array"), &value.span)),
        };

        // todo: make sure to add some tests with spreading into multidimensional arrays

        let mut len = 0;
        let output = ArrayInlineExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            elements: value.elements.iter().map(|e| match e {
                SpreadOrExpression::Expression(e) => {
                    let expr = Arc::<Expression>::from_ast(scope, e, expected_item.clone())?;
                    if expected_item.is_none() {
                        expected_item = expr.get_type();
                    }
                    len += 1;
                    Ok((expr, false))
                },
                SpreadOrExpression::Spread(e) => {
                    let expr = Arc::<Expression>::from_ast(scope, e, None)?;
                    match expr.get_type() { // todo: partially expected types here
                        Some(Type::Array(item, spread_len)) => {
                            if expected_item.is_none() {
                                expected_item = expr.get_type();
                            }
                            if let Some(expected_item) = expected_item.as_ref() {
                                if !expected_item.is_assignable_from(&*item) {
                                    return Err(AsgConvertError::unexpected_type(&expected_item.to_string(), Some(&*item.to_string()), &value.span));
                                }
                            }
                            len += spread_len;
                        },
                        type_ => return Err(AsgConvertError::unexpected_type(expected_item.as_ref().map(|x| x.to_string()).as_deref().unwrap_or("unknown"), type_.map(|x| x.to_string()).as_deref(), &value.span)),
                    }
                    Ok((expr, true))
                },
            }).collect::<Result<Vec<_>, AsgConvertError>>()?,
        };
        if let Some(expected_len) = expected_len {
            if len != expected_len {
                return Err(AsgConvertError::unexpected_type(&*format!("array of length {}", expected_len), Some(&*format!("array of length {}", len)), &value.span));
            }
        }
        Ok(output)
    }
}

impl Into<leo_ast::ArrayInlineExpression> for &ArrayInlineExpression {
    fn into(self) -> leo_ast::ArrayInlineExpression {
        leo_ast::ArrayInlineExpression {
            elements: self.elements.iter().map(|(element, spread)| {
                let element = element.as_ref().into();
                if *spread {
                    SpreadOrExpression::Spread(element)
                } else {
                    SpreadOrExpression::Expression(element)
                }
            }).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}