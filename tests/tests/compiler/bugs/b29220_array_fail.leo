// Regression test for https://github.com/ProvableHQ/leo/issues/29220
// The compiler should emit a diagnostic error, not panic, when an array type's
// length evaluates to a value that exceeds u32::MAX.

program test.aleo {
    const COUNT: u128 = 2u128;

    fn main(public a: u32, b: u32) -> u32 {
        let x: [u32; COUNT * 9223372036854775807] = [0u32; COUNT * 9223372036854775807];
        return x[0u32];
    }
}
