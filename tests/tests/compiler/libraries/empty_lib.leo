// Test: a library with zero consts. Verifies that iteration over an empty
// consts vec doesn't panic and that the consuming program compiles normally.

// --- library: empty_lib --- //

// (intentionally empty)

// --- Next Program --- //

program empty_consumer.aleo {
    fn main() -> u32 {
        return 42u32;
    }

    @noupgrade
    constructor() {}
}
