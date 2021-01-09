use crate::{ PartialType, Span, Scope, AsgConvertError };

pub trait Node {

    fn span(&self) -> Option<&Span>;
}

pub(super) trait FromAst<T: leo_ast::Node + 'static>: Sized + 'static {
    // expected_type contract: if present, output expression must be of type expected_type.
    // type of an element may NEVER be None unless it is functionally a non-expression. (static call targets, function ref call targets are not expressions)
    fn from_ast(scope: &Scope, value: &T, expected_type: Option<PartialType>) -> Result<Self, AsgConvertError>;
}
