/*
namespace: Compile
expectation: Fail
*/

type int = u32;
type int = u8;

function main(y: bool) -> bool {
    return y;
}