use snarkos_models::gadgets::utilities::boolean::Boolean;

use std::fmt::Debug;

/// Implements the base struct for a signed integer gadget
macro_rules! int_impl {
    ($name: ident, $_type: ty, $size: expr) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub bits: Vec<Boolean>,
            pub value: Option<$_type>,
        }

        impl $name {
            pub fn constant(value: $_type) -> Self {
                let mut bits = Vec::with_capacity($size);

                let mut tmp = value;

                for _ in 0..$size {
                    // If last bit is one, push one.
                    if tmp & 1 == 1 {
                        bits.push(Boolean::constant(true))
                    } else {
                        bits.push(Boolean::constant(false))
                    }

                    tmp >>= 1;
                }

                Self {
                    bits,
                    value: Some(value),
                }
            }
        }
    };
}

int_impl!(Int8, i8, 8);
int_impl!(Int16, i16, 16);
int_impl!(Int32, i32, 32);
int_impl!(Int64, i64, 64);
int_impl!(Int128, i128, 128);
