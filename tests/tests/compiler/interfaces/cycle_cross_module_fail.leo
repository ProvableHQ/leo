// Test: Cross-module interface inheritance cycle (should fail)

interface A : mod_b::B {
    fn a() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }
}

// --- Next Module: mod_b.leo --- //

interface B : C {
    fn b() -> u64;
}

interface C : B {
    fn b() -> u64;
}
