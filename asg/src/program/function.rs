use crate::{ Identifier, Type, WeakType, Statement, Span, AsgConvertError, BlockStatement, FromAst, Scope, InnerScope, Circuit, Variable, ReturnPathReducer, MonoidalDirector };

use std::sync::{ Arc, Weak };
use std::cell::RefCell;
use leo_ast::FunctionInput;

#[derive(PartialEq)]
pub enum FunctionQualifier {
    SelfRef,
    MutSelfRef,
    Static,
}

pub struct Function {
    pub name: Identifier,
    pub output: WeakType,
    pub has_input: bool,
    pub argument_types: Vec<WeakType>,
    pub circuit: RefCell<Option<Weak<Circuit>>>,
    pub body: RefCell<Weak<FunctionBody>>,
    pub qualifier: FunctionQualifier,
}

pub struct FunctionBody {
    pub span: Option<Span>,
    pub function: Arc<Function>,
    pub arguments: Vec<Variable>,
    pub body: Arc<Statement>,
    pub scope: Scope,
}

impl Function {
    pub(crate) fn from_ast(scope: &Scope, value: &leo_ast::Function) -> Result<Function, AsgConvertError> {
        let output: Type = value.output.as_ref().map(|t| scope.borrow().resolve_ast_type(t)).transpose()?
            .unwrap_or_else(|| Type::Tuple(vec![]));
        let mut qualifier = FunctionQualifier::Static;
        let mut has_input = false;

        let mut argument_types = vec![];
        {
            for input in value.input.iter() {
                match input {
                    FunctionInput::InputKeyword(_) => {
                        has_input = true;
                    },
                    FunctionInput::SelfKeyword(_) => {
                        qualifier = FunctionQualifier::SelfRef;
                    },
                    FunctionInput::MutSelfKeyword(_) => {
                        qualifier = FunctionQualifier::MutSelfRef;
                    },
                    FunctionInput::Variable(leo_ast::FunctionInputVariable { type_, .. }) => {
                        argument_types.push(scope.borrow().resolve_ast_type(&type_)?.into());
                    },
                }
            }
        }
        if qualifier != FunctionQualifier::Static && scope.borrow().circuit_self.is_none() {
            return Err(AsgConvertError::invalid_self_in_global(&value.span));
        }
        Ok(Function {
            name: value.identifier.clone(),
            output: output.into(),
            has_input,
            argument_types,
            circuit: RefCell::new(None),
            body: RefCell::new(Weak::new()),
            qualifier,
        })
    }

}


impl FunctionBody {
    pub(super) fn from_ast(scope: &Scope, value: &leo_ast::Function, function: Arc<Function>) -> Result<FunctionBody, AsgConvertError> {
        let new_scope = InnerScope::make_subscope(scope);
        let mut arguments = vec![];
        {
            let mut scope_borrow = new_scope.borrow_mut();
            if function.qualifier != FunctionQualifier::Static {
                let circuit = function.circuit.borrow();
                let self_variable = Arc::new(RefCell::new(crate::InnerVariable {
                    name: Identifier::new("self".to_string()),
                    type_: Type::Circuit(circuit.as_ref().unwrap().upgrade().clone().unwrap()),
                    mutable: function.qualifier == FunctionQualifier::MutSelfRef,
                    declaration: crate::VariableDeclaration::Parameter,
                    const_value: None,
                    references: vec![],
                    assignments: vec![],
                }));
                scope_borrow.variables.insert("self".to_string(), self_variable.clone());
            }
            scope_borrow.function = Some(function.clone());
            for input in value.input.iter() {
                match input {
                    FunctionInput::InputKeyword(_) => {},
                    FunctionInput::SelfKeyword(_) => {},
                    FunctionInput::MutSelfKeyword(_) => {},
                    FunctionInput::Variable(leo_ast::FunctionInputVariable { identifier, mutable, type_, span: _span }) => {
                        let variable = Arc::new(RefCell::new(crate::InnerVariable {
                            name: identifier.clone(),
                            type_: scope_borrow.resolve_ast_type(&type_)?,
                            mutable: *mutable,
                            declaration: crate::VariableDeclaration::Parameter,
                            const_value: None,
                            references: vec![],
                            assignments: vec![],
                        }));
                        arguments.push(variable.clone());
                        scope_borrow.variables.insert(identifier.name.clone(), variable);
                    },
                }
            }
        }
        let main_block = BlockStatement::from_ast(&new_scope, &value.block, None)?;
        let mut director = MonoidalDirector::new(ReturnPathReducer::new());
        if !director.reduce_block(&main_block).0 && !function.output.is_unit() {
            return Err(AsgConvertError::function_missing_return(&function.name.name, &value.span));
        }
        for (span, error) in director.reducer().errors {
            return Err(AsgConvertError::function_return_validation(&function.name.name, &error, &span));
        }

        Ok(FunctionBody {
            span: Some(value.span.clone()),
            function,
            arguments,
            body: Arc::new(Statement::Block(main_block)),
            scope: new_scope,
        })
    }
}

impl Into<leo_ast::Function> for &Function {
    fn into(self) -> leo_ast::Function {
        let (input, body, span) = match self.body.borrow().upgrade() {
            Some(body) => {
                (
                    body.arguments.iter().map(|variable| {
                        let variable = variable.borrow();
                        leo_ast::FunctionInput::Variable(leo_ast::FunctionInputVariable {
                            identifier: variable.name.clone(),
                            mutable: variable.mutable,
                            type_: (&variable.type_).into(),
                            span: Span::default(),
                        })
                    }).collect(),
                    match body.body.as_ref() {
                        Statement::Block(block) => block.into(),
                        _ => unimplemented!(),
                    },
                    body.span.clone().unwrap_or_default(),
                )
            },
            None => {
                (
                    vec![],
                    leo_ast::Block {
                        statements: vec![],
                        span: Default::default(),
                    },
                    Default::default(),
                )
            },
        };
        let output: Type = self.output.clone().into();
        leo_ast::Function {
            identifier: self.name.clone(),
            input,
            block: body,
            output: Some((&output).into()),
            span,
        }
    }
}