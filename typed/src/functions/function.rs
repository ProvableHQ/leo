use crate::{Identifier, InputVariable, Span, Statement, Type};
use leo_ast::functions::Function as AstFunction;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub identifier: Identifier,
    pub input: Vec<InputVariable>,
    pub returns: Vec<Type>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl<'ast> From<AstFunction<'ast>> for Function {
    fn from(function: AstFunction<'ast>) -> Self {
        let function_name = Identifier::from(function.identifier);
        let parameters = function
            .parameters
            .into_iter()
            .map(|parameter| InputVariable::from(parameter))
            .collect();
        let returns = function
            .returns
            .into_iter()
            .map(|return_type| Type::from(return_type))
            .collect();
        let statements = function
            .statements
            .into_iter()
            .map(|statement| Statement::from(statement))
            .collect();

        Function {
            identifier: function_name,
            input: parameters,
            returns,
            statements,
            span: Span::from(function.span),
        }
    }
}

impl Function {
    pub fn get_name(&self) -> String {
        self.identifier.name.clone()
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.identifier)?;
        let parameters = self
            .input
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",");
        let returns = self
            .returns
            .iter()
            .map(|r| format!("{}", r))
            .collect::<Vec<_>>()
            .join(",");
        let statements = self
            .statements
            .iter()
            .map(|s| format!("\t{}\n", s))
            .collect::<Vec<_>>()
            .join("");
        if self.returns.len() == 0 {
            write!(f, "({}) {{\n{}}}", parameters, statements,)
        } else if self.returns.len() == 1 {
            write!(f, "({}) -> {} {{\n{}}}", parameters, returns, statements,)
        } else {
            write!(f, "({}) -> ({}) {{\n{}}}", parameters, returns, statements,)
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
