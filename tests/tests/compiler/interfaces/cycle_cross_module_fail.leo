// Test: Cross-module interface inheritance cycle (should fail)

export interface A: mod_b::B {
    fn a() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: mod_b.leo --- //

export interface B: C {
    fn b() -> u64;
}

export interface C: B {
    fn b() -> u64;
}
