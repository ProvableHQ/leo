interface A {
    fn foo() -> u64;
}

interface B: A {
    fn bar() -> u64;
    record Token;
    mapping account: address => u64;
    storage counter: u32;
}

program test.aleo: B {
    fn foo() -> u64 { return 1u64; }
    fn bar() -> u64 { return 2u64; }
    record Token { owner: address }
}
