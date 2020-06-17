use snarkos_errors::gadgets::SynthesisError;

use leo_inputs::SyntaxError as InputSyntaxError;
use leo_types::InputValue;
use pest::error::{Error, ErrorVariant};
use std::str::ParseBoolError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),

    #[error("{}", _0)]
    SyntaxError(#[from] InputSyntaxError),
}

impl<'ast> From<InputValue<'ast>> for BooleanError {
    fn from(value: InputValue<'ast>) -> Self {
        let error = Error::new_from_span(
            ErrorVariant::CustomError {
                message: format!("expected boolean input, actual {}", value.to_string()),
            },
            value.span().to_owned(),
        );

        BooleanError::SyntaxError(InputSyntaxError::from(error))
    }
}
