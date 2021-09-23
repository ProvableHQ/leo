/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

function foo() {}

const foo = 8u8;

function main(y: bool) -> bool {
    return y;
}