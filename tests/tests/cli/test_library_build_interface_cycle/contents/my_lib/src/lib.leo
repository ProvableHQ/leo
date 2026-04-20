// A and B inherit from each other, which `CheckInterfaces` must reject.
interface A: B {
    fn foo() -> u64;
}

interface B: A {
    fn bar() -> u64;
}
