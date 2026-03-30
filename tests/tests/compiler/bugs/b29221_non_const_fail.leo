// Regression test for https://github.com/ProvableHQ/leo/issues/29221
// Using a runtime (non-const) variable as the array length in a Deserialize
// type parameter must fail with a clear error, not a panic.

program test.aleo {
    fn main(bits: [bool; 182]) -> [u32; 2] {
        let n: u32 = 2u32;
        let result: [u32; 2] = Deserialize::from_bits::[[u32; n]](bits);
        return result;
    }
}
