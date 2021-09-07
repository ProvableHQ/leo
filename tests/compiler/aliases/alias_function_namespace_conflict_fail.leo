/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

type int = u32;

function int() {}

function main(y: bool) -> bool {
    return y;
}