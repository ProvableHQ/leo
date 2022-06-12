/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

function int() {}

type int = u32;

function main(y: bool) -> bool {
    return y;
}