// Test: Program correctly implements an interface

interface Counter {
    fn increment(amount: u64) -> u64;
}

program test.aleo : Counter {
    fn increment(amount: u64) -> u64 {
        return amount + 1u64;
    }

    fn main(x: u64) -> u64 {
        return increment(x);
    }
}
