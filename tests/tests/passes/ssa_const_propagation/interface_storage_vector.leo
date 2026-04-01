// Test: interface storage variable with a vector (unbounded array) type does not panic in
// const propagation.

interface Queue {
    storage items: [address];
}

program test.aleo: Queue {
    storage items: [address];

    fn foo() {}
}
