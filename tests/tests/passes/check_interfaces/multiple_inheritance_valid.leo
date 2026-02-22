// Test: Valid program implementing multiple interfaces

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

    fn add(a: u64, b: u64) -> u64 {
        return a + b;
    }

    fn main(x: u64) -> u64 {
        return increment();
    }
}
