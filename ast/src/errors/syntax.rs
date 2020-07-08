use crate::ast::Rule;

use pest::error::Error;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("aborting due to syntax error")]
    Error,
}

impl From<Error<Rule>> for SyntaxError {
    fn from(mut error: Error<Rule>) -> Self {
        error = error.renamed_rules(|rule| match *rule {
            Rule::LINE_END => "`;`".to_owned(),
            Rule::type_integer => "`u32`".to_owned(),
            Rule::type_field => "`value.field`".to_owned(),
            Rule::type_group => "`group`".to_owned(),
            Rule::file => "an import, circuit, or function".to_owned(),
            Rule::identifier => "a variable name".to_owned(),
            Rule::type_ => "a type".to_owned(),
            Rule::access => "`.`, `::`, `()`".to_owned(),

            Rule::operation_and => "`&&`".to_owned(),
            Rule::operation_or => "`||`".to_owned(),
            Rule::operation_eq => "`==`".to_owned(),
            Rule::operation_ne => "`!=`".to_owned(),
            Rule::operation_ge => "`>=`".to_owned(),
            Rule::operation_gt => "`>`".to_owned(),
            Rule::operation_le => "`<=`".to_owned(),
            Rule::operation_lt => "`<`".to_owned(),
            Rule::operation_add => "`+`".to_owned(),
            Rule::operation_sub => "`-`".to_owned(),
            Rule::operation_mul => "`*`".to_owned(),
            Rule::operation_div => "`/`".to_owned(),
            Rule::operation_pow => "`**`".to_owned(),

            rule => format!("{:?}", rule),
        });

        log::error!("{}\n", error);

        SyntaxError::Error
    }
}
