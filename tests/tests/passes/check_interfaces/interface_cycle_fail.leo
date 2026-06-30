// Test: Interface inheritance cycle (should fail)

export interface A: B {
    fn foo() -> u64;
}

export interface B: A {
    fn bar() -> u64;
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn main(x: u64) -> u64 {
        return x;
    }
}
