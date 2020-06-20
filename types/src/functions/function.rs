use crate::{FunctionInput, Identifier, Span, Statement, Type};
use leo_ast::functions::Function as AstFunction;

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Function {
    pub function_name: Identifier,
    pub inputs: Vec<FunctionInput>,
    pub returns: Vec<Type>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl<'ast> From<AstFunction<'ast>> for Function {
    fn from(function_definition: AstFunction<'ast>) -> Self {
        let function_name = Identifier::from(function_definition.function_name);
        let parameters = function_definition
            .parameters
            .into_iter()
            .map(|parameter| FunctionInput::from(parameter))
            .collect();
        let returns = function_definition
            .returns
            .into_iter()
            .map(|return_type| Type::from(return_type))
            .collect();
        let statements = function_definition
            .statements
            .into_iter()
            .map(|statement| Statement::from(statement))
            .collect();

        Function {
            function_name,
            inputs: parameters,
            returns,
            statements,
            span: Span::from(function_definition.span),
        }
    }
}

impl Function {
    pub fn get_name(&self) -> String {
        self.function_name.name.clone()
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.function_name)?;
        let parameters = self
            .inputs
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
