// Test: Deeply transitive interface inheritance cycle (should fail)

interface A : B {
    fn a() -> u64;
}

interface B : C {
    fn b() -> u64;
}

interface C : D {
    fn c() -> u64;
}

interface D : A {
    fn d() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }
}
