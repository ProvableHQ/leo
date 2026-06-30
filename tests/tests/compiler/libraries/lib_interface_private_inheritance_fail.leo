// Test: a program cannot inherit from a non-`export` interface declared in a library.
// The interface lacks `export`, so it is module-private and may not be used as a parent
// interface across compilation units.

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
