/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/dummy.in
*/

type int = u32;
type int = u8;

function main(y: bool) -> bool {
    return y;
}