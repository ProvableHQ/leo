#[macro_use]
extern crate thiserror;

pub mod node;
pub use node::*;

pub mod type_;
pub use type_::*;

pub mod program;
pub use program::*;

pub mod expression;
pub use expression::*;

pub mod statement;
pub use statement::*;

pub mod variable;
pub use variable::*;

pub mod scope;
pub use scope::*;

pub mod error;
pub use error::*;

pub mod import;
pub use import::*;

pub mod const_value;
pub use const_value::*;

mod input;
pub use input::*;

pub mod prelude;
pub use prelude::*;

pub mod reducer;
pub use reducer::*;

pub mod checks;
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

pub fn load_asg_from_ast<T: ImportResolver + 'static>(content: leo_ast::Program, resolver: &mut T) -> Result<Program, AsgConvertError> {
    InnerProgram::new(&content, resolver)
}

pub fn load_asg<T: ImportResolver + 'static>(content: &str, resolver: &mut T) -> Result<Program, AsgConvertError> {
    InnerProgram::new(&load_ast("input.leo", content)?, resolver)
}