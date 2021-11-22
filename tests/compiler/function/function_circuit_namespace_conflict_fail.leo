/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

function Foo() {}

struct Foo {
    x: u8;
}

function main(y: bool) -> bool {
    return y;
}