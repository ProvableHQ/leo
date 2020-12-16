use leo_asg::*;
use leo_ast::Program as AstProgram;
use leo_grammar::Grammar;
use std::path::Path;

mod pass;
mod fail;

fn load_ast(content: &str) -> Result<AstProgram, AsgConvertError> {
    // Parses the Leo file and constructs a grammar ast.
    let ast = Grammar::new(Path::new("test.leo"), content).map_err(|e| AsgConvertError::InternalError(format!("ast: {:?}", e)))?;

    // Parses the pest ast and constructs a Leo ast.
    Ok(leo_ast::Ast::new("leo_tree", &ast).into_repr())
}

fn load_asg(content: &str) -> Result<Program, AsgConvertError> {
    Program::new(&load_ast(content)?, &NullImportResolver)
}

fn load_asg_imports<T: ImportResolver + 'static>(content: &str, imports: &T) -> Result<Program, AsgConvertError> {
    Program::new(&load_ast(content)?, imports)
}

fn mocked_resolver() -> MockedImportResolver {
    let packages = indexmap::IndexMap::new();
    MockedImportResolver {
        packages,
    }
}