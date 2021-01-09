use crate::Span;
use crate::{ Statement, Expression, Variable, Identifier, FromAst, Scope, AsgConvertError, Type, Node, PartialType, CircuitMember, IntegerType };
use std::sync::{ Weak, Arc };
pub use leo_ast::AssignOperation;
use leo_ast::AssigneeAccess as AstAssigneeAccess;

pub enum AssignAccess {
    ArrayRange(Option<Arc<Expression>>, Option<Arc<Expression>>),
    ArrayIndex(Arc<Expression>),
    Tuple(usize),
    Member(Identifier),
}

pub struct AssignStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub operation: AssignOperation,
    pub target_variable: Variable,
    pub target_accesses: Vec<AssignAccess>,
    pub value: Arc<Expression>,
}

impl FromAst<leo_ast::AssignStatement> for AssignStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::AssignStatement, _expected_type: Option<PartialType>) -> Result<Self, AsgConvertError> {
        let variable = scope.borrow().resolve_variable(&statement.assignee.identifier.name).ok_or_else(|| AsgConvertError::unresolved_reference(&statement.assignee.identifier.name, &statement.assignee.identifier.span))?;
        
        if !variable.borrow().mutable {
            return Err(AsgConvertError::immutable_assignment(&statement.assignee.identifier.name, &statement.span));
        }
        let mut target_type = variable.borrow().type_.clone();

        let mut target_accesses = vec![];
        for access in statement.assignee.accesses.iter() {
            target_accesses.push(match access {
                AstAssigneeAccess::ArrayRange(left, right) => {
                    match &target_type {
                        Type::Array(_item, _) => {
                            //target_type doesnt change here
                        },
                        _ => return Err(AsgConvertError::index_into_non_array(&statement.assignee.identifier.name, &statement.span)),
                    }
                    let index_type = Some(Type::Integer(IntegerType::U32).into());
                    let left = left.as_ref().map(|left: &leo_ast::Expression| -> Result<Arc<Expression>, AsgConvertError> { Ok(Arc::<Expression>::from_ast(scope, left, index_type.clone())?) }).transpose()?;
                    let right = right.as_ref().map(|right: &leo_ast::Expression| -> Result<Arc<Expression>, AsgConvertError> { Ok(Arc::<Expression>::from_ast(scope, right, index_type)?) }).transpose()?;
                    
                    AssignAccess::ArrayRange(
                        left,
                        right,
                    )
                },
                AstAssigneeAccess::ArrayIndex(index) => {
                    target_type = match target_type.clone() {
                        Type::Array(item, _) => {
                            *item
                        },
                        _ => return Err(AsgConvertError::index_into_non_array(&statement.assignee.identifier.name, &statement.span)),
                    };
                    AssignAccess::ArrayIndex(
                        Arc::<Expression>::from_ast(scope, index, Some(Type::Integer(IntegerType::U32).into()))?
                    )
                },
                AstAssigneeAccess::Tuple(index, _) => {
                    let index = index.value.parse::<usize>().map_err(|_| AsgConvertError::parse_index_error())?;
                    target_type = match target_type {
                        Type::Tuple(types) => {
                            types.get(index).cloned().ok_or_else(|| AsgConvertError::tuple_index_out_of_bounds(index, &statement.span))?
                        },
                        _ => return Err(AsgConvertError::index_into_non_tuple(&statement.assignee.identifier.name, &statement.span)),
                    };
                    AssignAccess::Tuple(index)
                },
                AstAssigneeAccess::Member(name) => {
                    target_type = match target_type {
                        Type::Circuit(circuit) => {
                            let circuit = circuit;

                            let members = circuit.members.borrow();
                            let member = members.get(&name.name).ok_or_else(|| AsgConvertError::unresolved_circuit_member(&circuit.name.name, &name.name, &statement.span))?;

                            let x = match &member {
                                CircuitMember::Variable(type_) => type_.clone(),
                                CircuitMember::Function(_) => return Err(AsgConvertError::illegal_function_assign(&name.name, &statement.span)),
                            };
                            x.into()
                        },
                        _ => return Err(AsgConvertError::index_into_non_tuple(&statement.assignee.identifier.name, &statement.span)),
                    };
                    AssignAccess::Member(name.clone())
                },
                
            });
        }
        let value = Arc::<Expression>::from_ast(scope, &statement.value, Some(target_type.into()))?;

        Ok(AssignStatement {
            parent: None,
            span: Some(statement.span.clone()),
            operation: statement.operation.clone(),
            target_variable: variable,
            target_accesses,
            value,
        })
    }
}

impl Into<leo_ast::AssignStatement> for &AssignStatement {
    fn into(self) -> leo_ast::AssignStatement {
        leo_ast::AssignStatement {
            operation: self.operation.clone(),
            assignee: leo_ast::Assignee {
                identifier: self.target_variable.borrow().name.clone(),
                accesses: self.target_accesses.iter().map(|access| match access {
                    AssignAccess::ArrayRange(left, right) =>
                        AstAssigneeAccess::ArrayRange(
                            left.as_ref().map(|e| e.as_ref().into()),
                            right.as_ref().map(|e| e.as_ref().into()),
                        ),
                    AssignAccess::ArrayIndex(index) =>
                        AstAssigneeAccess::ArrayIndex(index.as_ref().into()),
                    AssignAccess::Tuple(index) => AstAssigneeAccess::Tuple(leo_ast::PositiveNumber { value: index.to_string() }, self.span.clone().unwrap_or_default()),
                    AssignAccess::Member(name) => AstAssigneeAccess::Member(name.clone()),
                }).collect(),
                span: self.span.clone().unwrap_or_default(),
            },
            value: self.value.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}