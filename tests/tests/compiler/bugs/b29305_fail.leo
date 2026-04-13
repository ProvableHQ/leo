// Regression test for https://github.com/ProvableHQ/leo/issues/29305
// The compiler should emit a type error, not panic, when a multi-identifier
// definition's RHS is not a tuple type.
program test.aleo {
    fn main() -> u32 {
        let (a, b) = 42u32;
        return a;
    }
}
