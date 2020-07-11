use snarkos_models::gadgets::utilities::boolean::Boolean;

/// Sign extends an array of bits to the desired length.
/// Least significant bit first
pub trait SignExtend
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn sign_extend(bits: &[Self], length: usize) -> Vec<Self>;
}

impl SignExtend for Boolean {
    fn sign_extend(bits: &[Boolean], length: usize) -> Vec<Self> {
        let msb = bits.last().expect("empty bit list");
        let bits_needed = length - bits.len();
        let mut extension = vec![msb.clone(); bits_needed];

        let mut result = Vec::from(bits);
        result.append(&mut extension);

        result
    }
}
