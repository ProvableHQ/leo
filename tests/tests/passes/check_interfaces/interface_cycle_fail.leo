// Test: Interface inheritance cycle (should fail)

interface A : B {
    fn foo() -> u64;
}

interface B : A {
    fn bar() -> u64;
}

program test.aleo {
    fn main(x: u64) -> u64 {
        return x;
    }
}
