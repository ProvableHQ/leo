pub use leo_ast::{ BinaryOperation, BinaryOperationClass };
use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct BinaryExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub operation: BinaryOperation,
    pub left: Arc<Expression>,
    pub right: Arc<Expression>,
}

impl Node for BinaryExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for BinaryExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.left.set_parent(Arc::downgrade(expr));
        self.right.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        match self.operation.class() {
            BinaryOperationClass::Boolean => Some(Type::Boolean),
            BinaryOperationClass::Numeric => self.left.get_type(),
        }
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        use BinaryOperation::*;
        let left = self.left.const_value()?;
        let right = self.right.const_value()?;

        match (left, right) {
            (ConstValue::Int(left), ConstValue::Int(right)) => {
                Some(match self.operation {
                    Add => ConstValue::Int(left.value_add(&right)?),
                    Sub => ConstValue::Int(left.value_sub(&right)?),
                    Mul => ConstValue::Int(left.value_mul(&right)?),
                    Div => ConstValue::Int(left.value_div(&right)?),
                    Pow => ConstValue::Int(left.value_pow(&right)?),
                    Eq => ConstValue::Boolean(left == right),
                    Ne => ConstValue::Boolean(left != right),
                    Ge => ConstValue::Boolean(left.value_ge(&right)?),
                    Gt => ConstValue::Boolean(left.value_gt(&right)?),
                    Le => ConstValue::Boolean(left.value_le(&right)?),
                    Lt => ConstValue::Boolean(left.value_lt(&right)?),
                    _ => return None,
                })
            },
            // (ConstValue::Field(left), ConstValue::Field(right)) => {
            //     Some(match self.operation {
            //         Add => ConstValue::Field(left.checked_add(&right)?),
            //         Sub => ConstValue::Field(left.checked_sub(&right)?),
            //         Mul => ConstValue::Field(left.checked_mul(&right)?),
            //         Div => ConstValue::Field(left.checked_div(&right)?),
            //         Eq => ConstValue::Boolean(left == right),
            //         Ne => ConstValue::Boolean(left != right),
            //         _ => return None,
            //     })
            // },
            (ConstValue::Boolean(left), ConstValue::Boolean(right)) => {
                Some(match self.operation {
                    Eq => ConstValue::Boolean(left == right),
                    Ne => ConstValue::Boolean(left != right),
                    And => ConstValue::Boolean(left && right),
                    Or => ConstValue::Boolean(left || right),
                    _ => return None,
                })
            },
            //todo: group?
            (left, right) => {
                Some(match self.operation {
                    Eq => ConstValue::Boolean(left == right),
                    Ne => ConstValue::Boolean(left != right),
                    _ => return None,
                })
            },
        }
    }
}

impl FromAst<leo_ast::BinaryExpression> for BinaryExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::BinaryExpression, expected_type: Option<PartialType>) -> Result<BinaryExpression, AsgConvertError> {
        let class = value.op.class();
        let expected_type = match class {
            BinaryOperationClass::Boolean => {
                match expected_type {
                    Some(PartialType::Type(Type::Boolean)) | None => None,
                    Some(x) => return Err(AsgConvertError::unexpected_type(&x.to_string(), Some(&*Type::Boolean.to_string()), &value.span)),
                }
            },
            BinaryOperationClass::Numeric => {
                match expected_type {
                    Some(PartialType::Type(x @ Type::Integer(_))) => Some(x),
                    Some(x) => return Err(AsgConvertError::unexpected_type(&x.to_string(), Some("integer"), &value.span)),
                    None => None,
                }
            },
        }.map(Type::partial);

        // left
        let (left, right) = match Arc::<Expression>::from_ast(scope, &*value.left, expected_type.clone()) {
            Ok(left) => {
                if let Some(left_type) = left.get_type() {
                    let right = Arc::<Expression>::from_ast(scope, &*value.right, Some(left_type.partial()))?;
                    (left, right)
                } else {
                    let right = Arc::<Expression>::from_ast(scope, &*value.right, expected_type.clone())?;
                    if let Some(right_type) = right.get_type() {
                        (Arc::<Expression>::from_ast(scope, &*value.left, Some(right_type.partial()))?, right)
                    } else {
                        (left, right)
                    }
                }
            },
            Err(e) => {
                let right = Arc::<Expression>::from_ast(scope, &*value.right, expected_type.clone())?;
                if let Some(right_type) = right.get_type() {
                    (Arc::<Expression>::from_ast(scope, &*value.left, Some(right_type.partial()))?, right)
                } else {
                    return Err(e);
                }
            }
        };

        let left_type = left.get_type();
        match class {
            BinaryOperationClass::Numeric => match left_type {
                Some(Type::Integer(_)) => (),
                Some(Type::Group) | Some(Type::Field) if value.op == BinaryOperation::Add || value.op == BinaryOperation::Sub => (),
                Some(Type::Field) if value.op == BinaryOperation::Mul || value.op == BinaryOperation::Div => (),
                type_ => return Err(AsgConvertError::unexpected_type("integer", type_.map(|x| x.to_string()).as_deref(), &value.span)),
            },
            BinaryOperationClass::Boolean =>
                match &value.op {
                    BinaryOperation::And | BinaryOperation::Or =>
                        match left_type {
                            Some(Type::Boolean) | None => (),
                            Some(x) => return Err(AsgConvertError::unexpected_type(&x.to_string(), Some(&*Type::Boolean.to_string()), &value.span)),
                        },
                    BinaryOperation::Eq | BinaryOperation::Ne => (), // all types allowed
                    _ => match left_type {
                        Some(Type::Integer(_)) | None => (),
                        Some(x) => return Err(AsgConvertError::unexpected_type(&x.to_string(), Some("integer"), &value.span)),
                    }
                }
        }

        let right_type = right.get_type();

        match (left_type, right_type) {
            (Some(left_type), Some(right_type)) => {
                if !left_type.is_assignable_from(&right_type) {
                    return Err(AsgConvertError::unexpected_type(&left_type.to_string(), Some(&*right_type.to_string()), &value.span));
                }
            },
            (None, None) => return Err(AsgConvertError::unexpected_type("any type", Some("unknown type"), &value.span)),
            (_, _) => (), //todo: improve this with type drilling
        }
        Ok(BinaryExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            operation: value.op.clone(),
            left,
            right,
        })
    }
}

impl Into<leo_ast::BinaryExpression> for &BinaryExpression {
    fn into(self) -> leo_ast::BinaryExpression {
        leo_ast::BinaryExpression {
            op: self.operation.clone(),
            left: Box::new(self.left.as_ref().into()),
            right: Box::new(self.right.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}