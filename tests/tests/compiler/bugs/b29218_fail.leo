// Regression test for https://github.com/ProvableHQ/leo/issues/29218
// The compiler should emit a diagnostic error, not panic, when Deserialize::from_bits
// or Deserialize::from_bits_raw is called with the wrong number of type parameters
// (zero or more than one).
program test.aleo {
    fn from_bits_zero(bits: [bool; 234]) -> [u8; 4] {
        return Deserialize::from_bits(bits);
    }

    fn from_bits_two(bits: [bool; 234]) -> [u8; 4] {
        return Deserialize::from_bits::[[u8; 4], [u16; 2]](bits);
    }

    fn from_bits_raw_zero(bits: [bool; 234]) -> [u8; 4] {
        return Deserialize::from_bits_raw(bits);
    }

    fn from_bits_raw_two(bits: [bool; 234]) -> [u8; 4] {
        return Deserialize::from_bits_raw::[[u8; 4], [u16; 2]](bits);
    }
}
