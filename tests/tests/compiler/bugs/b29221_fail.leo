// Regression test for https://github.com/ProvableHQ/leo/issues/29221
// The compiler should emit a diagnostic error, not panic, when Deserialize::from_bits
// is called with an array type parameter whose length is an unresolved identifier.
program test.aleo {
    fn main(bits: [bool; 182]) -> [u32; 2] {
        let result: [u32; 2] = Deserialize::from_bits::[[u32; N]](bits);
        return result;
    }
}
