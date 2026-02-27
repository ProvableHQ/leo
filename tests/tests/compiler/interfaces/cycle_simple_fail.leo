// Test: Simple interface inheritance cycle (should fail)

interface A : B {
    fn foo() -> u64;
}

interface B : A {
    fn bar() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }
}
