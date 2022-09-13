/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u8;
}

function Foo() {}

function main(y: bool) -> bool {
    return y;
}
