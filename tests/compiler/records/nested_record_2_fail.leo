/*
namespace: Compile
expectation: Fail
*/


record Token2 {
    owner: address,
    gates: u64,
    foo: (Foo, Foo),
}

record Foo {
    owner: address,
    gates: u64,
    amount: u64,
}
