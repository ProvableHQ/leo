use crate::Span;
use crate::{ Expression, Variable, Node, Type, PartialType, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct VariableRef {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub variable: Variable,
}

impl Node for VariableRef {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for VariableRef {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, _expr: &Arc<Expression>) {
    }

    fn get_type(&self) -> Option<Type> {
        Some(self.variable.borrow().type_.clone())
    }

    fn const_value(&self) -> Option<ConstValue> {
        self.variable.borrow().const_value.clone()
    }
}

impl FromAst<leo_ast::Identifier> for Arc<Expression> {
    fn from_ast(scope: &Scope, value: &leo_ast::Identifier, expected_type: Option<PartialType>) -> Result<Arc<Expression>, AsgConvertError> {
        let variable = scope.borrow().resolve_variable(&value.name).ok_or_else(|| AsgConvertError::unresolved_reference(&value.name, &value.span))?;
        let variable_ref = VariableRef {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            variable: variable.clone(),
        };
        let expression = Arc::new(Expression::VariableRef(variable_ref));

        if let Some(expected_type) = expected_type {
            let type_ = expression.get_type().ok_or_else(|| AsgConvertError::unresolved_reference(&value.name, &value.span))?;
            if !expected_type.matches(&type_) {
                return Err(AsgConvertError::unexpected_type(&expected_type.to_string(), Some(&*type_.to_string()), &value.span));
            }
        }

        let mut variable_ref = variable.borrow_mut();
        variable_ref.references.push(Arc::downgrade(&expression));

        Ok(expression)
    }
}

impl Into<leo_ast::Identifier> for &VariableRef {
    fn into(self) -> leo_ast::Identifier {
        self.variable.borrow().name.clone()
    }
}