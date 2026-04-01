// Regression test for https://github.com/ProvableHQ/leo/issues/29184
// The compiler should emit a diagnostic error, not panic, when a tuple
// destructuring has fewer identifiers than the tuple has elements, and
// those variables are subsequently used in an expression.
program b29184.aleo {
    fn foo() -> (u8, u8) {
        let (a, b) = (1u8, 2u8, 3u8);
        return (a, b);
    }
}
