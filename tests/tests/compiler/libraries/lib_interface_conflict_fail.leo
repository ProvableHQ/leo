// Test: a library interface that overrides a parent member with an incompatible
// signature should be rejected.

// --- library: conflict_lib --- //

interface Parent {
    fn compute(x: u32) -> u32;
}

// Child redeclares `compute` with a different signature — conflicts with Parent.
interface Child : conflict_lib::Parent {
    fn compute(x: u64) -> u64;
}

// --- Next Program --- //

program main.aleo {
    fn run() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
