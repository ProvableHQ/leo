use std::sync::{ Arc };
use crate::{ Variable, Circuit, Function, Type, AsgConvertError, Input };
use std::cell::RefCell;
use indexmap::IndexMap;
use uuid::Uuid;

pub struct InnerScope {
    pub id: Uuid,
    pub parent_scope: Option<Scope>,
    pub function: Option<Arc<Function>>,
    pub circuit_self: Option<Arc<Circuit>>,
    pub variables: IndexMap<String, Variable>,
    pub functions: IndexMap<String, Arc<Function>>,
    pub circuits: IndexMap<String, Arc<Circuit>>,
    pub input: Option<Input>,
}

pub type Scope = Arc<RefCell<InnerScope>>;

impl InnerScope {
    pub fn resolve_variable(&self, name: &str) -> Option<Variable> {
        if let Some(resolved) = self.variables.get(name) {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_variable(name) {
                Some(resolved)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn resolve_current_function(&self) -> Option<Arc<Function>> {
        if let Some(resolved) = self.function.as_ref() {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_current_function() {
                Some(resolved)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn resolve_input(&self) -> Option<Input> {
        if let Some(input) = self.input.as_ref() {
            Some(input.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_input() {
                Some(resolved.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn resolve_function(&self, name: &str) -> Option<Arc<Function>> {
        if let Some(resolved) = self.functions.get(name) {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_function(name) {
                Some(resolved)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn resolve_circuit(&self, name: &str) -> Option<Arc<Circuit>> {
        if let Some(resolved) = self.circuits.get(name) {
            Some(resolved.clone())
        } else if name == "Self" && self.circuit_self.is_some() {
            self.circuit_self.clone()
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_circuit(name) {
                Some(resolved)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn resolve_circuit_self(&self) -> Option<Arc<Circuit>> {
        if let Some(resolved) = self.circuit_self.as_ref() {
            Some(resolved.clone())
        } else if let Some(resolved) = self.parent_scope.as_ref() {
            if let Some(resolved) = resolved.borrow().resolve_circuit_self() {
                Some(resolved)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn make_subscope(scope: &Scope) -> Scope {
        Arc::new(RefCell::new(InnerScope {
            id: Uuid::new_v4(),
            parent_scope: Some(scope.clone()),
            circuit_self: None,
            variables: IndexMap::new(),
            functions: IndexMap::new(),
            circuits: IndexMap::new(),
            function: None,
            input: None,
        }))
    }

    pub(crate) fn resolve_ast_type(&self, type_: &leo_ast::Type) -> Result<Type, AsgConvertError> {
        use leo_ast::Type::*;
        Ok(match type_ {
            Address => Type::Address,
            Boolean => Type::Boolean,
            Field => Type::Field,
            Group => Type::Group,
            IntegerType(int_type) => Type::Integer(int_type.clone()),
            Array(sub_type, dimensions) => {
                let mut item = Box::new(self.resolve_ast_type(&*sub_type)?);
                for dimension in dimensions.0.iter().rev() {
                    let dimension = dimension.value.parse::<usize>().map_err(|_| AsgConvertError::parse_index_error())?;
                    item = Box::new(Type::Array(item, dimension));
                }
                *item
                //Type::Array(item, dimensions.0.get(0).map(|x| x.value.parse::<usize>().ok()).flatten().ok_or_else(|| AsgConvertError::parse_index_error())?)
            },
            Tuple(sub_types) => Type::Tuple(sub_types.iter().map(|x| self.resolve_ast_type(x)).collect::<Result<Vec<_>, AsgConvertError>>()?),
            Circuit(name) if name.name == "Self" => Type::Circuit(self.circuit_self.clone().ok_or_else(|| AsgConvertError::unresolved_circuit(&name.name, &name.span))?.clone()),
            Circuit(name) => Type::Circuit(self.circuits.get(&name.name).ok_or_else(|| AsgConvertError::unresolved_circuit(&name.name, &name.span))?.clone()),
            SelfType => Type::Circuit(self.circuit_self.clone().ok_or_else(|| AsgConvertError::reference_self_outside_circuit())?),
        })
    }
}