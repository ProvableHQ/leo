pub use leo_ast::BinaryOperation;
use crate::Span;
use crate::{ Expression, Function, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, CircuitMember, ConstValue };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct CallExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub function: Arc<Function>,
    pub target: Option<Arc<Expression>>,
    pub arguments: Vec<Arc<Expression>>,
}

impl Node for CallExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for CallExpression {
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
        self.arguments.iter().for_each(|element| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        Some(self.function.output.clone().into())
    }

    fn const_value(&self) -> Option<ConstValue> {
        // static function const evaluation
        None
    }
}

impl FromAst<leo_ast::CallExpression> for CallExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::CallExpression, expected_type: Option<Type>) -> Result<CallExpression, AsgConvertError> {
        
        let (target, function) = match &*value.function {
            leo_ast::Expression::Identifier(name) => (None, scope.borrow().resolve_function(&name.name).ok_or_else(|| AsgConvertError::unresolved_function(&name.name, &name.span))?),
            leo_ast::Expression::CircuitMemberAccess(leo_ast::CircuitMemberAccessExpression { circuit: ast_circuit, name, span }) => {
                let target = Arc::<Expression>::from_ast(scope, &**ast_circuit, None)?;
                let circuit = match target.get_type() {
                    Some(Type::Circuit(circuit)) => circuit,
                    type_ => return Err(AsgConvertError::unexpected_type("circuit", type_.map(|x| x.to_string()).as_deref(), &span)),
                };
                let member = circuit.members.borrow();
                let member = member.get(&name.name).ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit.name.name, &name.name, &span))?;
                match member {
                    CircuitMember::Function(body) => (Some(target), body.clone()),
                    CircuitMember::Variable(_) => return Err(AsgConvertError::circuit_variable_call(&circuit.name.name, &name.name, &span))?,
                }
            },
            leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression { circuit: ast_circuit, name, span }) => {
                let circuit = if let leo_ast::Expression::Identifier(circuit_name) = &**ast_circuit {
                    scope.borrow().resolve_circuit(&circuit_name.name).ok_or_else(|| AsgConvertError::unresolved_circuit(&circuit_name.name, &circuit_name.span))?
                } else {
                    return Err(AsgConvertError::unexpected_type("circuit", None, &span));
                };
                let member = circuit.members.borrow();
                let member = member.get(&name.name).ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit.name.name, &name.name, &span))?;
                match member {
                    CircuitMember::Function(body) => (None, body.clone()),
                    CircuitMember::Variable(_) => return Err(AsgConvertError::circuit_variable_call(&circuit.name.name, &name.name, &span))?,
                }
            },
            _ => return Err(AsgConvertError::illegal_ast_structure("non Identifier/CircuitMemberAccess/CircuitStaticFunctionAccess as call target")),
        };
        match expected_type {
            Some(expected) => {
                let output: Type = function.output.clone().into();
                if !expected.is_assignable_from(&output) {
                    return Err(AsgConvertError::unexpected_type(&expected.to_string(), Some(&*output.to_string()), &value.span));
                }
            },
            _ => (),
        }
        if value.arguments.len() != function.argument_types.len() {
            return Err(AsgConvertError::unexpected_call_argument_count(function.argument_types.len(), value.arguments.len(), &value.span));
        }

        Ok(CallExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            arguments: value.arguments.iter().zip(function.argument_types.iter())
                .map(|(expr, argument)| Arc::<Expression>::from_ast(scope, expr, Some(argument.clone().into())))
                .collect::<Result<Vec<_>, AsgConvertError>>()?,
            function,
            target,
        })
    }
}

impl Into<leo_ast::CallExpression> for &CallExpression {
    fn into(self) -> leo_ast::CallExpression {
        let target_function = if let Some(target) = &self.target {
            target.as_ref().into()
        } else {
            let circuit = self.function.circuit.borrow().as_ref().map(|x| x.upgrade()).flatten();
            if let Some(circuit) = circuit {
                leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                    circuit: Box::new(leo_ast::Expression::Identifier(circuit.name.clone())),
                    name: self.function.name.clone(),
                    span: self.span.clone().unwrap_or_default(),
                })
            } else {
                leo_ast::Expression::Identifier(self.function.name.clone())
            }
        };
        leo_ast::CallExpression {
            function: Box::new(target_function),
            arguments: self.arguments.iter().map(|arg| arg.as_ref().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}