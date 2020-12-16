use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct TupleAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub tuple_ref: Arc<Expression>,
    pub index: usize,
}

impl Node for TupleAccessExpression {

    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for TupleAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.tuple_ref.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        match self.tuple_ref.get_type()? {
            Type::Tuple(subtypes) => subtypes.get(self.index).cloned(),
            _ => None,
        }
    }

    fn const_value(&self) -> Option<ConstValue> {
        let tuple_const = self.tuple_ref.const_value()?;
        match tuple_const {
            ConstValue::Tuple(sub_consts) => sub_consts.get(self.index).cloned(),
            _ => None,
        }
    }
}

impl FromAst<leo_ast::TupleAccessExpression> for TupleAccessExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::TupleAccessExpression, expected_type: Option<Type>) -> Result<TupleAccessExpression, AsgConvertError> {
        //todo: partial expected types
        let index = value.index.value.parse::<usize>().map_err(|_| AsgConvertError::parse_index_error())?;

        let tuple = Arc::<Expression>::from_ast(scope, &*value.tuple, None)?;
        let tuple_type = tuple.get_type();
        if let Some(Type::Tuple(items)) = tuple_type {
            if items.len() <= index {
                return Err(AsgConvertError::tuple_index_out_of_bounds(index, &value.span));
            }
            if let Some(expected_type) = expected_type {
                let item = items.get(index).unwrap();
                if &expected_type != item {
                    return Err(AsgConvertError::unexpected_type(&expected_type.to_string(), Some(&*item.to_string()), &value.span));
                }
            }
        } else {
            return Err(AsgConvertError::unexpected_type("a tuple", tuple_type.map(|x| x.to_string()).as_deref(), &value.span));
        }

        Ok(TupleAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            tuple_ref: tuple,
            index,
        })
    }
}

impl Into<leo_ast::TupleAccessExpression> for &TupleAccessExpression {
    fn into(self) -> leo_ast::TupleAccessExpression {
        leo_ast::TupleAccessExpression {
            tuple: Box::new(self.tuple_ref.as_ref().into()),
            index: leo_ast::PositiveNumber { value: self.index.to_string() },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}