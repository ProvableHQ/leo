// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    AsgConvertError,
    BlockStatement,
    Circuit,
    FromAst,
    Identifier,
    InnerScope,
    MonoidalDirector,
    ReturnPathReducer,
    Scope,
    Span,
    Statement,
    Type,
    Variable,
    WeakType,
};
use leo_ast::FunctionInput;

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum FunctionQualifier {
    SelfRef,
    MutSelfRef,
    Static,
}

#[derive(Debug)]
pub struct Function {
    pub id: Uuid,
    pub name: RefCell<Identifier>,
    pub output: WeakType,
    pub has_input: bool,
    pub arguments: Vec<Variable>,
    pub circuit: RefCell<Option<Weak<Circuit>>>,
    pub body: RefCell<Weak<FunctionBody>>,
    pub qualifier: FunctionQualifier,
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        if self.name.borrow().name != other.name.borrow().name {
            return false;
        }
        self.id == other.id
    }
}
impl Eq for Function {}

#[derive(Debug)]
pub struct FunctionBody {
    pub span: Option<Span>,
    pub function: Arc<Function>,
    pub body: Arc<Statement>,
    pub scope: Scope,
}

impl PartialEq for FunctionBody {
    fn eq(&self, other: &FunctionBody) -> bool {
        self.function == other.function
    }
}
impl Eq for FunctionBody {}

impl Function {
    pub(crate) fn from_ast(scope: &Scope, value: &leo_ast::Function) -> Result<Function, AsgConvertError> {
        let output: Type = value
            .output
            .as_ref()
            .map(|t| scope.borrow().resolve_ast_type(t))
            .transpose()?
            .unwrap_or_else(|| Type::Tuple(vec![]));
        let mut qualifier = FunctionQualifier::Static;
        let mut has_input = false;

        let mut arguments = vec![];
        {
            for input in value.input.iter() {
                match input {
                    FunctionInput::InputKeyword(_) => {
                        has_input = true;
                    }
                    FunctionInput::SelfKeyword(_) => {
                        qualifier = FunctionQualifier::SelfRef;
                    }
                    FunctionInput::MutSelfKeyword(_) => {
                        qualifier = FunctionQualifier::MutSelfRef;
                    }
                    FunctionInput::Variable(leo_ast::FunctionInputVariable {
                        identifier,
                        mutable,
                        const_,
                        type_,
                        span: _span,
                    }) => {
                        let variable = Arc::new(RefCell::new(crate::InnerVariable {
                            id: Uuid::new_v4(),
                            name: identifier.clone(),
                            type_: scope.borrow().resolve_ast_type(&type_)?.weak(),
                            mutable: *mutable,
                            const_: *const_,
                            declaration: crate::VariableDeclaration::Parameter,
                            references: vec![],
                            assignments: vec![],
                        }));
                        arguments.push(variable.clone());
                    }
                }
            }
        }
        if qualifier != FunctionQualifier::Static && scope.borrow().circuit_self.is_none() {
            return Err(AsgConvertError::invalid_self_in_global(&value.span));
        }
        Ok(Function {
            id: Uuid::new_v4(),
            name: RefCell::new(value.identifier.clone()),
            output: output.into(),
            has_input,
            arguments,
            circuit: RefCell::new(None),
            body: RefCell::new(Weak::new()),
            qualifier,
        })
    }
}

impl FunctionBody {
    pub(super) fn from_ast(
        scope: &Scope,
        value: &leo_ast::Function,
        function: Arc<Function>,
    ) -> Result<FunctionBody, AsgConvertError> {
        let new_scope = InnerScope::make_subscope(scope);
        {
            let mut scope_borrow = new_scope.borrow_mut();
            if function.qualifier != FunctionQualifier::Static {
                let circuit = function.circuit.borrow();
                let self_variable = Arc::new(RefCell::new(crate::InnerVariable {
                    id: Uuid::new_v4(),
                    name: Identifier::new("self".to_string()),
                    type_: WeakType::Circuit(circuit.as_ref().unwrap().clone()),
                    mutable: function.qualifier == FunctionQualifier::MutSelfRef,
                    const_: false,
                    declaration: crate::VariableDeclaration::Parameter,
                    references: vec![],
                    assignments: vec![],
                }));
                scope_borrow.variables.insert("self".to_string(), self_variable);
            }
            scope_borrow.function = Some(function.clone());
            for argument in function.arguments.iter() {
                let name = argument.borrow().name.name.clone();
                scope_borrow.variables.insert(name, argument.clone());
            }
        }
        let main_block = BlockStatement::from_ast(&new_scope, &value.block, None)?;
        let mut director = MonoidalDirector::new(ReturnPathReducer::new());
        if !director.reduce_block(&main_block).0 && !function.output.is_unit() {
            return Err(AsgConvertError::function_missing_return(
                &function.name.borrow().name,
                &value.span,
            ));
        }

        #[allow(clippy::never_loop)] // TODO @Protryon: How should we return multiple errors?
        for (span, error) in director.reducer().errors {
            return Err(AsgConvertError::function_return_validation(
                &function.name.borrow().name,
                &error,
                &span,
            ));
        }

        Ok(FunctionBody {
            span: Some(value.span.clone()),
            function,
            body: Arc::new(Statement::Block(main_block)),
            scope: new_scope,
        })
    }
}

impl Into<leo_ast::Function> for &Function {
    fn into(self) -> leo_ast::Function {
        let (input, body, span) = match self.body.borrow().upgrade() {
            Some(body) => (
                body.function
                    .arguments
                    .iter()
                    .map(|variable| {
                        let variable = variable.borrow();
                        leo_ast::FunctionInput::Variable(leo_ast::FunctionInputVariable {
                            identifier: variable.name.clone(),
                            mutable: variable.mutable,
                            const_: variable.const_,
                            type_: (&variable.type_.clone().strong()).into(),
                            span: Span::default(),
                        })
                    })
                    .collect(),
                match body.body.as_ref() {
                    Statement::Block(block) => block.into(),
                    _ => unimplemented!(),
                },
                body.span.clone().unwrap_or_default(),
            ),
            None => (
                vec![],
                leo_ast::Block {
                    statements: vec![],
                    span: Default::default(),
                },
                Default::default(),
            ),
        };
        let output: Type = self.output.clone().into();
        leo_ast::Function {
            identifier: self.name.borrow().clone(),
            input,
            block: body,
            output: Some((&output).into()),
            span,
        }
    }
}
