// Test: a program claims to implement a submodule interface but is missing a required function.

// --- library: ifaces --- //

// --- Next Module: traits.leo --- //

interface Worker {
    fn work() -> u32;
    fn status() -> u32;
}

// --- Next Program --- //

program main.aleo : ifaces::traits::Worker {
    fn work() -> u32 {
        return 1u32;
    }

    // Missing: fn status() -> u32

    @noupgrade
    constructor() {}
}
