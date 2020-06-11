use crate::InputValue;
use leo_inputs::{types::IntegerType, InputParserError};
use snarkos_models::curves::{Field, PairingEngine};
use std::str::FromStr;

pub struct InputFields<E: PairingEngine>(pub Vec<E::Fr>);

impl<E: PairingEngine> InputFields<E> {
    pub(crate) fn from_boolean(boolean: &bool) -> Self {
        if *boolean {
            Self(vec![E::Fr::one()])
        } else {
            Self(vec![E::Fr::zero()])
        }
    }

    pub(crate) fn from_integer(type_: &IntegerType, integer: &u128) -> Self {
        let bits: usize = match type_ {
            IntegerType::U8Type(_) => 8,
            IntegerType::U16Type(_) => 16,
            IntegerType::U32Type(_) => 32,
            IntegerType::U64Type(_) => 64,
            IntegerType::U128Type(_) => 128,
        };
        let mut fields = vec![];

        for i in 0..bits {
            let boolean = (integer.to_le() >> i) & 1 == 1;
            let mut boolean_fields = InputFields::<E>::from_boolean(&boolean);

            fields.append(&mut boolean_fields.0);
        }

        Self(fields)
    }

    pub(crate) fn from_field(field: &str) -> Result<Self, InputParserError> {
        let field = E::Fr::from_str(field).map_err(|_| InputParserError::ParseFieldError(field.to_string()))?;

        Ok(Self(vec![field]))
    }

    pub(crate) fn from_group(group: &str) -> Result<Self, InputParserError> {
        let s = group.trim();
        // if s.is_empty() {
        //     return Err(());
        // }
        // if s.len() < 3 {
        //     return Err(());
        // }
        // if !(s.starts_with('(') && s.ends_with(')')) {
        //     return Err(());
        // }
        let mut fields = vec![];
        for substr in s.split(|c| c == '(' || c == ')' || c == ',' || c == ' ') {
            if !substr.is_empty() {
                let mut input_fields = InputFields::<E>::from_field(&substr)?;

                fields.append(&mut input_fields.0);
            }
        }

        Ok(Self(fields))
    }

    pub(crate) fn from_array(array: &Vec<InputValue>) -> Result<Self, InputParserError> {
        let mut fields = vec![];

        for input in array.iter() {
            let mut input_fields = input.to_input_fields::<E>()?;

            fields.append(&mut input_fields.0);
        }

        Ok(Self(fields))
    }
}
