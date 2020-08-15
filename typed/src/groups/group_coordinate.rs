use crate::common::span::Span;
use leo_ast::values::{
    GroupCoordinate as AstGroupCoordinate,
    Inferred as AstInferred,
    NumberValue as AstNumberValue,
    SignHigh as AstSignHigh,
    SignLow as AstSignLow,
};
use leo_input::values::{
    GroupCoordinate as InputGroupCoordinate,
    Inferred as InputInferred,
    NumberValue as InputNumberValue,
    SignHigh as InputSignHigh,
    SignLow as InputSignLow,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupCoordinate {
    Number(String, Span),
    SignHigh,
    SignLow,
    Inferred,
}

impl<'ast> From<AstGroupCoordinate<'ast>> for GroupCoordinate {
    fn from(coordinate: AstGroupCoordinate<'ast>) -> Self {
        match coordinate {
            AstGroupCoordinate::Number(number) => GroupCoordinate::from(number),
            AstGroupCoordinate::SignHigh(sign_high) => GroupCoordinate::from(sign_high),
            AstGroupCoordinate::SignLow(sign_low) => GroupCoordinate::from(sign_low),
            AstGroupCoordinate::Inferred(inferred) => GroupCoordinate::from(inferred),
        }
    }
}

impl<'ast> From<InputGroupCoordinate<'ast>> for GroupCoordinate {
    fn from(coordinate: InputGroupCoordinate<'ast>) -> Self {
        match coordinate {
            InputGroupCoordinate::Number(number) => GroupCoordinate::from(number),
            InputGroupCoordinate::SignHigh(sign_high) => GroupCoordinate::from(sign_high),
            InputGroupCoordinate::SignLow(sign_low) => GroupCoordinate::from(sign_low),
            InputGroupCoordinate::Inferred(inferred) => GroupCoordinate::from(inferred),
        }
    }
}

impl fmt::Display for GroupCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupCoordinate::Number(number, _) => write!(f, "{}", number),
            GroupCoordinate::SignHigh => write!(f, "+"),
            GroupCoordinate::SignLow => write!(f, "-"),
            GroupCoordinate::Inferred => write!(f, "_"),
        }
    }
}

impl<'ast> From<AstNumberValue<'ast>> for GroupCoordinate {
    fn from(number: AstNumberValue<'ast>) -> Self {
        let value = number.to_string();
        let span = Span::from(number.span().clone());

        GroupCoordinate::Number(value, span)
    }
}

impl<'ast> From<AstSignHigh<'ast>> for GroupCoordinate {
    fn from(_sign: AstSignHigh<'ast>) -> Self {
        GroupCoordinate::SignHigh
    }
}

impl<'ast> From<AstSignLow<'ast>> for GroupCoordinate {
    fn from(_sign: AstSignLow<'ast>) -> Self {
        GroupCoordinate::SignLow
    }
}

impl<'ast> From<AstInferred<'ast>> for GroupCoordinate {
    fn from(_sign: AstInferred<'ast>) -> Self {
        GroupCoordinate::Inferred
    }
}

impl<'ast> From<InputNumberValue<'ast>> for GroupCoordinate {
    fn from(number: InputNumberValue<'ast>) -> Self {
        let value = number.to_string();
        let span = Span::from(number.span().clone());

        GroupCoordinate::Number(value, span)
    }
}

impl<'ast> From<InputSignHigh<'ast>> for GroupCoordinate {
    fn from(_sign: InputSignHigh<'ast>) -> Self {
        GroupCoordinate::SignHigh
    }
}

impl<'ast> From<InputSignLow<'ast>> for GroupCoordinate {
    fn from(_sign: InputSignLow<'ast>) -> Self {
        GroupCoordinate::SignLow
    }
}

impl<'ast> From<InputInferred<'ast>> for GroupCoordinate {
    fn from(_sign: InputInferred<'ast>) -> Self {
        GroupCoordinate::Inferred
    }
}
