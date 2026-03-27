// Test: interfaces in a library that form a cycle should be rejected.

// --- library: cyclic_lib --- //

interface A : cyclic_lib::B {
    fn a_fn() -> u32;
}

interface B : cyclic_lib::A {
    fn b_fn() -> u32;
}

// --- Next Program --- //

program main.aleo {
    fn run() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
