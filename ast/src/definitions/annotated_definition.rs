use crate::{annotations::Annotation, ast::Rule, definitions::Definition, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::definition_annotated))]
pub struct AnnotatedDefinition<'ast> {
    pub annotation: Annotation<'ast>,
    pub definition: Box<Definition<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
