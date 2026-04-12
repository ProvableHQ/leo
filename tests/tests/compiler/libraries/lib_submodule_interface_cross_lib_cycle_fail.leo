// Test: a cycle spanning a top-level interface in one library and a submodule
// interface in another library is rejected. Because libraries are processed
// sequentially, the cycle appears as an "interface not found" error when lib_a
// is compiled before lib_b's submodule interfaces are registered.

// --- library: lib_a --- //

interface Top : lib_b::sub::Middle {
    fn top_fn() -> u32;
}

// --- Next Program --- //

// --- library: lib_b --- //

// --- Next Module: sub.leo --- //

interface Middle : lib_a::Top {
    fn middle_fn() -> u32;
}

// --- Next Program --- //

program main.aleo {
    fn run() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
