use crate::{ Type, Identifier, Function, FunctionBody, Span, Node, AsgConvertError, InnerScope, Scope, WeakType };
use indexmap::IndexMap;

use std::sync::{ Arc, Weak };
use std::cell::{ RefCell };
use uuid::Uuid;

pub enum CircuitMemberBody {
    Variable(Type),
    Function(Arc<FunctionBody>),
}

pub enum CircuitMember {
    Variable(WeakType),
    Function(Arc<Function>),
}

pub struct Circuit {
    pub id: Uuid,
    pub name: Identifier,
    pub body: RefCell<Weak<CircuitBody>>,
    pub members: RefCell<IndexMap<String, CircuitMember>>,
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Circuit) -> bool {
        if self.name != other.name {
            return false;
        }
        self.id == other.id
    }
}

pub struct CircuitBody {
    pub scope: Scope,
    pub span: Option<Span>,
    pub circuit: Arc<Circuit>,
    pub members: RefCell<IndexMap<String, CircuitMemberBody>>,
}

impl Node for CircuitMemberBody {

    fn span(&self) -> Option<&Span> {
        None
    }
}


impl Circuit {
    pub(super) fn init(value: &leo_ast::Circuit) -> Result<Arc<Circuit>, AsgConvertError> {

        let circuit = Arc::new(Circuit {
            id: Uuid::new_v4(),
            name: value.circuit_name.clone(),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(IndexMap::new()),
        });
        
        Ok(circuit)
    }

    pub(super) fn from_ast(self: Arc<Circuit>, scope: &Scope, value: &leo_ast::Circuit) -> Result<(), AsgConvertError> {
        let new_scope = InnerScope::make_subscope(scope); // temporary scope for function headers
        new_scope.borrow_mut().circuit_self = Some(self.clone());

        let mut members = self.members.borrow_mut();
        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitVariable(name, type_) => {
                    members.insert(name.name.clone(), CircuitMember::Variable(new_scope.borrow().resolve_ast_type(type_)?.into()));
                },
                leo_ast::CircuitMember::CircuitFunction(function) => {
                    let asg_function = Arc::new(Function::from_ast(&new_scope, function)?);

                    members.insert(function.identifier.name.clone(), CircuitMember::Function(asg_function));
                },
            }
        }

        for (_, member) in members.iter() {
            if let CircuitMember::Function(func) = member {
                func.circuit.borrow_mut().replace(Arc::downgrade(&self));
            }
        }
        
        Ok(())
    }
}


impl CircuitBody {

    pub(super) fn from_ast(scope: &Scope, value: &leo_ast::Circuit, circuit: Arc<Circuit>) -> Result<CircuitBody, AsgConvertError> {
        let mut members = IndexMap::new();
        let new_scope = InnerScope::make_subscope(scope);
        new_scope.borrow_mut().circuit_self = Some(circuit.clone());

        //todo: what is our plan for conflict names with circuit member function/variables with the same name?
        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitVariable(name, type_) => {
                    members.insert(name.name.clone(), CircuitMemberBody::Variable(new_scope.borrow().resolve_ast_type(type_)?));
                },
                leo_ast::CircuitMember::CircuitFunction(function) => {
                    let asg_function = {
                        let circuit_members = circuit.members.borrow();
                        match circuit_members.get(&function.identifier.name).unwrap() {
                            CircuitMember::Function(f) => f.clone(),
                            _ => unimplemented!(),
                        }
                    };
                    let function_body = Arc::new(FunctionBody::from_ast(&new_scope, function, asg_function.clone())?);
                    asg_function.body.replace(Arc::downgrade(&function_body));

                    members.insert(function.identifier.name.clone(), CircuitMemberBody::Function(function_body));
                },
            }
        }

        Ok(CircuitBody {
            span: Some(value.circuit_name.span.clone()), // todo: better span
            circuit,
            members: RefCell::new(members),
            scope: scope.clone(),
        })
    }
}

impl Into<leo_ast::Circuit> for &Circuit {
    fn into(self) -> leo_ast::Circuit {
        let members = match self.body.borrow().upgrade() {
            Some(body) => {
                body.members.borrow().iter().map(|(name, member)| {
                    match &member {
                        CircuitMemberBody::Variable(type_) => leo_ast::CircuitMember::CircuitVariable(Identifier::new(name.clone()), type_.into()),
                        CircuitMemberBody::Function(func) => leo_ast::CircuitMember::CircuitFunction(func.function.as_ref().into()),
                    }
                }).collect()
            },
            None => vec![],
        };
        leo_ast::Circuit {
            circuit_name: self.name.clone(),
            members,
        }
    }
}