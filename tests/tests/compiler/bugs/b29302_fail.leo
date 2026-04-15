// Regression test for https://github.com/ProvableHQ/leo/issues/29302
// The compiler should emit a diagnostic error, not panic, when an invalid
// method call appears as a const type argument to a struct in an interface.
struct S::[N: u32] {
    x: u32,
}

interface Foo {
    fn main(x: S::[2u32.exp(2u32)]);
}

program bad_op.aleo {
    fn main() {}
}
