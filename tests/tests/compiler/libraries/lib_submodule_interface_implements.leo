// Test: a program implements an interface defined in a library submodule.

// --- library: ifaces --- //

// --- Next Module: traits.leo --- //

interface Adder {
    fn compute(a: u32, b: u32) -> u32;
}

// --- Next Program --- //

program main.aleo : ifaces::traits::Adder {
    fn compute(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}
