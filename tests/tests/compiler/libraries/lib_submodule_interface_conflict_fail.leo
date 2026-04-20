// Test: conflicting member definitions are detected when a submodule interface
// inherits from another submodule interface.

// --- library: conflict_lib --- //

// --- Next Module: base.leo --- //

interface Base {
    fn compute(x: u32) -> u32;
}

// --- Next Module: derived.leo --- //

interface Derived : conflict_lib::base::Base {
    // Redefines compute with a different return type — should conflict.
    fn compute(x: u32) -> u64;
}

// --- Next Program --- //

program main.aleo {
    fn run() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
