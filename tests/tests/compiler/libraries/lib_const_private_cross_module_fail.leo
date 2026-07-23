// Test: referencing a non-`export` library const from another compilation unit fails.

// --- library: numbers_lib --- //

const HIDDEN: u32 = 42u32;

// --- Next Program --- //

program main.aleo {
    fn get() -> u32 {
        return numbers_lib::HIDDEN;
    }

    @noupgrade
    constructor() {}
}
