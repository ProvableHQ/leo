// Test: Missing function from second interface

interface Counter {
    fn increment() -> u64;
}

interface Calculator {
    fn add(a: u64, b: u64) -> u64;
}

program test.aleo : Counter + Calculator {
    fn increment() -> u64 {
        return 1u64;
    }
    // Missing add function from Calculator

    fn main(x: u64) -> u64 {
        return 0;
    }
}
