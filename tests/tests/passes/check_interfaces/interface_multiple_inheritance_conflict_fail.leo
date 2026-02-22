// Test: Interface inheriting conflicting members

interface A {
    fn process() -> u64;
}

interface B {
    fn process() -> u32;
}

interface C : A + B {}

program test.aleo {}
