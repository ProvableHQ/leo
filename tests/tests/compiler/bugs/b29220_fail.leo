// Regression test for https://github.com/ProvableHQ/leo/issues/29220
// The compiler should emit a diagnostic error, not panic, when a repeat
// expression's count evaluates to a value that exceeds u32::MAX.

fn repeat_value::[COUNT: u128](val: u32) -> [u32; COUNT * 2] {
    return [val; COUNT * 9223372036854775807];
}

program test.aleo {
    fn main() -> [u32; 6] {
        return repeat_value::[3](42u32);
    }
}
