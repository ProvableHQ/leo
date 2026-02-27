// Test: Cross-program interface inheritance cycle (should fail)

interface A : lib.aleo/B {
    fn a() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }
}

// --- Next Program --- //

import test.aleo;

interface B : test.aleo/A {
    fn b() -> u64;
}

program lib.aleo {
    fn unused() {}
}
