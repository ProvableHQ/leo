// Test: interface definitions are allowed at the top level of a library.

// --- library: utils --- //

export interface Measurable {
    fn size() -> u32;
}

export fn compute(x: u32) -> u32 {
    return x + 1u32;
}

// --- Next Program --- //

program main.aleo {
    fn run(x: u32) -> u32 {
        return utils::compute(x);
    }

    @noupgrade
    constructor() {}
}
