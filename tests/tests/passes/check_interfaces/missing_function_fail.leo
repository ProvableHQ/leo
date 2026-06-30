// Test: Program missing a required interface function (should fail)

export interface Calculator {
    fn add(a: u64, b: u64) -> u64;
    fn subtract(a: u64, b: u64) -> u64;
}

program test.aleo: Calculator {
    @noupgrade
    constructor() {}

    // Missing subtract function
    fn add(a: u64, b: u64) -> u64 {
        return a + b;
    }

    fn main(x: u64) -> u64 {
        return add(x, 1u64);
    }
}
