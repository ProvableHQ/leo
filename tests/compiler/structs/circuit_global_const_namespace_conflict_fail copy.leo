/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

struct Foo {
    x: u8;
}

const Foo = 8u8;

function main(y: bool) -> bool {
    return y;
}