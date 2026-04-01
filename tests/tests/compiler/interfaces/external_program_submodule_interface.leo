// Test: implements a program-level interface whose definition lives inside a
// submodule of an imported external program.
//
// NOTE: Multi-segment interface paths in program headers (e.g.,
// `program foo.aleo: lib.aleo::contracts::Adder`) are not currently supported.
// This test documents the resulting compiler error.

program lib.aleo {
    fn dummy() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: contracts.leo --- //

interface Adder {
    fn add(a: u32, b: u32) -> u32;
    fn zero() -> u32;
}

// --- Next Program --- //

import lib.aleo;

// Expected: compile error — multi-segment interface paths in program headers are unsupported.
program consumer.aleo: lib.aleo::contracts::Adder {
    fn add(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn zero() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
