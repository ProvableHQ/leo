// Test: Function signature doesn't match interface requirement (should fail)

interface Processor {
    fn process(value: u64, multiplier: u64) -> u64;
}

program test.aleo : Processor {
    // Wrong parameter name: 'mult' instead of 'multiplier'
    fn process(value: u64, mult: u64) -> u64 {
        return value * mult;
    }

    fn main(x: u64) -> u64 {
        return process(x, 2u64);
    }
}
