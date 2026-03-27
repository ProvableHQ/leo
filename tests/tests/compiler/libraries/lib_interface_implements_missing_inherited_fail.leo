// Test: a program that implements a library interface with a multi-hop inheritance chain
// should fail when it is missing functions inherited across two library hops, and when it
// provides an inherited function with the wrong signature.
//
// Inheritance chain: base_lib::Base <- mid_lib::Mid <- top_lib::Top <- main.aleo
//
// Errors expected:
//   1. `process` is inherited from mid_lib::Mid but implemented with wrong input type (u64 vs u32).
//   2. `fetch` is inherited from base_lib::Base (two hops away) and is missing entirely.
//   3. `reset` is inherited from base_lib::Base (two hops away) and is missing entirely.

// --- library: base_lib --- //

interface Base {
    fn fetch() -> u32;
    fn reset() -> bool;
}

// --- Next Program --- //

// --- library: mid_lib --- //

interface Mid : base_lib::Base {
    fn process(v: u32) -> u32;
}

// --- Next Program --- //

// --- library: top_lib --- //

interface Top : mid_lib::Mid {
    fn apply(v: u32) -> u32;
}

// --- Next Program --- //

// main implements top_lib::Top but:
//   - provides `process` with the wrong input type (u64 instead of u32)
//   - is missing `fetch` (inherited from base_lib::Base via mid_lib::Mid)
//   - is missing `reset` (inherited from base_lib::Base via mid_lib::Mid)
program main.aleo : top_lib::Top {
    fn apply(v: u32) -> u32 {
        return v;
    }

    fn process(v: u64) -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
