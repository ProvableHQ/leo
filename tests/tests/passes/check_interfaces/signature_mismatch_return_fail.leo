// Test: Function return type doesn't match interface requirement (should fail)

export interface Converter {
    fn convert(value: u64) -> u32;
}

program test.aleo: Converter {
    @noupgrade
    constructor() {}

    // Wrong return type: u64 instead of u32
    fn convert(value: u64) -> u64 {
        return value;
    }

    fn main(x: u64) -> u64 {
        return convert(x);
    }
}
