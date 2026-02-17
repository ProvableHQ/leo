program test.aleo {
    fn main(bits: [bool; 256]) -> [u8; 32] {
        return Deserialize::from_bits_raw::[[u8; 32]](bits);
    }
}
