// Test: a submodule interface inherits from an interface defined in a submodule
// of a different library. A program implements the derived interface.

// --- library: base_lib --- //

// --- Next Module: core.leo --- //

interface Core {
    fn fetch() -> u32;
}

// --- Next Program --- //

// --- library: ext_lib --- //

// --- Next Module: advanced.leo --- //

interface Advanced : base_lib::core::Core {
    fn store(v: u32) -> u32;
}

// --- Next Program --- //

program main.aleo : ext_lib::advanced::Advanced {
    fn fetch() -> u32 {
        return 42u32;
    }

    fn store(v: u32) -> u32 {
        return v + 1u32;
    }

    @noupgrade
    constructor() {}
}
