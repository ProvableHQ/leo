#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_for))]
pub struct ForStatement<'ast> {
    pub index: Identifier<'ast>,
    pub start: Expression<'ast>,
    pub stop: Expression<'ast>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}