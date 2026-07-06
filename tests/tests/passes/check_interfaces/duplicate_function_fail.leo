// Test: an interface declaring the same function name twice must be rejected.
// Regression for ProvableHQ/leo#29560.

interface Foo {
    fn main(public a: u32, b: u32);
    fn main(public a: u32, b: u32) -> u32;
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn main(a: u32, b: u32) -> u32 {
        return a;
    }
}
