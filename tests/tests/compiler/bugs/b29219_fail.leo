// Regression test for https://github.com/ProvableHQ/leo/issues/29219
// The compiler should emit a diagnostic error, not panic, when Deserialize::from_bits
// is called with an array type parameter whose length exceeds u32::MAX.
program test.aleo {
    fn main(bits: [bool; 8]) -> [u8; 1] {
        return Deserialize::from_bits::[[u8; 4294967300]](bits);
    }
}
