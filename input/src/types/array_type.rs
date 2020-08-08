use crate::{ast::Rule, types::DataType, values::PositiveNumber};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub _type: DataType,
    pub dimensions: Vec<PositiveNumber<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> ArrayType<'ast> {
    pub fn next_dimension(&mut self) -> Option<PositiveNumber<'ast>> {
        self.dimensions.pop()
    }
}

impl<'ast> std::fmt::Display for ArrayType<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._type)?;
        for row in &self.dimensions {
            write!(f, "[{}]", row)?;
        }
        write!(f, "")
    }
}
