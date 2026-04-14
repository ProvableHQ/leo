// Test: Function signature doesn't match interface requirement (should fail)

interface Processor {
    fn process(value: u64, multiplier: u64) -> u64;
}

program test.aleo: Processor {
    // Wrong parameter type: `u32` instead of `u64`
    fn process(value: u64, multiplier: u32) -> u64 {
        return value;
    }

    fn main(x: u64) -> u64 {
        return process(x, 2u32);
    }
}
