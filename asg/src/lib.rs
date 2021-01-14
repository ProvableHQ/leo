#[macro_use]
extern crate thiserror;

mod node;
pub use node::*;

mod type_;
pub use type_::*;

mod program;
pub use program::*;

mod expression;
pub use expression::*;

mod statement;
pub use statement::*;

mod variable;
pub use variable::*;

mod scope;
pub use scope::*;

mod error;
pub use error::*;

mod import;
pub use import::*;

mod const_value;
pub use const_value::*;

mod input;
pub use input::*;

mod prelude;
pub use prelude::*;

mod reducer;
pub use reducer::*;

mod checks;
pub use checks::*;

pub use leo_ast::{ Span, Identifier };

use std::path::Path;

pub fn load_ast<T: AsRef<Path>, Y: AsRef<str>>(path: T, content: Y) -> Result<leo_ast::Program, AsgConvertError> {
    // Parses the Leo file and constructs a grammar ast.
    let ast = leo_grammar::Grammar::new(path.as_ref().clone(), content.as_ref())
        .map_err(|e| AsgConvertError::InternalError(format!("ast: {:?}", e)))?;

    // Parses the pest ast and constructs a Leo ast.
    Ok(leo_ast::Ast::new("load_ast", &ast).into_repr())
}

pub fn load_asg_from_ast<T: ImportResolver + 'static>(content: leo_ast::Program, resolver: &T) -> Result<Program, AsgConvertError> {
    InnerProgram::new(&content, resolver)
}

pub fn load_asg<T: ImportResolver + 'static>(content: &str, resolver: &T) -> Result<Program, AsgConvertError> {
    InnerProgram::new(&load_ast("input.leo", content)?, resolver)
}