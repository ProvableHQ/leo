use crate::{
    annotations::{AnnotationArguments, AnnotationName, AnnotationSymbol},
    ast::Rule,
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::annotation))]
pub struct Annotation<'ast> {
    pub symbol: AnnotationSymbol<'ast>,
    pub name: AnnotationName<'ast>,
    pub arguments: AnnotationArguments<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
