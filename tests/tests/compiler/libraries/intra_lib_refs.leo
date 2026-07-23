// Test: consts within a library that reference earlier consts in the same library.
// Verifies that Library.consts ordering is preserved and intra-library references resolve.

// --- library: math_lib --- //

export const BASE: u32 = 10u32;
export const DOUBLE: u32 = BASE + BASE;
export const TRIPLE: u32 = DOUBLE + BASE;

// --- Next Program --- //

program intra_lib.aleo {
    // BASE=10, DOUBLE=20, TRIPLE=30
    fn compute() -> u32 {
        return math_lib::TRIPLE;
    }

    @noupgrade
    constructor() {}
}
