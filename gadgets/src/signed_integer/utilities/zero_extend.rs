use snarkos_models::gadgets::utilities::boolean::Boolean;

/// Zero extends an array of bits to the desired length.
/// Least significant bit first
pub trait ZeroExtend
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn zero_extend(&self, zero: Boolean, length: usize) -> Self;
}

impl ZeroExtend for Vec<Boolean> {
    fn zero_extend(&self, zero: Boolean, length: usize) -> Self {
        let bits_needed = length - self.len();
        let mut extension = vec![zero.clone(); bits_needed];

        let mut result = self.clone();
        result.append(&mut extension);

        result
    }
}
