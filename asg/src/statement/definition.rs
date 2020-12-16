use crate::Span;
use crate::{ Statement, Variable, InnerVariable, Expression, FromAst, AsgConvertError, Scope, ExpressionNode, Type, ConstValue };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;

pub struct DefinitionStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub variables: Vec<Variable>,
    pub value: Arc<Expression>,
}

impl FromAst<leo_ast::DefinitionStatement> for DefinitionStatement {
    fn from_ast(scope: &Scope, statement: &leo_ast::DefinitionStatement, _expected_type: Option<Type>) -> Result<Self, AsgConvertError> {
        let type_ = statement.type_.as_ref().map(|x| scope.borrow().resolve_ast_type(&x)).transpose()?;
        
        //todo: tuple partially expected types
        let value = Arc::<Expression>::from_ast(scope, &statement.value, type_.clone())?;

        let type_ = type_.or_else(|| value.get_type());

        let mut output_types = vec![];

        let mut variables = vec![];
        if statement.variable_names.len() == 0 {
            return Err(AsgConvertError::illegal_ast_structure("cannot have 0 variable names in destructuring tuple"));
        } if statement.variable_names.len() == 1 {
            // any return type is fine
            output_types.push(type_);
        } else { // tuple destructure
            match type_.as_ref() {
                Some(Type::Tuple(sub_types)) if sub_types.len() == statement.variable_names.len() => {
                    output_types.extend(sub_types.clone().into_iter().map(Some).collect::<Vec<_>>());
                },
                type_ => return Err(AsgConvertError::unexpected_type(&*format!("{}-ary tuple", statement.variable_names.len()), type_.map(|x| x.to_string()).as_deref(), &statement.span)),
            }
        }

        let const_values = if statement.variable_names.len() == 1 {
            match value.const_value() {
                Some(item) => Some(vec![item]),
                _ => None,
            }
        } else {
            match value.const_value() {
                Some(ConstValue::Tuple(items)) => Some(items),
                _ => None,
            }
        };

        for (i, (variable, type_)) in statement.variable_names.iter().zip(output_types.into_iter()).enumerate() {
            if statement.declaration_type == leo_ast::Declare::Const && variable.mutable {
                return Err(AsgConvertError::illegal_ast_structure("cannot have const mut"));
            }
            variables.push(Arc::new(RefCell::new(InnerVariable {
                name: variable.identifier.clone(),
                type_: type_
                    .ok_or_else(|| AsgConvertError::unresolved_type(&variable.identifier.name, &statement.span))?,
                mutable: variable.mutable,
                declaration: crate::VariableDeclaration::Definition,
                const_value: if !variable.mutable {
                    const_values.as_ref().map(|x| x.get(i).cloned()).flatten()
                } else { None },
                references: vec![],
            })));
        }

        {
            let mut scope_borrow = scope.borrow_mut();
            for variable in variables.iter() {
                scope_borrow.variables.insert(variable.borrow().name.name.clone(), variable.clone());
            }
        }

        Ok(DefinitionStatement {
            parent: None,
            span: Some(statement.span.clone()),
            variables,
            value,
        })
    }
}

impl Into<leo_ast::DefinitionStatement> for &DefinitionStatement {
    fn into(self) -> leo_ast::DefinitionStatement {
        assert!(self.variables.len() > 0);

        let mut variable_names = vec![];
        let mut type_ = None::<leo_ast::Type>;
        for variable in self.variables.iter() {
            let variable = variable.borrow();
            variable_names.push(leo_ast::VariableName {
                mutable: variable.mutable,
                identifier: variable.name.clone(),
                span: variable.name.span.clone(),
            });
            if type_.is_none() {
                type_ = Some((&variable.type_).into());
            }
        }

        leo_ast::DefinitionStatement {
            declaration_type: leo_ast::Declare::Let,
            variable_names,
            type_,
            value: self.value.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}