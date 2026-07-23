// Test: Cross-program interface inheritance cycle (should fail)

export interface A: lib.aleo::B {
    fn a() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import test.aleo;

export interface B: test.aleo::A {
    fn b() -> u64;
}

program lib.aleo {
    fn unused() {}

    @noupgrade
    constructor() {}
}
