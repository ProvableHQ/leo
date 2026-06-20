// Test: Simple interface inheritance cycle (should fail)

export interface A: B {
    fn foo() -> u64;
}

export interface B: A {
    fn bar() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }

    @noupgrade
    constructor() {}
}
