use crate::Span;
use crate::{ Expression, Circuit, Identifier, Type, Node, ExpressionNode, FromAst, Scope, AsgConvertError, CircuitMember, ConstValue, PartialType };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;
use indexmap::IndexMap;

pub struct CircuitInitExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub circuit: Arc<Circuit>,
    pub values: Vec<(Identifier, Arc<Expression>)>,
}

impl Node for CircuitInitExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for CircuitInitExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.values.iter().for_each(|(_, element)| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        Some(Type::Circuit(self.circuit.clone()))
    }

    fn const_value(&self) -> Option<ConstValue> {
        None
    }
}

impl FromAst<leo_ast::CircuitInitExpression> for CircuitInitExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::CircuitInitExpression, expected_type: Option<PartialType>) -> Result<CircuitInitExpression, AsgConvertError> {
        let circuit = scope.borrow().resolve_circuit(&value.name.name).ok_or_else(|| AsgConvertError::unresolved_circuit(&value.name.name, &value.name.span))?;
        match expected_type {
            Some(PartialType::Type(Type::Circuit(expected_circuit))) if expected_circuit == circuit => (),
            None => (),
            Some(x) => return Err(AsgConvertError::unexpected_type(&x.to_string(), Some(&circuit.name.name), &value.span)),
        }
        let members: IndexMap<&String, (&Identifier, &leo_ast::Expression)> = value.members.iter().map(|x| (&x.identifier.name, (&x.identifier, &x.expression))).collect();

        let mut values: Vec<(Identifier, Arc<Expression>)> = vec![];

        {
            let circuit_members = circuit.members.borrow();
            for (name, member) in circuit_members.iter() {
                let type_: Type = if let CircuitMember::Variable(type_) = &member {
                    type_.clone().into()
                } else {
                    continue;
                };
                if let Some((identifier, receiver)) = members.get(&name) {
                    let received = Arc::<Expression>::from_ast(scope, *receiver, Some(type_.partial()))?;
                    values.push(((*identifier).clone(), received));
                } else {
                    return Err(AsgConvertError::missing_circuit_member(&circuit.name.name, name, &value.span));
                }
            }

            for (name, (identifier, _expression)) in members.iter() {
                if circuit_members.get(*name).is_none() {
                    return Err(AsgConvertError::extra_circuit_member(&circuit.name.name, *name, &identifier.span));
                }
            }
        }

        Ok(CircuitInitExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            circuit,
            values,
        })
    }
}

impl Into<leo_ast::CircuitInitExpression> for &CircuitInitExpression {
    fn into(self) -> leo_ast::CircuitInitExpression {
        leo_ast::CircuitInitExpression {
            name: self.circuit.name.clone(),
            members: self.values.iter().map(|(name, value)| {
                leo_ast::CircuitVariableDefinition {
                    identifier: name.clone(),
                    expression: value.as_ref().into(),
                }
            }).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}