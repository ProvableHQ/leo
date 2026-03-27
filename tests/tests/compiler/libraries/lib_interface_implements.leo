// Test: a program implements an interface defined at the top level of a library.

// --- library: ifaces --- //

interface Adder {
    fn compute(a: u32, b: u32) -> u32;
}

// --- Next Program --- //

program main.aleo : ifaces::Adder {
    fn compute(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}
