/*
namespace: Parse
expectation: Pass
*/

function x(const y: u32) {
    if y < 999u32 {
        x(y+1);
    }
}

function main(y: bool) -> bool {
    x(1u32);
    return y;
}