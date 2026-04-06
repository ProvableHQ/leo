// Regression test for https://github.com/ProvableHQ/leo/issues/29221
// Using a defined constant as the array length in a Deserialize type parameter
// must compile successfully and not panic or emit a false error.

program test.aleo {
    const N: u32 = 2u32;

    fn main(bits: [bool; 182]) -> [u32; 2] {
        let result: [u32; 2] = Deserialize::from_bits::[[u32; N]](bits);
        return result;
    }
}
