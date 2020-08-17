use crate::{Function, Identifier};
use leo_ast::functions::TestFunction as AstTestFunction;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestFunction {
    pub function: Function,
    pub input_file: Option<Identifier>,
}

impl<'ast> From<AstTestFunction<'ast>> for TestFunction {
    fn from(test: AstTestFunction) -> Self {
        TestFunction {
            function: Function::from(test.function),
            input_file: None, // pass custom input file with `@context` annotation
        }
    }
}
