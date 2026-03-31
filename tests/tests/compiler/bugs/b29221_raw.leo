// Regression test for https://github.com/ProvableHQ/leo/issues/29221
// Using a const N as the array length in a Deserialize::from_bits_raw type
// parameter must compile successfully — verifying the fix covers both
// Deserialize variants.

program test.aleo {
    const N: u32 = 2u32;

    fn main(bits: [bool; 64]) -> [u32; 2] {
        let result: [u32; N] = Deserialize::from_bits_raw::[[u32; N]](bits);
        return result;
    }
}
