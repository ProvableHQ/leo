// Test: Conflicting signatures from two interfaces

export interface IntCounter {
    fn get_value() -> u64;
}

export interface BoolChecker {
    fn get_value() -> bool;
}

program test.aleo: IntCounter + BoolChecker {
    @noupgrade
    constructor() {}

    fn get_value() -> u64 {
        return 0u64;
    }

    fn main(x: u64) -> u64 {
        return 0;
    }
}
