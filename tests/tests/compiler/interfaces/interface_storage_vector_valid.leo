// Test: interface storage variable with a vector (unbounded array) type compiles end-to-end.

interface Queue {
    storage items: [address];
}

program test.aleo: Queue {
    storage items: [address];

    fn foo() {}
}
