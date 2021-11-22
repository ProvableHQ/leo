/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

struct Foo {
    x: u8;
}

function Foo() {}

function main(y: bool) -> bool {
    return y;
}