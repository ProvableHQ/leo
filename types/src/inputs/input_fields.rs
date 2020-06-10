use leo_inputs::{types::IntegerType, InputParserError};
use snarkos_models::curves::{Field, PairingEngine};

pub struct InputFields<E: PairingEngine>(pub Vec<E::Fr>);

impl<E: PairingEngine> InputFields<E> {
    pub(crate) fn from_boolean(boolean: &bool) -> Self {
        if *boolean {
            Self(vec![E::Fr::one()])
        } else {
            Self(vec![E::Fr::zero()])
        }
    }

    pub(crate) fn from_integer(type_: &IntegerType, integer: &u128) -> Result<Self, InputParserError> {
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

        Ok(Self(fields))
    }
}
