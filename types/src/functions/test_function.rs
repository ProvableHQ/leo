use crate::Function;
use leo_ast::functions::TestFunction as AstTestFunction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestFunction(pub Function);

impl<'ast> From<AstTestFunction<'ast>> for TestFunction {
    fn from(test: AstTestFunction) -> Self {
        TestFunction(Function::from(test.function))
    }
}
