/*
namespace: Compile
expectation: Fail
*/

record Foo {
    owner: address,
    gates: u64,
    amount: u64,
}

record Token {
    owner: address,
    gates: u64,
    foo: Foo,
}
