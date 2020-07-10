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
