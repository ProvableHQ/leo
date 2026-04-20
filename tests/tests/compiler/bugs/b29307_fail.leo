// Regression test for https://github.com/ProvableHQ/leo/issues/29307
// The compiler should emit a diagnostic error, not panic, when a `let` binding
// uses a non-tuple type annotation for a multi-variable destructuring and one of
// the resulting variables is referenced inside a `final { ... }` block.
program test.aleo {
    fn main() -> (u32, Final) {
        let (a, b): u32 = (1u32, 2u32);
        return (a, final { let _ = b; });
    }
}
