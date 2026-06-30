// A and B inherit from each other, which `CheckInterfaces` must reject.
export interface A: B {
    fn foo() -> u64;
}

export interface B: A {
    fn bar() -> u64;
}
