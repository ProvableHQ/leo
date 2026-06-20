// Test: a program cannot inherit from a non-`export` interface declared in a submodule.

program main.aleo : util::Adder {
    fn compute(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: util.leo --- //

interface Adder {
    fn compute(a: u32, b: u32) -> u32;
}
