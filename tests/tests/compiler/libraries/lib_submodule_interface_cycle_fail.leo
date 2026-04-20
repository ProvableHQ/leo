// Test: a cycle between interfaces in different submodules of the same library is rejected.

// --- library: cyclic --- //

// --- Next Module: mod_a.leo --- //

interface A : cyclic::mod_b::B {
    fn a_fn() -> u32;
}

// --- Next Module: mod_b.leo --- //

interface B : cyclic::mod_a::A {
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
