/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

const int = 8u8;

type int = u32;

function main(y: bool) -> bool {
    return y;
}