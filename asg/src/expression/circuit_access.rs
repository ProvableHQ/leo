use crate::Span;
use crate::{ Expression, Identifier, Type, Node, ExpressionNode, FromAst, Scope, AsgConvertError, Circuit, CircuitMember, ConstValue, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct CircuitAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub circuit: Arc<Circuit>,
    pub target: Option<Arc<Expression>>,
    pub member: Identifier,
}

impl Node for CircuitAccessExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for CircuitAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        if let Some(target) = self.target.as_ref() {
            target.set_parent(Arc::downgrade(expr));
        }
    }

    fn get_type(&self) -> Option<Type> {
        if self.target.is_none() {
            None // function target only for static
        } else {
            let members = self.circuit.members.borrow();
            let member = members.get(&self.member.name)?;
            match member {
                CircuitMember::Variable(type_) => Some(type_.clone().into()),
                CircuitMember::Function(_) => None,
            }
        }
    }

    fn const_value(&self) -> Option<ConstValue> {
        None
    }
}

impl FromAst<leo_ast::CircuitMemberAccessExpression> for CircuitAccessExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::CircuitMemberAccessExpression, expected_type: Option<PartialType>) -> Result<CircuitAccessExpression, AsgConvertError> {
        let target = Arc::<Expression>::from_ast(scope, &*value.circuit, None)?;
        let circuit = match target.get_type() {
            Some(Type::Circuit(circuit)) => circuit,
            x => return Err(AsgConvertError::unexpected_type("circuit", x.map(|x| x.to_string()).as_deref(), &value.span)),
        };

        if let Some(member) = circuit.members.borrow().get(&value.name.name) {
            if let Some(expected_type) = expected_type {
                if let CircuitMember::Variable(type_) = &member {
                    let type_: Type = type_.clone().into();
                    if !expected_type.matches(&type_) {
                        return Err(AsgConvertError::unexpected_type(&expected_type.to_string(), Some(&type_.to_string()), &value.span));
                    }
                } // used by call expression
            }
        } else {
            return Err(AsgConvertError::unresolved_circuit_member(&circuit.name.name, &value.name.name, &value.span));
        }

        Ok(CircuitAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            target: Some(target),
            circuit,
            member: value.name.clone(),
        })
    }
}

impl FromAst<leo_ast::CircuitStaticFunctionAccessExpression> for CircuitAccessExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::CircuitStaticFunctionAccessExpression, expected_type: Option<PartialType>) -> Result<CircuitAccessExpression, AsgConvertError> {
        let circuit = match &*value.circuit {
            leo_ast::Expression::Identifier(name) => {
                scope.borrow().resolve_circuit(&name.name).ok_or_else(|| AsgConvertError::unresolved_circuit(&name.name, &name.span))?
            },
            _ => return Err(AsgConvertError::unexpected_type("circuit", Some("unknown"), &value.span)),
        };

        if let Some(expected_type) = expected_type {
            return Err(AsgConvertError::unexpected_type(&expected_type.to_string(), Some("none"), &value.span));
        }
        
        if let Some(CircuitMember::Function(_)) = circuit.members.borrow().get(&value.name.name) {
            // okay
        } else {
            return Err(AsgConvertError::unresolved_circuit_member(&circuit.name.name, &value.name.name, &value.span));
        }

        Ok(CircuitAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            target: None,
            circuit,
            member: value.name.clone(),
        })
    }
}

impl Into<leo_ast::Expression> for &CircuitAccessExpression {
    fn into(self) -> leo_ast::Expression {
        if let Some(target) = self.target.as_ref() {
            leo_ast::Expression::CircuitMemberAccess(leo_ast::CircuitMemberAccessExpression {
                circuit: Box::new(target.as_ref().into()),
                name: self.member.clone(),
                span: self.span.clone().unwrap_or_default(),
            })
        } else {
            leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                circuit: Box::new(leo_ast::Expression::Identifier(self.circuit.name.clone())),
                name: self.member.clone(),
                span: self.span.clone().unwrap_or_default(),
            })
        }
    }
}